# Step 92, final review pass: fix verification and re-seed detection

Reviewed at detached `1b9d4cf` ("docs: narrow drift-guard complement claim to asset copies") in worktree `.claude/worktrees/final-a-pdg`. Lens: (1) did the authorised fix land exactly as prescribed and nothing more, and (2) did it re-seed a new defect.

## Findings: ZERO

No findings at any severity. Explicitly: no `critical`, no `high`, no `medium`, no `low`.

This is a deliberate statement, not an omission. Every check below was run, each returned the expected result, and I found nothing to raise. I did not re-audit the settled items listed in my brief, and I have no new evidence against any of them.

## Verification, with evidence

### 1. Scope of the diff: exactly as prescribed

| Claim | Command | Result |
| --- | --- | --- |
| One file touched | `git diff --name-status HEAD~1 HEAD` | `M src/agents_md_drift.rs`, nothing else. |
| One line changed | `git diff --numstat HEAD~1 HEAD` | `1 1 src/agents_md_drift.rs`. |
| One hunk | `git diff -U0 HEAD~1 HEAD \| grep -c '^@@'` | `1`. |
| Exactly one word added, nothing removed | `git diff --word-diff=porcelain HEAD~1 HEAD` | the only marked token is `+asset`. There is no corresponding `-` token. |
| No second region of the file touched | the single hunk header is `@@ -59,7 +59,7 @@` | one region, the COMPLEMENT paragraph. |
| No added explanatory prose | same word-diff | the change is one word inside an existing sentence. No new sentence, clause, parenthetical, or comment line was authored anywhere in the file, and in particular nothing about `docs/plans/TEMPLATE.md`, its prettier exclusion, or its generation path, which the triager forbade at `docs/plans/agent-scaffold.reviews/prompt-drift-guard-verify-triage.md:82`. |
| Working tree clean at the reviewed tip | `git status --porcelain` | empty. |

The prescription is recorded at `docs/plans/agent-scaffold.reviews/prompt-drift-guard-verify-triage.md:68` ("At `src/agents_md_drift.rs:62`, change `the Markdown copies` to `the Markdown asset copies`. One word, one line, one site, no reflow, no restructure."). The landed line is `src/agents_md_drift.rs:62`, read at that line in this worktree:

```
//! uncovered is a scope call whose cost is uneven: widening to the Markdown asset copies is a
```

That is a byte-for-byte match for the prescription. The commit message carries the rationale, which is where rationale belongs and is not "prose authored in the comment block".

### 2. Single-site: no other occurrence left behind

`grep -rn "Markdown copies" .` (excluding `.git/`) returns no hit under `src/`. The remaining hits are in `docs/plans/agent-scaffold.ledger.md` and five files under `docs/plans/agent-scaffold.reviews/`, all of which are historical records of earlier rounds quoting the pre-fix text. Quoting the old text is what a record is for, so these are not stale docs and need no update.

`grep -rn "Markdown asset copies" .` returns exactly one hit in `src/`: `src/agents_md_drift.rs:62`.

### 3. The resulting sentence is grammatical and reads naturally

The sentence, `src/agents_md_drift.rs:61-64`:

> Leaving them uncovered is a scope call whose cost is uneven: widening to the Markdown asset copies is a small change to check 3's filter, since they are prose under the same prettier settings and already satisfy the precondition, while the TOML copies need a comparison of their own [...]

Grammatical: "widening to the Markdown asset copies" is a well-formed gerund phrase, and the following "they" binds unambiguously to "the Markdown asset copies", the nearest plural. The Markdown / TOML contrast the sentence is built on survives the edit intact.

Reads naturally in context: "asset" is already established block vocabulary at `src/agents_md_drift.rs:45` ("For each rendered asset whose `dest` starts with `PROMPT_DEST_PREFIX`") and `src/agents_md_drift.rs:73` ("by deleting an asset row from `pack/pack.toml`"), and "copies" is the paragraph's own word for a committed copy, used at `:59-60` for the `.agents/user-prompts/` copies and the `.toml` copies. The compound is built entirely from words the reader has already met three lines earlier, so it does not read as a term dropped in to patch a hole.

### 4. The fix actually fixes the defect it was authorised for

RD-V1 was that the claim is false of `docs/plans/TEMPLATE.md`. The narrowing works only if that file is genuinely not a Markdown *asset* copy. Confirmed two ways:

- `grep -n 'dest = ' pack/pack.toml` has no row whose `dest` is `docs/plans/TEMPLATE.md`. The `docs/plans/TEMPLATE.` rows are `TEMPLATE.plan.toml` and nine `.md` sidecars plus two `.gitkeep` files; the bare `TEMPLATE.md` is the render composed from them, not an asset.
- `flake.nix:53` lists `"docs/plans/TEMPLATE.md"` in `settings.global.excludes`, so it is also outside "the same prettier settings", independently of the asset question.

### 5. Re-seed check: the NEW claim is true of everything it now covers

The new sentence makes three sub-claims about "the Markdown asset copies": they are prose under the same prettier settings, they already satisfy the precondition, and widening check 3's filter reaches them. I verified all three independently of the triager rather than taking the measurement on trust.

The set is 16 files (`grep -c` over `pack/pack.toml`): 9 Markdown rows under `docs/plans/TEMPLATE.`, 6 under `.agents/user-prompts/`, and `.agents/LEDGER.template.md`. None is under `.agents/prompts/`, so all 16 are genuinely in the complement rather than already guarded by check 3.

- **Precondition.** I reimplemented the body of `assert_no_unprotected_construct` (`src/agents_md_drift.rs:197-222`: fence toggling on a `trim_start`-ed `` ``` `` or `~~~`, in-fence lines skipped, every other line required to equal `line.split_whitespace().collect::<Vec<_>>().join(" ")`) as a standalone script and ran it over the 16 committed files. Result: `16 ok, 0 FAIL, checked 16 files`. Re-runnable script kept at `/tmp/claude-1000/-home-jessea-Documents-cv/b1add4df-96ab-4436-ada1-5f3542063be1/scratchpad/precheck_rd.py`. The same script rejects `docs/plans/TEMPLATE.md`, which is the correct discrimination and reproduces the original RD-V1 evidence.
- **Prettier settings.** The exclude list is exactly the three entries at `flake.nix:51-53`, and the prettier `includes` at `flake.nix:68-73` is `*.md`, `*.yml`, `*.yaml`, `*.json`. None of the 16 matches any exclude, and all 16 are `.md`, so all 16 are under the same prettier settings.
- **Filter reachability.** Each of the 16 has exactly one `[[asset]]` row in `pack/pack.toml`, so a widened `dest` filter over `self_scaffold_assets()` does reach every one of them.

So the narrowed sentence is true of all 16, and no member of the narrowed set falsifies any of its three sub-claims. The fix did not trade one false claim for another.

I also checked the one shape a narrowing edit could plausibly re-seed: leaving a member of the paragraph's own four illustrations stranded outside both halves of its Markdown / TOML split. `docs/plans/TEMPLATE.md` is now in neither half. This is not a defect, because the paragraph declares itself a rule and not an inventory at `src/agents_md_drift.rs:57-59` ("A rule rather than an inventory, because an inventory carries an obligation to stay complete that prose reliably fails"), and the illustrations "illustrate the rule and do not bound it" (`:61`). Saying nothing about one file is silence, not a false claim, and the alternative (a sentence explaining the omission) is exactly the explanatory prose the triager forbade and the human declined.

### 6. ASCII-only

`grep -nP '[^\x00-\x7F]' src/agents_md_drift.rs` returns no match (exit 1). No em-dash, en-dash, emoji, unicode arrow, or unicode symbol anywhere in the file, including the changed line. `grep -n ' $' src/agents_md_drift.rs` returns nothing, so no trailing whitespace was introduced.

### 7. Tests

- `cargo test agents_md_drift`: **5 passed, 0 failed.** The five are `the_committed_scaffold_matches_a_fresh_render`, `the_committed_role_prompts_match_a_fresh_render`, `normalization_tolerates_wrapping_but_not_content_change`, `precondition_rejects_non_space_whitespace_and_round_one_cases`, and `precondition_exempts_fenced_indented_lines_but_not_bare_ones`.
- `cargo test` (full suite): **379 passed, 0 failed, 0 ignored** across all six binaries (367 + 5 + 1 + 3 + 1 + 2). The known `checks::tests` worktree-naming flake (step 93) did **not** fire on this run, so there is nothing to attribute to it.
- `cargo clippy --all-targets`: clean, no warnings emitted.

## Not findings, recorded for the triager

- `docs/plans/agent-scaffold.reviews/prompt-drift-guard-verify-triage.md:74` says "the 7 at `.agents/user-prompts/*.md`". The correct count is 6 (`grep -c 'dest = "\.agents/user-prompts/.*\.md"' pack/pack.toml` returns `6`, and the directory holds `kickoff`, `explore`, `review`, `pause`, `compaction-prep`, `resume`). The triage document's total of 16 is nonetheless correct (9 + 6 + 1), as are all three of its sub-claims, which I verified independently above. This is an arithmetic slip in an archived review record, it changes no conclusion, and it is outside the change under review, so I am recording it rather than raising it.
- The fixed line at `:62` now runs longer than its neighbours. Per the standing ruling, line length and prose wrapping are never findings here, and reflowing it was deliberately declined.

## Verdict

1. **Did the authorised fix land exactly as prescribed, and nothing more?** Yes. One file, one hunk, one line, one word added, no prose, no reflow, no second site, clean tree.
2. **Did it re-seed a new defect?** No. The narrowed claim is true of all 16 files it now covers, verified on all three sub-claims independently; the file is ASCII-only; and the full suite and clippy are green.
