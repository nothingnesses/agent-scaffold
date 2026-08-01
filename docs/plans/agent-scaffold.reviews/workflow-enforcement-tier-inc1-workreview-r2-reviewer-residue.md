# Work review, round 2, `workflow-enforcement-tier-inc1`, REVIEWER (residue and fix verification)

Reviewer: independent of the implementer, the planner, and the parallel round-2 reviewer. Worktree `.claude/worktrees/wr2-inc1-a`, branch `wr2/inc1-a`, at `f8f2e09` (the commit under review). Lens: did each round-1-prescribed fix land in the prescribed form across both writer lanes, and did either lane re-seed anything.

## Summary verdict

CLEAN. ZERO FINDINGS.

All four round-1-prescribed edits landed, each verified byte-for-byte (or token-for-token) against the triage's supplied text. The zero-authored-words claim holds mechanically for both lanes: the planner's two edits total exactly 114 words of supplied text (35 + 79), matching its own reported figure, and the implementer's three edits are likewise verbatim. The generated `docs/plans/agent-scaffold.md` is confirmed regenerated (`render --check`: up to date) and its changed line is byte-identical to the sidecar's. The new assertion in `plain_validate_and_a_sourceless_run_keep_their_behaviour` was reproduced as a genuine pin: mutating `default_ledger_path`'s no-anchor arm in a scratch copy fully outside any git repository produces exactly one suite failure (8 passed, 1 failed) with the predicted panic message, and the full suite is otherwise 395/0 as expected (373+5+1+1+9+3+1+2=395, unchanged from before the fix, correctly, since the new assertion is inside an existing `#[test]`). A residue sweep across `src/`, `tests/`, all three step sidecars, `agent-scaffold.plan.toml`, `README.md` and `CHANGELOG.md` found no orphaned twins of the old phrasing, no adjacent claim falsified by either fix, and no stray files touched beyond the four prescribed edit sites. Both producer disclosures are ruled NOT DEFECTS, with reasoning below. `cargo clippy --all-targets` and a plain `validate` on the project's own plan are both clean.

Round 2 finding count: 0. This closes one of the two consecutive clean rounds `risky` requires; the increment is not yet converged pending the parallel round-2 reviewer and the triage.

## Findings table

| id | severity | verdict |
| --- | --- | --- |
| (none) | - | ZERO FINDINGS |

## Per-edit verdicts

### Edit 1, `W1A-1`, `src/main.rs:1170-1171` (implementer, `be2c897`): LANDED, VERIFIED VERBATIM

Prescribed replacement (triage lines 69-74): `, so the match is against whatever \`docs/plans\` lies lexically above that \`..\`, which is the plan's own only when the \`..\` does not climb out through one`.

Actual, read from the file:
```
1170	/// `None` for it), so the match is against whatever `docs/plans` lies lexically above that
1171	/// `..`, which is the plan's own only when the `..` does not climb out through one.
```
Word-token diff of the inserted clause against the prescribed text: IDENTICAL (`diff` exit 0). The only surrounding differences are the untouched prefix `` `None` for it) `` and the untouched trailing period that closed the original sentence before "Project root" (mechanical splicing, not authored content: the prescribed clause opens with a comma, so the awkward pre-comma space from the old `... it) and the real ...` join is naturally absent, exactly as plain text-substitution produces without adding a word).

### Edit 2, `W1A-2` narrowing clause, sidecar + generated (planner, `f8f2e09`): LANDED, VERIFIED VERBATIM, BOTH FILES BYTE-IDENTICAL

Prescribed (triage lines 109-111): `and with a \`..\` that stays below the project's own \`docs/plans\` (that last works because \`Path::file_name\` returns \`None\` for a \`..\` component, so the walk skips past it and reaches that project's \`docs/plans\` above it)`.

```
$ diff w1a2-narrow-new.txt w1a2-narrow-actual.txt && echo IDENTICAL
IDENTICAL
```
`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:162` and `docs/plans/agent-scaffold.md:1557` were extracted and compared: byte-identical (`test "$A" = "$B"` -> true).

### Edit 3, `W1A-2` recorded consequence, sidecar + generated (planner, `f8f2e09`): LANDED, VERIFIED VERBATIM

Prescribed (triage lines 115-117), the two-sentence recorded consequence beginning `NOT IN THAT MATRIX, AND MEASURED AT WORK REVIEW: ...` and ending `... that rejects it.`.

```
$ diff w1a2-consequence.txt w1a2-consequence-actual.txt && echo IDENTICAL
IDENTICAL
$ wc -w w1a2-consequence.txt w1a2-consequence-actual.txt
  79 w1a2-consequence.txt
  79 w1a2-consequence-actual.txt
```
79 words, both files. Combined with the narrowing clause's 35 words: 35 + 79 = 114, matching the planner's own reported word count exactly. No prose in this paragraph beyond these two supplied blocks was touched: `git diff be2c897 f8f2e09 -- docs/plans/agent-scaffold.md docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` shows exactly one changed line in each file, nothing else in either file moved, and the "do not promote to accepted costs" instruction was honoured (that section, lines 252-258 of the triage's citation, is untouched).

### Edit 4, `W1A-3` (a), module doc, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:19` (implementer, `be2c897`): LANDED, VERIFIED VERBATIM (deletion path taken)

Prescribed (triage lines 195-197): `//! Several of the tests are pins rather than red-then-green cases, marked as such on each.` replacing the old four-line paragraph (old lines 19-22).

```
$ diff w1a3a-prescribed.txt w1a3a-actual.txt && echo IDENTICAL
IDENTICAL
```
Confirmed the old three extra lines (the miscounted "Four of the tests...", the mislabelled no-convention case, and the false "pass identically" clause) are gone: the new line is immediately followed by a blank line then `use std::{`.

### Edit 5, `W1A-3` (b), new assertion, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:438-445` (implementer, `be2c897`): LANDED, VERIFIED VERBATIM, AND MECHANICALLY PINS THE GAP

Prescribed code block (triage lines 203-212), six lines of assertion plus its two-line comment, inserted into `plain_validate_and_a_sourceless_run_keep_their_behaviour` immediately after the bare-`validate` block and before `remove_dir_all`.

```
$ diff w1a3b-prescribed.rs w1a3b-actual.rs && echo IDENTICAL
IDENTICAL
```

Note: the triage counted four prescribed edits (`W1A-1`, `W1A-2`'s clause, `W1A-2`'s consequence, `W1A-3`(a), `W1A-3`(b): five text/code sites across four findings). All five sites verified above.

## Mutation reproduction (item 4 of the lens)

Built a scratch copy fully outside any git repository:
```
$ rm -rf /tmp/wr2a-mutant && mkdir -p /tmp/wr2a-mutant
$ git archive HEAD | (mkdir -p /tmp/wr2a-mutant/src-tree && tar -x -C /tmp/wr2a-mutant/src-tree)
$ cd /tmp/wr2a-mutant/src-tree && git rev-parse --is-inside-work-tree
fatal: not a git repository (or any parent up to mount point /)
```
`git archive` never includes `target/`, so the tree was necessarily a clean, uncompiled export; no risk of the stale-`CARGO_BIN_EXE` trap the round-1 triager hit, since nothing was copied from a pre-built tree.

Mutated `default_ledger_path`'s no-anchor arm (`src/main.rs:1242`) from `docs/plans/{task}.ledger.md` to `MUTANT/{task}.ledger.md`, built and ran the full suite with `TMPDIR` outside the repo (`--no-fail-fast`):

```
     Running tests/metrics_and_ledger_anchor_to_the_plan_source.rs (...)
test plain_validate_and_a_sourceless_run_keep_their_behaviour ... FAILED
---- plain_validate_and_a_sourceless_run_keep_their_behaviour stdout ----
thread '...' panicked at tests/metrics_and_ledger_anchor_to_the_plan_source.rs:442:5:
assertion `left == right` failed: a sourceless resume keeps the current-directory-relative ledger path
  left: "no ledger at MUTANT/task.ledger.md; nothing to resume\n"
 right: "no ledger at docs/plans/task.ledger.md; nothing to resume\n"
test result: FAILED. 8 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```
All seven other suites (373+5+1+1+3+1+2 = 386 tests) stayed green under the mutation; only this one test in this one suite failed. Total under mutation: 394 passed, 1 failed. Exactly one suite failure, exactly the predicted test, exactly the predicted panic message (the "right" side is byte-identical to the prescribed assertion string). The pin holds.

## Suite total and binary-identity checks (items 3 and the contamination trap)

Forced a clean recompile in the worktree under review (`cargo clean -p agent-scaffold` then rebuild) before measuring, to avoid inheriting any stale binary:
```
$ strings target/debug/deps/metrics_and_ledger_anchor_to_the_plan_source-bf4905c55850edca | grep -m1 'target/debug/agent-scaffold'
.../agent-scaffold/.claude/worktrees/wr2-inc1-a/target/debug/agent-scaffold
```
The embedded `CARGO_BIN_EXE_agent-scaffold` path is this worktree's own, not a stale tree's. Then, full suite, `TMPDIR` outside the repo:
```
test result: ok. 373 passed  (unit tests, src/main.rs)
test result: ok. 5 passed    (audit_command.rs)
test result: ok. 1 passed    (checks_missing_tmpdir.rs)
test result: ok. 1 passed    (checks_staged_hook_env.rs)
test result: ok. 9 passed    (metrics_and_ledger_anchor_to_the_plan_source.rs)
test result: ok. 3 passed    (scaffold_precommit_hook.rs)
test result: ok. 1 passed    (validate_toml_primary_skips_markdown_plan.rs)
test result: ok. 2 passed    (validate_workflow_toml_source_needs_no_plan.rs)
```
373+5+1+1+9+3+1+2 = 395. Matches the pre-fix total exactly, correctly: the new assertion lives inside an existing `#[test]`, so libtest's function-count-based total does not move. A finding that the count should have become 396 would itself have been wrong; none is filed.

## `render --check` (item 3)

```
$ cargo run --quiet -- render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
```
Confirms `docs/plans/agent-scaffold.md` is the mechanical output of `render`, not a hand-edit, and its changed line (`:1557`) was independently confirmed byte-identical to the sidecar's changed line (`:162`) above.

## Residue sweep (item 5): the harder half

Grepped for leftover twins of the pre-fix phrasing, across `src/`, `tests/`, and every non-excluded doc:

```
$ grep -rn "still finds the real\|still matches\." src/ tests/ docs/plans/*.md docs/plans/agent-scaffold.steps/*.md
(no output)
$ grep -rln "skips past it and still finds the real" docs/
docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md
$ grep -rn "Four of the tests are pins" src/ tests/ docs/
(no output)
$ grep -rn "the two no-anchor cases\|accepted-cost bare-filename miss" src/ tests/ docs/
(no output)
```
The single remaining hit for the old sidecar sentence is exactly the dated explorer record the triage deliberately excluded (`docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:150`); it was not edited, as instructed, and is not re-filed here.

Checked the other two step sidecars named in scope (`test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`) for any reference to the anchor derivation, the `..` rule, or `default_ledger_path`: only unrelated `status --source ... --resume --json` command examples, no claim to falsify.

Checked `README.md`'s anchoring paragraph (around line 226) and the CHANGELOG's anchoring entry (line 22) in full: neither mentions `..` at all, confirming the round-1 triage's own site count (SITE COUNT: 1 for `W1A-1`) still holds after the fix; nothing there was left inconsistent.

Checked `docs/plans/agent-scaffold.plan.toml`: no restatement of the `..` claim (the only `..`-adjacent hits are the unrelated symlink-sidecar-ref concern and unrelated waiver-model prose). The Roadmap row for `workflow-enforcement-tier` (`docs/plans/agent-scaffold.plan.toml:1296-1298`) still reads `status = "in-progress"`, which is correct under the TENSE RULE: the step is not yet marked complete, round 2 is still open, and this entry makes no completion claim the fixes would falsify.

Checked that no file outside the four prescribed edit sites was touched by either commit:
```
$ git diff --stat f491c4e f8f2e09
 docs/plans/agent-scaffold.md                               |  2 +-
 .../agent-scaffold.steps/workflow-enforcement-tier.md      |  2 +-
 src/main.rs                                                |  9 +++++----
 tests/metrics_and_ledger_anchor_to_the_plan_source.rs      | 14 ++++++++++----
```
Exactly four files, matching exactly the four prescribed edits. No stray file.

Checked build health for anything either fix pass might have disturbed:
```
$ cargo clippy --all-targets
Finished `dev` profile [unoptimized + debuginfo] target(s) in 10.51s
(no warnings)
$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml
docs/metrics/workflow.jsonl: 245 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
```
Both clean.

## Producer disclosure (i): the planner's `..`-above-vs-below observation on `W1A-2`

RULING: NOT A DEFECT.

The planner is factually correct that, in the fixture `away/sub/../docs/plans/p.plan.toml`, the `..` component sits earlier in the path string than `docs/plans`, and that walking `Path::ancestors()` finds `docs/plans` as the very first candidate ancestor, never reaching the `..` component at all (traced against `project_root_of_source`, `src/main.rs:1179-1196`). Read as a literal directory-nesting claim ("the `..` is a descendant somewhere under `docs/plans`"), "stays below the project's own `docs/plans`" fits only the OTHER fixture used in round 1's own measurement (`away/docs/plans/sub/../p.plan.toml`, where the `..` is genuinely a descendant of the matched `docs/plans`), not this one.

But "below"/"stays below" is not being used as a literal nesting claim here; it is this document's own established idiom for "does not escape the project's boundary," already in use before this fix pass. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:320` (acceptance check 13, untouched by either fix, predates it): "an explicit `--metrics` that climbs out of the plan's root ... exits NON-ZERO, while a `..` that stays INSIDE the root is allowed". "Stays below"/"stays inside" is paired, both there and in the new sentence, against "climbs out" as its established antonym: a containment-boundary metaphor, not a directory-position claim. Under that reading, the fixture the planner cites does "stay below" in the relevant sense: it resolves to its own project's correct log without escaping to a foreign one, exactly as round 1 measured (`metrics: 11 records`, `away`'s own log).

Decisive: the sentence's actual truth-claim, "the `docs/plans` convention resolves correctly in every spelling constructed: ... and with a `..` that stays below the project's own `docs/plans`," is TRUE of the cited fixture under either reading, because the fixture DOES resolve correctly. What is at most ambiguous is the descriptive label, not the claim. That is a materially different situation from the `W1A-1`/`W1A-2` defect this closes, where the original text made a demonstrably FALSE affirmative claim (a live false-green reproduction, `exit=0` claiming `workflow invariants hold` against the wrong project's log). No comparable falsehood is shown here.

Process note, also weighed: the triage supplied this exact text for the planner to copy rather than compose ("Exact text is supplied below so the planner copies rather than composes, which is what keeps a fix pass from manufacturing the next round's finding"). The planner copied it verbatim and flagged the interpretive tension as a disclosure instead of silently rewording triage-prescribed text, which is the correct process behaviour even if the wording were judged imperfect.

If a future round wants a sharper form with zero ambiguity, the cheapest change would be to swap "stays below" for phrasing that mirrors the code doc's own resolved wording from `W1A-1` ("does not climb out through one", `src/main.rs:1171`), giving the code and the plan the identical mechanism-based phrase. That is a possible future polish, not a defect, and is not filed as a finding here.

## Producer disclosure (ii): the implementer's doc-comment-completeness observation on the new assertion

RULING: NOT A DEFECT.

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:399-406`, the doc comment on `plain_validate_and_a_sourceless_run_keep_their_behaviour`, reads (verified against the current file):
```
/// Acceptance check 10: plain `validate` (no `--workflow`) is unaffected by the tier
/// policy and still exits 0 with a stderr note on a missing log, and a bare `validate`
/// with NO plan source has nothing to anchor to and keeps the historical
/// current-directory-relative path.
///
/// The anchored-but-missing case is red-then-green (before the change this read this
/// directory's three-record log and printed it as valid); the bare-`validate` case is a
/// pin on the no-anchor rule.
```
It does not name the third run the `W1A-3`(b) fix appended (`status --resume` with no anchor, asserting `default_ledger_path`'s fallback). Three points support the implementer's position:

1. Scope: the comment is explicitly framed around "Acceptance check 10," which is a `validate`-specific acceptance criterion. The appended block tests a different property (`default_ledger_path`'s no-anchor arm, used by `run_resume`/`run_next`, not `run_validate`), added to this test purely for economy per the triage's own fix instructions ("no new fixture, no new test function, six lines appended to the existing no-anchor test"). It carries its own local two-line comment at the point of use (`:438-439`) identifying exactly what it pins, which is the normal place for that documentation to live.
2. No exclusivity claim: "the bare-`validate` case is a pin on the no-anchor rule" states that one case IS a pin; it never says these are the ONLY assertions in the function, or that the enumeration above it is exhaustive. Nothing in the current text becomes false by the addition; it becomes incomplete, which is a different defect class than the ones round 1 found and fixed (`W1A-1`, `W1A-2`, and `W1A-3`(a) were all demonstrated FALSE, not merely incomplete).
3. Direct round-1 precedent: the triage examined this exact line (previously numbered `:409` before the three-line deletion in `W1A-3`(a) shifted it to `:406`) and ruled it "ACCURATE and must be left alone" (triage, `SITE COUNT` for `W1A-3`), and the fully-specified `W1A-3`(b) fix instructions that added the new block never asked for a doc-comment update alongside it. Had the triage intended the addition to require a doc-comment change, the fully-prescriptive fix text would have said so, as it did for every other edit in this round.

No finding is filed. A future polish (adding a clause noting the appended ledger-path pin) is available at essentially zero cost but is not owed by round 1's ruling and does not correct a falsehood.

## Enumeration (what was swept, including negatives)

Edits verified (5 sites, 4 findings): `W1A-1` (main.rs doc, landed verbatim); `W1A-2` narrowing clause (sidecar+generated, landed verbatim, byte-identical across both files); `W1A-2` recorded consequence (sidecar+generated, landed verbatim, 79 words, byte-identical across both files); `W1A-3`(a) module doc (landed verbatim, deletion form taken); `W1A-3`(b) new assertion (landed verbatim, mutation-confirmed pin).

Zero-authored-words claim: verified mechanically via `diff` for all five sites (all IDENTICAL), and the planner's reported 114-word figure independently reconstructed from `wc -w` (35 + 79 = 114).

Regeneration: `render --check` reports up to date; sidecar line 162 and generated line 1557 are byte-identical.

Mutation reproduction: performed fresh, in `/tmp/wr2a-mutant/src-tree`, confirmed outside any git repository, confirmed not built from a stale/copied `target/` (via `git archive`, which excludes `target/`), confirmed the embedded `CARGO_BIN_EXE_agent-scaffold` path via `strings` before trusting any run. Result: exactly one suite failure, exactly the predicted test, exactly the predicted panic message.

Suite total: 395, unchanged, correctly (new assertion added inside an existing `#[test]`; libtest counts functions).

Residue grep sweep, all negative except the one deliberately-excluded exploration file: old `main.rs` phrasing, old sidecar phrasing, old module-doc "Four of the tests" phrasing, "the two no-anchor cases"/"accepted-cost bare-filename miss" twins.

Files checked for adjacent-claim falsification, all negative: `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, `README.md`, `CHANGELOG.md`, `docs/plans/agent-scaffold.plan.toml` (including the Roadmap row's `status = "in-progress"`, correct under the TENSE RULE).

File-touch scope check: `git diff --stat f491c4e f8f2e09` shows exactly the four expected files, nothing extra.

Build health: `cargo clippy --all-targets` clean; `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml` reports both the plan and its own metrics log valid (245 records).

Producer disclosures ruled: (i) planner's `..`-above/below observation on `W1A-2`, NOT A DEFECT; (ii) implementer's doc-comment-completeness observation on the new assertion, NOT A DEFECT. Reasoning for both given above.

Excluded by design, correctly not re-filed: `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:150`, the dated explorer record carrying the ancestor sentence.

Not reopened, per scope: the `..` escape BEHAVIOUR itself (settled not-a-defect by three independent agents in round 1); any refusal mechanism, containment predicate, or canonical root derivation (inc2/inc3, out of scope for inc1).
