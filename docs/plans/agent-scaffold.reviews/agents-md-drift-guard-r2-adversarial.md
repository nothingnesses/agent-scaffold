# Drift-guard review, round 2, adversarial / correctness lens

Target: `src/agents_md_drift.rs` (the whole-file AGENTS.md drift guard).
Reviewed change: `git diff cba4fcc HEAD -- src/agents_md_drift.rs`.
Scope: verify the round-1 blocking fix (F1: absolute "cannot mask a content
change" doc claim over a `normalize_wrapping` that actually masks nested lists,
indented code, and multi-space inline code), then hunt for a new hole in the
fix itself, especially the `assert_no_unprotected_construct` precondition.

## Round-1 defect: RESOLVED

The doc claim is now accurate. The module header (line 21) and the
`normalize_wrapping` doc (lines 171-208) drop the absolute guarantee, state the
precondition explicitly (no indentation-significant construct, no run of two or
more spaces), and enumerate the three unprotected constructs (a/b/c). The masking
cases from round 1 are now CAUGHT by the new precondition assertion rather than
by `normalize_wrapping` (which was intentionally not hardened). Reproduced against
a byte-exact replica of both functions:

- nested `  - child` line -> `assert_no_unprotected_construct` returns
  `Err("line 2 leading-indent")` (guard FAILS loudly). Confirmed.
- multi-space inline span `use ` + backtick + `a  b` + backtick -> returns
  `Err("line 1 double-space")` (guard FAILS loudly). Confirmed.

The precondition runs on BOTH committed files AND BOTH fresh renders before the
equality checks (lines 285-288), so a pack edit that introduces such a construct
is caught on the render side even before scaffold-self is run. It does NOT
false-trip on today's real content: the committed `AGENTS.md` and
`.agents/AGENTS.reference.md` contain no leading-indent line, no `"  "` run, and
no tab (verified by grep), and the full `the_committed_scaffold_matches_a_fresh_render`
test passes. Ordinary drift (dropped word, dropped list item, merged block,
prettier reflow tolerance) is still exercised by
`normalization_tolerates_wrapping_but_not_content_change`, which passes.

## Findings

Two new holes in the fix. Both concern a MISMATCH between the set of per-line
whitespace transforms `normalize_wrapping` performs and the set the precondition
fences. `normalize_wrapping` collapses each non-fence line via
`trimmed.split_whitespace().collect::<Vec<_>>().join(" ")` (line 245), which
trims all leading/trailing whitespace and collapses EVERY run of ANY Unicode
whitespace to a single ASCII space. The precondition only fences two narrow
patterns: a leading space/tab, and a literal `"  "` (two ASCII spaces). The
fence set is therefore strictly narrower than the collapse set along one axis
(F1) and strictly wider along another (F2).

### F1 -- precondition misses non-space whitespace that normalize collapses (mid-line tab, NBSP, form feed) inside inline code

- Severity: MEDIUM (masking gap; incomplete/inconsistent with the round-1 fix's
  own remedy for multi-space inline code; trigger is exotic and the doc's
  formally-worded guarantee technically survives, see note).
- Location: `assert_no_unprotected_construct`, the `line.contains("  ")` check
  (line 96-99); root cause is the `split_whitespace()` collapse in
  `normalize_wrapping` (line 245).
- Problem: `split_whitespace()` splits on every `char::is_whitespace()`
  codepoint, so a SINGLE mid-line tab, a non-breaking space (U+00A0), a form
  feed, or a vertical tab between two tokens is collapsed to one ASCII space,
  exactly as a double-space run is. Inside an inline code span that difference is
  significant content (the same reasoning that made round-1's multi-space inline
  code a valid finding: whitespace inside code is content, not reflow). The
  precondition catches the double-SPACE form of this construct but not the
  tab / NBSP / form-feed forms, which it neither rejects nor mentions. Concrete
  breaking inputs (byte-exact replica; both pass the precondition AND normalize
  equal, so the guard would pass on masked drift):
  - `use ` + backtick + `a` + TAB + `b` + backtick + ` now`  vs
    `use ` + backtick + `a b` + backtick + ` now` : precondition `Ok` on both,
    `normalize` equal `true`.
  - `use ` + backtick + `a` + U+00A0 + `b` + backtick + ` now`  vs the
    single-ASCII-space version : precondition `Ok` on both, `normalize` equal
    `true`.
  So a future pack edit that renders an inline code span containing a tab or NBSP,
  while the committed copy has a plain space (or vice versa), is real drift that
  `normalize_wrapping` erases and the precondition fails to fence. This is the
  same defect class the round-1 fix set out to close (whitespace-significant
  inline construct), left half-closed: spaces fenced, other whitespace not.
- Note on the formal doc wording: the narrowed guarantee promises "identical
  non-whitespace content and block structure" (line 189-191). A tab-vs-space
  difference is a whitespace difference, so a pedantic reading says the promised
  guarantee still holds. But by that same reading the round-1 multi-space-inline
  finding would also have been excused, and it was not: the project's adopted
  standard treats whitespace inside inline code as significant. F1 is the
  inconsistency between that standard and the fence that was actually built.
- Fix: make the precondition match `normalize_wrapping`'s per-line collapse
  exactly, which closes both the double-space case and every non-space-whitespace
  case in one predicate. Replace the two `assert!`s with a single check that each
  non-fence line already equals its own canonical single-ASCII-space form and has
  no leading/trailing whitespace, e.g. assert
  `line == line.split_whitespace().collect::<Vec<_>>().join(" ")`. Any tab, NBSP,
  form feed, leading/trailing whitespace, or multi-whitespace run then fails the
  precondition, precisely fencing what `normalize_wrapping` would otherwise
  collapse. (Update the doc's precondition wording from "no run of two or more
  spaces" to "no whitespace other than single ASCII inter-word spaces".)

### F2 -- precondition does not track fenced code blocks, so it false-trips on safe fenced content and misdirects the fix

- Severity: MEDIUM (false-positive that will realistically fire on the first
  indented fenced code example added to guidance; fails safe, i.e. loud not
  masking, but blocks legitimate content and gives a misleading remediation).
- Location: `assert_no_unprotected_construct` (lines 86-101); it iterates
  `content.lines()` with no fence-state tracking, unlike `normalize_wrapping`
  (lines 214-231) which does track `in_fence` and passes fenced lines VERBATIM.
- Problem: lines inside a ``` or ~~~ fenced code block are compared byte-for-byte
  by `normalize_wrapping` (verbatim on both sides), so they CANNOT mask drift, no
  matter how they are indented or spaced. The precondition nonetheless asserts on
  them. An ordinary fenced code example with indentation (near-universal in agent
  guidance: shell, Rust, YAML, JSON snippets) therefore trips the leading-indent
  assertion. Concrete breaking input (byte-exact replica):
  `text\n\n` + fence + `rust\nfn main() {\n    println!("x");\n}\n` + fence + `\n`
  -> precondition returns `Err("line 5 leading-indent")` on the `    println!`
  line, so the guard FAILS. The failure message tells the author to
  "Harden normalize_wrapping (make list indentation significant, treat indented
  code verbatim)", which is misleading: `normalize_wrapping` ALREADY treats
  fenced code verbatim, so nothing needs hardening and the content is already
  safe. A legitimate, masking-safe pack edit is blocked with a wrong instruction.
- Fix: give `assert_no_unprotected_construct` the same fence tracking
  `normalize_wrapping` uses, and skip lines while inside a fence (a line whose
  `trim_start()` starts with ``` or ~~~ toggles `in_fence`; skip when `in_fence`).
  This restricts the precondition to exactly the prose region where
  `normalize_wrapping` actually collapses whitespace, matching the two functions'
  domains.

## Attacks that did NOT break it (soundness evidence)

- Nested list via single leading space, and via leading tab: both caught by the
  leading-space / leading-tab check.
- 4-space indented code block (outside a fence): caught by leading-space check.
- Multi-space inline code span: caught by the `"  "` check.
- Hard line break via two trailing spaces: caught by the `"  "` check.
- Hard line break via trailing backslash: NOT masked; the `\` is non-whitespace,
  survives normalization, so `foo\` + newline + `bar` -> `foo\ bar` differs from
  the soft-wrap `foo bar`.
- Loose vs tight list (blank line between items): NOT masked; the blank line is
  recorded as a block boundary, so the two normalize differently.
- Merged / split paragraph (block-boundary change): NOT masked (existing test
  `merged_blocks` asserts this).
- Single trailing space or single trailing tab with no other change: normalize
  trims it, but it is not markdown-significant content, so no real drift is
  masked (not a finding).
- Ordinary drift (dropped/added/reordered word, list item, slot; changed heading;
  stale pack source): caught by the equality asserts on the real render (byte
  identical today) and by the constructed-pair test; prettier reflow still
  tolerated.

## Verdict

Round-1 F1 is RESOLVED. Two new MEDIUM holes remain in the fix: F1, the
whitespace fence is incomplete for non-space whitespace (tab / NBSP / form feed)
inside inline code, matching the exact class the round-1 fix targeted; and F2,
the precondition over-reaches into fenced code blocks and will false-trip on
common indented code examples with a misleading remediation. Both have a single
clean fix (match the precondition's per-line predicate to `normalize_wrapping`'s
own collapse, and share its fence tracking).

Test status: `cargo test` -> 351 passed; 0 failed (344 + 1 + 3 + 1 + 2 across
the five suites); the drift-guard tests are green.
