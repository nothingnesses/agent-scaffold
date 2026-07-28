# Step 92 `prompt-drift-guard`: authorised-fix verification round

Reviewer lens: SCOPED FIX VERIFICATION AND RE-SEED DETECTION. Not a general re-review. Two questions only: did the authorised fix land exactly as prescribed and nothing more, and did it re-seed a new defect.

Artifact: commit `3e4fb6c`, `docs: drop overclaiming self-extension clause in drift-guard coverage`, the fix for finding `RD4-1`.
Worktree: `.claude/worktrees/verify-a-pdg`, detached at `3e4fb6c`. `git status --porcelain` was empty on entry and is empty at the end apart from this findings file. I ran no mutations: every claim below is settled by a diff, a citation, or a test run, and the reviewer contract says not to manufacture a test where a command already settles the point.

## Verdict

**Q1, did the fix land exactly as prescribed and nothing more: YES.**
**Q2, did the fix re-seed a new defect: NO.**

**FINDINGS: ZERO.** No `critical`, no `high`, no `medium`, no `low`. Two candidates were considered and rejected on evidence; they are recorded below as non-findings so the triager does not re-derive them.

## What was authorised, checked clause by clause

The escalation record (`docs/plans/agent-scaffold.ledger.md:369`) prescribes: at `src/agents_md_drift.rs:50-53`, delete the clause after the colon in "which is what makes it self-extending: an `[[asset]]` row added to `pack/pack.toml` whose `dest` falls under the prefix is guarded with no edit here", terminate the sentence at "self-extending", change `:` to `.`, ZERO new words, one file, one hunk.

| Prescribed constraint | Verdict | Evidence |
| --- | --- | --- |
| One file | Yes | `git show --numstat 3e4fb6c` -> `2 3 src/agents_md_drift.rs`, a single row |
| One hunk | Yes | `git show 3e4fb6c -- src/agents_md_drift.rs \| grep -c '^@@'` -> `1` (the hunk is `@@ -48,9 +48,8 @@`) |
| Only the target region of `src/agents_md_drift.rs` | Yes | See "byte-unmoved" below |
| Clause deleted, sentence ends at "self-extending" | Yes | `src/agents_md_drift.rs:51` now reads `//! and not a hand-written list, which is what makes it self-extending. Checks 1 and 2 embed` |
| `:` changed to `.` | Yes | Same line; the only surviving colon in the paragraph is the unrelated one at `:54` ("what the self-extension costs:") |
| ZERO new words | Yes | Token census below |
| Nothing else changed | Yes | Working tree clean, no other commit on top |

### ZERO new words, mechanically

I stripped the `//!` and `//` prefixes from every line of both revisions, split on whitespace, and diffed the token multisets:

```
git show 3e4fb6c^:src/agents_md_drift.rs > before.rs   # then compare against the worktree file
ADDED tokens (after - before):   [('self-extending.', 1)]
REMOVED tokens (before - after): [('`[[asset]]`',1), ('`dest`',1), ('`pack/pack.toml`',1), ('added',1),
                                  ('an',1), ('edit',1), ('falls',1), ('guarded',1), ('here.',1), ('is',1),
                                  ('no',1), ('prefix',1), ('row',1), ('self-extending:',1), ('the',1),
                                  ('to',1), ('under',1), ('whose',1), ('with',1)]
total tokens: before 4006, after 3988
```

The single "added" token is `self-extending.` displacing `self-extending:`, which IS the authorised punctuation change and adds no word. The 18 removed tokens are exactly the deleted clause. Net `-18` tokens. No new word entered the file, and no word was reworded: the remaining tokens are a strict superset relationship with nothing substituted.

### Byte-unmoved outside the edited paragraph

The hunk spans the doc-comment paragraph at `:48-55`. Everything from the end of that paragraph to EOF is byte-identical across the commit:

```
tail -n +56 before.rs > b_tail.rs; tail -n +55 after.rs > a_tail.rs; diff -q b_tail.rs a_tail.rs
-> IDENTICAL
```

(The offsets differ by one because the paragraph lost a line.) So no second region of `src/agents_md_drift.rs` was touched, and no code line was touched at all. This is a comment-only change, as claimed.

## Re-seed checks

### The resulting sentence is grammatical and reads naturally

`src/agents_md_drift.rs:50-55`, read live in this worktree:

> Check 3 is a filter over a rendered set, not a directory listing and not a hand-written list, which is what makes it self-extending. Checks 1 and 2 embed their committed side with `include_str!` and check 3 cannot, since that macro needs a literal path, so it reads the working tree at test time. That is what the self-extension costs: less hermetic than a compile-time snapshot, which is acceptable for a repo-local guard whose purpose is to inspect the working tree.

Grammatical, one complete sentence, and the "self-extension" back-reference at `:54` still has its antecedent at `:51`.

### The claim that remains is TRUE as stated

"Check 3 is a filter over a rendered set, not a directory listing and not a hand-written list":

- `src/agents_md_drift.rs:426-429` is `self_scaffold_assets().into_iter().filter(|asset| asset.dest.starts_with(PROMPT_DEST_PREFIX)).collect()`. The subject of the filter is the RENDERED asset set returned by `self_scaffold_assets` (`:134-141`), not a `read_dir` and not a literal array. No `read_dir` or hard-coded prompt list exists anywhere in the test module.
- "Checks 1 and 2 embed their committed side with `include_str!`": `src/agents_md_drift.rs:116` and `:120`.
- "check 3 cannot, since that macro needs a literal path, so it reads the working tree at test time": `committed_asset` at `:158-166` does `Path::new(env!("CARGO_MANIFEST_DIR")).join(dest)` then `std::fs::read_to_string`, a runtime read.

Every surviving clause of the sentence is backed by code at the cited lines. All line numbers in this file were read at those lines in this worktree.

### The COVERAGE block is still internally consistent

The deleted clause was the ONLY place that quantified over `[[asset]]` ROWS rather than RENDERED assets, and it was the only site contradicting R1. After the deletion:

- Numbered item 3 (`:45-46`) quantifies over "each rendered asset whose `dest` starts with `PROMPT_DEST_PREFIX`", correct.
- R1 (`:70-83`) says a module-tagged row is not rendered and therefore not guarded, and names `.agents/prompts/checks-reviewer.md` as the standing benign instance (`:78-81`), correct.
- The `PROMPT_DEST_PREFIX` doc (`:122-125`) says "the rendered asset set", correct.
- `self_scaffold_assets`' doc (`:132-133`) says the absent `--module` selection is load-bearing for what check 3 covers and points at R1, correct.
- `committed_asset`'s doc (`:156-157`) routes the reverse case to R1, correct.

There is no longer any site claiming a manifest row under the prefix is guarded. `grep -rn "no edit here\|falls under" src/ .agents/` returns nothing, and `grep -rn "self-extend" src/` returns exactly one line, `:51`. No comment elsewhere cited the deleted text, so nothing dangles.

### ASCII-only

`grep -nP '[^\x00-\x7F]' src/agents_md_drift.rs` -> no matches (exit 1). `file src/agents_md_drift.rs` -> `ASCII text`. No em-dash, en-dash, emoji, unicode arrow or symbol. `grep -nP '[ \t]+$'` -> no trailing whitespace; no doubled spaces in `:48-56`.

### Tests

```
cargo test --bins agents_md_drift
  agents_md_drift::tests::precondition_exempts_fenced_indented_lines_but_not_bare_ones ... ok
  agents_md_drift::tests::precondition_rejects_non_space_whitespace_and_round_one_cases ... ok
  agents_md_drift::tests::normalization_tolerates_wrapping_but_not_content_change ... ok
  agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render ... ok
  agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... ok
test result: ok. 5 passed; 0 failed
```

(`cargo test agents_md_drift` alone also passes; `--lib` errors because the crate has no library target, so I used `--bins`.)

Full suite, run for completeness even though a comment-only change cannot reach it: `cargo test` -> 379 tests across six binaries, `0 failed`. The known step-93 `checks::tests` worktree-name flake did NOT fire on this run. `cargo clippy --all-targets` emits zero warnings and zero errors.

## Non-findings, considered and rejected

1. **"self-extending" is now used without the illustration that defined it.** Rejected, not a defect. The same sentence still gives the reason ("a filter over a rendered set, not a directory listing and not a hand-written list"), numbered item 3 at `:45-46` states the filter precisely, and R1 at `:70-83` states its limit. The meaning is recoverable from the block without the deleted example, and restoring an example is exactly what the escalation forbade (ZERO new words). Raising this would relitigate the authorised decision, not report a defect.
2. **`:52` is now a short line** because the deletion pulled words up without reflowing the rest of the paragraph. Not a finding: line length and prose wrapping are never findings in this project, and incidental reflow is settled under Q-57 (cited in the module doc itself at `:21-22`).

## Settled items I did not re-audit

Per the round ledger: the mechanism's correctness, the deliberate no-explicit-exclusion of `.agents/prompts/checks-reviewer.md`, accepted residual R1, and the upholding at `:312`. I found no new evidence against any of those verdicts, and the byte-unmoved check above independently confirms this commit could not have disturbed them.
