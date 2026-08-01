# `workflow-enforcement-tier` plan review, round 4: reviewer, residue and fix-verification lens

Reviewer model: Claude Sonnet 5. Exact model id `claude-sonnet-5`.
Worktree: `.claude/worktrees/review-q55-r4a`, branch `review/q55-r4a` at commit `e34c2c9`, the exact commit under review.
`TMPDIR` was `/tmp/r4a-scratch`, outside any git repository, for every `cargo test` run.

Scope verified: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, `test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`, the `[[step]]` / `[[question]]` entries this fold touches in `docs/plans/agent-scaffold.plan.toml`, and the rendered `docs/plans/agent-scaffold.md` (evidence for the render-fidelity checks, not itself the artifact to author against).

## Summary verdict

ONE valid finding, severity `low`. Both commits since round 3 (`1a1655c`, applying all eight round 3 fixes, and `e34c2c9`, the deletion-only supplementary pass) land clean: every one of `R3A-1` and `R3B-1` through `R3B-7` closed exactly as the round 3 triage prescribed, at the exact prescribed sites, with no partial application and no new residue from either pass. The deletion-only pass's "zero new authored words" constraint holds under a mechanical token-multiset check, with one net-new token ("tests", +1) that I rule is a defensible reuse of a word already present in the same string, not new vocabulary.

The one finding, `R4A-1`, is the producer's own first disclosure, ruled here as a REAL defect the producer was right to flag: `workflow-enforcement-tier.md:280` carries an exclusivity claim of the identical shape `R3B-1` already removed once from this file (line 290), now surviving at a second site neither `R3B-1`'s grep-scoped fix nor either producer pass reached. The producer's second disclosure (the plan TOML step `title` field not being projected into the rendered `agent-scaffold.md`) I confirm is TRUE by reading the renderer's source directly, and I rule it a pre-existing property of the tool, out of scope for this fold.

Because this round returns one valid finding, the round is not clean; the risky-classification streak does not advance from this round.

## Findings table

| id | severity | one-line summary |
| --- | --- | --- |
| `R4A-1` | low | `workflow-enforcement-tier.md:280`'s "It is the only part of the mechanism that changes what a currently-succeeding invocation REPORTS..." is false of inc1 (check 4), the same exclusivity-claim defect `R3B-1` removed from line 290, surviving at an untouched twin site. |

---

## `R4A-1`. Producer disclosure 1, ruled `VALID`. Severity `low`. Line 280's "only part of the mechanism" claim is false, the same shape as the defect `R3B-1` removed from line 290

TENSE APPLIED: PRESENT for the operative half. The claim is a descriptive statement about what the three increments (as designed) each do, evaluable against the increments' own specifications elsewhere in the same document; it does not require the code to exist yet, only that the document's other sections (which are all forward-looking design commitments) agree with each other.

QUOTE, `workflow-enforcement-tier.md:280` (identical text also at the rendered `docs/plans/agent-scaffold.md:1675`, since the sidecar is inlined verbatim):

"WHY THE PREDICATE IS ITS OWN INCREMENT. It is the only part of the mechanism that changes what a currently-succeeding invocation REPORTS, whether by failing (the validator) or by withholding (the projections); it carries a known false positive (accepted cost (ii)); and it deliberately uses a DIFFERENT resolution from the default, so its review must check the lexical/canonical SPLIT rather than one rule."

THE FALSIFIER IS THE SAME ONE `R3B-1` USED FOR INC1, AND I RE-VERIFIED IT INDEPENDENTLY RATHER THAN TRUSTING THE PRIOR ROUND. Check 4, `workflow-enforcement-tier.md:311`: "AFTER INC1, the false pass is dead: rerun the borrowed-slug demonstration (fixture step `complete` with slug `triager-runs-only-on-findings`) from the agent-scaffold root. Before the fix it exits 0 with `workflow invariants hold`. After, no green. Give the fixture a log of its OWN with no evidence for that slug and expect the correct RED instead of the absence of a green." This scenario is exactly defect B's demonstration, already established earlier in the same file (`:81-92`): a fixture carrying its own log with no evidence for a slug that DOES have round records in agent-scaffold's own log is, TODAY, a currently-succeeding (falsely green, exit 0) invocation, because the pre-fix anchor bug makes the tool read agent-scaffold's log instead of the fixture's own. After inc1 alone (no inc2, no inc3), the anchor is corrected, the tool reads the fixture's OWN log, and the pre-existing W3 check correctly reports RED (exit 1). Round 1's triage measured this exact flip directly (`workflow-enforcement-tier-planreview-r1-triage.md:126-136`, quoted below), and round 3's triage used it as inc1's falsifier for `R3B-1`.

```
=== today, run from the agent-scaffold root ===
...
exit: 0

=== the pairing inc1's anchor will produce, same files ===
...
exit: 1
```

So inc1 ALONE changes what a currently-succeeding invocation reports, by FAILING it. Line 280 asserts that inc2 (the predicate) is "the only part of the mechanism" that does this, "whether by failing... or by withholding". The "withholding" disjunct is exclusive to inc2 (nothing else in the fold introduces an omit/withhold behaviour: `:172` assigns it to inc2's own decision, `Q-55-refusalscope`, alone). The "failing" disjunct is NOT exclusive to inc2: inc1 produces exactly this effect, on the same evidence `R3B-1` already used to strike the parallel claim at line 290 ("the tier policy... is the only increment that makes a previously-green run fail").

WHAT "THE MECHANISM" REFERS TO, WHICH I VERIFIED RATHER THAN ASSUMED. The document defines the term at its own section header, `:150`, "## The mechanism, decided rather than chosen here", whose content is `Q-55-mechanism`'s decision: "ANCHOR PLUS REFUSAL, IDENTITY QUEUED" (`:152`). "The mechanism" is therefore inc1 (the anchor) plus inc2 (the refusal), a two-part referent, not the three increments and not inc2 alone. Since inc1 IS a part of "the mechanism" under the document's own definition, and inc1 DOES change what a currently-succeeding invocation reports (by failing, per check 4), the claim that inc2 is "the only part of the mechanism" that does this is false under the document's own scoping of the term, not only under a looser reading that would pull inc3 in too.

THIS IS A TWIN OF `R3B-1`, NOT A NEW ROOT CAUSE, AND A LITERAL GREP FOR `R3B-1`'S STRINGS WOULD NOT HAVE CAUGHT IT. `grep -n "only increment that makes a previously-green"` and `grep -n "previously-green"` both return zero hits anywhere in the fold (verified below, under Enumeration); `R3B-1`'s fix was correctly scoped to its own site and could not have reached this one. The shape is identical: an increment-separation or ordering argument asserts an increment is uniquely responsible for turning a currently-succeeding invocation into a failing one, when in fact another increment (elsewhere in the same document, on the same check) also does it.

WHY `low`, evaluated the same way round 3 evaluated `R3B-1` and declined to raise it to `medium`. The false claim sits inside a "WHY X IS ITS OWN INCREMENT" justification paragraph, one of THREE independent reasons given in the same sentence (the false claim, the known false positive at accepted cost (ii), and the deliberately-different lexical/canonical resolution), plus a further paragraph-level argument about the negative correctness property. Removing the false reason does not change the conclusion (inc2 is still its own increment; the remaining reasons carry it), and the operative facts elsewhere in the document (`:274`, `:298`, check 4) are correct and unambiguous. The risk is a reviewer or implementer who reads this paragraph in isolation and infers, incorrectly, that inc1 can never produce a new failing exit code, which is exactly the belief `R3B-1`'s own correction (at `:274`, "NO new REFUSAL mechanism: any new non-zero exit comes from the pre-existing W3 check finally running against the right project, which is check 4's whole point") was written to prevent at the other site. Same containment, same band.

MINIMAL FIX, PROPOSED AS A RULING RATHER THAN AS DIRECTION TO A FIX PASS (I do not fix; I report). Following this project's own established preference for deletion-class fixes (five retrospective and one prospective confirmation cited at `:363` of this same file): strike "is the only part of the mechanism that" from the opening clause, leaving "It changes what a currently-succeeding invocation REPORTS, whether by failing (the validator) or by withholding (the projections); it carries a known false positive (accepted cost (ii)); and it deliberately uses a DIFFERENT resolution from the default, so its review must check the lexical/canonical SPLIT rather than one rule." This is a pure deletion (eight words, no substitution, no new vocabulary), keeps the true and still-relevant fact that the predicate changes reports, and drops only the false exclusivity assertion. I checked whether this reads grammatically: it does, and it parallels `R3B-1`'s own fix shape (strike the false "only" clause, keep the true remainder in the same sentence).

SITE COUNT. Grepped for the exact opening clause and the wider phrase across all three sidecars, the plan TOML, and the rendered doc (see Enumeration): ONE finding, TWO literal sites (the sidecar and its verbatim render), both edited by the same single sidecar change plus a re-render.

---

## Ruling on producer disclosure 1 (repeated for clarity, since the brief asks for it explicitly)

VALID, real defect, severity `low`, minimal fix is the eight-word deletion given under `R4A-1` above. The producer's own diagnosis (that this is the same exclusivity error `R3B-1` removed, with the increments swapped) is correct. I independently re-derived the falsifier (check 4 plus defect B) rather than accepting the producer's framing, and independently resolved the "does 'the mechanism' include inc3" ambiguity the producer raised: it does not need to, because the document's own definition of "the mechanism" at `:150`-`:152` is inc1 plus inc2, and inc1 alone already falsifies the claim. The producer's observation that the document uses "the mechanism" both narrowly (`:288`, "for inc1 and inc2") and broadly (`:298`, "the only place in the mechanism where...") is accurate as a textual observation, but does not change the ruling: under EITHER reading the claim at `:280` is false, since inc1 is inside the narrow reading and a fortiori inside the broad one.

## Ruling on producer disclosure 2 (the plan TOML step `title` field)

CONFIRMED TRUE, by reading `src/plan/render.rs` directly rather than inferring it from output. `assemble()` (`:286-333`) builds the rendered file from: the banner, `plan.meta.title` (the top-level plan title, a DIFFERENT field on a DIFFERENT struct), the status line, front/tail prose blobs, the principles section, the vocabulary section, the questions section, `roadmap_section(step_blobs)`, and `step_details_section(step_blobs)`. `roadmap_section` (`:459-471`) emits one table row per step using only `step.slug`, `step.status.label()`, and `notes_cell(step)` (which reads `blocked_by`, `waivers`, and `provenance`, never `title`). `step_details_section` (`:546-557`) inlines each step's SIDECAR BODY BLOB verbatim; that blob carries its own `### <slug>: <heading text>` line written directly in the `.md` sidecar file, not derived from the TOML. `Step.title` (`src/plan/source.rs:139`, "The human-readable step title") is parsed from the TOML and never read by `render.rs`, nor by `next.rs`, `main.rs`, `validate.rs`, `workflow.rs`, or `checks.rs` (checked by name across all five; zero hits). I confirmed this concretely for the two steps this fold touches: `docs/plans/agent-scaffold.md:325` renders `test-tmpdir-repo-assumption`'s roadmap row as `| \`test-tmpdir-repo-assumption\` | not started |  |`, no title text at all, and its Step Details heading at `:1778` is the sidecar's own `###` line, which already differs in content from the TOML title (the TOML title carries a parenthetical, "(a false red, 3 tests)", that the sidecar heading has never carried, independent of this fold's edits).

RULED: a PRE-EXISTING property of the tool, OUT OF SCOPE for this fold. `Step.title` has never been projected into the rendered output at any point this codebase's render pipeline has existed; nothing in this fold's two passes changed that, and nothing in the fold's own text makes a competing claim that the title IS rendered (the field's own doc comment, "The human-readable step title", promises nothing about where it surfaces). The producer's re-render after editing the TOML title was still the correct action to take, independently: the SAME commit also edited the two sidecar files, which ARE inlined verbatim, so the re-render was owed regardless of whether the title edit itself had any rendering effect. `render --check` reports "up to date" at `e34c2c9` (reproduced below), confirming no drift. I raise no finding here.

## Enumeration: what I swept

MECHANICAL TOKEN-MULTISET CHECK OF PASS 2'S "ZERO NEW AUTHORED WORDS" CLAIM, run rather than eyeballed:

```
git diff 1a1655c e34c2c9 > pass2_full.diff
grep -E '^-' pass2_full.diff | grep -vE '^--- ' | sed 's/^-//' > removed_full.txt   # 7 lines
grep -E '^\+' pass2_full.diff | grep -vE '^\+\+\+ ' | sed 's/^\+//' > added_full.txt # 7 lines
tr -c 'A-Za-z0-9_' '\n' < removed_full.txt | grep -v '^$' | sort > removed_tokens.txt # 800 tokens
tr -c 'A-Za-z0-9_' '\n' < added_full.txt   | grep -v '^$' | sort > added_tokens.txt   # 785 tokens
comm -13 removed_tokens.txt added_tokens.txt
```

Result: exactly ONE token, `tests`, present in the added set with no matching removed occurrence (13 occurrences in removed lines, 14 in added lines, net +1). Every other token count in the added set is less than or equal to its removed-set count (spot-checked: `of` 11->8, `386` 5->0, `when` 2->0, `it` 20->18, `does` 4->2, `the` 35->33; all net reductions, consistent with a deletion-dominant pass). The +1 `tests` is entirely attributable to `docs/plans/agent-scaffold.plan.toml:1322`'s title edit, `"a false red, 3 of 386"` -> `"a false red, 3 tests"`, which deletes `of` and `386` and reuses the word `tests`, which ALREADY appears earlier in the exact same title string ("three tests read the ambient repository state..."). RULED: this is a defensible reuse of existing vocabulary from within the same string, not new authored content, so the zero-new-authored-words constraint holds. I note as a side observation (not a finding, per the disclosure-2 ruling above) that this specific edit line has NO mirrored occurrence in the rendered `agent-scaffold.md`, unlike the other three pass-2 edits, which each appear twice (sidecar plus render) — independent confirmation that the TOML title is not projected.

LINE-NUMBER VERIFICATION OF ALL FOUR PASS-2 EDITS, against the task's own description, all four reproduced exactly as stated:
- `workflow-enforcement-tier.md:290` (`grep -n "THE ORDER IS inc1"`): "when it does" absent, sentence reads "...the tier policy goes LAST because EVERY escape hatch a user reaches for is closed by an earlier increment."
- `workflow-enforcement-tier.md:306` (`grep -n "BEFORE RUNNING ANY OF THIS"`): "386 " absent, reads "Three of the suite's tests (...)".
- `test-tmpdir-repo-assumption.md:3` (`grep -n "^A suite defect"`): "of the 386 " absent, reads "Three tests require the directory...".
- `agent-scaffold.plan.toml:1322` (`grep -n '3 tests)"'`): title reads "...(a false red, 3 tests)".

ROUND 3 FIX VERIFICATION, ALL EIGHT, checked against the exact prescribed minimal fix in `workflow-enforcement-tier-planreview-r3-triage.md`, all CLOSED:
- `R3A-1` (check 11, `:318`): precondition "with the fixture's single step carrying the borrowed slug `triager-runs-only-on-findings` at `complete`," added verbatim as prescribed. CLOSED.
- `R3B-1` (`:290`): "because it is the only increment that makes a previously-green run fail, and" struck, exact prescribed text. CLOSED (and its own residue, the dangling "when it does", is what pass 2 then closed).
- `R3B-2` (`:365`, Documentation impact, INC3 list): new bullet "`CHANGELOG.md` under `## [Unreleased]`. The entry must name the exit-code flip and the population it breaks, every project scaffolded without `--instrument`." added, matching the prescribed scope (file, section, content; no `Added`/`Changed`/`Fixed` restatement). CLOSED.
- `R3B-3` (two mandatory sites): `:308` (now unnumbered after later shifts, verified by content) "(386 expected) " struck from check 1; `test-tmpdir-repo-assumption.md`'s own check 1 "386 expected, 0 failed." struck. Both CLOSED. (The three narrative sites the triage scoped as "may stand" were additionally taken by pass 2, which is in-scope generosity per the triage's own note: "If it takes only two, take those two" implies taking more is not a defect.)
- `R3B-4` (`:146`): "the pack mentions" -> "`pack/AGENTS.md` mentions", exact prescribed narrowing. CLOSED.
- `R3B-5` (check 15, `:329`): "reports the missing log BY PATH" -> "THE REPORTED PROBLEM names the resolved log path and says the workflow check could not run", exact prescribed substitution. CLOSED.
- `R3B-6` (`:300`): "of seventy-seven" struck, exact prescribed deletion. CLOSED.
- `R3B-7` (`:296`): ", and the whole thing including siblings and ledger at `+163/-18`" struck, exact prescribed deletion. CLOSED.

RE-RENDER FIDELITY: `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` at `e34c2c9` reports "docs/plans/agent-scaffold.plan.toml: up to date". Reproduced.

VALIDATE: `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` at `e34c2c9` reports "docs/metrics/workflow.jsonl: 242 records, valid", "docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid", "workflow invariants hold", exit 0. Reproduced.

SUITE AND LINT, run with `TMPDIR=/tmp/r4a-scratch` (outside any git repository): `cargo test` gives `373 + 5 + 1 + 1 + 3 + 1 + 2 = 386 passed, 0 failed` across all binaries. `cargo clippy --all-targets -- -D warnings` compiles clean, no warnings or errors. Both reproduced; the 386 count matches today's tree exactly, which is precisely why `R3B-3`'s fix (removing the pinned expectation rather than updating the number) was the correct shape.

TWIN SWEEPS, NEGATIVE RESULTS, run over all three sidecars, the plan TOML, and the rendered doc unless noted:
- `386`: zero matches in the fold's own content. The only hit anywhere in `agent-scaffold.md` is a coincidental substring inside `16386` (a clock-resolution measurement in an unrelated step's content), not a suite-count reference.
- `of 386`: zero matches.
- `when it does`: one match, `test-tmpdir-repo-assumption.md:52`, a different and well-formed usage ("failing with an environment message when it does not hold") with a clear antecedent ("the precondition"), not a twin of the fixed defect.
- `previously-green` / `only increment that makes a previously-green`: zero matches anywhere.
- `BY PATH` / `by path`: zero matches of the fixed clause; the only "by path" hits are unrelated uses in `findings-files.md`, `task-entry-regrounding.md`, and their rendered mirrors ("references them by path"), a different sense entirely.
- `naming the path it looked for`: two matches, `:256` and `:332`, both the twin sites round 3's triage explicitly ruled MUST NOT CHANGE. Confirmed unchanged, both still read correctly against the now-updated check 15.
- `two places` / `only two`: one on-topic match, the now-corrected `:146` ("The only two places `pack/AGENTS.md` mentions..."); other hits are unrelated ("only two are documented", "only two (the required consecutive clean rounds...)" in different steps).
- `seventy-seven`, `+163/-18`: zero matches anywhere in the fold.
- `only the` / `is the only`, checked as the general exclusivity-claim pattern (beyond the two already-known sites): four hits at `:51`, `:111`, `:162`, `:280`/`:298`. `:51` ("the only thing on stdout is the ok summary") and `:111` ("the only one the scaffolded guidance documents") are unrelated claims about output streams and documented invocations, not increment-exclusivity claims. `:162` is a quoted self-report by an external explorer about their own reasoning process, not a claim by the document. `:298` ("the only place in the mechanism where TWO DIFFERENT RESOLUTIONS... run against each other") is a genuinely different and true claim (inc1 has one resolution rule; inc3 has none; only inc2 runs two against each other), verified against the mechanism section (`:150-166`) and not a twin. `:280` is `R4A-1`.
- `INC2-7` (the round-2 accepted residual, the correlation rule at `:234`): present and unchanged, `grep -c validation-constraints` over the sidecar returns 4, matching round 3's measurement. Not raised.
- `F-5` (the round-1 accepted residual): confirmed via the same `validation-constraints` count above and by reading `:234`'s surrounding paragraph directly; unchanged. Not raised.
- `status-resume-ignores-json.md`: zero diff between `d9726fa` and `e34c2c9` (`git diff --stat` empty); untouched by both passes, as expected, and I swept it for every edited string above anyway (no hits beyond the ones already listed).
- The CHANGELOG.md bullet added by `R3B-2` (`:365`): re-read for internal consistency; matches the established shorthand used elsewhere in the same file for the non-instrumented population (`:300`, "for every non-instrumented project"). No defect.

WHAT I DID NOT FIND: no partial application of any of the eight round 3 fixes; no site where a prescribed deletion left stray punctuation or a broken sentence; no numeral that now heads an enumeration out of step with a changed count; no cross-reference restating a fact either pass changed; no adjacent sentence made false by either pass's edits (each surrounding paragraph re-read in full: `:274`, `:278`, `:282-290` together, `:296-300` together, `:302-306`, `:329-333`, `:336-366`).

## Commands run

```
git log --oneline -5
git show 1a1655c --stat
git show e34c2c9 --stat
git diff d9726fa 1a1655c -- <sidecars + rendered doc>
git diff 1a1655c e34c2c9  (full, and scoped to sidecars+toml)
grep -n / grep -c sweeps listed inline above
cargo run -- render docs/plans/agent-scaffold.plan.toml --check
cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
TMPDIR=/tmp/r4a-scratch cargo test
cargo clippy --all-targets -- -D warnings
```
