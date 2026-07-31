# `workflow-enforcement-tier` plan review, round 2: RESIDUE (fix-induced defects)

Reviewer model: Claude Sonnet 5. Exact model id `claude-sonnet-5`.
Date: 2026-07-31.
Worktree: `.claude/worktrees/rev-q55-r2-residue`, branch `review/q55-r2-residue`, checked out at commit `8756578` on `plan/q55-enforcement`.
Artifact reviewed: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (primary), `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, and the `Q-55` `[[question]]` entry plus the three `[[step]]` blocks (`workflow-enforcement-tier` order 94, `test-tmpdir-repo-assumption` order 95, `status-resume-ignores-json` order 96) in `docs/plans/agent-scaffold.plan.toml`.

Lens: fix-induced residue only. The fix commit under review is `8756578`, "docs: apply the round 1 plan review fixes" (net +2 lines over the three sidecars plus the generated `docs/plans/agent-scaffold.md`). I read the round 1 files read-only (`workflow-enforcement-tier-planreview-r1-reviewer-executability.md`, `-r1-reviewer-fidelity.md`, `-r1-triage.md`) from the main repository and did not copy them into this worktree or this file.

## Result

1 finding. Severity: 0 critical, 0 high, 0 medium, 1 low.

No fix-induced prose residue found in the three one-clause prose additions (EX-1, EX-2, EX-10) or in the three narrowings (EX-2's qualifier deletion and correlation-rule narrowing, EX-5's replacement clause, EX-9's replacement clause): each reads consistently with its own surrounding text and with the rest of the document on independent re-derivation. The one finding is a citation left stale by the round 1 fix's own stated scoping, not a defect authored by new prose.

The fix pass did not overreach beyond the 14 valid findings and the one triager-flagged (not-triaged) item (the step-93 status label). I found no edit in the `8756578` diff that does not map to one of those 15 items.

## What I swept, enumerated (not asserted)

- Read `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` in full, twice (lines 1-234 and 235-380), post-fix.
- Read `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md` and `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md` in full, post-fix.
- Read the full `git show 8756578` diff over the three sidecar files (the fourth file in the commit, `docs/plans/agent-scaffold.md`, is generated; I did not diff it line by line, see below).
- Read the `Q-55` `[[question]]` block's `ask` field in full (`docs/plans/agent-scaffold.plan.toml:1683-1706`), and the three `[[step]]` blocks at orders 94, 95, 96 (`:1296-1344`).
- For each of the 14 `VALID` findings and the one non-triaged item from the r1 triage, located the exact site(s) in the current (post-fix) text and confirmed the fix is present and matches the triager's stated minimal fix. Individually verified, with the current line/location:
  - EX-1 (carrier clause): present at `workflow-enforcement-tier.md:212` ("THE ENUM IS THE MACHINE VALUE ONLY... the CALLER assembles that message... (`run_status` and `run_next`)").
  - EX-2 (qualifier deletion + precedence sentence + correlation narrowing): qualifier gone at `:217`; precedence rule added at `:231`; correlation rule narrowed with "WHEN the loop's absence is metrics-derived rather than step-derived" at `:233`.
  - EX-3 (diagnosis correction + target text): present at `:204`, and I independently re-read the cited code (`src/next.rs:108-109`, `:388-396`, `:415-417`, `:421-426`, `:589-614`, `:607-611`) and confirmed every citation resolves exactly as the target text states.
  - EX-4 (decoy clause deleted from check 7): confirmed gone at `:313`.
  - EX-5 (narrow true clause replacing the false "no new failure mode"): present at `:273`, and I independently confirmed `src/next.rs:997-999` is exactly `derive_task`'s `source.as_ref().or(plan.as_ref())`.
  - EX-6 (add `:461` citation): present at `:342`, and I independently confirmed `src/main.rs:461` is `StatusArgs::resume`'s help string and does carry the stale `docs/plans/<task>.ledger.md` phrasing.
  - EX-7 (drop unreachable `no-ready-step`): confirmed gone from the vocabulary list at `:219-223`; grepped the whole sidecar and `src/next.rs` for `no-ready-step` and `"no in-progress or ready step"` and found only the two code sites and the sidecar's own note that the string is unreachable.
  - EX-8 (delete stale 235-count expectations at two sites only): confirmed gone at check 5 (`:311`) and check 14b (`:321`); grepped the whole sidecar for `235` and confirmed the six historical mentions (`:72`, `:75`, `:81`, `:122`, `:164`, `:263`) are untouched, exactly the set the triager said must not be touched.
  - EX-9 (replace the false `w3_problems`-direct-call claim): present at `:265`, and I independently grepped `src/*.rs` for `w3_problems` and confirmed the only production caller is `src/workflow.rs:217`, with `src/next.rs:1339` inside `#[cfg(test)]` (a differential test), matching the corrected claim.
  - EX-10 (ledger-with-no-source, source-vs-plan precedence): both clauses present, bundled into the same `:273` edit as EX-5.
  - F-1 (four -> five, three sites, plus `:441` -> `:442`): all three sites fixed in `status-resume-ignores-json.md` (`:92`, `:120`, `:125`); independently grepped `src/main.rs` for `conflicts_with`/`requires =` and confirmed the five attributes sit at exactly `:396`, `:442`, `:465`, `:525`, `:557`.
  - F-2 (`560-563` -> `561-564`, two sites): both fixed, `workflow-enforcement-tier.md:202` and `:353`.
  - F-3 (`995-998` -> `992-998` at the one flagged site, line 298/now 299 left alone): fixed at `:59`; confirmed `:299` (the old `:298`) still reads `995-998` and is correct as the triager found (a bare precedent citation, not a quote).
  - F-4 (`:1150` -> `:1150-1151`): fixed at `:174`, and I independently read `src/main.rs:1147-1151` and confirmed the comment matches (line 1150 ends "since", line 1151 carries the quoted clause).
  - The non-triaged item (step-93 status label): the `(order 93, \`deferred\`)` parenthetical was dropped (not corrected) at `test-tmpdir-repo-assumption.md:76`; I independently confirmed in `docs/plans/agent-scaffold.plan.toml:1279` that the step's actual status is `complete`, so `deferred` was wrong and deletion is one of the two forms the triager explicitly allowed.
- Grepped `docs/plans/agent-scaffold.plan.toml` for every stale string the 14 fixes removed or corrected (`235 records`, `no-ready-step`, `every pending step blocked`, `560-563`/`561-564`, `995-998`/`992-998`, `four constraint`/`Four constraint`, `:441`, `w3_problems`, `next.rs:1339`, `still exits 0`, `NO new failure mode`, `decoy`) to check whether any fixed claim has an unfixed twin elsewhere in the fold's TOML content. This is where the one finding below came from.
- Ran `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` (reports "up to date") and `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow` (reports "workflow invariants hold", 240 records, 95 steps, 69 questions) to confirm the fold is render-clean and validate-clean at this commit. I did not line-by-line diff the generated `docs/plans/agent-scaffold.md`; the render-check pass is the evidence I relied on for "the regeneration happened and matches," consistent with this file being out of scope as authored content.
- Ran `TMPDIR=/tmp/rev-r2-residue-scratch cargo test` (outside any repository, per the brief) and got 386 passed across all binaries (373 + 5 + 1 + 1 + 3 + 1 + 2), 0 failed, confirming the "386 expected" claim is still current.

What I did NOT do: I did not re-run the acceptance check's manual CLI reproductions (the fixture-building, borrowed-slug, symlink, and `..`-escape demonstrations); those exercise behaviour that inc1/inc2/inc3 have not yet implemented (`src/` is unchanged by this fold and is evidence-only per the task scope), so there is nothing yet to run them against. I relied on the round 1 triager's first-hand reproductions for the underlying behavioural claims themselves and confined my own running to the parts of the artifact that are checkable today (build, test, render, validate) and to source-code citations.

## R2-1. New. Severity `low`. The round 1 fix corrected a stale `run_resume` doc-comment citation in the sidecar but left the identical citation stale in the plan TOML's `Q-55` decision text, so the artifact now disagrees with itself about the same fact

F-4 (round 1, `VALID`, low) found that `workflow-enforcement-tier.md` cited `src/main.rs:1150` for a quote that actually spans `1150-1151`, and the fix corrected it there. The triager's own site count for F-4 was `` `grep -rn "src/main.rs:1150"` over the steps directory returns one hit ``, which by construction excludes `docs/plans/agent-scaffold.plan.toml`.

The same sentence, in near-identical wording, also lives in the plan TOML's `Q-55` question, in the `Q-55-refusalscope` paragraph of its `ask` field, and it still says `:1150`:

`docs/plans/agent-scaffold.plan.toml:1702`: "...and `run_resume`'s doc comment at `src/main.rs:1150` matches it, so a log that does not belong to this plan is exactly a part that is not available for this projection and omitting it is the documented contract applied literally."

Compare the now-fixed sidecar, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:174`: "...and `run_resume`'s doc comment at `src/main.rs:1150-1151` matches it (\"A missing ledger or absent section prints a note and exits 0, since `status` is a best-effort projection, not a validator\")."

I independently read `src/main.rs:1147-1151`:

```
1147	/// The `status --resume` slice: print the ledger's `## RESUME STATE` block verbatim,
1148	/// reusing the shared `next::extract_resume_state`. The ledger path is `--ledger-fragment`
1149	/// or the `docs/plans/<task>.ledger.md` default (with `<task>` derived from the plan
1150	/// source filename). A missing ledger or absent section prints a note and exits 0, since
1151	/// `status` is a best-effort projection, not a validator.
```

Line 1150 ends mid-clause ("...exits 0, since"); the "best-effort projection, not a validator" half that the TOML text is invoking as the matching claim is on line 1151. The TOML's bare `:1150` is exactly the same imprecision F-4 already named and fixed one document over.

Before the round 1 fix, both citations agreed (both said `:1150`, both equally imprecise). After it, they disagree: the sidecar now correctly says `:1150-1151` and the TOML still says `:1150`, so a reader who follows the TOML's cross-reference and then checks the sidecar's version of the same fact meets two different answers for where the same doc comment is. This is a direct product of the fix's own scoping (the site-count grep was, by the triager's own words, run "over the steps directory"), not a new authored claim, so it is best read as a residue of the fix's boundary rather than of the fix's wording.

WHAT SHOULD CHANGE. Widen `docs/plans/agent-scaffold.plan.toml:1702`'s citation from `` `src/main.rs:1150` `` to `` `src/main.rs:1150-1151` ``, matching the sidecar. A number edit, no prose, single site (I grepped the whole TOML for every other stale string the round 1 fixes touched or removed and found no other twin; see the sweep list above).

## Findings NOT raised

I did not find residue in the region the calibration data flags hardest (the three prose additions and three narrowings). Specifically checked and cleared:

- EX-1's carrier clause naming `run_status` and `run_next` as the callers that assemble the human message: this is accurate for `metrics_absent_reason` (used by both) and for `no_active_loop_reason` and `resume_state_absent_reason` (both `NextProjection`-only, so `run_next`-only applies, and the general architectural clause does not claim otherwise). `run_resume`, a third function, never touches these enums at all (`status --resume` returns before serialisation per `src/main.rs:1067-1069`, confirmed by direct read), so its absence from the named-callers list is not an omission.
- EX-2's precedence rule ("on both path fields... the unsafe variant wins") resolves both the metrics-field collision (after the qualifier deletion) and the pre-existing, undisputed ledger-field collision (sub-claim 2, which the triager found reproduced as stated and untouched by the fix beyond the shared precedence sentence).
- EX-2's correlation-rule narrowing ("WHEN the loop's absence is metrics-derived rather than step-derived") is consistent with the mechanism section's statement that an unsafe metrics pairing forces `active_loop` to `None` regardless of what step phases would otherwise yield, and does not contradict check 14f's three-way vocabulary demonstration.
- EX-5/EX-10's bundled edit at `workflow-enforcement-tier.md:273`: independently confirmed both new sub-clauses against code (`src/next.rs:997-999` for the source-then-plan order, and the unchanged CWD-relative-fallback text at `:158` for the no-source case).
- EX-9's replacement clause about `next` deriving its verdict independently of `w3_problems`: independently confirmed via `grep -rn "w3_problems(" src/*.rs` that the only production (non-test) caller is `src/workflow.rs:217`.

I also checked for overreach by mapping every hunk in the `8756578` diff to one of the 15 items (14 valid findings plus the step-93 label); every hunk maps to exactly one, and I found no edit that does not.

## Scope not covered

`docs/plans/agent-scaffold.md` (generated) was checked only via `render --check` reporting "up to date," not diffed by hand; `src/`, the ledger, and the three round 1 review files themselves were read only as evidence/context, per the task's scope. F-5 (the dangling `validation-constraints` reference) is not re-raised; it was ruled an accepted residual and remains untouched by this fix pass, correctly.
