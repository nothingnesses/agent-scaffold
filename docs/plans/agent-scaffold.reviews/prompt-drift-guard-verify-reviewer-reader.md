# Review: prompt drift guard, verification round, cold-reader lens on the COVERAGE prose

Reviewer lens: read the module-level COVERAGE block in `src/agents_md_drift.rs` (lines 34-100) as a first-time reader and ask only whether any statement in it asserts something FALSE about what the guard actually covers. Each claim was checked against the CODE, not against the other comments.

- Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/verify-b-pdg`, detached at `3e4fb6c`, `git status --short` clean before and after this review. No mutation was left in place; no file under `src/`, the plan, the ledger, or the metrics log was modified.
- Test state: `cargo test` passes in full (367 + 5 + 1 + 3 + 1 + 2 tests, 0 failed). The known `checks::tests` flake (step 93) did not fire on this run.

Findings: 2. Severities: 1 medium, 1 low. No critical and no high findings; I looked for one and did not find one.

Statements in the block I checked and found TRUE against the code, so no finding is raised on them: the three-comparison enumeration and "the other tests in this file exercise the helpers and add none" (the other three tests are `normalization_tolerates_wrapping_but_not_content_change`, `precondition_rejects_non_space_whitespace_and_round_one_cases`, `precondition_exempts_fenced_indented_lines_but_not_bare_ones`); check 3's description as a `dest`-prefix filter compared against `CARGO_MANIFEST_DIR`-relative committed files (`src/agents_md_drift.rs:426-455`, `:158-159`); "both sides of each comparison go through `assert_no_unprotected_construct` and then `normalize_wrapping`" (`:391-400`, `:447-452`); the `include_str!` asymmetry between checks 1/2 and check 3; "self-extending" as it now stands after `3e4fb6c` (a new untagged `[[asset]]` row under the prefix is picked up with no edit to the test, and the module-gating exception is stated in R1 two paragraphs down, so the remaining sentence is not false); the whole of R1, including the `checks-reviewer.md` sentence (`pack/pack.toml:219-223` tags it `module = "checks"`, `git ls-files .agents/prompts/` lists 7 files and no `checks-reviewer.md`, and a fresh pinned render emits no such file); R1's mirror-case sentence (`manifest::load` iterates `manifest.asset`, `src/manifest.rs:523-531`, so an unregistered `pack/prompts/` file is never rendered); the TOML half of the COMPLEMENT paragraph (`.agents/principles.toml` has 21 non-canonical lines, including indented array continuations at lines 248, 249, 264, 514, 515, 516); and R2's closing claim "No guarded file carries such a construct today" (a port of `is_hard_start` run over `AGENTS.md`, `.agents/AGENTS.reference.md`, and the 7 committed prompts found no raw HTML line and no joined line-structured construct; none of those files contains a table at all).

---

## Finding 1: the COMPLEMENT paragraph's "the Markdown copies" sentence is false of `docs/plans/TEMPLATE.md`, on three separate counts

Severity: medium.

The block names `docs/plans/TEMPLATE` as one of its four illustrations of the complement:

`src/agents_md_drift.rs:59-61`

```
//! an obligation to stay complete that prose reliably fails; the `.agents/user-prompts/`
//! copies, `.agents/LEDGER.template.md`, the `.toml` copies under `.agents/`, and the
//! `docs/plans/TEMPLATE` family illustrate the rule and do not bound it. Leaving them
```

and then makes a three-part claim about the cost of covering them:

`src/agents_md_drift.rs:62-64`

```
//! uncovered is a scope call whose cost is uneven: widening to the Markdown copies is a
//! small change to check 3's filter, since they are prose under the same prettier settings
//! and already satisfy the precondition, while the TOML copies need a comparison of their
```

`docs/plans/TEMPLATE.md` is a member of the family the sentence names (it is the family's principal file, the projected plan view), it is Markdown, it is emitted by the scaffold, and the repo commits it byte-identically. All three of the sentence's sub-claims are false of it.

(a) "already satisfy the precondition" is false. It is rejected by `assert_no_unprotected_construct`.

Reproduction, using the body of `assert_no_unprotected_construct` copied verbatim out of `src/agents_md_drift.rs:197-222` into a standalone program (nothing in the repo is mutated; the program is at `<scratch>/precondition_demo.rs`):

```
rustc --edition 2021 -O -o precondition_demo precondition_demo.rs
./precondition_demo docs/plans/TEMPLATE.md
REJECTED docs/plans/TEMPLATE.md
    docs/plans/TEMPLATE.md line 45 is not in canonical whitespace form. The line is "| `example-step` | not started |  |"; its canonical form is "| `example-step` | not started | |".
```

The offending line is a table row with an empty last cell written as two spaces:

`docs/plans/TEMPLATE.md:45`

```
| `example-step` | not started |  |
```

confirmed with `grep -n "example-step. | not started" docs/plans/TEMPLATE.md | cat -A`, which prints `45:| \`example-step\` | not started |  |$` (the double space is real, and the line is not inside a fence).

Running the same program over every other Markdown file the paragraph names gives 1 rejected out of 17 checked: the six `.agents/user-prompts/*.md`, `.agents/LEDGER.template.md`, and the other nine `docs/plans/TEMPLATE.*` Markdown files are all accepted. So the claim is true of 16 of the 17 files and false of exactly the one the paragraph's own example list points at most prominently.

This is not a stale-committed-copy artefact: a fresh pinned render reproduces the same bytes. `cargo run -- scaffold --output-dir <tmp> --write --force --principles default --instrument` emits `docs/plans/TEMPLATE.md`, and `diff <tmp>/docs/plans/TEMPLATE.md docs/plans/TEMPLATE.md` is empty. The rendered side fails the precondition at the same line 45.

(b) "prose under the same prettier settings" is false. `docs/plans/TEMPLATE.md` is explicitly excluded from the formatter, by name, with a comment saying why:

`flake.nix:47-53`

```
            # scaffolded `docs/plans/TEMPLATE.md` is likewise a generated render
            # artifact (the initial `render` the `scaffold` command runs), so it is
            # excluded too, keeping `scaffold-self` a stable fixed point.
            settings.global.excludes = [
              "src/plan/testdata/render-fixture*"
              "docs/plans/agent-scaffold.md"
              "docs/plans/TEMPLATE.md"
            ];
```

It is under NO prettier settings, let alone the same ones, which is also the reason (a) holds: nothing normalises that double space away.

(c) "a small change to check 3's filter" is false, because the file is not in the set check 3 filters. Check 3 filters `self_scaffold_assets()`, i.e. `build_assets(...)`, i.e. the `[[asset]]` rows of `pack/pack.toml`. There is no such row for it: `grep -c 'dest = "docs/plans/TEMPLATE.md"' pack/pack.toml` prints `0`. The file is generated after the assets land, and the code says so:

`src/main.rs:1658-1665`

```
		// After the assets land, generate the projected `<task>.md` for every
		// `<task>.plan.toml` skeleton the pack dropped, so a fresh scaffold ships the
		// rendered plan view beside its structured source. The generated view is NOT a
		// manifest asset (it is derived, and `render`/`render --check` own it), so it is
		// (re)generated here rather than copied.
```

So no widening of check 3's `dest`-prefix filter can ever reach it. Covering it needs a comparison of its own, exactly like the TOML case the same sentence carves out.

Why this matters at medium rather than low: this is a false coverage-scope claim inside the one block the file designates as the sole statement of coverage, in the very file whose purpose is detecting stale content, and it is the class of defect this step keeps producing. The claim is also load-bearing in a practical sense: it is the block's stated justification for a scope call, and the next author who acts on "small change, already satisfies the precondition" will find that the file cannot be reached by that change at all and would trip the precondition if it could. It is not high or critical because the failure mode is loud (a precondition panic at test time) rather than a silent coverage hole, and no guarded file is affected today.

Note on what is NOT being claimed here: `docs/plans/TEMPLATE.md` is guarded elsewhere, by `render --check` (`flake.nix:41-49` says so). The finding is about the truth of the sentence, not about the file being unprotected.

---

## Finding 2: "Comments past this point cite it and do not restate it" is false of the file's own comments

Severity: low.

The block closes by asserting a property of the rest of the file:

`src/agents_md_drift.rs:100`

```
//! End of COVERAGE. Comments past this point cite it and do not restate it.
```

with the same claim made in the module intro:

`src/agents_md_drift.rs:4`

```
//! on what is and is not guarded; it is the one place in this file that states coverage,
```

and the governing rule:

`src/agents_md_drift.rs:38`

```
//! comments to cite. Write a coverage claim here or not at all.
```

Two comments past line 100 do more than cite.

`src/agents_md_drift.rs:376-382`

```
		// Checks 1 and 2 of COVERAGE, the drift guard on the PACK generation path: the
		// committed root `AGENTS.md` and its reference copy must match a fresh render of
		// the built-in pack under the self-scaffold config, once prettier's
		// wrapping/whitespace is normalized away on both sides. This fails on a real
		// content drift, a hand edit, a dropped slot, or a stale pack source that the
		// per-fragment guards do not cover, while tolerating an incidental formatter
		// reflow. The fix is `just scaffold-self`.
```

The first sentence after the citation restates GUARDED SET items 1 and 2 (`:43-44`) plus the render/normalisation sentence (`:48-50`). The second sentence ("This fails on a real content drift, a hand edit, a dropped slot, or a stale pack source that the per-fragment guards do not cover") is a coverage claim that appears nowhere in the block: the block states what is compared, never what a comparison fails on.

`src/agents_md_drift.rs:422-425`

```
		// Two-way in CONTENT: because it compares a fresh render against the committed
		// bytes, a pack edit with a stale copy and a hand edit of the copy with the pack
		// left alone both fail, and the fix in either direction is to edit the pack source
		// and run `just scaffold-self`. One-way in SET MEMBERSHIP, which is residual R1.
```

"Two-way in CONTENT", with its supporting argument, is a directional coverage claim about check 3 that is stated only here. The second half properly cites R1; the first half is an independent statement of coverage, which is what line 38 forbids and line 100 denies exists.

Both restatements are, as far as I can tell, TRUE of the code today. That is why this is low and not medium: no reader is currently misled about coverage. What is false is the block's claim about its own monopoly, and the cost of that is precisely the failure mode the block was restructured to end, a second site that can drift out of step with the first.

Reproducible check: `grep -n "End of COVERAGE" src/agents_md_drift.rs` gives 100; the two comments above are at 376-382 and 422-425, both after it. All four line ranges quoted here were re-read at those lines in this worktree at `3e4fb6c`.

Counter-argument I weighed and rejected: one could read "restate" narrowly as "assert coverage the block does not assert", in which case `:376-378` is merely a local gloss. But `:379-381` and `:422-424` state coverage the block does not state, so the sentence is inaccurate under the narrow reading too. The cheapest honest fix is to soften line 100 to describe what those comments actually do (cite the block and orient the reader locally), rather than to delete accurate local orientation from the tests.

---

## Not raised

- The settled items in my brief (the mechanism, the deliberate `checks-reviewer.md` exclusion, accepted residual R1, the `:312` upholding) were re-checked only far enough to confirm the block's statements about them are accurate. I found no evidence that any prior verdict was wrong, so none is re-raised.
- Line length, prose wrapping, and incidental formatter reflow: never findings in this project, and none is raised.
