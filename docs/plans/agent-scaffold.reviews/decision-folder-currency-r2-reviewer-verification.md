# Round 2 review: fix verification and re-seed detection (commit `3285320`)

Lens: did the authorised round-1 fix land exactly as prescribed and nothing more, and did it re-seed a new defect. All commands below were run in a clean worktree at detached HEAD `3285320` (`git status --porcelain` empty).

## Verdict

1. **Landed exactly as prescribed, nothing more.** Confirmed byte-exact.
2. **No re-seeded defect found.** No finding of `medium`, `high`, or `critical` severity. One `low` finding, cosmetic, in a review record rather than in the change itself.

## Evidence

### The diff is two files and one line

```
git diff --stat HEAD~1 HEAD
 .agents/prompts/orchestrator.md | 2 +-
 pack/prompts/orchestrator.md    | 2 +-
 2 files changed, 2 insertions(+), 2 deletions(-)
```

Exactly the two expected files. A line-by-line comparison of `git show HEAD~1:pack/prompts/orchestrator.md` against the working copy reports 34 lines on both sides and a single differing line number, `31`. Nothing else in either file moved.

### The replacement is byte-identical to the authorised text

Reconstruction test: take the old line 31, replace the authorised old branch-2 string with the authorised new branch-2 string, and compare the result with the actual new line 31.

- Authorised old string occurs in the old line exactly once.
- Authorised new string occurs in the new line exactly once.
- Synthesised line `==` actual new line: **True** (full string equality, not a normalised compare).

Because the synthesised line is produced by that one substitution alone, this simultaneously proves that everything outside branch 2 is byte-unchanged. Checked separately as well, splitting the tail after `pick the mode it needs: ` on `; `:

- Branch count: 3 before, 3 after.
- Prefix before `pick the mode it needs: ` identical: True.
- Branch 1 (`answer a purely factual question directly`) identical: True.
- Branch 3 (the `exploring` / `Q-69` branch, `for one whose design space is not yet decidable ... the design-space exploration mode in \`AGENTS.md\`.`) identical: **True, byte-untouched.**

### The semicolon trap was avoided

Semicolon count inside the new branch-2 body (the branch text with its single terminating `;` stripped): **0**. The three-branch semicolon delimiting is therefore intact, which the branch-count check above independently confirms (splitting on `; ` still yields exactly 3 parts, not 4).

### `pack/AGENTS.md` and the two guidance copies untouched

```
git diff --name-only HEAD~1 HEAD -- pack/AGENTS.md AGENTS.md .agents/AGENTS.reference.md
```

returns nothing. The commit touches neither the guidance source nor its renders.

### Pack / deployed parity, the step's acceptance check

```
diff pack/prompts/orchestrator.md .agents/prompts/orchestrator.md
```

produces **no output**. Both copies carry the new branch 2 at line 31.

The deployed copy is genuinely guarded, not merely coincidentally equal: `src/agents_md_drift.rs:414-456` (`the_committed_role_prompts_match_a_fresh_render`) renders the pack fresh and compares it against the committed bytes for every asset under `.agents/prompts/`, with an emptiness assertion at `:434` so the filter cannot go vacuous. That test passes.

### Guards

`cargo test`: **all green, 379 passed, 0 failed** across the six binaries (367 + 5 + 3 + 2 + 1 + 1). No `checks::tests` failure this run, so the known worktree-naming flake did not fire.

`cargo clippy --all-targets`: **clean**, zero lines matching `^(warning|error)`, finished successfully.

`cargo test agents_md_drift` specifically: 5 passed, 0 failed, including `the_committed_role_prompts_match_a_fresh_render` and `the_committed_scaffold_matches_a_fresh_render`.

### ASCII

Both changed files contain zero bytes above 0x7F (byte scan of `pack/prompts/orchestrator.md`; `LC_ALL=C grep -c '[^ -~\t]'` returns 0 for both files). The commit message is a single conventional-prefix line, no dashes, no trailers, no tool attribution.

### Re-seed checks beyond the mechanical ones

- **No stale quote of the old sentence was left in an authored source.** `grep -rn "record the resolved answer\|routing its fold\|when that fold is non-trivial"` finds the old branch-2 phrasing only in documents that are quoting it deliberately: the step brief (`docs/plans/agent-scaffold.steps/decision-folder-currency.md:13`, `:20`), the plan's copy of the same brief (`docs/plans/agent-scaffold.md:1205`, `:1212`), the round-1 findings and triage files, and the ledger. Those are prescriptive or historical records of what was to be changed, so quoting the pre-change text is correct there. The remaining hits on `when that fold is non-trivial` are `pack/AGENTS.md:41` and its two renders, a different sentence that the fix did not touch and does not contradict.
- **The new wording now matches its guidance source instead of contradicting it.** `pack/AGENTS.md:43` already reads "the resolved answer becomes a durable Open-Questions decision like any other, its non-trivial fold routed to the planner to author as above rather than edited in directly". The new branch 2 is the second-person form of exactly that, so the round-1 contradiction is resolved rather than relocated.
- **No contradiction with the neighbouring prompt paragraphs.** `pack/prompts/orchestrator.md:27` ("not authoring a decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job and which you route to it rather than author yourself") and `:33` ("the planner authors that fold when it is non-trivial") both say the same thing as the new `:31`. The trivial fold stays implicitly with the orchestrator in all three, so nothing was widened into an unqualified "the planner authors that fold".
- **No behavioural surface was touched.** The change is prose in two Markdown assets; no Rust source, manifest entry, or template is in the diff.

## Findings

One finding, `low`. No `medium`, `high`, or `critical` findings: I looked for each and found none.

### F1 (`low`). The ledger's description of the fix understates its size: "45 words to 43" where the actual change is 46 words to 42

`docs/plans/agent-scaffold.ledger.md:413` records the authorised fix as "Re-voice branch 2 ONLY, 45 words to 43". Counting whitespace-separated tokens in the two authorised strings, which are the strings the ledger itself quotes:

- Old branch 2: **46** tokens (`for` ... `yourself;`).
- New branch 2: **42** tokens.

So the re-voicing removed four words, not two, and neither endpoint matches the recorded number. No alternative counting convention I tried reproduces 45/43 (treating `Open-Questions` and `non-trivial` as two words each gives 48/44).

Reproduce:

```
git show 3285320:pack/prompts/orchestrator.md | sed -n 31p
git show 3285320~1:pack/prompts/orchestrator.md | sed -n 31p
```

then count the tokens between `for a question whose options are already clear` and the branch's terminating `;` in each.

Impact if left unfixed: none functional. The prescribed text landed byte-exact, so the number is a slip in a descriptive sentence about the fix, not a defect in the fix. It matters only as an unbacked number in a durable record that a later reader might rely on when judging how much the sentence moved. Suggested handling: correct the two numbers in place, or accept as residual. Not worth a fix round on its own.

## Explicitly not raised

- The branch 2 / branch 3 voice unevenness, known, deliberate, held as `Q-69`.
- Line length and prose wrapping.
- The stale drift-guard claim at `docs/plans/agent-scaffold.steps/decision-folder-currency.md:30` and `:38`, already ruled VALID BUT ACCEPT RESIDUAL.
- The exploration-mode passages, `pack/user-prompts/explore.md`, `pack/LEDGER.template.md`, out of scope by human decision.
