# Fidelity and grounding review, round 1: `workflow-enforcement-tier` fold

Reviewer model: Claude Sonnet 5 (claude-sonnet-5), running as the Claude Code CLI. Date: 2026-07-31.

Artifact reviewed: the plan fold on branch `plan/q55-enforcement`, specifically:

- `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`
- `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`
- `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`
- the `[[step]]` and `[[question]]` entries this fold adds or changes in `docs/plans/agent-scaffold.plan.toml`

Lens: fidelity and grounding only. Every factual assertion, citation, quote, count, and cross-reference was checked against its source; no judgement is offered on the decided policy itself.

## Summary

5 findings: 0 critical, 0 high, 1 medium, 4 low.

Citations checked: every `file:line` citation across all three sidecars and the TOML's Q-55 decision text was opened at the cited location, roughly 60 distinct citations (many repeated across files). 4 were wrong or imprecise (F-1's line-number component, and F-2 through F-4); the remainder matched exactly, including multi-line ranges checked against the actual start/end lines of the cited comment, function, or struct. Every quoted passage (code comments, doc comments, README prose, pack prose, exploration-record prose, ledger paragraph-boundary strings, test assertions) was diffed byte-for-byte against its source; all quotes matched exactly except where noted below.

Behavioural claims were run, not read: I built the worktree, scaffolded a throwaway fixture with the exact command in the sidecar, and reproduced Defect A (false green, exit 0), Defect B (cross-project contamination, `workflow invariants hold` at exit 0 against a foreign 239-record log), the `next` fabrication in Defect C (state: converged, streak 1/1, rounds 2/5, next: mark the step complete...), and the three TMPDIR-dependent test failures with `TMPDIR` pointed inside this worktree (exact panic messages matched the sidecar's quoted block verbatim). I also ran the full suite with `TMPDIR` outside the repo and got exactly 386 passed, matching the stated count.

Counts and enumerations were independently re-derived rather than trusted: the three exploration records' line counts (521 + 483 + 510 = 1514), the six decision receipts in `docs/metrics/workflow.jsonl` (grepped and diffed each `chosen` value against the TOML's prose), the 386-test total, the two occurrences of `docs/metrics/workflow.jsonl` in `pack/AGENTS.md` (both inside "When instrumentation is on" clauses), the zero occurrences of `skip_serializing_if` in `src/next.rs`/`src/main.rs`, the single occurrence of `#[serde(skip)]` in all of `src/`, and the five clap `requires`/`conflicts_with` constraint attributes in `src/main.rs` (see F-1) were each verified by an independent grep or read, not by re-reading the sidecar's own claim.

Cross-checked against the exploration records at `docs/plans/workflow-enforcement-tier.explorations/` (1514 lines across the three files): the `value_source` vs `Option<PathBuf>` diff-size and panic-message measurements, the symlinked-`docs/plans` false-refusal measurement (37 records to `exit=1 REFUSED`), the rejected `[meta].metrics` candidate's render-cost and zero-occurrence measurements, and explorer C's `src/next.rs:1339` limitation report were all traced to their source records and found faithfully reported, including stated limitations and caveats.

Coverage method: each of the three sidecars was read start to finish and every citation, quote, and quantified claim was logged before verification began, rather than spot-checking sections that looked citation-heavy. The TOML's Q-55 decision text was diffed in full against `main`. The `docs/metrics/workflow.jsonl` diff was read in full. All 90 step slugs in the plan TOML were enumerated once (not searched piecemeal) to check every cross-referenced step (orders 63, 64, 84, 88, 92, 93, 94, 95, 96) against the sidecars' claims about it, which is how F-5 was caught: a targeted search for the one slug the sidecar treats as a live destination turned up nothing in that enumeration. Two additional items were investigated and cleared rather than reported as findings: the exploration-record byte counts (521 + 483 + 510 = 1514, matching the total the sidecar and the ledger both state, even though the two list the three files in a different order) and the cross-reference from `status-resume-ignores-json.md`'s risk classification to `workflow-enforcement-tier-inc3`'s (which does argue on the CLI-exit-code basis the citing text says it does). Neither needed a change, so neither is listed below.

## Findings

### F-1 (medium): "Four constraint attributes" contradicts its own five-item list, and one of the five line numbers points at the wrong line

`docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, in the "in-repo precedent" section: "So a fix here FOLLOWS AN ESTABLISHED CONVENTION rather than inventing one, and that is the proposition to state in the commit. Four constraint attributes already exist in `src/main.rs` (`:396`, `:441`, `:465`, `:525`, `:557`), so the mechanism is routine in this codebase."

The parenthetical lists five line numbers, not four. I confirmed by grep (`grep -n "requires = \|conflicts_with = " src/main.rs`) that there are exactly five clap `requires`/`conflicts_with` constraint attributes in `src/main.rs`, at lines 396, 442, 465, 525, and 557 (`dry_run` conflicts with `write`; `workflow_spec` requires `workflow`; `StatusArgs::ledger_fragment` requires `resume`; `render --strict` requires `check`; `audit --out` conflicts with `json`). So the correct count is five, and the sentence should say "Five constraint attributes."

Separately, of the five line numbers cited, four (`:396`, `:465`, `:525`, `:557`) point exactly at the attribute line itself. The fifth, `:441`, points at the first line of the `workflow_spec` field's doc comment; the attribute itself (`#[arg(long, requires = "workflow")]`) is at line 442. This is consistent with how the same field is cited correctly elsewhere in the same file, as a range, `src/main.rs:441-443`.

What should change: say "Five constraint attributes" (or drop one of the five from the list, if a fifth was never intended), and correct `:441` to `:442` to match the pattern the other four follow.

### F-2 (low): `src/main.rs:560-563` is cited twice for a doc comment that actually spans lines 561-564

`workflow-enforcement-tier.md` cites `src/main.rs:560-563` twice, once in the "documented-contract change" section ("`src/main.rs:560-563`, `status`'s `Projection` doc comment, HAS THE SAME DEFECT: 'Every part is optional so a missing plan or metrics file yields a partial projection rather than a failure'") and once in the INC2 documentation-impact list.

I read the actual lines: line 560 is blank, lines 561 through 564 are the four-line doc comment (`/// A derived, best-effort projection...` through `/// (it is regenerable from the plan and the metrics log).`), and line 565 is `#[derive(Serialize)]`. The quoted text spans lines 562 to 563 within that block, and the whole comment the sidecar is discussing is 561 to 564. The cited range should be `561-564`, not `560-563`.

What should change: both occurrences of `src/main.rs:560-563` become `src/main.rs:561-564`.

### F-3 (low): `src/main.rs:995-998` is cited as containing a comment that is actually at lines 992-994

`workflow-enforcement-tier.md`, Defect A section: "The sibling arm at `src/main.rs:995-998` handles '`--workflow` requested but no plan source resolved' and pushes a hard problem, with the comment stating the identical reasoning: '`--workflow` was explicitly requested, so skipping would green-pass while checking nothing; make it a hard problem instead.'"

Lines 995 to 998 are exactly the match arm itself (`(None, None, _) => problems.push(...)`), which is a correct citation for "pushes a hard problem." But the quoted comment text is not inside that range: it is the three-line `//` comment immediately above the arm, at lines 992 to 994 (`// ... --workflow was explicitly requested, so` / `// skipping would green-pass while checking nothing; make it a hard problem` / `// instead. Independent of the metrics log.`). The quote itself is word-for-word accurate; only its attributed location is off.

What should change: either widen the citation to `src/main.rs:992-998` (comment plus arm), or cite the comment separately as `:992-994`.

### F-4 (low): `src/main.rs:1150` is cited for a quote that spans lines 1150-1151

`workflow-enforcement-tier.md`, in the `Q-55-refusalscope` "THE GROUND" paragraph: "`run_resume`'s doc comment at `src/main.rs:1150` matches it ('A missing ledger or absent section prints a note and exits 0, since `status` is a best-effort projection, not a validator.')."

Line 1150 reads "source filename). A missing ledger or absent section prints a note and exits 0, since"; the clause "`status` is a best-effort projection, not a validator." is on line 1151. The full doc comment (correctly cited elsewhere in the same document as `src/main.rs:1147-1151`) runs five lines; the quoted sentence itself spans two of them, 1150 and 1151, not one.

What should change: cite `src/main.rs:1150-1151` for this specific quote.

### F-5 (low): "the validation-constraints step" is referenced repeatedly but is not a step that exists in `agent-scaffold.plan.toml`

`workflow-enforcement-tier.md` refers to "the validation-constraints step" as the destination for queued project-identity work in at least two places (the "What this step does not fix" section and the "Scope: what this step does not do" section), and the TOML's own Q-55 folded-decision prose refers to it three times (in the `Q-55-scope`, `Q-55-mechanism`, and `Q-55-jsonreason` paragraphs), each time as though it names a resolvable step.

I enumerated every `[[step]]` slug in `docs/plans/agent-scaffold.plan.toml` (90 slugs total, via `grep -n "^slug = "`) and confirmed there is no step with slug `validation-constraints` or any similar name. A reader of the in-scope documents who tries to follow this cross-reference to see what it actually contains, or to check whether it is blocked on anything, finds nothing.

I checked `docs/plans/agent-scaffold.ledger.md` for context only (it is out of scope for content review, but resolving a citation the in-scope sidecar makes is in scope): the ledger confirms this is a real, human-decided future step ("GATE 4, the validation-constraints step"), not a fabrication, so the substance behind the reference is sound. The gap is that the in-scope artifact set treats it as a resolvable handle when it is not yet one.

What should change: either note explicitly, at first mention, that the validation-constraints step does not yet exist in the plan TOML (a forward reference to already-decided future work), or add a stub `[[step]]` entry for it before this fold merges, so the cross-reference resolves for a reader working only from the plan and its sidecars.

## Notes on what I deliberately did not raise

Per the reviewer brief, I did not raise: the choice of enforcement tier, the one-step/three-increment shape, the anchor-plus-refusal mechanism, the fall back to the source's own directory, the omit-and-exit-0 behaviour for `status`/`next`, the serialised-reason decision, the two accepted costs, or the nearest-wins judgement on a nested `docs/plans`. All six decision receipts (`Q-55`, `Q-55-scope`, `Q-55-mechanism`, `Q-55-noconvention`, `Q-55-refusalscope`, `Q-55-jsonreason`) are present in `docs/metrics/workflow.jsonl` with `chosen` values that match the TOML's prose exactly, so none of those decisions are re-litigated above.
