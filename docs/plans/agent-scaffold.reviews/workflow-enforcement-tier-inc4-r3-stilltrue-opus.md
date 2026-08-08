# `workflow-enforcement-tier-inc4`, round 3: the still-true lens

Reviewer lens: attack every claim the pass DECIDED TO LEAVE. Every "still true" judgement is a decision, and a wrong one is invisible because a claim that survived a sweep reads as having been checked.

Tree under review: worktree `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-inc4-r3-b`, branch `review/wet-inc4-r3-b`, at `93ee357`. Every command below was run in that worktree with `TMPDIR` pointed at a scratch directory outside any git repository (`git rev-parse --show-toplevel` there prints `fatal: not a git repository`). Every fixture was built under `.../scratchpad/rev-inc4-r3-b/`. Two `chmod` changes were made and both were restored (`docs/metrics` back to `755`, the log file back to `644`), both inside my own fixture subdirectory.

## Summary

FIVE DEFECTIVE SITES in the sidecar, and TWO further stale documentation sites in shipped `src/main.rs` text, across FOUR findings. 132 discrete claims checked, 127 confirmed TRUE.

| id | severity | one line |
| --- | --- | --- |
| `R3B-1` | medium | Round 2's `R2B-1` was closed at ONE of the THREE sites it named. `:163` and `:179` still assert the checked-plan root as the whole rule, and both are measurably false. |
| `R3B-2` | medium | `:104`, the REQUIRED END PROPERTY, names an exception narrower than the accepted cost it cites and than check 19 which pins it. Measured: the normal invocation IS refused on a layout `:104` does not except. |
| `R3B-3` | medium | `StatusArgs::resume`'s `--help` string and `src/main.rs:1194`'s comment still enumerate TWO causes for the `--resume` note. Inc2 made it three, and their twin at `run_resume`'s doc comment says so. |
| `R3B-4` | low | `:367` quotes `run_resume`'s doc comment in a PRESENT-tense frame, and the quoted clause has no match in the tree. Check 21's own rule says re-tense or delete. |
| `R3B-5` | low | `:157` says the `toml_primary` binding is "made immediately above the match". Since inc2 the containment guard sits between them, 26 lines of it. |

No `high` and no `critical`. I looked for them: the two candidates were `R3B-1` (an unclosed medium from round 2, whose falsity the round 2 triage itself rated medium) and `R3B-2` (a false statement of what "done" means). Neither misleads a user of the tool, because the SHIPPED prose is right in both cases (`README.md:236` states the anchor fallback for all three surfaces, and `README.md:236` states the symlink cost without narrowing it to `docs/plans`). Both are wrong only in the durable design record, which is what caps them at medium.

---

## `R3B-1` (medium). Round 2's `R2B-1` was fixed at one of its three named sites. `:163` and `:179` are still false.

### What round 2 found and what the fix pass did

`workflow-enforcement-tier-inc4-r2-coldread-opus.md` raised `R2B-1` and named THREE sites, its primary at `:157` and, in its own words, "TWO SUPPORTING SITES IN THE SAME FAMILY, both also untouched by both passes, both stating the checked-plan root as the whole rule":

- `:179`: "The trigger is the SAME containment predicate the validator's refusal uses (the canonically-derived root of the plan THAT SURFACE READS, and whether the resolved artifact lives under it)."
- `:163`: "`status --resume` is the one surface that reads NO plan".

`workflow-enforcement-tier-inc4-r2-triage.md:396-400` CONFIRMED the finding at medium and stated a MINIMAL REMEDY that re-tenses only the `:157` sentence. The fix pass applied exactly that. `:157` now reads "so the predicate did not fire and every surface behaved as it did then". `:163` and `:179` are BYTE-UNCHANGED since before round 2:

```
$ git -C <worktree> log -S 'is the one surface that reads NO plan' --oneline -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
(no commit after the round 2 fix passes 7ea9842 or 93ee357 touches it)
```

### Both are measurably false

`status` and `next` also read NO plan whenever a Markdown-primary `--source` resolves neither a TOML-primary source nor a readable `--plan`, and in that configuration the predicate STILL fires, rooted on the ANCHOR through `containment_roots` -> `resume_roots`.

Fixture:

```sh
mkdir -p "$S/r1/away"
printf '[meta]\ntitle = "Away"\nprimary = "markdown"\n' > "$S/r1/away/p.plan.toml"
printf '## RESUME STATE\n\nsecret\n' > "$S/r1/away/p.ledger.md"
```

Run from the agent-scaffold worktree root:

```
$ ./target/debug/agent-scaffold status --json --source "$S/r1/away/p.plan.toml" --metrics docs/metrics/workflow.jsonl
{
  "plan": null,
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project"
}
exit=0

$ ./target/debug/agent-scaffold next --json --source "$S/r1/away/p.plan.toml" --metrics docs/metrics/workflow.jsonl --ledger-fragment docs/plans/agent-scaffold.ledger.md
{
  "task": "p",
  "source": "no plan source",
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  "active_loop": null,
  "resume_state": null,
  "resume_state_absent_reason": "ledger-not-this-project",
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

`"plan": null` and `"source": "no plan source"` are the tool reporting that it read no plan. `log-not-this-project` and `ledger-not-this-project` are the predicate firing anyway. So `status --resume` is NOT the one surface that reads no plan (`:163`), and the trigger is NOT the root of the plan that surface reads (`:179`).

The code says so in as many words, and this is not a subtle reading. `src/main.rs:1406-1438` (`containment_roots`) is titled "One predicate, TWO ROOT-SUPPLY POLICIES, and this is the one place they meet", and `src/main.rs:1717-1723` (in `run_next`) reads "rooted on the plan `next` itself projects from, or on the anchors where it projects from no plan at all".

### A third site of the same family, which round 2 did not name

`:182` scopes the anchor-supplied root to `status --resume` alone ("THIS SURFACE READS NO PLAN, so the rule SUPPLIES a root ... (`Q-55-resumepairing`)"). That sentence is not FALSE, but with `:163` and `:179` uncorrected it is the ONLY sentence in the file that describes the anchor-root policy, and it attributes it to one surface out of three. A reader of `:179` to `:183` cannot reach the implemented rule from the file.

### Scope

IN SCOPE on the round 3 triager's recorded condition 3, which `R1C-3`, `R1C-4` and `Q-55-twinsites` were all admitted on: a stale claim the increment's own change broke is in scope regardless of authorship. Inc2 broke it. It is also, more simply, an already-confirmed round 2 finding whose remedy did not reach two of the three sites the finding named.

NOT a re-raise of `Q-55-fallbacksurface` (ledger `:475`, "ABSORBED, FILE NOTHING"). That decision declined to record the behaviour as a FIFTH ACCEPTED COST with a test. It did not rule that the per-surface behaviour section may state the opposite of the implemented rule.

### Remedy class

DELETION is available here, which `:157`'s remedy note said it was not. At `:163`, deleting the clause "`status --resume` is the one surface that reads NO plan" and keeping "(`src/main.rs:run_resume` derives `<task>` from the source-or-plan filename and reads only the ledger), so it has no checked plan to root on" leaves a true sentence. At `:179`, deleting the parenthetical leaves "The trigger is the SAME containment predicate the validator's refusal uses. The predicate is never re-implemented per surface (One source of truth)", which is true (`is_outside_root` is one function) and carries the paragraph's actual point.

---

## `R3B-2` (medium). `:104`, the REQUIRED END PROPERTY, states an exception narrower than the cost it cites and than the check that pins it.

### The three sites, at `93ee357`

`:104`:

> A run made from the plan's own project root with a bare relative `--source`, which is the normal invocation, must be unchanged (Safe on existing projects), except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii).

`:257`, the cost it points at, was explicitly WIDENED past that layout:

> THE COST IS THE DIVERGENCE AND NOT THE LAYOUT: any symlink that makes the canonicalised checked plan and the canonicalised resolved log fall under different roots produces it, on either side, and `docs/plans` is the placement that was MEASURED rather than the population.

`:342`, check 19, pins the second layout as expected behaviour:

> A SECOND LAYOUT PINS THE LOG SIDE: `<root>/docs/metrics` a SYMLINK out of the plan's project root, with the plan where it belongs, gives the same refusal and the same omission.

### Measured: the normal invocation IS changed on a layout `:104` does not except

`docs/plans` a real directory, `docs/metrics` a symlink out of the root, bare relative `--source`, run from the project root:

```sh
mkdir -p "$S/symlog/root" "$S/symlog/elsewhere"
agent-scaffold scaffold --output-dir "$S/symlog/root" --write --force --principles default
printf '{"type":"round", ...}\n' > "$S/symlog/elsewhere/workflow.jsonl"
ln -s ../../elsewhere "$S/symlog/root/docs/metrics"
cd "$S/symlog/root"
```

```
$ ls -l docs/
lrwxrwxrwx metrics -> ../../elsewhere
drwxr-xr-x plans

$ agent-scaffold validate --source docs/plans/TEMPLATE.plan.toml --workflow
--workflow would join docs/plans/TEMPLATE.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root <S>/symlog/root; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair
exit=1

$ agent-scaffold status --source docs/plans/TEMPLATE.plan.toml
plan: 1 steps (1 not started); 0 open-questions items
metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's project root <S>/symlog/root, so its records cannot be paired with this plan
exit=0
```

This is EXACTLY the invocation `:104` promises must be unchanged, on a layout `:104`'s exception clause does not name.

### Why this is a fix-induced twin-site residue and not original sloppiness

```
$ git log -S 'THE COST IS THE DIVERGENCE AND NOT THE LAYOUT' --oneline -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
a5786ae docs: record the in-root bound as a rule and correct its ledger half

$ git log -S 'A SECOND LAYOUT PINS THE LOG SIDE' --oneline -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
a5786ae docs: record the in-root bound as a rule and correct its ledger half

$ git log -L 104,104:docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md --oneline | grep -E '^[0-9a-f]{7} '
cda0bab / 424b2b8 / 7807c6b / 1a04071
```

`a5786ae` widened cost (ii) at TWO sites and left `:104` behind. `:104` has not been touched since `cda0bab`, which changed a different clause of it. This is the FOURTH recorded occurrence of this project's twin-site failure class (the three already recorded are `Q-55-twinsites`, the `Q-55-w1figure` 13-versus-20 disagreement, and the ledger's own "bitten THREE TIMES by a fix landing at one site while its twin survived a literal grep").

### Impact if unfixed

`:104` is the file's definition of DONE for defect B, and it is the sentence a later reader checks the step against. As written, a reader who meets the `docs/metrics`-symlink refusal reads `:104` and concludes the end property was NOT met, when `:257` and check 19 record the case as accepted. That is precisely the outcome `:253` exists to prevent ("an implementer must NOT 'fix' them, and a reviewer must NOT raise them as defects").

NOT a re-raise of accepted cost (ii) itself, which I am not raising and which `:253` forbids raising. The finding is that `:104` mis-states the cost's scope.

### Remedy class

TOKEN. "except for the symlinked-`docs/plans` layout recorded below as accepted cost (ii)" becomes "except for the symlink divergence recorded below as accepted cost (ii)". Four words deleted, one added, no new fact: `:257` already states the population.

---

## `R3B-3` (medium). Two shipped `src/main.rs` documentation sites still enumerate TWO causes for the `status --resume` note. Inc2 made it three.

### The twin that WAS fixed

`src/main.rs:1629-1636`, `run_resume`'s doc comment, correctly carries all three:

> A missing ledger, an absent section, or a ledger that is not this plan's all print a note and exit 0, since `status` is a best-effort projection, not a validator; the third is a member of that list rather than an exception to it

### The two twins that were NOT

`src/main.rs:1192-1195`, the inline comment in `run_status` describing the same slice:

```
// The thin `status --resume` slice: print the ledger's `## RESUME STATE` block
// verbatim (reusing the same extractor `next` uses) instead of the state projection.
// A missing ledger or absent section is a note and exit 0, not a failure (`status` is
// best-effort).
```

`src/main.rs:StatusArgs::resume`, which is a USER-VISIBLE `--help` string:

```
$ ./target/debug/agent-scaffold status --help
      --resume
          Print the ledger's `## RESUME STATE` block verbatim (from --ledger-fragment, or `<task>.ledger.md` beside the plan source) instead of the state projection. Exits 0 with a note when the ledger or the section is absent
```

### The third cause exists and behaves exactly like the two named

```sh
mkdir -p "$S/res/proj/docs/plans" "$S/res/foreign"
cp docs/plans/agent-scaffold.plan.toml "$S/res/proj/docs/plans/p.plan.toml"
printf '## RESUME STATE\n\nforeign secret state\n' > "$S/res/foreign/p.ledger.md"
```

```
$ agent-scaffold status --resume --source "$S/res/proj/docs/plans/p.plan.toml" --ledger-fragment "$S/res/foreign/p.ledger.md"
the ledger <S>/res/foreign/p.ledger.md is not under the plan's project root <S>/res/proj; nothing to resume
exit=0
```

The ledger exists and carries a `## RESUME STATE` section, so neither named cause applies, and the surface prints a note and exits 0.

### Why this is the same defect the step already fixed four times over

`:200`, `:201` and `:202` record three doc comments corrected for exactly this shape, and `:202`'s wording is the template: "`resume_state`'s doc comment ... WAS SHORT BY ONE IN THE SAME WAY: 'or `None` when the ledger is absent or carries no such section' names two causes, and an unsafe `--ledger-fragment` is a third." All four of those, plus `active_loop`'s, ARE corrected in the tree; I opened each. These two were missed because inc2's documentation-impact bullet at `:367` names only `run_resume`'s doc comment.

IN SCOPE on `Q-55-twinsites`, which is the decision that admitted a code-comment twin of a corrected sidecar claim into inc4, on the ground that "this task has been bitten THREE TIMES by a fix landing at one site while its twin survived a literal grep". The INC4 exclusions bullet at `:388` excludes `README.md`, `pack/AGENTS.md`, the deployed `.agents/` copies and `CHANGELOG.md`. It does NOT exclude `src/main.rs`, which the INC4 list at `:386` already draws from.

The `--help` half is the more expensive one: it is the only one of the three sites a user of the tool reads.

### Remedy class

TOKEN plus a short clause, matching what `run_resume`'s own comment already says, so no new fact is authored:

- `src/main.rs:1194`: "A missing ledger, an absent section, or a ledger that is not this plan's is a note and exit 0, not a failure".
- `StatusArgs::resume`: "Exits 0 with a note when the ledger is absent, carries no such section, or is not this plan's".

### One weaker site in the same family, reported and NOT raised

`src/main.rs:368`, the `status` subcommand summary, says "Best-effort; a missing file yields a partial projection". That is a summary rather than an exhaustive enumeration, and it stays true. I mention it so a fix pass does not treat it as a third site and author prose there.

---

## `R3B-4` (low). `:367` quotes a doc comment in a present-tense frame, and the quotation has no match in the tree.

`:367`, verbatim:

> - `src/main.rs:run_resume`'s doc comment, whose "A missing ledger or absent section prints a note and exits 0" clause gains the unsafe-fragment case as a third member of the same list, not as an exception to it.

```
$ grep -n 'A missing ledger or absent section prints a note and exits 0' src/main.rs
(no output)
```

The nearest live text is at `src/main.rs:1194` and differs ("is a note and exit 0, not a failure"), and `run_resume`'s own doc comment, which the bullet attributes the quotation to, now reads the corrected three-cause form.

Check 21 at `:345` states the rule this violates: "run each quoted fragment of source, test, `README.md` or `pack/AGENTS.md` text as a literal search against the file it is attributed to ... A quotation with no match in the tree is either RE-TENSED, so the sentence describes the pre-increment state it was written about, or DELETED".

The pass's own recorded rule for this family agrees. Ledger `:571`: "THE IMPERATIVE STAYS AN IMPERATIVE AND THE FACTUAL ASSERTION INSIDE IT MOVES TO THE PAST WHERE AN INCREMENT FALSIFIED IT". Here the imperative kept its mood AND its falsified assertion.

THE INCONSISTENCY IS WITHIN ONE FAMILY, WHICH IS WHAT MAKES IT A DEFECT RATHER THAN A STYLE PREFERENCE. The INC1 and INC3 documentation-impact bullets all moved their quotations to the past: `:356` "Each DESCRIBED", `:357` "all of which SAID", `:358` "which STATED ... which RESTATED", `:374` "which STATED", `:375` "whose module doc FRAMED ... and which CARRIED", `:377` "which READ". I opened each of those and confirmed the quoted text is genuinely absent from the tree, so the past tense is correct at all six. `:367` is the one bullet of the family left in the present.

Remedy: TOKEN, "whose ... clause GAINED the unsafe-fragment case".

---

## `R3B-5` (low). `:157` says the `toml_primary` binding is "made immediately above the match". Since inc2 the guard sits between them.

`:157`, the parenthetical justifying that the guard need not go down into the arms:

> (the `toml_primary` binding is made immediately above the match and `args.plan` is its fallback)

At `93ee357`, `src/main.rs:979` binds `toml_primary` and `src/main.rs:1005` opens the match. Lines 980 to 1004 are the containment guard the sentence is arguing FOR: the comment, `checked_root`, `checked_display`, `unsafe_pairing`, and the refusal branch. Twenty-six lines, including a whole `if/else`.

The SUBSTANTIVE claim ("available BEFORE the match", "does not force the guard down into the arms") is TRUE and I confirmed it. Only "immediately" is now wrong, and it is wrong because the thing the sentence proposed got built there. Reported because the pass's own report lists this as a retention it measured, so a reader will believe the whole parenthetical was checked.

Remedy: DELETION of one word.

---

## MY RULING ON THE TWO FAMILY-LEVEL BOUNDARIES

### Boundary 1: the acceptance checks at `:310-:349` were deliberately NOT swept

**CORRECTLY DRAWN.** The "Before inc2 this prints X" clauses at checks 11, 13b, 14b, 14c, 14e, 14g, 19b and 15 are red-case specifications, and `Q-66` requires them to stay reproducible against a named pre-fix revision. Re-tensing them would damage the red-then-green contract the checks exist to state. I agree with the exemption and would have drawn it the same way.

**AND I FOUND NO FALSE NON-RED-CASE CLAIM INSIDE THE BOUNDARY.** I did not take the exemption on trust: I extracted every present-tense claim in the checks that is NOT a red-case specification and checked each. Eight of them I ran rather than read.

- `:314`: the three test names resolve (`src/checks.rs:1478`, `src/main.rs:2290`, `src/main.rs:2879`), and `test-tmpdir-repo-assumption` is order 95, `not-started`. TRUE.
- `:316` (check 1): `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings` and `render --check` all clean at `93ee357`. RUN. TRUE.
- `:317` (check 2): `ls "$S/docs"` prints only `plans`, and the scaffold reports "(30 changed, 0 left untouched)", the figure `:24` states. RUN. TRUE.
- `:325` (check 10): plain `validate` on an absent log exits 0 with the stderr note. RUN. TRUE.
- `:336` (check 14g): "`status --resume` has NO JSON surface (`src/main.rs:run_status`'s `return run_resume(&args)` early return happens before serialisation)". The return is at `src/main.rs:1197` and the serialisation at `:1255`. TRUE.
- `:338` (check 15): exits 1 naming the resolved log. RUN. TRUE.
- `:339` (check 16): BOTH measured spellings reproduce exactly as stated at uid 1000. Mode-600 directory: plain `validate` exit 0 with `no metrics log at docs/metrics/workflow.jsonl; nothing to validate`, `--workflow` exit 1 with `could not be checked (Permission denied (os error 13))`. Trailing slash: plain `validate` exit 0, `--workflow` exit 1 with `could not be checked (Not a directory (os error 20))`. The recorded residual also reproduces: mode-000 FILE under plain `validate` gives `Error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }` at exit 1. RUN. TRUE. (Both chmods restored.)
- `:346` (check 21b): `reserve_runner_worktree` exists at `src/checks.rs:498`, and `nanos()`'s doc comment at `src/checks.rs:1015-1022` does state the opposite of per-process uniqueness ("no guarantee at all ... Uniqueness itself comes from the atomic sequence and the `create_dir` reservation"). TRUE.
- `:347` (check 22): `Projection.plan`'s doc comment no longer says "present only when a readable `--plan` was given", and `status --json --source docs/plans/agent-scaffold.plan.toml` with no `--plan` serialises a populated `"plan"` object. RUN. TRUE.
- `:348` (check 23): `render --check` reports "up to date"; `validate --source docs/plans/agent-scaffold.plan.toml --workflow` prints `workflow invariants hold` at exit 0 over 293 records. RUN. TRUE.
- `:341` (check 18): the test it asks for exists, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:463` `a_bare_filename_from_inside_docs_plans_stays_a_miss_and_now_fails_loudly`. TRUE.

ONE CAVEAT, and it points OUT of the boundary rather than into it. Check 19 at `:342` is where the WIDENED cost (ii) is pinned, and `R3B-2`'s defect is that `:104`, OUTSIDE the checks, disagrees with it. The unswept boundary did not cause that; the checks are the site that is right.

### Boundary 2: increment-description and requirement bullets left as describing CONTENT rather than STATE

**SOUND at `:279` to `:282`. NOT SOUND at `:104`. APPLIED INCONSISTENTLY inside the documentation-impact family.**

SOUND for the increment bullets. I did not accept the framing; I checked every factual assertion inside them against the tree, and all hold, so the ruling costs nothing there even if one disagrees with its ground:

- `--metrics` is `Option<PathBuf>` on `ValidateArgs`, `StatusArgs` and `NextArgs` (`src/main.rs:429-431`, `:456-458`, `:479-481`), with no `default_value`, and `--help` prints no `[default:]` for it.
- `derive_task` resolves `--source` then `--plan` (`src/next.rs:1126-1128`).
- `default_ledger_path` is called from both `run_resume` (`src/main.rs:1645`) and `run_next` (`:1748`).
- With NEITHER anchor, the ledger keeps `docs/plans/<task>.ledger.md` (`src/main.rs:1543-1546`) and the metrics rule keeps its current-directory-relative path (`:1352-1355`).
- The `_` catch-all IS now a reported problem (`src/main.rs:1067-1076`).
- `pack/AGENTS.md:93` carries the instrumentation qualifier, and the deployed copies are regenerated (`cargo test` passes, which runs the `agents-md-drift-guard` and `prompt-drift-guard` comparisons).

NOT SOUND at `:104`. The distinction was applied one site too far. A REQUIREMENT can go stale in exactly one way, which is when its stated EXCEPTIONS stop matching the exceptions the step actually took, and that is what happened: see `R3B-2`. The requirement half of `:104` (never pair and report success) is genuinely a requirement and genuinely still met, so the pass was right about the sentence's FIRST half and wrong about its LAST clause. The lesson generalises: a requirement is exempt from re-tensing, but its exception list is a factual claim about the tree and is not.

APPLIED INCONSISTENTLY in the documentation-impact list. The INC1 and INC3 bullets moved their quotations to the past at all six sites; the INC2 bullets did not, and `:367` is the one where the quoted text is genuinely gone from the file it is attributed to. See `R3B-4`. `:364`, `:365`, `:366`, `:369` and `:370` survive the same test only because their assertions happen to still hold, which I verified individually rather than assumed:

- `:364`: the `--workflow` help's closing sentences DO enumerate the error cases including the refusal (`src/main.rs:ValidateArgs::workflow`).
- `:365`: `README.md:210` DOES name the refusal as a failure mode.
- `:366` and `:369`: `README.md` has no `next` section. TRUE, with a caveat below.
- `:370` and `:360`: `CHANGELOG.md`'s `## [Unreleased]` has `### Added` and `### Changed` and NO `### Fixed`.

---

## WHAT I VERIFIED AND CONFIRMED TRUE

**132 discrete claims checked. 127 confirmed TRUE. 5 sidecar sites defective (`R3B-1` at `:163` and `:179`, `R3B-2` at `:104`, `R3B-4` at `:367`, `R3B-5` at `:157`), plus 2 shipped-code documentation sites found stale in the same sweep (`R3B-3`).**

### The pass's STATED retention set: 46 claims, 45 TRUE, 1 FALSE

Every one of the twelve "today" sites the pass reported leaving:

1. `:212` `LoopState` derives `Serialize` with `#[serde(rename_all = "kebab-case")]` (`src/next.rs:273-275`). TRUE.
2. `:212` `awaiting-reviewers` is on the wire in `GOLDEN_JSON` (`src/next.rs:2088`). TRUE.
3. `:219` the human renderer maps each variant to the string it prints (`src/next.rs:154-155`). TRUE.
4. `:219` no variant exists for "no in-progress or ready step"; the enum has exactly three (`src/next.rs:119-130`). TRUE. The string survives only as an unreachable renderer fallback at `src/next.rs:1196`, which the sidecar does not claim otherwise about.
5. `:221` "no plan steps found". TRUE.
6. `:222` "all steps complete". TRUE.
7. `:235` `run_status` returns from `run_resume` at `:1197` before the serialisation at `:1255`. TRUE.
8. `:235` `--json` alongside `--resume` is silently ignored. TRUE, by the same code path.
9. `:235` `--ledger-fragment` carries `requires = "resume"` (`src/main.rs:StatusArgs::ledger_fragment`). TRUE.
10. `:235` `audit --out` carries `conflicts_with = "json"` (`src/main.rs:AuditArgs::out`). TRUE.
11. `:235` `status-resume-ignores-json` is order 96. TRUE.
12. `:237` no malformed-log state is distinguished; `count_records` counts non-blank lines (`src/metrics.rs:610-612`). TRUE.
13. `:237` `parse_rounds` is best-effort (`src/metrics.rs:655-660`, "A line that is blank, not JSON, not a `round` ... is skipped here"). TRUE.
14. `:243` `[meta].metrics` is absent from this repository's plan. Measured at zero occurrences. TRUE.
15. `:243` and from the scaffolded template. Measured at zero occurrences in `pack/`. TRUE.
16. `:269` the LEDGER half of the in-root bound has no owner. TRUE. (It has none in the strongest possible sense: the `validation-constraints` handle is itself dangling, which is recorded residual `F-5` at ledger `:1003` and which I am therefore NOT raising.)
17. `:269` `next` derives its forward convergence verdict independently of `w3_problems` (`src/next.rs:850-853`). TRUE.
18. `:279` with neither anchor the ledger keeps `docs/plans/<task>.ledger.md`. TRUE.
19. `:279` and the metrics rule keeps its current-directory-relative path. TRUE.
20. `:337` `"resume_state": null` appears in the golden (`src/next.rs:2116`). TRUE.
21. `:360` `CHANGELOG.md`'s `## [Unreleased]` has `Added`. TRUE.
22. `:360` and `Changed`, and no `Fixed`. TRUE.
23. `:366` `README.md` documents no `next` section. TRUE, with the caveat below.
24. `:369` the same claim at its second site. TRUE.
25. `:398` no reason beside `status`'s `plan` field and no malformed-log variant. TRUE (`src/main.rs:570-577`).

The defect D tail at `:8`, the two `:157` present-tense claims, the increment bullets, the requirement, and the citations checked in passing:

26. `:8` a non-instrumented project has no deterministic enforcement, and after this step is told why the tool refuses. TRUE: `pack/AGENTS.md:93` carries the qualifier and check 15 measures exit 1.
27. `:157` the `(None, None, _)` arm still exists (`src/main.rs:1042-1045`). TRUE.
28. `:157` `toml_primary` is bound above the match (`src/main.rs:979`). TRUE as to "above". "immediately" is `R3B-5`.
29. to 33. The five increment-content assertions listed under Boundary 2 above. ALL TRUE.
34. `:104` the pairing requirement (never pair and report success). TRUE, and check 13b pins it.
35. `:104` the exception clause. **FALSE.** `R3B-2`.
36. `src/main.rs:1010`, the `(Some(source), _, Some(metrics_text))` arm. TRUE, at exactly that line.
37. `src/main.rs:1024`, the `(None, Some(plan_text), Some(metrics_text))` arm. TRUE, at exactly that line.
38. to 40. The three `TMPDIR` test names: `a_non_repo_target_with_runnable_checks_errors` (`src/checks.rs:1478`), `init_plan_defaults_to_git_and_skips_inside_a_repo` (`src/main.rs:2290`), `install_precommit_hook_skips_a_non_repo` (`src/main.rs:2879`). ALL TRUE.
41. `pack/instrument.md`'s `validate` sentence, "exits non-zero and reports any malformed record" (`pack/instrument.md:13`). TRUE, literal match.
42. `src/plan/render.rs:296` is `sections.push(format!("# {} plan", plan.meta.title));`. TRUE.
43. `src/plan/render.rs:167-169` is the `plan.meta.sidecars.front` / `.tail` read. TRUE.
44. `src/plan/source.rs:102` is `#[serde(deny_unknown_fields)]` on `Meta`. TRUE.
45. `pack/AGENTS.md:61` is a "When instrumentation is on" clause naming `docs/metrics/workflow.jsonl`. TRUE.
46. `pack/AGENTS.md:63` likewise, and `grep -n 'docs/metrics/workflow.jsonl' pack/AGENTS.md` returns EXACTLY those two lines, which is the stronger claim `:139` makes. TRUE.

### Retentions the pass did NOT state: 86 further claims, 82 TRUE, 4 defective

Swept as instructed, because three fix passes moved text without re-verifying what they left.

- **All 38 distinct `file:line` citations in the sidecar, extracted mechanically and resolved.** Every one that names a tree file resolves, and none over-runs its file. The only unresolved names are the three exploration records, which live at `docs/plans/workflow-enforcement-tier.explorations/` as `:12` says, and one fixture filename.
- **All 70 quoted fragments of 30 characters or more, matched whitespace-normalised against `src/`, `tests/`, `pack/`, `.agents/`, `README.md`, `CHANGELOG.md` and `justfile`.** Every fragment either matches, or is attributed to something outside that corpus (the ledger, the first planner pass, a decision option label, a review question, a fixture path), or is framed in the past tense, which check 21 permits. The single exception is `:367`, `R3B-4`.
- **Eight step order and status claims.** `instrument-magic-filename` 60 `deferred`, `sidecar-ref-empty-string` 63 `deferred`, `sidecar-ref-symlink` 64 `deferred`, `reviewer-reproducible-evidence` 88 `complete`, `prompt-drift-guard` 92 `complete`, `checks-runner-worktree-name-collision` 93 `complete`, `test-tmpdir-repo-assumption` 95 `not-started`, `status-resume-ignores-json` 96 `not-started`. ALL TRUE.
- **Eight calibration figures, recomputed from `docs/metrics/workflow.jsonl` at 293 records.** `prompt-drift-guard-inc1` is 6 rounds and 15 valid findings; the distribution is `{9: 1, 7: 1, 6: 2, 5: 10, 4: 7, 3: 11, 2: 37, 1: 16}` over 85 artifacts, so 6 rounds IS joint-third and the median IS 2; inc1 is 3 rounds and 13 valid findings, per round `[3, 4, 6]`, matching the `-w1` waiver note as corrected by `Q-55-w1figure`; `-w2` is `[9, 5, 6, 4]` totalling 24; `-w3` is `[6, 4, 2, 0, 2]` totalling 14. ALL TRUE. The "five retrospective and one prospective" figure matches the ledger's own most recent statement of it (`:1001`, `:1033`, `:1099`). TRUE.
- **Four exploration-record figures.** 521, 510, 483, total 1514. Exact. TRUE.
- **Five ledger quotations** at `:12`, `:106` and `:253`, all located in `docs/plans/agent-scaffold.ledger.md` at `:937`, `:941`, `:955`. TRUE.
- **Ten structural claims about `src/next.rs` and `src/main.rs`.** No `skip_serializing_if` in either file (0 hits each); `select_active_loop` returns `Some(build_pending_loop(step, LoopState::Blocked, ...))` from its last branch (`src/next.rs:733`); `NextInputs` carries both reason fields with the "Computed by the CALLER" doc (`src/next.rs:617-631`); `run_next`'s `else` arm on `metrics_path.exists()` yields an empty rounds list and still projects (`src/main.rs:1741-1743`); `status`'s `Projection` carries `metrics: Option<MetricsProjection>` (`src/main.rs:573`); `status --json` has no golden; its only serialisation path is the `to_string_pretty` call in `run_status`; and all FOUR doc comments `Q-55-jsonreason` falsified, plus `active_loop`'s, are corrected (`src/next.rs:176-192`, `src/main.rs:561-577`). ALL TRUE.
- **Five further code citations.** `src/workflow.rs:180-195` is `check_workflow_toml`; `:448-449` is `round_step_slug(round) == step.slug`; `src/plan/source.rs:480-495` is `is_safe_sidecar_ref`; `src/findings_naming.rs:52-55` is `join_dir`; `justfile:46-48` is `scaffold-self` followed by `nix fmt`. ALL TRUE.
- **Four `README.md` ranges.** `:210` is the `validate` paragraph, `:212-232` the `validate` example block, `:238` the `status` paragraph, `:242-260` the `status` example block. All exact. The two quoted contract halves at `:359` and `:173` are literal matches at `README.md:238`. TRUE.
- **Nine in-scope citations in the three sidecars check 21b covers.** `test-tmpdir-repo-assumption.md`'s `src/main.rs:2279-2287`, `:2289-2305`, `:2878-2889`; `instrument-magic-filename.md`'s `src/main.rs:257-258`; `checks-runner-worktree-name-collision.md`'s `src/main.rs:2280-2285`, `src/manifest.rs:552-558`, `src/plan/render.rs:638`, `tests/audit_command.rs:20`, `tests/checks_staged_hook_env.rs:50`, `tests/scaffold_precommit_hook.rs:14`. ALL RESOLVE at their cited ranges. (`src/checks.rs` citations in that file are out of scope and I did not check them.)
- **`:404`, measured rather than read.** `next --source <a foreign plan>` from this repository emits `ledger: <S>/fp/docs/plans/TEMPLATE.ledger.md` (anchored) beside `review_findings: docs/plans/TEMPLATE.reviews/example-step-reviewer-<disambiguator>.md` (current-directory-relative), which is exactly the split `:404` states. TRUE, and inc2 did not close it, because with only a `--source` the log anchors INTO the foreign root and containment passes.
- **`Q-55-twinsites`' two authorised corrections.** Both landed in `tests/unsafe_pairings_are_refused_and_omitted.rs` (`:1370` now reads "pinned on BOTH commands because `status --json` has no golden", with the "no test on its serialisation at all" clause gone). TRUE.
- **Inc4's file scope against its own impact list.** The increment's commits touch the sidecar, the three named sidecars, the rendered plan and plan TOML, the ledger, the metrics log, the review files, `src/main.rs` (one line, the `Projection.plan` doc comment) and `tests/unsafe_pairings_are_refused_and_omitted.rs` (five lines). Nothing outside the list at `:384-:388`. TRUE.

### One VERIFIED-TRUE claim with a caveat a triager should see, NOT raised as a finding

`:366` and `:369`: "`README.md` documents no `next` section today". LITERALLY TRUE. `grep -n '^#' README.md` shows no `next` heading, and `grep -n 'agent-scaffold next' README.md` returns nothing. But `README.md:238` and `:240` now DO document `next`'s behaviour and its three `--json` reason fields, inside the `status` paragraph, which inc2 added. So the conditional the two bullets carry ("if the implementer adds one, it carries the same statement") was satisfied at a different place than the sentence anticipates. The claim is true, the fix was made, and I would not spend a fix pass on it. Recorded so a fourth round does not raise it as new.

## DIMENSIONS VARIED, AND WHERE I DID NOT GENERALISE

This project has a recorded case of a 216-invocation negative result falsified because the matrix never varied the one dimension that mattered, so I state the axes rather than the count.

VARIED:

- Surfaces: `validate`, `validate --workflow`, `status`, `status --json`, `status --resume`, `next`, `next --json`, `--help`.
- Anchor configuration: TOML-primary `--source`; Markdown-primary `--source` with NO `--plan` (the no-plan-read case, which is what `R3B-1` turns on); explicit `--metrics`; explicit `--ledger-fragment`; no anchors at all.
- Layout: conventional `docs/plans` plus `docs/metrics`; `docs/metrics` as a symlink OUT of the root with `docs/plans` real (which is what `R3B-2` turns on); a foreign project outside the root; a bare filename from inside `docs/plans` (via the suite).
- Probe outcome: file absent; trailing slash giving `ENOTDIR`; mode-600 directory giving `EACCES`; mode-000 file.
- Guards: `cargo build`, `cargo test`, `cargo clippy --all-targets -- -D warnings`, `render --check`, `validate --workflow` on this repository, all at `93ee357`.

NOT VARIED, so nothing above generalises past it:

- uid. Everything ran at uid 1000. The trailing-slash spelling is the one check 16 says holds "at every uid including root"; I did not test as root.
- The `..`-escape spellings beyond what the suite already pins, and the nested-`docs/plans` matrix. I did not rebuild explorer A's case matrix.
- `--workflow-spec`, `--module`, and `--instrument` scaffolds. I built only the non-instrumented default fixture, plus whatever `cargo test`'s drift guards build.
- Filesystem semantics. Linux only, case-sensitive, no network or overlay filesystem.
- `src/checks.rs` citations in `checks-runner-worktree-name-collision.md`, `run_validate`'s "`--plan` still clap-required" claims, `src/next.rs:162` and `:181-183`, and `docs/plans/agent-scaffold.md:7`. All named out of scope in my brief and all left untouched.
- The eight recorded residuals and five settled dismissals. I read the previous findings files and triages first and re-raised none of them. Where my sweep landed on settled ground I stopped and said so in place: the dangling `validation-constraints` handle (`F-5`), accepted cost (ii) itself, the plain-`validate` mode-000 inconsistency, and `Q-55-fallbacksurface`.

## FIXTURE HYGIENE

All fixtures under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/rev-inc4-r3-b/`. Nothing written to bare `/tmp`. Nothing deleted outside that subdirectory. Both `chmod` changes restored and verified (`stat -c '%a'` reported `755` and `644` afterwards). No file in the repository was modified except this findings file.
