# Inc 6 review: code correctness / mechanism (opus)

Lens: CODE CORRECTNESS / MECHANISM, reviewed adversarially. Scope: the four
commits f926b5b / 88e7d8a / 7f20bfc / b1f993a, restricted to `src/` and
`pack/pack.toml` (docs and template-prose are other reviewers' lenses).

Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/inc6`.
Diff base: `593f8c9..impl/structured-skeleton-inc6`.

Verdict: two findings, both medium-or-below, in the `--workflow` clap
relaxation (item 2). The scaffold-render wiring (item 1), the drift-guard drop
(item 3), the manifest golden (item 4), and the `pack.toml` remap (item 5) are
correct. `cargo test` and `cargo clippy --all-targets` are green (see the
verbatim results at the end).

## M-1: `--workflow` requested but silently skipped -> exit 0 (false-green regression)

- Severity: medium.
- Evidence: `src/main.rs:802-840` (the `if args.workflow` match) and the field
  help at `src/main.rs:380-381` ("the Markdown path still needs --plan present").
- The relaxation removed clap's `requires = "plan"` from `--workflow`. That was
  the ONLY guard forcing a plan source to be present. The runtime match now has
  a catch-all `_ => eprintln!("--workflow needs a metrics log and either a TOML
  source or a Markdown plan present; skipping the workflow check")` arm
  (`src/main.rs:836-838`) that prints to stderr and falls through to exit 0
  whenever `--workflow` is set but no TOML-primary source AND no readable
  `--plan` is available. So a requested gate is silently not run and reports
  success.
- The field help claims "the Markdown path still needs --plan present", but
  nothing enforces that: omitting `--plan` on the Markdown path does not error,
  it skips and exits 0.
- This is a REGRESSION, not just a pre-existing soft-skip: before Inc 6 these
  invocations were hard clap usage errors (exit 2). Empirically confirmed in the
  worktree (built binary, metrics log present):
  - `validate --workflow --source <markdown-primary>.plan.toml` (no `--plan`)
    -> exit 0, "skipping the workflow check".
  - `validate --workflow` (no source, no `--plan`) -> exit 0, "skipping".
  - `validate --workflow --source does-not-exist.plan.toml` (a typo'd/missing
    source path, no `--plan`) -> exit 0, "no source plan at ...; nothing to
    validate" + "skipping the workflow check".
- Why it matters: the typo'd-source case is the realistic CI failure mode. A
  pipeline running `validate --workflow --source docs/plans/foo.plan.toml` as a
  gate gets a green exit if the path is wrong, the source stops parsing, or its
  `[meta].primary` is not `"toml"` (any of which makes `toml_source`/`source_plan`
  yield `None`). The gate that is supposed to enforce W3/W4 passes while checking
  nothing. Principle 12 (fail loudly) says a requested-but-unrunnable check
  should exit non-zero, not print a stderr note and return success. The task
  brief itself flagged this exact expectation ("--workflow with a MARKDOWN source
  and no --plan still fails cleanly ... not a silent false-green"); the code does
  the opposite.
- Note on the fix's shape (for the author to decide, not prescribed here): the
  `_` arm also legitimately covers "metrics log missing" (which pre-dates Inc 6
  and is arguably a valid skip), so the remedy needs to distinguish "no plan
  source while `--workflow` was requested" (should be a hard error / pushed into
  `problems`) from "no metrics log". A blanket exit-non-zero on the whole `_` arm
  would also change the metrics-missing behavior.

## M-2: the new regression test's doc comment overstates its coverage; no negative direction tested

- Severity: low.
- Evidence: `tests/validate_workflow_toml_source_needs_no_plan.rs:6-9` and the
  single test body at lines 62-82.
- The module doc comment claims the test "pins two directions: the TOML-primary
  `--workflow --source` with no `--plan` ... and the Markdown path still works
  when `--plan` is supplied." The file contains exactly ONE test function,
  `workflow_on_a_toml_source_runs_without_a_markdown_plan`, which exercises only
  the TOML-primary direction. The claimed "Markdown path still works when --plan
  is supplied" direction is not in any test body.
- More importantly for the lens: neither this test nor the pre-existing
  `tests/validate_toml_primary_skips_markdown_plan.rs` covers the NEGATIVE
  direction that the relaxation newly makes reachable (M-1): `--workflow` with no
  resolvable plan source and no `--plan` should fail, and there is no test
  asserting it does (or documenting that it exits 0). So the one behavior the
  relaxation most needed to pin against regression is untested, and the green
  suite gives false confidence that the relaxation is fully covered.
- The positive test itself is sound and non-tautological: it runs the built
  binary and asserts `exit 0` + stdout `"workflow invariants hold"`, which is the
  real behavior, not a restatement of the code.

## Items with NO findings (checked, correct)

- Item 1, scaffold-render wiring (`src/main.rs:1341-1366`): correct. Confirmed
  empirically: dry-run (no `--write`) renders nothing and leaves no files;
  `--write` renders `docs/plans/TEMPLATE.md` after the asset writes; the
  `strip_suffix(".plan.toml")` -> `<stem>.md` dest handling is right
  (`TEMPLATE.plan.toml` -> `TEMPLATE.md`); the loop iterates all assets, so
  multiple `.plan.toml` drops would each render; a render failure is fatal via
  `std::process::exit(2)` with a per-problem stderr line (Principle 12). Verified
  no panic on a missing sidecar: `render` on a `.plan.toml` whose
  `[meta.sidecars].front` names a missing file exits 1 with a clean "missing or
  unreadable sidecar" message, so the scaffold's fatal path is a clean non-zero
  exit, not a panic. The post-failure tree (source + sidecars present, generated
  `.md` absent) is recoverable by re-running `render` and is not a misleading
  half-write.
- Item 2, positive path: `validate --workflow --source <toml-primary>` with no
  `--plan` reaches `check_workflow_toml` and passes (exit 0, "workflow invariants
  hold"), confirmed on both a hand-written minimal source and a fresh scaffold's
  `TEMPLATE.plan.toml`. The Markdown path with `--plan` present is unchanged
  (arm 2). The defect is only the missing-plan-source false-green (M-1).
- Item 3, drift-guard drop (`src/plan.rs`, `plan_template_documents_every_accepted_status`
  removed): genuinely subsumed, no real coverage lost. The rendered vocabulary is
  generated from the constants (`vocabulary_section` at
  `src/plan/render.rs:410-419` joins `ROADMAP_STATUSES` and
  `QuestionStatus::ALL.label()`), and the render golden asserts FULL byte-equality
  (`assert_eq!(first, GOLDEN)` at `src/plan/render.rs:650`, golden at
  `src/plan/testdata/render-fixture.md:25-30`), so any status rename/add/remove
  changes render output and fails the golden. The compile-time drift asserts still
  exist: `StepStatus::ALL == VARIANTS` (`src/plan/source.rs:218`),
  `QuestionStatus::ALL == VARIANTS` (`src/plan/source.rs:335`), plus
  `StepStatus::ALL` labels subset of `ROADMAP_STATUSES`
  (`src/plan/render.rs:1074-1078`). No status that render would omit escapes these
  checks. The only thing truly gone is the tie from the now-deleted
  `pack/plan-template.md` to `QUEUE_EXACT_STATUSES`; `QUEUE_EXACT_STATUSES` is the
  legacy Markdown-queue accepted set, is distinct by design from
  `QuestionStatus::ALL` (Markdown uses the parametric `decided -> folded into`
  form; TOML uses the `decided` variant + `folded_into`), and is still exercised
  by its own `is_valid_queue_status` tests. Not a defect.
- Item 4, manifest golden (`src/manifest.rs:580-595`): the dest-list update is a
  legitimate golden pinning the 13-way remap in order; combined with the observed
  live scaffold it is not tautological. (The test-comment nit is M-2.)
- Item 5, `pack.toml` remap: correct. All 13 mapped `source` files exist on disk
  under `pack/` (verified none missing), `pack/plan-template.md` is deleted, and
  the diff touches only the template asset block, leaving `AGENTS.md` and the
  `.agents/*` reference assets byte-identical (the manifest golden confirms their
  dests are unchanged). A fresh module-free scaffold differs from before only by
  the template swap plus the new `render` step, as intended.

## Severity summary

- critical: 0.
- high: 0.
- medium: 1 (M-1).
- low: 1 (M-2).

## Verbatim test / clippy results (run in the worktree)

`cargo test`: all suites passed, 0 failed. Library unit suite: `test result:
ok. 292 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`. Integration
suites all `test result: ok ... 0 failed`, including
`scaffold_precommit_hook` (3 passed), `validate_toml_primary_skips_markdown_plan`
(1 passed), `validate_workflow_toml_source_needs_no_plan` (1 passed),
`checks_staged_hook_env` (1 passed).

`cargo clippy --all-targets`: clean. `Finished \`dev\` profile [unoptimized +
debuginfo] target(s)` with no warnings or errors emitted.
