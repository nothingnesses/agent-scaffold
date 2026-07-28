# Step 92 `prompt-drift-guard`: work review round 3, THE COLD READER lens

Artifact: `src/agents_md_drift.rs` at `9174a74` ("docs: consolidate the drift guard's coverage prose into one COVERAGE block").
Worktree: `.claude/worktrees/rev3-pdg-reader`, detached at `9174a74`. Every mutation below was reverted with the Edit tool; `git status --porcelain` is empty and `git diff` is empty at the end. `cargo test` on the final tree: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed. I ran no `nix fmt` or `just fmt`, and edited no source, plan, or pack file as a deliverable.

Method, as briefed: I read `.agents/prompts/reviewer.md` and `AGENTS.md`, then read ONLY the comments of `src/agents_md_drift.rs` (module doc plus every `///` and `//` line, extracted with `grep -n '^\s*//'`), wrote down what I believed, and only then read the code and tested each belief. I read `docs/plans/agent-scaffold.reviews/` last, after the beliefs were fixed.

## Part 1: the beliefs I formed from the comments alone

Recorded before reading any code, so the mismatches below are real data rather than hindsight.

- B1. Guarded: (1) root `AGENTS.md`, (2) `.agents/AGENTS.reference.md`, (3) every asset of the pinned render whose `dest` starts with `PROMPT_DEST_PREFIX`, compared against the file at the same relative path under `CARGO_MANIFEST_DIR`.
- B2. The render is pinned to `just scaffold-self`: built-in pack, `--principles default`, `--instrument`, `Detail::Summary`, no `--var`, no `--module`.
- B3. Both sides of all three comparisons go through `assert_no_unprotected_construct`, then `normalize_wrapping`, then equality.
- B4. Everything else the scaffold emits or the repo commits is unguarded by this module, stated as a rule, illustrated by `.agents/user-prompts/`, `.agents/LEDGER.template.md`, the `.agents/` TOML copies, and the `docs/plans/TEMPLATE` family.
- B5. Editing a `pack/prompts/<role>.md` that has a non-module-tagged `[[asset]]` row and not regenerating FAILS check 3. Editing a `pack/prompts/` file with no manifest row fails nothing (and ships nothing).
- B6. Hand editing a deployed copy (`.agents/prompts/*.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`) FAILS.
- B7. Adding a new prompt = a new `pack/prompts/` file plus an `[[asset]]` row in `pack/pack.toml` with a `dest` under the prefix; it is then guarded with no edit to this file. Forgetting to commit the copy panics with a missing-file message.
- B8. Deleting: removing the committed copy while the row remains FAILS loudly; removing the row while the copy remains is R1 and stays green.
- B9. Reformatting: a pure re-wrap normalizes away and PASSES; a reformat that introduces indentation or a multi-space run trips the precondition and FAILS loudly.
- B10. R1 = check 3 is one-way (render -> committed), so a committed file under the prefix that the pinned render does not emit is invisible. Reached four ways, and in each of them "the copy is orphaned and the suite stays green".
- B11. R2 = the precondition is per-line and says nothing about the cross-line join, so a construct whose lines are each canonical but whose meaning depends on NOT being joined is joined anyway and can mask drift; a raw HTML block is the known instance; no guarded file has one today.
- B12. `.agents/prompts/checks-reviewer.md` is module-gated, so it is not rendered, not guarded, and the repo commits no copy of it. To see the gating I would look in `src/manifest.rs` (line 80 says so), even though line 52 says asset rows live in `pack/pack.toml`.
- B13. The non-vacuity assertion catches only a total collapse of check 3's filter.
- B14. Comments past the "End of COVERAGE" marker cite the block and make no coverage claims of their own.

## Part 2: which beliefs held

Held, verified against the code and by execution: B1, B2, B3, B4, B5, B6, B7, B9, B11, B13.

- B5/B6/B9 verified by mutation, see RD-2's evidence block and the runs below.
- B2 verified at `src/agents_md_drift.rs:135-142`: `build_assets(&source, &selected, pack::Detail::Summary, &HashMap::new(), true, &[])`, and against `justfile:47`.
- B4's cost claim verified: `.agents/principles.toml` does carry indented array continuations (`.agents/principles.toml:248-249`, `:264`), so the stated reason the TOML copies need a different mechanism is accurate. The claim that the Markdown copies "already satisfy the precondition" also holds: I ran the exact per-line canonical-form predicate (fence-exempt) over `.agents/user-prompts/*.md`, `.agents/LEDGER.template.md`, and the ten `docs/plans/TEMPLATE.*` sidecars, and all pass. (Only the generated `docs/plans/TEMPLATE.md` view fails, at its line 45 `| `example-step` | not started |  |`, and that file is not a manifest asset so widening check 3's filter would not pick it up.)
- B11's "no guarded file carries such a construct today" verified: `grep -n "^<"` and `grep -c '^```'` over all nine guarded files return zero HTML block starts and zero fences.
- The inherited "Empirically ... byte-identical" paragraph (`:27-32`) is FACTUALLY TRUE today. I rendered the pinned config to a temp directory and byte-compared: `AGENTS.md` SAME, `.agents/AGENTS.reference.md` SAME, all seven `.agents/prompts/*.md` SAME. Not a finding; recorded because I was asked to raise it if wrong, and it is not.

Did not hold: B8 (partly), B10, B12, B14 (partly), plus one belief about the in-test precondition comment. Those are RD-1 to RD-4.

## Findings

Four findings, all `low`, all comment-only. No `medium`, `high`, or `critical` finding. Nothing in the mechanism: every behavioural probe I ran matched what the code should do; the gaps are all between what a comment says and what the code does.

### `RD-1`: R1's "the suite stays green" is false for three of the four ways it says the residual is reached

SEVERITY: `low`. Doc-only. The most consequential of my four.

WHERE. `src/agents_md_drift.rs:74-77`:

    //! prefix that the pinned render does not emit is invisible to it. Reached by deleting an
    //! asset row from `pack/pack.toml`, by module-tagging one (the pinned config selects no
    //! modules, so a tagged row is not rendered), by changing a row's `dest`, or by hand-placing
    //! a stale extra file in `.agents/prompts/`: the copy is orphaned and the suite stays green.

THE BELIEF A READER FORMS. R1 lists four changes and closes with one outcome clause covering all four: after any of them, the copy is orphaned AND the suite stays green. I believed I could make any of those four changes and see a green `cargo test`.

WHAT THE CODE DOES. Only the fourth is green. Three of the four produce a red suite, one of them from this module's own guard.

Path 1, DELETE the `[[asset]]` row for `prompts/reviewer.md` from `pack/pack.toml` (leaving `.agents/prompts/reviewer.md` committed):

    cargo test
    -> test manifest::tests::builtin_manifest_lists_the_expected_assets ... FAILED
    -> test result: FAILED. 366 passed; 1 failed

The drift guard is green (the orphan really is invisible to it, so R1's mechanism claim is correct), but `src/manifest.rs:584` asserts an exact 30-entry dest list and fails.

Path 2, MODULE-TAG the same row (add `module = "checks"`, changing nothing else):

    cargo test
    -> test manifest::tests::builtin_manifest_lists_the_expected_assets ... FAILED
    -> test manifest::tests::builtin_checks_module_adds_its_five_assets ... FAILED
    -> test result: FAILED. 365 passed; 2 failed

Path 3, CHANGE the row's `dest` to another path under the prefix (`.agents/prompts/reviewer.md` -> `.agents/prompts/reviewer-renamed.md`). This one fails inside THIS module:

    cargo test --bin agent-scaffold the_committed_role_prompts_match_a_fresh_render
    -> panicked at src/agents_md_drift.rs:162:13:
       failed to read the committed .agents/prompts/reviewer-renamed.md at
       <worktree>/.agents/prompts/reviewer-renamed.md: No such file or directory (os error 2).
       The self-scaffold render produces this file, so the repo must commit it; run `just scaffold-self`
    -> test result: FAILED

The old copy is orphaned as R1 says, but the guard is loud about the new dest, so "the suite stays green" is wrong even for this module alone. (Renaming a `dest` OUT of the prefix would be green in this module but still red in `builtin_manifest_lists_the_expected_assets`.)

Path 4, HAND-PLACE a stale extra file (`.agents/prompts/stale-orphan.md`, no asset row):

    cargo test
    -> test result: ok. 367 passed; 0 failed  (and 5 + 1 + 3 + 1 + 2 in the integration binaries)

Green, as claimed. This is the one path the sentence describes correctly.

WHY IT MATTERS RATHER THAN BEING A SAFE-DIRECTION SLIP. R1 is one of the two residuals the block asks every other comment to cite instead of re-deriving, so its text is the project's record of how big the hole is. As written it reads as "these four changes are undetectable", when the suite detects three of them; someone deciding whether to close R1 (a candidate backlog item) would over-value the fix, and someone who made one of these changes on the strength of this sentence would meet an unexpected red suite in a file this comment never mentions.

PROVENANCE, checked because it bears on scope. The clause is NEW in the artifact under review. `git show 9174a74 -- src/agents_md_drift.rs` shows the deleted line 81 `neither rendered nor guarded and the suite stays green` (the pre-consolidation text, where the clause attached to the unregistered-pack-file case and was TRUE, as round 2's verification reviewer confirmed) and the added line 137 carrying the same clause onto the four orphan paths, where it is false for three. The consolidation moved a true clause onto claims it does not hold for. This is not the R1 mechanism (accepted, untouched by this finding): I ask for no mechanism change, no exclusion, and no new test.

WHAT A FIX MUST ACHIEVE. Stop attaching one outcome clause to four different changes. Either scope it to this module ("check 3 stays green"; that is true of all four), or keep "the suite" and name the one path it holds for.

### `RD-2`: the in-test precondition comment still claims a loudness the block's R2 denies, at the site round 2 required to be narrowed

SEVERITY: `low`. Doc-only. Not a re-raise: the finding it descends from (`V2-1`) was ruled VALID, not dismissed; this is fix incompleteness.

WHERE. `src/agents_md_drift.rs:387-392`:

    // Precondition for normalize_wrapping's safety argument (see its doc comment):
    // both the fresh render and the committed copy must be free of any
    // indentation- or whitespace-significant construct, or equal normalization
    // would no longer imply equal content and the equality checks below could pass
    // on masked drift. Asserted on both sides so the guard fails loudly the day
    // such a construct enters the guidance.

THE BELIEF A READER FORMS. If a whitespace-significant construct enters the guidance, the guard fails loudly. A construct whose meaning depends on its line breaks (an HTML `<pre>` block being the obvious case) is whitespace-significant on any ordinary reading of the phrase, so a reader includes it.

WHAT THE CODE DOES. It passes silently and masks a real content change. Temporary probe inside the module (added, run, reverted with Edit):

    let multi  = "# T\n\n<pre>\nline one\nline two\n</pre>\n";
    let single = "# T\n\n<pre>\nline one line two\n</pre>\n";
    precondition rejects multi : false
    precondition rejects single: false
    normalized multi : "# T\n\n<pre> line one line two </pre>"
    normalized single: "# T\n\n<pre> line one line two </pre>"
    MASKED (normalize equal): true

Two inputs that differ in `<pre>`-significant content both pass the precondition and compare equal. That is exactly R2 (`:86-99`), and the block itself says so at `:177-179` ("It is a per-line check, so it does not cover the class of constructs described by residual R2 ... Do not read a pass here as the precondition being fully established"). So the file states the qualification in one place and denies it 200 lines later.

THE STRUCTURAL POINT, AND WHY I RAISE IT DESPITE THE NEARBY POINTER. This is the exact text round 2's triage required to be fixed: `prompt-drift-guard-r2-triage.md:97`, "Secondarily, `src/agents_md_drift.rs:401-402` ('Asserted on both sides so the guard fails loudly the day such a construct enters the guidance') must be narrowed to the per-line scope the predicate has ... it is the same defect and should be fixed in the same pass." The consolidation did not touch it. `git show 9174a74^:src/agents_md_drift.rs` lines 397-402 are byte-identical to the current `:387-392`. Under the option the human chose, this site should have become a citation of R2 like the other eight; it is the one surviving totality claim outside the block, and the block's own closing marker (`:101`) promises that comments past it cite rather than restate.

WHAT A FIX MUST ACHIEVE. Delete the loudness clause and cite R2, in the form the block prescribes: the assertion is per-line, it rules out the constructs `assert_no_unprotected_construct` can see, and the cross-line class is R2.

### `RD-3`: R1 cites the wrong file for where the `checks-reviewer` row is module-gated, contradicting the same block two paragraphs earlier

SEVERITY: `low`. Doc-only.

WHERE. `src/agents_md_drift.rs:79-82`:

    //! The standing benign instance is
    //! `.agents/prompts/checks-reviewer.md`, whose row is module-gated in `src/manifest.rs`, so
    //! the pinned render omits it, check 3 omits it, and the repo commits no copy for it to
    //! drift from; it needs no explicit exclusion and has none.

THE BELIEF A READER FORMS. The asset row and its module tag live in `src/manifest.rs`. That directly contradicts `:51-53` of the same block, which says an `[[asset]]` row is "added to `pack/pack.toml`".

WHAT THE CODE DOES. The row and its `module = "checks"` tag are data, in `pack/pack.toml:219-223`:

    [[asset]]
    source = "prompts/checks-reviewer.md"
    dest = ".agents/prompts/checks-reviewer.md"
    ownership = "reference"
    module = "checks"

`src/manifest.rs` holds the loader (`builtin()` at `:319-321` returns the embedded `pack/` directory; `expand_modules` at `:356` applies the gate) and two tests that assert the gating behaviour (`:584`, `:649-692`). It contains no asset row. `grep -rn "checks-reviewer" src/ pack/` returns the pack.toml row plus test-list entries at `src/manifest.rs:658,685` and `src/main.rs:2094`, which is what a reader following this pointer would find and could mistake for the registration.

WRONG-ACTION RISK, which is why I raise a citation error at all. A maintainer acting on R1 (un-gating the prompt to bring it into the guarded set, or module-tagging another row to check the residual is real) is sent to a Rust file to change a TOML row. The block is the file's designated single source for coverage facts, so a wrong file name in it is worth one clause to fix. The rest of the sentence is accurate: `git ls-files .agents/prompts` returns seven files and no `checks-reviewer.md`, and the pinned render omits it, so the "no committed copy to drift from" claim holds.

Note the phrase is carried forward, not invented here (the pre-consolidation text at `9174a74^` line 96 said "it is module-gated in `src/manifest.rs`"), but `git show 9174a74` shows the current sentence as an ADDED line, and no prior round raised it, so it is neither settled nor out of scope.

### `RD-4`: the thematic-break comment describes a space-insensitive rule the code does not implement

SEVERITY: `low`. Doc-only, or a one-line predicate change if the intent was the stated rule.

WHERE. `src/agents_md_drift.rs:255-256`:

    // Thematic break: three or more of the same marker (`-`, `*`, or `_`) and
    // nothing else once spaces are removed.

and `is_hard_start`'s own doc at `:225-227` lists "a thematic break" without qualification.

THE BELIEF A READER FORMS. Thematic breaks written with spaces between the markers are recognised, because the comment says the test is applied "once spaces are removed".

WHAT THE CODE DOES. `:257-263` tests `bytes.iter().all(|&b| b == marker)` on the line as given; spaces are never removed. Temporary probe (added, run, reverted with Edit):

    assert!(is_hard_start("---"));    // passes
    assert!(is_hard_start("- - -"));  // passes, but via the LIST-MARKER branch at :244, not this one
    assert!(is_hard_start("* * *"));  // same
    assert!(is_hard_start("_ _ _"));  // FAILS: panicked at src/agents_md_drift.rs:466:9

`_ _ _` is a valid CommonMark thematic break, the comment says it is handled, and it is classified soft and joined onto the preceding logical line. The two spaced forms that do pass do so by accident of the `- `/`* ` list-marker branch, so the comment also misdescribes why they work.

IMPACT AND ITS LIMIT. This is an instance of the class R2 already accepts, and no guarded file contains a spaced thematic break today, so nothing is masked now. I raise it because the comment is the thing a maintainer reads when deciding whether a construct they are about to add needs recognition (the same reason round 2 rated the `is_hard_start` site the most consequential of its `low` findings), and this one tells them the space case is covered when it is not. The text is inherited from `cba4fcc` (step 80) and untouched by this step: `git log -S "once spaces are removed" -- src/agents_md_drift.rs` returns `cba4fcc` only. I raise it under the "may raise if factually WRONG" allowance rather than as scope expansion, and it is a one-clause fix ("three or more of the same marker and nothing else, with no spaces") or a one-line predicate widening.

## Checked and NOT raised

Recorded so the triager can see the negative results, and so a later round does not re-derive them.

- THE MARKER AT `:101` IS OTHERWISE HONOURED. I ran round 2's own greppable criterion (`prompt-drift-guard-r2-triage.md:184`): no "every", "only", "all four", "not total", "authoritative", or "exhaustive" appears in any comment outside the COVERAGE block. The two test-site comments at `:377-383` and `:418-427` do describe what their check catches, which is closer to restating than to citing, but every statement in them is TRUE (verified by the mutations in RD-1's evidence) and neither quantifies over the coverage set. The one surviving quantified claim is RD-2's. Not a finding.
- THE GUARDED SET, THE COMPLEMENT RULE, AND THE TWO-WAY CONTENT PROPERTY are accurate as stated. Pack-source edit fails (`:452` assertion, reproduced), deployed-copy hand edit fails (same assertion), `AGENTS.md` hand edit fails (`:404` assertion), reflow passes (I split a paragraph of `.agents/prompts/reviewer.md` across two lines: all five `agents_md_drift` tests green).
- THE `include_dir!` REBUILD QUESTION. A reader could reasonably worry that a `pack/` edit is invisible to a stale build; it is not, `build.rs:22` tracks the directory, and my path-1 and path-3 mutations both took effect on the next `cargo test`. Not a finding, and the module doc has no obligation to mention it.
- THE JUSTFILE CONFIG. `justfile:47` is `scaffold --output-dir . --write --force --principles default --instrument`; `:129` quotes it as `scaffold --principles default --instrument`. The omitted flags are output plumbing and change no rendered byte. Not a finding, and the duplication itself is already settled.

## Direct answer to the brief

WOULD A COMPETENT DEVELOPER, READING ONLY THE COMMENTS, BE CORRECTLY INFORMED? NO, but narrowly, and the failures are not where the previous six findings were.

Of the five questions I was asked to answer from the comments alone, four are answered correctly: editing a `pack/prompts/` source fails, editing a deployed copy fails, adding a prompt is guarded automatically by adding an `[[asset]]` row (in `pack/pack.toml`, per `:52`, though `:80` then names the wrong file), and reformatting is tolerated unless it introduces an indentation- or whitespace-significant line. The consolidation worked for the thing it was built for: the guarded set is defined by naming the filter, and I could not find a gap between that definition and the code.

Where it fails is the DELETE question and the residuals. I could state R1 and R2 from the comments alone, and my statement of R2 was correct, but my statement of R1 carried a false outcome ("the suite stays green") for three of the four changes R1 itself lists, and I could not tell from the comments that a `dest` rename would panic in this very file. Separately, one totality claim survives outside the block (`RD-2`) and contradicts R2, which is the same species the consolidation was meant to end, at the one site round 2 had already flagged and required to be fixed.

## Tree state

`git status --porcelain` shows only this untracked findings file. `git diff` is empty. HEAD is `9174a74`. `cargo test`: 379 passed, 0 failed on the final tree. Every mutation (two `pack/pack.toml` row edits, one `pack/prompts/reviewer.md` edit, two `.agents/prompts/reviewer.md` edits, one `AGENTS.md` edit, two temporary probe tests in `src/agents_md_drift.rs`, one planted `.agents/prompts/stale-orphan.md`) was reverted with the Edit tool or removed by deleting the exact file added.
