# Adversarial / correctness review: AGENTS.md whole-file drift guard

Lens: try to BREAK the guard; assume a false-negative (guard passes while real drift
exists) until proven otherwise.

Target: `src/agents_md_drift.rs` (new `#[cfg(test)]` module) + one `mod` line in
`src/main.rs`. Diff reviewed: `git diff main HEAD` in worktree
`.claude/worktrees/dg-rev-adversarial`.

`cargo test` result: `test result: ok. 343 passed; 0 failed; 0 ignored` for the lib
target; the two new tests pass:
`agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... ok` and
`agents_md_drift::tests::normalization_tolerates_wrapping_but_not_content_change ... ok`.
Nothing else broke.

## Summary

The core of the safety argument is sound: `normalize_wrapping` never deletes, adds, or
reorders a non-whitespace character, so any drift that changes the ordered non-whitespace
token stream or the blank-line block-boundary structure IS caught. I confirmed the catch
direction on the real committed file (a one-character heading edit makes the guard FAIL
loudly), confirmed config fidelity against the `scaffold-self` justfile recipe, and
confirmed a missing asset panics rather than silently skipping. Controls (word move,
heading-level change) are caught.

There is ONE real false-negative class: the documented safety claim is stated too
absolutely. The transform strips leading indentation and collapses all non-fenced
whitespace, so any drift expressible purely as a change in leading indentation or
intra-token whitespace of a NON-fenced construct is masked. I demonstrated this on the
actual guarded file. It is latent today (both files are completely flat: no nested lists,
no 4-space indented code, no multi-space inline code), so there is no active miss, but the
guard would not catch such a drift if the guidance ever gains an indentation-significant
construct, which is exactly the "whole-file safety net" job it claims.

Findings: 1 (medium).

---

## F1 (medium) - Leading-indentation and intra-token whitespace collisions mask real structural drift; the doc-comment safety claim is overstated

Location: `src/agents_md_drift.rs`, `normalize_wrapping` (lines 154-210), `is_hard_start`
(lines 85-113), and the safety argument in the doc comment (lines 140-153, especially
"identical ordered stream of non-whitespace characters, the identical block-boundary
structure ... Any real drift ... changes that token stream").

Problem: `normalize_wrapping` protects only FENCED code verbatim. For every other line it
applies `raw_line.trim()` then `split_whitespace().join(" ")`, and `is_hard_start` is
evaluated on the already-trimmed line. This discards two kinds of whitespace that ARE
content in Markdown, not prettier proseWrap reflow:

1. Leading indentation that sets list nesting depth (and list-item continuation
   attachment). A nested sub-item and a top-level sibling normalize identically.
2. Whitespace inside a 4-space indented code block (the implementer's flagged gap) and
   inside an inline code span (`` `a  b` `` vs `` `a b` ``).

The doc comment's argument that "two inputs normalize equal only when they carry the
identical ordered stream of non-whitespace characters, the identical block-boundary
structure ... Any real drift ... changes that token stream" is therefore false for these
constructs: the non-whitespace token stream and the block-boundary count are identical
while the rendered meaning differs. The comment only acknowledges the fenced-vs-indented
code distinction; it does not acknowledge the list-nesting and inline-code cases, and it
asserts the property absolutely.

Concrete breaking inputs (all verified with a standalone harness running the exact copied
`normalize_wrapping`/`is_hard_start`):

- Nested-vs-sibling list item: `"- parent\n  - child\n"` and `"- parent\n- child\n"` both
  normalize to `"- parent\n- child"`. COLLISION.
- Two-level nest flattened: `"- a\n  - b\n    - c\n"` and `"- a\n- b\n- c\n"` both
  normalize to `"- a\n- b\n- c"`. COLLISION.
- List-item continuation de-indented out of its item:
  `"- item one\n  detail line\n- item two\n"` and
  `"- item one\ndetail line\n- item two\n"` both normalize to
  `"- item one detail line\n- item two"`. COLLISION.
- Indented (4-space) code block reflow (implementer's known gap, confirmed):
  `"text\n\n    foo   bar\n    baz\n\nafter\n"` and `"text\n\n    foo bar baz\n\nafter\n"`
  both normalize to `"text\n\nfoo bar baz\n\nafter"`. COLLISION.
- Inline code span internal spacing: `"use `a  b` here\n"` and `"use `a b` here\n"` both
  normalize to `` "use `a b` here" ``. COLLISION.

Demonstrated on the real guarded file (not just the harness): I turned the top-level list
item `- Planner ...` (line 20 of `AGENTS.md`) into a nested child of the preceding
`- Orchestrator ...` item by prefixing two spaces (`  - Planner ...`). This is a genuine
semantic change to the role list that prettier preserves rather than reflows. The guard
test `the_committed_scaffold_matches_a_fresh_render` still reported
`test result: ok. 1 passed`. The change was then reverted; the worktree is clean apart
from this findings file. For contrast, a one-character edit to the `# Agent guidance`
heading made the same test FAIL as it must, confirming the guard is otherwise live.

How real / does it block: Latent, not active. Both `AGENTS.md` and
`.agents/AGENTS.reference.md` are currently 151 lines with zero leading indentation (no
nested lists, no indented code blocks, no fences at all) and no multi-space inline code, so
today nothing is masked. The gap activates only if the generated guidance later contains an
indentation-significant construct AND a drift (hand edit or a pack-source change not
regenerated) alters only that indentation. Given that bullet lists are already the dominant
structure (22 top-level items) and nested lists are a natural next edit, this is a plausible
future miss for the exact "hand edit outside the pinned fragments" case the guard is meant
to catch. I rate it medium: demonstrated and it undercuts the load-bearing safety claim, but
with no current instance and a narrow masked class.

Suggested fix (pick one, in rough order of preference):

1. Make leading indentation significant for structural lines: preserve (or bucket to a
   canonical depth) the indentation of list items and their continuations rather than
   `trim()`-ing it away, so nesting depth is part of the compared form. This closes the
   list cases directly.
2. Treat 4-space indented code blocks as verbatim, the same as fenced blocks, closing the
   indented-code gap the implementer already flagged.
3. Cheapest stopgap that keeps the guard honest to its claim: add an assertion that the
   guarded render/committed files contain no indentation-significant construct (no line
   with leading whitespace that is a list marker or code indent, no inline code span with a
   double space), i.e. guard the guard's own precondition. Then the "only whitespace
   differs" claim is enforced rather than assumed, and the day the guidance gains a nested
   list the drift guard fails and forces a real fix instead of silently degrading.
4. At minimum, correct the doc comment (lines 140-153): narrow the absolute claim to state
   that it holds only because the guarded files contain no indentation-significant or
   inline-code-whitespace construct, and that list nesting depth, indented code, and inline
   code internal spacing are NOT protected. As written the comment would mislead a future
   reader into trusting the guard for cases it does not cover.

---

## Attacks that FAILED (evidence of soundness)

- Word move across a block boundary (`"alpha beta\n\ngamma"` vs `"alpha\n\nbeta gamma"`):
  distinct. Caught.
- Heading-level change (`## Title` vs `### Title`): distinct. Caught.
- Dropped word / dropped list item / merged block boundary: covered by the module's own
  `normalization_tolerates_wrapping_but_not_content_change` test and independently
  reconfirmed; all caught.
- Real one-character hand edit of committed `AGENTS.md`: guard FAILS loudly. Catch
  direction is live.
- Config fidelity: `self_scaffold_asset` uses `manifest::builtin()`, `resolve_selection(_,
  "default")`, `Detail::Summary`, empty overrides `HashMap::new()`, `instrument=true`, and
  `&[]` modules, which matches the `scaffold-self` recipe
  (`scaffold --principles default --instrument`; `--principle-detail` defaults to
  `Detail::Summary`; no `--var`, no `--module`, no `--template`). No divergence. Not a
  finding.
- Missing-asset handling: `find(...).unwrap_or_else(|| panic!(...))` fails loudly if a dest
  is absent; it does not silently skip. Both dests (`AGENTS.md` and
  `.agents/AGENTS.reference.md`) are selected by exact `asset.dest` match. Not a finding.
- Real-file exercise: the guard `include_str!`s `../AGENTS.md` and
  `../.agents/AGENTS.reference.md` (repo root relative to `src/`), so it compares the
  shipped files against a fresh render. A stale pack source (pack edited, files not
  regenerated) changes the render side of the comparison and is caught by the same token-
  stream mechanism as a hand edit of the committed side; both directions verified. Not a
  finding.
- Non-whitespace token-stream collision hunt (`is_hard_start` misclassification turning a
  space into a newline to compensate for a token difference): could not construct one; the
  transform preserves whitespace presence/absence between consecutive non-whitespace chars
  (collapsed to a single space) and block boundaries (collapsed to `\n\n`), so a differing
  token stream always produces a differing output string. The only masked drifts are the
  pure-whitespace-of-a-significant-construct cases in F1.
