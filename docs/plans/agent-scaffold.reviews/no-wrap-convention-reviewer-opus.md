# Reviewer findings: no-wrap-convention (REFLOW INTEGRITY lens)

Reviewer: opus. Lens: reflow integrity, did the one-time repo-wide `nix fmt` (prettier `proseWrap: never`) silently change meaning or break markdown structure, especially in shipped `pack/` prompts.

Step range: `30dc348..b4d8b24`. Bulk-reflow commit: `dc5f69b`. Convention-add commit: `4fa5e74`. Blockquote-safety reword: `81aa4bd`.

## Verdict

The reflow is clean. It changed wrapping only; no prose meaning was altered and no markdown structure was broken. Evidence below.

## Technique note

`git diff -w` (--ignore-all-space) does NOT hide a prose reflow, because joining hard-wrapped lines changes line boundaries, and `-w` only ignores whitespace differences within a line, not the collapse of newlines into spaces. The correct tool is `git diff --word-diff`, which tokenises on whitespace so a pure reflow (only inter-word whitespace/newlines change) shows zero token changes; any added/removed/changed token is a real content change. I used word-diff for text, plus structural-marker counts for indentation-only (list-nesting) changes that word-diff is blind to.

## Findings by severity

- critical: none.
- high: none.
- medium: none.
- low: none.

No issues found at any severity.

## Evidence

1. Pack sources, word-level diff `30dc348..dc5f69b` over all 12 pack markdown files. Only three token additions appear, all intentional convention text:
   - `pack/AGENTS.md`: the new "Prose formatting." paragraph (line 84).
   - `pack/prompts/reviewer.md`: the "Line length and prose line-wrapping are never findings..." sentence.
   - `pack/prompts/triager.md`: the "A finding about line length or prose line-wrapping is never valid..." sentence. Every other pack file (`plan-template.md`, `orchestrator.md`, `planner.md`, `implementer.md`, `clarifying-questions.md`, `open-questions-gate.md`, `kickoff.md`, `resume.md`, `compaction-prep.md`) shows zero word-level changes. So no shipped prose was reworded, repunctuated, or dropped by the reflow.

2. Other markdown (`README.md`, `CHANGELOG.md`, `docs/plans/TEMPLATE.md`): zero word-level changes.

3. Ledger (`docs/plans/agent-scaffold.ledger.md`): only two real changes, both benign:
   - A round-summary table separator row: `| ------- | ... |` normalised to `| --- | --- | ... |` by prettier. Column count preserved (see point 6). Semantically identical.
   - The `agent-isolation` row reworded `container ... > worktree > file-safety fallback` to `container ..., then worktree, then file-safety fallback` (commit `81aa4bd`). This was done deliberately so the reflow would not read a wrapped `> worktree` at line start as a markdown blockquote. Meaning preserved (ordered preference list unchanged), and it is a working file, not shipped.

4. Structural-marker counts (ATX headings, code fences, `>` blockquote lines, `-`/`*` list markers, ordered-list markers, `|` table rows) are identical before and after the reflow in all 15 markdown files checked. No heading level changed, no code fence was opened/closed, no blockquote was created or destroyed, no list item was added or lost.

5. No file in the pack (or README/TEMPLATE) contains any nested (indented) list item, before or after. So there is no list re-nesting or flattening risk; all lists are single-level. Confirmed by comparing the set of list-item indentation widths before vs after (empty in both).

6. Ledger round-summary table integrity: 6 pipe-rows with 7 columns each (8 pipe-delimited fields) both before and after. No table row was broken across lines.

7. Generated-copy consistency (`just scaffold-self` output vs `pack/` sources), checked in the working tree at the step tip:
   - All seven `.agents/prompts/*.md` match their `pack/prompts/*.md` sources byte-for-byte.
   - All three `.agents/user-prompts/*.md` match their `pack/user-prompts/*.md` sources.
   - `docs/plans/TEMPLATE.md` matches `pack/plan-template.md`.
   - `.agents/AGENTS.reference.md` differs from `pack/AGENTS.md` only by the `{{principles}}` placeholder being expanded to the 22 numbered principles, exactly as expected. Root `AGENTS.md` is identical to `.agents/AGENTS.reference.md`. So the intentional convention additions (the "Prose formatting" paragraph and the two line-length sentences) propagated correctly into every generated copy, and no copy drifted from its source.

## Scope note

Per the task, line length and wrapping are not treated as findings; they are the intended change. This review only checked whether the mechanical reflow preserved meaning and structure. It did.
