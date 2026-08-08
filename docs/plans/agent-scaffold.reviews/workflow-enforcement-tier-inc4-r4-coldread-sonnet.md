# Round 4 cold-read: false claims in the step file, Q-55, and the increments/waivers

Reviewer lens: a fresh cold read, not started from the diff and not started from
earlier rounds' findings. Target: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`
whole (405 lines), the `Q-55` question record in `docs/plans/agent-scaffold.plan.toml`
(lines 1713-1737), and the four `[[step.increment]]` plus three `[[step.waiver]]` entries
for `workflow-enforcement-tier` (lines 1296-1349). Commit under review: `7ab5d48`.

## Result: NOTHING FOUND

No new false claim, stale citation, or wrong count survived reproduction. One dangling
reference was found and is a pre-existing, already-settled residual (`F-5`, ledger
`:1003` and `:1031`); it is reported below under "settled ground reached" rather than as
a finding, per the instruction not to re-raise a settled item without new evidence its
verdict was wrong. I have no new evidence; my measurement of it matches the ledger's.

## What I swept, and how

I read the step file end to end (all 405 lines, every section) and the full `Q-55`
record end to end (the `ask` field, all sub-decisions: scope, mechanism, refusalscope,
jsonreason, noconvention), plus the four increment stubs and three waiver notes. That is
100 percent of the assigned text by line count. Against that text I:

- Built the fixture fresh (`agent-scaffold scaffold --output-dir <scratch> --write
  --force --principles default`) and reproduced the exact output line quoted at the top
  of the step file: `Wrote to <scratch> (30 changed, 0 left untouched)`, and `ls
  <scratch>/docs` printing only `plans`. Both match exactly.
- Ran `cargo build` clean in the worktree.
- Reproduced acceptance check 15's claim directly: from inside the fixture,
  `validate --source docs/plans/TEMPLATE.plan.toml --workflow` exits 1 with `--workflow
  requested but no round log at docs/metrics/workflow.jsonl: the workflow check could
  not run, so it cannot report that the invariants hold; ...`. Matches the step file's
  description of the current (post-inc3) behaviour.
- Checked every `file:line` citation in the step file that names `src/main.rs`,
  `src/next.rs`, `src/workflow.rs`, `src/plan/source.rs`, `src/plan/render.rs`,
  `src/checks.rs`, `src/findings_naming.rs`, `justfile`, and the test files it names, by
  opening the exact range and confirming the named subject is there. All resolved. The
  ones I specifically opened and confirmed (function/struct start and, where the
  citation gives a closing line, the closing brace too):
  - `src/main.rs:ScaffoldArgs::instrument` (`:419-420`).
  - `src/main.rs:run_validate` (`:835`), `run_status` (`:1186`), `run_next` (`:1682`),
    `run_resume` (`:1640`), `project_root_of_source` (`:1312`), `resolve_metrics_path`
    (`:1344`), `default_ledger_path` (`:1538`), `struct Projection` (`:569`).
  - `src/workflow.rs:180-195` (`check_workflow_toml`, fn body spans exactly this range)
    and `:448-449` (`round_step_slug(round) == step.slug`).
  - `src/plan/source.rs:480-495` (`is_safe_sidecar_ref`) and `:102`
    (`#[serde(deny_unknown_fields)]` on `Meta`).
  - `src/plan/render.rs:296` (`plan.meta.title`) and `:167-169` (`plan.meta.sidecars`).
  - `justfile:46-48` (the `scaffold-self` recipe, cargo run plus `nix fmt`).
  - `src/findings_naming.rs:52-55` (`fn join_dir`, exact range, both braces).
  - `tests/validate_workflow_toml_source_needs_no_plan.rs:127-171`
    (`workflow_with_no_plan_source_hard_errors_instead_of_skipping`, exact range, both
    braces).
  - `src/main.rs:2279-2287`, `:2289-2305`, `:2878-2889` (the three test citations in
    `test-tmpdir-repo-assumption.md` that check 21b claims this step's own line movement
    broke and inc4 fixed): all three resolve to the exact function, brace to brace,
    confirming check 21b's own claim about itself.
  - `src/main.rs:257-258` (the `instrument.md` read cited in `instrument-magic-filename.md`,
    the other sidecar check 21b names): resolves exactly.
- Read the doc comments the step file says were falsified and corrected
  (`NextProjection`, `no_active_loop_reason`, `resume_state`, `active_loop`,
  `MetricsAbsentReason`, `NoActiveLoopReason`, `ResumeStateAbsentReason` in
  `src/next.rs`; `Projection` and its `plan`/`metrics`/`metrics_absent_reason` fields,
  `run_validate`'s and `run_resume`'s doc comments in `src/main.rs`) against the current
  source. Every one already carries the corrected wording the step file specifies (for
  example `Projection.plan`'s doc comment now says "present when a TOML-primary
  `--source` or a readable `--plan` supplies one" rather than the falsified "present
  only when a readable `--plan` was given" the step file says check 22 must remove).
- Grepped for `skip_serializing_if` in `src/next.rs` and `src/main.rs`: zero hits,
  matching the step file's "WHAT THE SWEEP FOUND NOTHING OF" claim.
- Confirmed `pack/AGENTS.md` (118 lines) still carries the inc3 qualifier at line 93
  ("when instrumentation is on, the deterministic `validate --workflow` check is the
  backstop ... and on a project with no round log yet ... that check exits non-zero
  reporting that it could not run rather than passing"), and that lines 61 and 63 are
  both inside "When instrumentation is on" clauses, and that `{{instrument}}` is at line
  116. All match the step file's citations for defect D and inc3's documentation half.
- Summed the waiver notes' round-count breakdowns against their totals: w1 "13 valid
  findings (3, 4, 6)" = 13; w2 "24 valid findings (9, 5, 6, 4)" = 24; w3 "14 valid
  findings (6, 4, 2, 0, 2)" = 14. All three sums are correct.
- Counted the three exploration files cited as "1514 lines: metrics-path-anchor-to-source.md
  521, metrics-path-plan-declared.md 483, metrics-path-independent-map.md 510":
  `wc -l` on all three gives exactly 521, 510, 483 (1514 total). Exact match.
- Confirmed the backlog steps the step file cites as fold/queue destinations exist with
  the stated order and status: `sidecar-ref-empty-string` (order 63, deferred),
  `sidecar-ref-symlink` (order 64, deferred), `status-resume-ignores-json` (order 96,
  not-started), `test-tmpdir-repo-assumption` (order 95, not-started),
  `checks-runner-worktree-name-collision` (order 93, complete). All five match exactly.
- Reproduced `run_next`'s resume-reason branching (`src/main.rs:1749-1762`): the
  containment check, `ledger_path.exists()`, and `extract_resume_state` are three
  distinct branches yielding `LedgerNotThisProject`, `LedgerAbsent`, and
  `NoResumeSection`/`Some`, matching the vocabulary the step file specifies.

## Settled ground reached, not raised

While checking backlog-step citations I found that "the validation-constraints step",
named six times in the step file (`:145`, `:161`, `:193`, `:275`, `:339`, `:393`) and
three times in `Q-55`'s own text (`:1726`, `:1730`, `:1734` of `plan.toml`) as an
existing fold/queue destination, does not exist as a `[[step]]` anywhere in
`docs/plans/agent-scaffold.plan.toml` (`grep -n 'slug = "validation-constraints'`
returns nothing; the full step-slug list has no match). This is exactly the kind of
dangling-destination defect this sweep was built to find, and I verified it
independently before checking the review history.

It is already settled. The ledger records it as accepted residual `F-5` (`docs/plans/agent-scaffold.ledger.md:881`
and `:1031`): "THE ACCEPTED RESIDUAL IS `F-5`, THE DANGLING `validation-constraints`
HANDLE, and the triager's ground is the right one: it is PRE-EXISTING DEBT rather than
a defect of this fold ... THE RIGHT ACTION IS ENTERING THAT STEP AS ITS OWN PLAN ITEM,
NOT PATCHING THIS FOLD to stop citing it. Do not re-raise it here." I have no new
evidence against that verdict, so I am not raising it. Round 3's still-true lens
reached the same handle and stopped at it for the same reason (`workflow-enforcement-tier-inc4-r3-triage.md`,
"Recorded residuals and settled dismissals").

## A count I tried to reproduce and could not, and am therefore not reporting

`workflow-enforcement-tier-inc3`'s risk-classification paragraph (`:306`) and the two
ledger sites it draws on both state step 92 spent "six rounds and fifteen findings ...
joint-third of the artifacts ever reviewed against a project median of two rounds." I
tried to reproduce the median by grouping `type:"round"` records in
`docs/metrics/workflow.jsonl` by `(task, artifact)` and counting rounds per group: this
gives 175 artifacts, median 1, not 2. But this method is unsound for this data: the
`artifact` field's text changes between rounds of the same real review loop (for
example `workflow-enforcement-tier-inc2`'s four rounds are recorded as four artifact
strings each different, "inc2 as built", "inc2 after the round 1 fix pass", etc.), so
my grouping systematically splits multi-round series into several 1-round entries and
cannot recover the true per-artifact round count from this log alone. I could not
devise a reliable reproduction within the scope of this round, so I am not reporting
this claim as false: I have a method that disagrees with the document, but I do not
trust the method. Flagging the attempt and the reason it failed, per the instruction to
say what was exercised even on a negative result.

## Severities

- `critical`: none found.
- `high`: none found.
- `medium`: none found.
- `low`: none found.

## Sections read but not independently re-verified beyond the checks above

- All 23 acceptance checks: read for internal consistency and cross-checked against a
  handful of citations they name, but not executed end to end as their own artifact;
  that is explicitly another reviewer's brief this round (the acceptance-check
  execution lens), and re-running all 23 here would duplicate that pass rather than add
  a different lens.
- The historical process claims that cite counts from past review rounds not
  reproducible from the current tree (for example "51 adversarial attacks", "two
  independent claim inventories of 81 and 118 claims", "30 mutations ... found 11
  uncaught", the `+79/-15` and `+96/-13` diff-size figures for a rejected alternative
  implementation that was never committed). These describe work in worktrees that no
  longer exist, so they are not independently reproducible against `7ab5d48`, and nothing
  in the tree contradicts them.
- Explicitly out of scope per the brief and not touched: `run_validate`'s "`--plan`
  still clap-required" claims, `src/next.rs:162` and `:181-183`, the Status narrative at
  `docs/plans/agent-scaffold.md:7`, the `src/checks.rs` citations in
  `checks-runner-worktree-name-collision.md` (one deliberately stale by `Q-55-check21b`),
  line length/wrapping, and the four items round 3 already ruled into a future backlog
  step (no gate for this class, the W6 waiver-note join, dangling decision-receipt
  detection, an unrun documentation gate).

## Fixture hygiene

All fixtures built under
`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/rev-inc4-r4-c/`.
Nothing written to bare `/tmp`. Nothing deleted outside that subdirectory. No `chmod`
was used, so none is owed a restore. No file in the repository was modified except this
findings file; `git status --short` in this worktree shows only this file untracked
before commit.

## ASCII check

`LC_ALL=C grep -n '[^ -~]' <this file>` returns 0 hits, verified before commit.
