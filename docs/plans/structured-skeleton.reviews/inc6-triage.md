# Inc 6 triage (round 1)

Independent triager, independent of the implementer (producer) and the orchestrator. Verdicts below are verified against the worktree `.claude/worktrees/inc6` at branch `impl/structured-skeleton-inc6` (base `593f8c9`), judged against `AGENTS.md` Principles 6, 11, and 12.

## M-1: `--workflow` requested but silently skipped -> exit 0 (false-green regression)

- Verdict: VALID.
- Final severity: medium (confirming the reviewer; see the high/medium weighing below).
- Reproduced (built worktree binary `target/debug/agent-scaffold`, empty-but-present metrics log):
  - (a) `validate --workflow --metrics <present>` with no `--source` and no `--plan` -> stderr "skipping the workflow check", exit 0.
  - (b) `validate --workflow --source <markdown-primary>.plan.toml` (no `--plan`) -> stderr "skipping the workflow check", exit 0.
  - (c) `validate --workflow --source <typo'd/missing>.plan.toml` (no `--plan`) -> "no source plan at ...; nothing to validate" + "skipping the workflow check", exit 0.
  - Positive control: `validate --workflow --source <toml-primary>.plan.toml` (no `--plan`) -> "workflow invariants hold", exit 0. The legitimate new case works.
- Root cause confirmed in the diff: `#[arg(long, requires = "plan")]` on `workflow: bool` became `#[arg(long)]` (src/main.rs:381-382). That clap `requires` was the only guard forcing a plan source to be present. The runtime match at `src/main.rs:802-840` has a `_` catch-all (lines 836-838) that prints a stderr note and falls through to exit 0 whenever `--workflow` is set but neither a TOML-primary source nor a readable `--plan` resolves.
- Why valid: this is a correctness regression in the tool's own validator, on the exact path Inc 6 changed. A requested gate reports success while checking nothing. This is precisely the failure Principle 12 (fail fast and loudly: report errors visibly rather than swallowing them) and Principle 6 (verify, don't trust) guard against. The field help at src/main.rs:381 claims "the Markdown path still needs --plan present," but nothing enforces it: case (b) proves the Markdown path skips and exits 0 with no `--plan`. Before Inc 6 all three cases were hard clap usage errors (exit 2), so this is a regression, not a pre-existing soft-skip. The typo'd-source case (c) is the realistic CI failure mode: a pipeline running `validate --workflow --source docs/plans/foo.plan.toml` as a W3/W4 gate goes green if the path is wrong, the source stops parsing, or its `[meta].primary` is not `"toml"`.
- Severity weighing (medium vs high): I confirm medium, not high. Impact if left unfixed is a silent false-green on the workflow gate, which is serious, but it fires only under a misconfigured invocation (typo'd/missing path, wrong `[meta].primary`, or the Markdown path missing `--plan`); a correctly configured gate still runs and reports truthfully. The tool already prints a visible stderr note in every skip case, so the defect is that the exit code does not honor it, not a fully silent swallow. The soft-skip-with-note shape also matches the pre-existing metrics-missing skip in the same `_` arm. Those mitigations keep it below high. It is at the high end of medium, and I would not object to high; it is not low.
- Concrete fix: split the `_` catch-all so that "`--workflow` requested but no usable plan source" becomes a hard problem, not a stderr skip. The discriminant is independent of metrics: when `args.workflow` is set and `toml_primary.is_none() && plan_contents.is_none()` (neither a TOML-primary `--source` nor a readable `--plan` resolved), push a message into `problems` (e.g. "--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan") so the run exits 1 via the existing `if problems.is_empty()` branch at src/main.rs:842-852. Leave the remaining `_` (a plan source is present but the metrics log is missing) as the pre-Inc-6 stderr skip, so the fix does not change the metrics-missing behavior the reviewer flagged as arguably valid and pre-existing. This permits TOML-only (no `--plan`) while rejecting no-source-at-all, and keeps the legitimate new case (`--workflow --source <toml>` with no `--plan`, arm 1: `(Some(source), _, Some(metrics_text))`) untouched. Verified: cases (b) and (c) both land in `toml_primary None + plan_contents None`, so both would hard-error under this fix; the positive control stays in arm 1.

## M-2: the regression test's doc comment overstates coverage; no negative direction tested

- Verdict: VALID.
- Final severity: low (confirming the reviewer).
- Confirmed: `tests/validate_workflow_toml_source_needs_no_plan.rs` module doc (lines 6-9) claims it "pins two directions": the TOML-primary `--workflow --source` with no `--plan`, and "the Markdown path still works when `--plan` is supplied." The file has exactly one test function, `workflow_on_a_toml_source_runs_without_a_markdown_plan` (lines 52-82), which exercises only the TOML-primary direction. The claimed Markdown-with-`--plan` direction is in no test body. Neither this test nor the pre-existing `tests/validate_toml_primary_skips_markdown_plan.rs` covers the negative missing-plan-source case M-1 exposes.
- Why valid: Principle 11 (tests must actually exercise what they claim). The doc comment claims a direction the code does not run, and the one behavior the relaxation most needed pinned against regression (the missing-plan-source path) is untested, so the green suite gave false confidence and did not catch M-1. Severity low: the positive test itself is sound and non-tautological (runs the built binary, asserts exit 0 + stdout "workflow invariants hold"); the gap is coverage and an overstated comment, no runtime impact of its own.
- Concrete fix: add the negative case so the suite would have caught M-1, and reconcile the doc comment with what the file tests. Specifically: (1) add a test that runs `validate --workflow` with a present metrics log but no resolvable plan source (no `--source`/`--plan`, and separately a Markdown-primary `--source` with no `--plan`, and a typo'd `--source`) and, once M-1 is fixed, asserts the hard error (exit 1, a problem line naming the missing plan source); (2) either add the Markdown-with-`--plan` positive direction the doc comment claims, or correct the doc comment to describe only the direction actually tested. Best done together with the M-1 fix so the negative assertion pins exit 1 rather than documenting the bug.

## Zero-findings reviews (noted)

- `inc6-doccurrency-sonnet.md` (doc/prompt currency and single-sourcing): zero findings. No verdict required.
- `inc6-template-acceptance-opus.md` (template coherence, acceptance criteria, self-scaffold regen): zero findings. No verdict required.

## Severity summary

- critical: 0.
- high: 0.
- medium: 1 (M-1, valid).
- low: 1 (M-2, valid).

No high/critical dismissals, so no backstop re-check is triggered. Neither finding is raised to high.
