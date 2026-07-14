# Reviewer findings: no-wrap-convention (Q-22), ROUND 3

Reviewer: fresh independent reviewer (sonnet), round-3 verification lens. Artifact classified RISKY (two consecutive clean rounds required). This is the second independent clean-round confirmation. The artifact has not changed since round 2. Per instructions: settled findings are not re-raised without new evidence; the round-1 reflow integrity is trusted as settled. Line length / wrapping is never a finding.

## Verdict

CLEAN. No new finding at any severity.

## Findings by severity

- critical: none.
- high: none.
- medium: none.
- low: none.

## What I verified

### 1. Convention statements - coherence and non-contradiction

Three sources carry the no-wrap convention:

- `pack/AGENTS.md` line 84 ("Prose formatting" paragraph, Workflow section, just before Principles): "Prose in Markdown and in comments is not hard-wrapped: write natural lines and let the editor or renderer soft-wrap them. Do not insert manual line breaks to hit a column width, and do not reflow existing prose to a width. Line length is never a review finding, so reviewers and triagers do not raise or act on it. Where the project's formatter owns Markdown wrapping (for example prettier), let it own wrapping rather than wrapping by hand."
- `pack/prompts/reviewer.md` line 11: "Line length and prose line-wrapping are never findings: the project does not hard-wrap prose and a formatter owns wrapping, so do not raise or comment on them."
- `pack/prompts/triager.md` line 5 (mid-paragraph): "A finding about line length or prose line-wrapping is never valid: the project does not hard-wrap prose and a formatter owns wrapping, so dismiss any such finding."

These three are mutually consistent and non-contradictory. The AGENTS.md paragraph is the master statement; reviewer.md and triager.md carry role-appropriate restatements (reviewer will not raise; triager will dismiss). The "never a finding / never valid" phrasing is identical between the two role prompts in the relevant sense. No statement contradicts another. The generated copies (`.agents/prompts/reviewer.md`, `.agents/prompts/triager.md`, root `AGENTS.md`) are byte-identical to their pack sources (confirmed by the scaffold-self no-op below).

The "Prose formatting" paragraph sits at the end of the Workflow section, immediately before the Principles heading, which is a logical location for a cross-cutting house rule. It does not contradict any Principle or any other Workflow subsection.

### 2. .prettierrc.json - correctness and pack isolation

- `.prettierrc.json` at the repo root contains `{ "proseWrap": "never" }`. This is the correct Prettier option to disable hard-wrapping in Markdown.
- The file is NOT under `pack/`.
- A `grep` of `pack/pack.toml` and `src/manifest.rs` for `prettierrc` returned no matches. The file is not listed as a shipped asset, which is correct: it is a dev-environment config that belongs to this repo only, not to the scaffolded output of consumer projects.

### 3. scaffold-self recipe - coherence and accuracy

`justfile` lines 36-45. The recipe runs two commands: the raw render (`cargo run ... --write --force --principles default`), then `nix fmt`. The comment reads: "The raw render is not formatter-clean (prettier owns Markdown wrapping via proseWrap=never), so run the repo-wide formatter afterwards to normalise the generated files. `nix fmt` formats the whole tree, not just the generated files; that is intentional and harmless because treefmt is idempotent (a no-op on files already clean), and it leaves the repo at a stable committed fixed point."

This comment is accurate:

- The raw render produces Markdown that may not be formatter-clean (prettier is not part of the render pipeline itself).
- `nix fmt` runs treefmt over the whole tree, which invokes prettier on Markdown files (whole-tree scope is confirmed by the run output: "traversed 52 files, emitted 46 files for processing").
- The comment's "intentional and harmless because treefmt is idempotent" matches the F1 fix rationale recorded in the ledger (scoping the fmt to generated files only would duplicate the manifest's dest-path list, against Principle 1; treefmt's idempotency makes whole-tree formatting safe).
- The comment does not mislead a reader into thinking only generated files are formatted (the earlier misleading reading that F1 caught is gone).

### 4. Stability check

Ran: `direnv exec . just scaffold-self && git status --short`

Output: scaffold-self ran (14 files written via the render step, 0 changed by nix fmt), then git status produced no output (empty - no working-tree changes). The tree is a stable committed fixed point as claimed.

Ran: `direnv exec . nix fmt`

Output: `formatted 0 files (0 changed) in 21ms`. Standalone nix fmt is also a no-op. Zero changes.

Both checks pass cleanly.

### 5. Settled findings not re-raised

- F1 (misleading justfile comment): fixed in `027aeba`, confirmed landed by round-2 reviewer and by this round's direct read. No new evidence its fix was wrong.
- F2 (style: prefix on a partly-behavioural commit): accepted as residual risk in round 1. No new evidence overturns this; the commit is not re-openable without rewriting published history.
- The round-1 reflow integrity (29 files, +554/-2530) was reviewed clean by the opus reviewer and is trusted per the task instructions. No new evidence contradicts it.
