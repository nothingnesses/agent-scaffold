# Drift guard R3 adversarial review (correctness lens)

Target: `src/agents_md_drift.rs` at HEAD (`ef3b80d`), consolidated fix diffed against
`e7f18e1`. Guard re-renders the pack and compares to committed `AGENTS.md` +
`.agents/AGENTS.reference.md` after `normalize_wrapping`, gated by the precondition
`assert_no_unprotected_construct`.

## Verdict: ZERO FINDINGS

I tried hard to find a residual false negative (a real content difference that
`normalize_wrapping` collapses to equal yet the canonical-form precondition accepts on
every non-fence line). I could not construct one. The consolidated fix is sound. R2-F1
and R2-F2 are both resolved. `cargo test` is green.

## R2 regressions: both resolved (reproduced)

Ran the actual guard functions (standalone copies) against the R2 cases:

- R2-F1 (non-space whitespace inside a non-fence line collapsed by `split_whitespace`
  but missed by the old `contains("  ")` check). Now REJECTED for every whitespace kind
  `split_whitespace` recognizes: mid-line tab (`\t`), NBSP (`U+00A0`), form feed
  (`U+000C`), vertical tab (`U+000B`), and bare CR (`\r`, which `str::lines()` does not
  treat as a line break so it stays in-line and trips the canonical check). All five
  rejected.
- R2-F2 (indented content inside a fence over-rejected). Now ACCEPTED: a 4-space
  indented example line and a tab-indented line inside a ``` fence both pass, while the
  SAME indented line OUTSIDE a fence is still rejected. Confirmed.

The two in-repo regression tests (`precondition_rejects_non_space_whitespace_and_round_one_cases`,
`precondition_exempts_fenced_indented_lines_but_not_bare_ones`) encode these and pass.

## Fence symmetry: provably identical, so the seam the brief worried about is closed

The brief's central concern was a line that `normalize_wrapping` treats as prose
(collapsible) while the precondition treats as fenced (skipped), which would be an
unfenced masking hole. This cannot happen: both functions use byte-for-byte the same
fence rule and the same iteration.

- Both iterate `str::lines()` from `in_fence = false`.
- Both compute `line.trim_start()` and toggle on `starts_with("```") || starts_with("~~~")`.
- In both, the fence-marker test runs BEFORE the `in_fence` body test, so a marker line
  inside a fence closes it identically in each.
- Neither tracks which marker opened the fence, so both simplify CommonMark the same way
  (a ``` fence can be "closed" by ~~~, leading spaces on a marker still toggle, an
  info-string still toggles, an odd marker count leaves an unterminated fence). Because
  the SIMPLIFICATION is shared, the two never disagree on any line's fence membership.

Since fence membership is identical per line, the dangerous direction (normalize
collapses a line the precondition skipped) is impossible. Edge cases exercised and all
caught or safely rejected: `~~~` "closing" a ``` fence with a double-space drift after it
(rejected on the prose side, so never reaches the equality check); indented fence marker
(`   ```` emitted verbatim by normalize, so an indentation difference in the marker is
preserved and caught); unterminated fence with differing content (verbatim -> caught);
odd-marker region with differing content (caught); adjacent fences and parity-shifting
extra fence pairs (caught); differing leading-space count inside a fence (verbatim ->
caught).

Note the "skipped but verbatim" asymmetry is safe in the harmless direction only:
verbatim is the STRONGEST preservation, so any real difference inside a fence still shows
in the normalized output. The masking risk only exists for COLLAPSED (prose) lines, and
those are exactly the lines the precondition checks for canonical form.

## Residual false-negative hunt: none found

Soundness argument, then evidence.

Under the precondition, every non-fence line already equals
`line.split_whitespace().collect::<Vec<_>>().join(" ")`, so it has no leading/trailing
whitespace and no internal whitespace run beyond one ASCII space. On such lines
`normalize_wrapping`'s per-line `trim` + `split_whitespace().join(" ")` is a no-op. The
only transforms normalize then applies are cross-line: (a) join a block's soft
(non-hard-start) continuation lines with a single space, (b) collapse a run of blank
lines to one boundary, (c) drop trailing boundaries. All three are precisely prettier's
`proseWrap=never` freedoms. None deletes, adds, or reorders a non-whitespace character;
none removes an internal blank-line boundary; hard-start line breaks are preserved (a
hard-start line flushes and starts a fresh output line). Fenced lines pass verbatim. So
two accepted inputs normalize equal only when they carry the identical ordered
non-whitespace token stream, the identical internal blank-boundary structure, the
identical hard-start line breaks, and byte-identical fence lines: identical content and
block structure.

The per-line precondition cannot itself see cross-line collapses (soft-wrap joins,
blank-run length, trailing boundary), which the brief flagged. That is fine: those three
are the ONLY things normalize erases, and each is a genuine reflow freedom, not content.
Real drift changes the token stream (reworded/added/dropped/reordered token), the
internal blank-boundary count (paragraph merge/split), or a fenced line; all three
survive normalize and are caught.

Evidence:

1. Hand-built adversarial pairs (24 cases): every pair with a differing non-whitespace
   token stream or differing internal blank-boundary structure was caught (dropped word,
   changed heading, reorder, paragraph merge, heading tight-vs-loose, list-item vs inline
   `- `, table row join, blockquote join, fence-content diffs, fence-marker indent diff,
   fenced leading-space diff, parity-shift tail, desync-indent tail). Every pair that
   normalized equal had an IDENTICAL token stream and differed only in reflow (soft wrap,
   blank-run length, join-vs-single line), i.e. correctly tolerated.

2. Brute force over all canonical docs up to 3 lines from the alphabet
   `{"", a, b, "# h", "- x", "> q", "a b", c}` (prose path) and all 4-line docs from
   `{"```", a, b, ""}` (fence path): for every pair that is precondition-accepted on both
   sides AND normalizes equal, the ordered non-whitespace token stream is identical. Zero
   violations in either sweep. This is the load-bearing necessary condition (normalize
   never merges or drops a non-whitespace token on accepted input), and it holds
   exhaustively over the sampled space, including across fence boundaries where fenced
   tokens survive verbatim.

## One observation (not a finding, out of the guard's operating envelope)

`normalize_wrapping` joins a heading with a following non-blank, non-hard-start line when
there is NO blank line between them: `"# H\ntext"` and `"# H text"` both normalize to
`"# H text"`. This is a pre-existing `is_hard_start` join imprecision, applied
symmetrically to both sides, and NOT introduced by the R3 fix. It cannot mask drift for
this guard because (a) the token streams are identical anyway, and (b) both compared
inputs are prettier `proseWrap=never` output (the committed file literally, the render
byte-identically per the module doc), and prettier always inserts a blank line after an
ATX heading, so the no-blank form is unreachable as either operand. If a real drift
dropped that blank line, the committed side would still carry the blank-line boundary and
the pair would differ, so it is caught. Recording it only for completeness; no action
needed.

## Attacks tried (soundness enumeration)

- Non-space whitespace inside a non-fence line: tab, NBSP, form feed, vertical tab, bare
  CR. All rejected.
- Fenced indented / tabbed content vs bare indented content. Fenced accepted, bare
  rejected.
- Fence detection asymmetry: `~~~` vs ``` mix, info-string marker, leading-space marker,
  trailing-content marker, unterminated fence, odd marker count, adjacent fences,
  nested/parity-shifted fences. No disagreement between the two functions; all diffs
  caught or safely rejected.
- Cross-line collapses: soft-wrap join, blank-run collapse, trailing-boundary drop,
  paragraph merge/split, tight-vs-loose list/heading. Reflow tolerated; structural drift
  caught.
- Trailing/leading whitespace, whitespace-only lines, hard-break trailing spaces. All
  rejected by the canonical check before reaching equality.
- Token-stream preservation brute force over prose and fence doc spaces: 0 false
  negatives.

## cargo test

`cargo test agents_md_drift` -> `test result: ok. 4 passed; 0 failed; 0 ignored`
(`the_committed_scaffold_matches_a_fresh_render`,
`normalization_tolerates_wrapping_but_not_content_change`,
`precondition_rejects_non_space_whitespace_and_round_one_cases`,
`precondition_exempts_fenced_indented_lines_but_not_bare_ones`). The committed files
contain no fenced blocks today, so the precondition currently guards flat content.
