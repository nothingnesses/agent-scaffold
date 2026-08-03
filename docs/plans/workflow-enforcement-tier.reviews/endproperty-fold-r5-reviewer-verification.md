# `workflow-enforcement-tier`, `Q-55-endproperty` fold, round 5 REVIEWER VERIFICATION of the authorised escalation fix

Reviewer: independent of the planner, of all prior reviewers, of all four triagers and of the round 4 backstop. READ-ONLY with respect to the reviewed artifact; this file is the only thing written and no fix is applied.

Worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-ep5-verify`, branch `review/q55-ep5-verify`, at `cda71ee`. Binary built here with `cargo build` (increment 1 in the tree, increment 2 absent, so every increment-2 verdict below is a hand computation over MEASURED operands and is labelled as one).

POSTURE. This is the verification round after an escalation, not the first round of a fresh convergence loop. The question is whether the authorised fix landed as prescribed and whether it re-seeded anything. Zero findings would have merged, so a finding here costs a merge and I only file what I can reproduce.

## Verdicts

| id | severity | one-line |
| --- | --- | --- |
| `R5A-1` | medium | `:183`'s new nesting condition is SYMMETRIC and is false in the other nesting direction, where the checked-plan rooting DOES catch the pairing; it contradicts `:229`, corrected by the same commit, on the same layout. |
| `R5A-2` | medium | `:267`'s new "the ledger being echoed verbatim" is unqualified, and `:182`'s own rule OMITS the block on `status --resume` for exactly that layout, so a claim in "What this step does not fix" is wrong for one of the two ledger surfaces. |
| `R5A-3` | medium | `:271`'s new "its direction is the opposite one, an over-refusal rather than a silence" is contradicted by accepted cost (i) at `:255` in the same file, and by measurement: at a conventionless root the fallback produces the in-root bound's own silence. |
| `R5A-4` | low | `:257`'s prescribed mechanism-clause replacement orphaned "and the guard wins"; the planner repaired the sibling antecedent ("the disagreement") and left this one. |

FOUR FINDINGS, ALL IN THE 265 AUTHORED WORDS, NONE IN THE MECHANISM. Three of the four are in part 3 and part 4, the two parts the fix's own measure did not price. Part 1 and part 2, which were prescribed word for word by the round 4 triage, produced one finding between them and it is the low.

## Job 1: did each authorised part land, at the prescribed site, in the prescribed shape

Every site below was located by content rather than by inheriting a line number, and every prescribed site has its projection twin. `git diff HEAD~1 HEAD --stat` is two files, 18 lines changed, 9 in each, and the two sets are identical.

PART 1, `R4A-2`, THREE SITES. LANDED, all three.

- Site 1, `:257` (projection `:1652`). The heading is now "A SYMLINK ON THE PLAN'S OR THE LOG'S PATH BECOMES A FALSE POSITIVE ON THE PREDICATE"; the mechanism clause is now "the canonicalised plan and the canonicalised log land under different roots"; and the added rule clause is "THE COST IS THE DIVERGENCE AND NOT THE LAYOUT: any symlink that makes the canonicalised checked plan and the canonicalised resolved log fall under different roots produces it, on either side, and `docs/plans` is the placement that was MEASURED rather than the population." That is the triage's prescription (heading to a rule, mechanism to "land under different roots", one added clause of about 40 words with NO enumeration) in its prescribed shape. `git diff --word-diff` confirms the added clause is 44 words and contains no placement list, so it cannot go stale on a sixth placement. See `R5A-4` for the one antecedent this site left behind.
- Site 2, `:339` (projection `:1734`), check 19. Added: "A SECOND LAYOUT PINS THE LOG SIDE: `<root>/docs/metrics` a SYMLINK to a sibling directory, with the plan where it belongs, gives the same refusal and the same omission." 28 words against the prescribed 25. I BUILT THAT LAYOUT AND MEASURED BOTH OPERANDS rather than trusting the triage's table:

```
$ ln -s $S/P2/shared-metrics $S/P2/proj/docs/metrics
$ agent-scaffold validate --source $S/P2/proj/docs/plans/p.plan.toml --workflow
/tmp/claude-1000/rev-ep5-scratch/fix/P2/proj/docs/metrics/workflow.jsonl: <n> records, valid
/tmp/claude-1000/rev-ep5-scratch/fix/P2/proj/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
... vs .../P2/proj/docs/metrics/workflow.jsonl: workflow invariants hold
$ agent-scaffold status --source $S/P2/proj/docs/plans/p.plan.toml
plan: 1 steps (1 complete); 0 open-questions items
metrics: <n> records
$ realpath $S/P2/proj                                    # root of the checked plan
/tmp/claude-1000/rev-ep5-scratch/fix/P2/proj
$ realpath $S/P2/proj/docs/metrics/workflow.jsonl        # resolved log, canonicalised
/tmp/claude-1000/rev-ep5-scratch/fix/P2/shared-metrics/workflow.jsonl
```

  (`<n> records` is elided deliberately; a standing rule forbids asserting a count of anything in `docs/metrics/workflow.jsonl`, and nothing in this finding turns on the number.) The layout is green at exit 0 today and prints its metrics half today; the canonicalised log is not under the root, so after inc2 it refuses and the projections omit, under BOTH readings of `:157` (the leaf exists and the deepest existing directory is the symlink itself, so the two readings coincide here). The pin is correct and reading-independent, which is what the triage said the log-side pin had to be.
- Site 3, `:157` (projection `:1552`). Reads "canonicalising its longest existing ancestor, the path itself when it exists, and re-appending the components below it". That is the prescribed 6-word insertion at the prescribed place, taking the REFUSING reading. It does not collide with `:165`'s lexical/canonical split (the insertion is inside the GUARD's resolution, not the default's) and it makes cost (ii)'s new "on the LOG'S PATH" heading cover the symlinked-log-FILE placement, which under the other reading would have been silent. Consistent.

PART 2, `R4A-3`, ONE SITE. LANDED. `:182` (projection `:1577`) now reads "a `--source` and a `--plan` that both exist must resolve to the SAME root or the block is omitted". `grep -rn "must resolve to the SAME root"` over `docs/plans/agent-scaffold.steps/`, `agent-scaffold.plan.toml`, `agent-scaffold.md` and `agent-scaffold.ledger.md` returns exactly those two lines, so there is no third site carrying the old wording.

PART 3, THE RULE. LANDED, and it is the site of `R5A-2`. `:267` (projection `:1662`) now opens "THE IN-ROOT BOUND. CONTAINMENT REFUSES ONLY WHAT LIES OUTSIDE THE CHECKED PLAN'S ROOT SUBTREE, so every foreign artifact inside that subtree is invisible to it: a log copied to this plan's own `docs/metrics/`, and equally a NESTED project's own log and ledger at their own conventional paths, the log then joining by bare slug and the ledger being echoed verbatim." The copied-log INSTANCE is retained as the measured example ("A measured the copied case by copying ...") rather than as the framing, which is the replacement the human authorised. It is NOT recorded as a fifth accepted cost: `:251`'s heading still reads "The four accepted costs" and no `(v)` exists.

PART 4, THE LEDGER HALF. LANDED, and it is the site of `R5A-1` and `R5A-3`. All three prescribed pieces are present: `:183`'s "catches that" is qualified, `:229`'s second member is restated as containment, and `:269` carries "THE QUEUED STEP OWNS THE LOG HALF OF THE IN-ROOT BOUND ONLY: filtering `Round` records cannot change which ledger file `src/main.rs:run_next` opens, so the LEDGER half of that bound has NO OWNER in this plan today, recorded here rather than scheduled." I checked that last claim against the code rather than against the backstop: `src/main.rs:run_next` opens the ledger at `.ledger_fragment.clone().unwrap_or_else(|| default_ledger_path(&task, &args.source, &args.plan))`, and `src/main.rs:default_ledger_path` resolves `anchor.parent()...join(format!("{task}.ledger.md"))` with the anchor `source.as_ref().or(plan.as_ref())`, so no filter over `Round` records can reach it. TRUE.

## Job 1b: the scope lock

NOTHING CROSSED A LINE. Each item checked rather than assumed:

- NO MECHANISM CHANGE. The predicate is still containment at `:157`, `:159` and `:179`; `:161`'s two rejected alternatives are untouched; the human's declined same-project-root test is not reintroduced anywhere.
- NO FIFTH ACCEPTED COST. `:251` still says four and the enumeration still stops at (iv).
- NO NEW `[[step]]` OR `[[question]]`, NO STATUS / INCREMENT / WAIVER CHANGE, NO RUST, NO LEDGER OR REVIEW-FILE EDIT. `git diff HEAD~1 HEAD --stat -- docs/plans/agent-scaffold.plan.toml docs/plans/agent-scaffold.ledger.md docs/plans/workflow-enforcement-tier.reviews/ src/ tests/` is EMPTY. The step count is still 95 and the question count still 69 (`validate` prints both).
- NO HAND-EDIT OF THE PROJECTION. `agent-scaffold render --check --strict docs/plans/agent-scaffold.plan.toml` prints "docs/plans/agent-scaffold.plan.toml: up to date" at exit 0, so `docs/plans/agent-scaffold.md` is byte-identical to a fresh render.
- NO ASSERTED COUNT FROM `docs/metrics/workflow.jsonl`. `git diff --word-diff` shows the only record counts in the touched lines ("235-record", "37-record") are CONTEXT, not additions, and both are past-tense provenance of explorer A's own measurement rather than a present-tense count. The fix asserts no count of anything.
- GUARDS. `cargo test` passes with 0 failures (373 in the lib target plus the integration binaries, including `agents-md-drift-guard` and `prompt-drift-guard`); `validate --source docs/plans/agent-scaffold.plan.toml --plan docs/plans/agent-scaffold.md --workflow` exits 0 with `workflow invariants hold`. `nix fmt` was NOT run.
- ASCII. `git diff HEAD~1 HEAD | grep -nP '^\+.*[^\x00-\x7F]'` returns nothing.

## The fixtures, built from descriptions, in my own scratch tree

`TMPDIR=/tmp/claude-1000/rev-ep5-scratch`, `S=$TMPDIR/fix`, `AS` the binary built in this worktree. Every fixture is a `agent-scaffold scaffold` project, so both projects in each pair are CONVENTIONALLY laid out and no root below goes through `src/main.rs:project_root_of_source`'s fallback except where I say so. EVERY ROOT IS READ OFF THE BINARY by asking where the DEFAULT log is and reading the path out of its `no metrics log at <path>` note; the containment test itself is the one unbuilt step and is a path-prefix comparison with no free parameters. `find $S/N $S/F $S/G -type l | wc -l` returns `0`, so canonical and lexical coincide in all three and the two readings of `:157` cannot separate them.

`N`, REVERSE NESTING (the outer project supplies the `--source`, the inner supplies the checked plan):

```sh
"$AS" scaffold --output-dir "$S/N/P" --write --force --principles default
"$AS" scaffold --output-dir "$S/N/P/packages/projQ" --write --force --principles default
sed 's/^primary = "toml"/primary = "markdown"/' "$S/N/P/docs/plans/TEMPLATE.plan.toml" > "$S/N/P/docs/plans/A.plan.toml"
sed -e 's/| `example-step` | not started |/| `triager-runs-only-on-findings` | complete |/' \
    -e 's/### `example-step`:/### `triager-runs-only-on-findings`:/' \
    "$S/N/P/packages/projQ/docs/plans/TEMPLATE.md" > "$S/N/P/packages/projQ/docs/plans/Q.md"
printf '# Ledger for P\n\n## RESUME STATE\n\nMARKER-OUTER-P-RESUME-LINE\n\n## Other\n\ntail\n' > "$S/N/P/docs/plans/A.ledger.md"
```

`F`, FORWARD NESTING (the same two projects with the roles swapped: the inner project supplies the `--source`, the outer supplies the checked plan). This is the round 4 backstop's `F1` rebuilt independently, and I rebuilt it because `R5A-2` needs both surfaces on it.

`G`, THE CONVENTIONLESS ROOT: a TOML-primary `myplan.plan.toml` at `$S/G/repo` with no `docs/` of its own and a scaffolded project at `repo/vendor/projA` carrying a copy of this repository's log at its own conventional path.

## `R5A-1` medium. `:183`'s new nesting condition is symmetric, and one of the two directions is the opposite of what it says

SITE: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:183`, projection `docs/plans/agent-scaffold.md:1578`. Both clauses are ADDITIONS of this commit (`git diff HEAD~1 HEAD --word-diff` shows `[-that;-]{+that WHERE THE TWO PROJECTS DO NOT NEST;+}` and `[-case.-]{+case, and where they DO nest neither rooting catches it (the IN-ROOT BOUND below).+}`).

THE SENTENCE, in full, with its subject:

> so a Markdown-primary `--source` in one project beside a `--plan` in another resolves the ledger in the FIRST project while projecting the SECOND project's steps, and echoes one project's `## RESUME STATE` under another's plan on the DEFAULT ledger path. The predicate rooted on the checked plan catches that WHERE THE TWO PROJECTS DO NOT NEST; an anchor-rooted one cannot, for the same reason it cannot catch the metrics case, and where they DO nest neither rooting catches it (the IN-ROOT BOUND below).

"The two projects" are the two the sentence just named, and "nest" is stated symmetrically. Fixture `N` is exactly the configuration the sentence names (a Markdown-primary `--source` in one project, a `--plan` in another) with the two projects NESTED, and on it the checked-plan rooting CATCHES the pairing on both artifacts:

```
$ "$AS" validate --plan "$S/N/P/packages/projQ/docs/plans/Q.md" --workflow
no metrics log at /tmp/claude-1000/rev-ep5-scratch/fix/N/P/packages/projQ/docs/metrics/workflow.jsonl; nothing to validate
    # so root(checked plan) = .../fix/N/P/packages/projQ, through the CONVENTION branch

$ "$AS" validate --source "$S/N/P/docs/plans/A.plan.toml" \
                 --plan "$S/N/P/packages/projQ/docs/plans/Q.md" --workflow
no metrics log at /tmp/claude-1000/rev-ep5-scratch/fix/N/P/docs/metrics/workflow.jsonl; nothing to validate
    # so the resolved LOG is .../fix/N/P/docs/metrics/workflow.jsonl, anchored on the --source

$ "$AS" status --resume --source "$S/N/P/docs/plans/A.plan.toml" \
                        --plan "$S/N/P/packages/projQ/docs/plans/Q.md"
## RESUME STATE

MARKER-OUTER-P-RESUME-LINE
exit=0
    # and $S/N/P/docs/plans/A.ledger.md is the ONLY ledger in the fixture
    # ($ find $S/N -name '*.ledger.md' returns that one path), so that is the resolved ledger
```

THE ONE UNBUILT STEP, stated so it can be checked by eye. Root `.../N/P/packages/projQ`; resolved log `.../N/P/docs/metrics/workflow.jsonl`; resolved ledger `.../N/P/docs/plans/A.ledger.md`. NEITHER artifact is under the root, so the predicate FIRES on both, and `--workflow` refuses while `next` omits its `RESUME STATE` echo. The two projects nest, and the checked-plan rooting catches it.

IT ALSO CONTRADICTS THE SITE THIS SAME COMMIT CORRECTED. `:229`, rewritten by this commit, states the condition DIRECTIONALLY and correctly: "a `--source` in a different project reaches this only when that project is not NESTED inside the root." On fixture `N` the `--source`'s project is NOT nested inside the root, so `:229` says the variant fires, which is right; `:183` says "they DO nest" and so "neither rooting catches it", which is wrong. One commit, two sentences, opposite answers on one layout.

WHY IT IS NOT PEDANTRY, and this is what keeps it above a wording preference. The reverse-nesting layout is an ordinary monorepo invocation (the repository-root plan as `--source`, a package's plan as `--plan`), and inc2 gives it a NEW non-zero exit and a new omission. A reader of `:183` in that layout is told the guard does not reach nested projects and will read the refusal as a regression, which is exactly the reading accepted cost (ii) exists to prevent for its own layout. It also under-describes the guard, in the same class as `R4A-2` but in the opposite direction.

MINIMAL FIX. A deletion is NOT available here, because deleting both clauses restores the unqualified "catches that" the backstop falsified. The minimum is a NARROWING to the containment vocabulary `:229` already uses, at `:183` and its projection: "catches that WHERE THE `--source` LIES OUTSIDE THE CHECKED PLAN'S ROOT; an anchor-rooted one cannot, for the same reason it cannot catch the metrics case, and where it lies INSIDE, neither rooting catches it (the IN-ROOT BOUND below)." That is directional, it introduces no new claim, and it is the same test the paragraph below states. About 4 words changed, net 0.

## `R5A-2` medium. `:267`'s new "the ledger being echoed verbatim" is false for `status --resume`, by the same fold's own rule

SITE: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:267`, projection `docs/plans/agent-scaffold.md:1662`. The clause is an ADDITION of this commit (`[-still greens.-]{+the ledger being echoed verbatim.+}`).

THE NEW SENTENCE:

> THE IN-ROOT BOUND. CONTAINMENT REFUSES ONLY WHAT LIES OUTSIDE THE CHECKED PLAN'S ROOT SUBTREE, so every foreign artifact inside that subtree is invisible to it: a log copied to this plan's own `docs/metrics/`, and equally a NESTED project's own log and ledger at their own conventional paths, the log then joining by bare slug and the ledger being echoed verbatim.

The outcome clause is UNQUALIFIED, and it sits under the heading "What this step does not fix, and where it goes instead", whose stated audience is "an implementer must NOT 'fix' them, and a reviewer must NOT raise them as defects" (`:253`). TWO surfaces echo a ledger, and the document specifies OPPOSITE answers for them on this layout.

MEASURED, on fixture `F` (forward nesting, the layout the sentence describes):

```
$ "$AS" validate --plan "$S/F/O/docs/plans/O.md" --workflow
no metrics log at /tmp/claude-1000/rev-ep5-scratch/fix/F/O/docs/metrics/workflow.jsonl; nothing to validate
    # root(checked plan O.md) = .../fix/F/O

$ "$AS" validate --source "$S/F/O/packages/projA/docs/plans/A.plan.toml" \
                 --plan "$S/F/O/packages/projA/docs/plans/A.md" --workflow
no metrics log at /tmp/claude-1000/rev-ep5-scratch/fix/F/O/packages/projA/docs/metrics/workflow.jsonl; nothing to validate
    # root(--source A.plan.toml) = .../fix/F/O/packages/projA

$ "$AS" status --resume --source "$S/F/O/packages/projA/docs/plans/A.plan.toml" \
                        --plan "$S/F/O/docs/plans/O.md"
## RESUME STATE

MARKER-INNER-PROJA-RESUME-LINE
exit=0                      # today. The resolved ledger is .../F/O/packages/projA/docs/plans/A.ledger.md,
                            # the only ledger in the fixture.
```

Both `--source` and `--plan` EXIST and their roots DIFFER (`.../F/O/packages/projA` against `.../F/O`, both measured above). `:182`, as this same commit left it, is categorical: "a `--source` and a `--plan` that both exist must resolve to the SAME root or the block is omitted". SO AFTER INC2 `status --resume` OMITS THE BLOCK ON THIS LAYOUT. `next` does not: its ledger test is containment (`:183`, `:229`), the resolved ledger IS under `.../F/O`, so containment is silent and the echo stands. The new clause states the `next` outcome as the outcome, and it is wrong for the other surface, on the very layout it was written for.

WHY THIS IS THE FIX'S AND NOT A PRE-EXISTING TENSION. The round 4 backstop recorded the underlying two-surface divergence as "adjacent and NOT filed" (its "corroborating observation" section) and no round has ruled on it. What is NEW is that this commit put an AFFIRMATIVE, unqualified outcome claim into the "does not fix" section, which is the one place in the document a reviewer is instructed not to re-raise what it lists. Narrowing my site does NOT close the underlying divergence (that `:179`'s "the predicate is never re-implemented per surface (One source of truth)" sits above a `status --resume` rule that tests two roots for AGREEMENT rather than containment); I flag that it survives the fix below, and I do not file it, because it is not this commit's work and the backstop already put it in front of the human.

MINIMAL FIX, A DELETION. Drop "and the ledger being echoed verbatim" (6 words) at `:267` and its projection. The sentence then claims only that the nested project's log and ledger are invisible TO CONTAINMENT, which is true on both surfaces, and `next`'s echo is already stated at `:183`. If the outcome is wanted, the alternative is a 2-word narrowing, "the ledger being echoed verbatim by `next`"; I prefer the deletion, since the narrowing writes a second surface-specific outcome into a paragraph whose subject is one predicate.

## `R5A-3` medium. `:271`'s new claim about the fallback's direction is contradicted by accepted cost (i) in the same file, and by measurement

SITE: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:271`, projection `docs/plans/agent-scaffold.md:1666`. Wholly added by this commit:

> That fallback is NOT the in-root bound's cause either; its direction is the opposite one, an over-refusal rather than a silence.

GROUND 1, A QUOTATION 16 LINES ABOVE IT, needing no fixture. Accepted cost (i) at `:255` describes THE SAME FALLBACK producing a SILENCE that containment cannot catch:

> `cd <root>/docs/plans && agent-scaffold validate --source p.plan.toml --workflow` derives the root from a source path with no parents to walk, FALLS BACK TO THE SOURCE'S OWN DIRECTORY, and looks for `docs/metrics/workflow.jsonl` beneath it, which does not exist. The project's real log is never read. ... the containment guard STRUCTURALLY CANNOT catch it, BECAUSE THE WRONG PATH IS STILL INSIDE THE RIGHT PROJECT: containment is not correctness. ... this case becomes a HARD FAILURE naming the path it looked for RATHER THAN A SILENT GREEN

That is the fallback, with the in-root bound's own mechanism (wrong artifact inside the root, containment silent), in the document's own words a "silent green". `:271` says the fallback's direction is "an over-refusal rather than a silence". The two cannot both be true.

GROUND 2, MEASURED, on fixture `G`, which is the conventionless shape both the round 4 triage and the backstop built and which BOTH of them recorded as one route into the bound (the triage: "The conventionless fallback is one way to make the subtree large"; the backstop: "The fallback is one way to widen the subtree"):

```
$ "$AS" validate --source "$S/G/repo/myplan.plan.toml" --workflow
no metrics log at /tmp/claude-1000/rev-ep5-scratch/fix/G/repo/docs/metrics/workflow.jsonl; nothing to validate
    # root = .../G/repo, through the FALLBACK: myplan.plan.toml has no `docs/plans`-shaped ancestor

$ "$AS" validate --source "$S/G/repo/myplan.plan.toml" \
    --metrics "$S/G/repo/vendor/projA/docs/metrics/workflow.jsonl" --workflow
/tmp/claude-1000/rev-ep5-scratch/fix/G/repo/vendor/projA/docs/metrics/workflow.jsonl: <n> records, valid
/tmp/claude-1000/rev-ep5-scratch/fix/G/repo/myplan.plan.toml: 1 steps, 0 questions, valid
... vs .../G/repo/vendor/projA/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
    # the step carries the borrowed slug `triager-runs-only-on-findings` at `complete` and has
    # no evidence of its own; the CONTROL, the same run against an EMPTY log in the same
    # directory, reports `Roadmap step `triager-runs-only-on-findings` is `complete` but has no
    # round records and no covering waiver`, so it is the foreign records that produce the green.
```

The fallback-derived root `.../G/repo` CONTAINS `vendor/projA`, the foreign log is inside it, containment is silent, and the pairing greens. The direction there is a silence, not an over-refusal.

WHAT IS ACTUALLY TRUE, and it is what both prior agents measured: the fallback is SUFFICIENT for the in-root bound and NOT NECESSARY. It widens the subtree when the plan sits at a directory containing other projects and narrows it when the plan sits in a subdirectory, which is why it also causes costs (iii) and (iv). A single direction cannot be attributed to it.

WHY IT MATTERS, given that the human declined a fifth accepted cost precisely because the finding's own suggested record "stated a measurably wrong cause": this sentence exists to stop an implementer queueing the bound under the wrong root cause, and it does that by asserting something about the code that is false. It also mis-scopes the queued work in the other direction, since the rejected alternative to the fallback was a hard error (`Q-55-noconvention`), and taking it would have closed one route into the bound.

MINIMAL FIX, A NARROWING PLUS A DELETION, keeping the authorised content. At `:271` and its projection, replace "That fallback is NOT the in-root bound's cause either; its direction is the opposite one, an over-refusal rather than a silence." with "That fallback is not REQUIRED for the in-root bound." That is what was measured, it still stops the wrong queueing, and it deletes 14 words rather than authoring any. A pure deletion of the whole sentence is NOT preferred, because the human authorised a statement at this site that the bound does not belong to the fallback root cause.

## `R5A-4` low. `:257`'s prescribed replacement orphaned "and the guard wins"

SITE: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:257`, projection `docs/plans/agent-scaffold.md:1652`.

BEFORE: "the LEXICAL DEFAULT and the CANONICAL GUARD DISAGREE about which project the plan belongs to, and THE GUARD WINS".
AFTER: "the canonicalised plan and the canonicalised log land under different roots, and THE GUARD WINS".

The prescribed mechanism-clause replacement removed the only other party in the sentence, so "wins" now has no loser: its two remaining subjects are the canonicalised plan and the canonicalised log, and the guard does not win over either of those. The planner disclosed repairing the SIBLING antecedent this same edit orphaned ("the canonicalising variant that would remove the disagreement" to "the divergence", which now resolves against the added "THE COST IS THE DIVERGENCE" clause) and left this one, in the same sentence.

LOW AND NOT MEDIUM: `:165` defines the guard against the lexical default earlier in the document, and the measured consequence follows immediately in the same sentence ("going from reading its 37-record log to `exit=1 REFUSED`"), so a reader recovers the meaning. It is a readability defect this commit created, not a wrong claim.

MINIMAL FIX, A DELETION. Drop ", and the guard wins" (4 words). The clause after the colon already states the outcome and states it as a measurement.

## Job 3: the two things the planner disclosed

DISCLOSED ITEM 1, THE ONE WORD BEYOND THE THREE PRESCRIBED EDITS. THE PLANNER IS RIGHT, AND IT IS DAMAGE REPAIR RATHER THAN A WIDENING, but the disclosure is INCOMPLETE. The prescribed mechanism replacement deleted the word "disagree", and "the disagreement" 60 words later referred to it, so the repair was forced by the prescribed edit; it is inside the prescribed site (the same paragraph, `:257`); and "the divergence" denotes the same thing the prescribed clause now denotes, so nothing is widened. What the disclosure misses is that the identical edit orphaned "and the guard wins" in the sentence it was editing, which is `R5A-4`. One of two antecedents repaired.

DISCLOSED ITEM 2, THE ROUND 4 TRIAGE'S GROUND FOR EXCLUDING `docs/plans/agent-scaffold.plan.toml`. THE PLANNER IS RIGHT THAT THE GROUND IS FACTUALLY WRONG, AND RIGHT TO LEAVE THE FILE ALONE. The triage's "SITE COUNT MEASURED" says that file "carries `Q-55-mechanism` and `Q-55-noconvention` question records but no symlink text". It does:

```
$ grep -no "SYMLINKED \`docs/plans\` directory becomes a FALSE REFUSAL[^.]*\." docs/plans/agent-scaffold.plan.toml
1714:SYMLINKED `docs/plans` directory becomes a FALSE REFUSAL, the canonicalising variant that would fix it having been measured to turn every printed metrics path absolute, changing output on the correct case.
```

and the ledger carries two more (`docs/plans/agent-scaffold.ledger.md:617` and `:629`). MY VIEW: LEAVING THEM IS CORRECT, for two reasons the triage did not give.

1. All three are inside DATED DECISION RECORDS (`Q-55-noconvention`, human, 2026-07-31, and the design-pass close of the same date). This project's convention is that a dated decision record is corrected by APPENDING, not by rewriting, so rewriting them to the widened rule would misreport what was put to the human on the day.
2. They are not falsified by the widening. Each records what was MEASURED and accepted, and cost (ii) now says in terms that "`docs/plans` is the placement that was MEASURED rather than the population", so the live specification and the historical record agree.

ONE THING A LATER PASS SHOULD NOT INHERIT: the triage's site sweep is wrong as a FACT, so a later pass must not reuse "the plan TOML carries no symlink text" as a settled result. I record that here rather than filing it, because the defect is in a completed review artifact that is read-only history, not in the reviewed product.

## Considered adversarially and NOT raised

- `:269`'s "the LEDGER half of that bound has NO OWNER in this plan today". It is an exhaustiveness claim over the whole plan, so I tested it. No step slug in `docs/plans/agent-scaffold.plan.toml` contains "constraint" or "identity"; the closest candidates by subject (`repoint-resume-prompts`, `resume-state-currency-signal`, `status-resume-ignores-json`, `sidecar-ref-symlink`) are about prompts, currency, `--json` and sidecar refs respectively, none about which ledger file is opened. The one place the document names that WOULD own it, `:161`'s whole-triple parsing ("it touches the resolution `validate`, `status`, `next` and the ledger path all share ... If it is ever wanted it belongs in the validation-constraints step"), is expressly REJECTED ON SCOPE and conditional, so it is not an owner. The claim stands.
- "the validation-constraints step" has no `[[step]]` in the plan at all. That predates this fold by four commits and by two human decisions (`Q-55-scope`, `Q-55-mechanism`) that name it, and the fix neither created nor worsened it. Not this round's subject.
- `:303`'s "(accepted cost (ii), the symlinked `docs/plans` directory)" and its "its ONE false positive". The round 4 triage ruled the first NOT a site because it reads as an example once the cost states a rule, which it now does; the second reads as one accepted-cost CLASS, which the widened cost still is. No residue.
- The retained "the DATA MODEL is untouched, and W3 still joins on a bare slug" at `:267`, now sitting under a paragraph whose subject includes the ledger. It explains the LOG half and does not claim to cover the ledger, and the ledger half's explanation is `:269`'s new sentence. Not a defect.
- `:229`'s "Both members are CONTAINMENT". True of the two members it lists; it makes no exhaustiveness claim, and the enumeration's silence about `status --resume`'s divergent-pairing cause predates this commit.
- The out-of-scope list held: no line-length or wrapping finding, nothing on the increment-1-falsified present-tense `src/main.rs` claims, nothing on increments 1 or 3, nothing on the mechanism defect itself, and none of the four already-ruled residuals re-raised. The six human decisions are untouched by every finding above; all four are prose-scope corrections inside text this commit authored.

## Did the 265 authored words re-seed

YES, THREE FINDINGS, AND THE PATTERN THIS PROJECT HAS MEASURED HELD. Every finding above is in text this commit AUTHORED; not one is in text it DELETED or in the mechanism. The two parts prescribed word for word by the round 4 triage (parts 1 and 2, about 116 words) produced one LOW between them, and it is an antecedent the prescription itself destroyed. The two parts the fix priced at nothing (parts 3 and 4, the rule and the ledger half, about 150 words) produced all three MEDIUMS, and two of the three are the new text disagreeing with text in the same commit or in the same file rather than with the code.

## Scratch hygiene

Every fixture was built under `/tmp/claude-1000/rev-ep5-scratch/`, created for this review and removed after the evidence above was captured. Nothing was written to bare `/tmp`. DIRECTORIES LEFT IN `/tmp`: 0.
