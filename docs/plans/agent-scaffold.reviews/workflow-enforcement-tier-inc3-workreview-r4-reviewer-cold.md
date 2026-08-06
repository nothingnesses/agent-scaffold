# `workflow-enforcement-tier-inc3` work review, ROUND 4, COLD-CONFORMANCE lens

Reviewed on branch `review/inc3-r4-cold` at `aed035c`, the tip of the branch under
review. Governing specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`,
the `workflow-enforcement-tier-inc3` bullet, its risk-classification paragraph, acceptance
checks 15 through 20, the `INC3:` documentation-impact block, and "Scope: what this step
does not do". The spec was read in full before the diff, and the diff before the three
prior rounds' triage files, per the brief's ordering.

METHOD. `which cargo` resolves to `/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo`
(`cargo 1.98.0-nightly`, confirmed via `direnv allow && eval "$(direnv export bash)"`, no
`2>/dev/null` on the export). `TMPDIR` was pointed at
`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/rev-r4-cold/tmpdir`,
outside any git repository, for every `cargo test`. All fixtures live under
`.../scratchpad/rev-r4-cold/`, a directory of my own naming.

TWO BINARIES were used for red-then-green comparisons, so nothing below rests on trusting a
prior round's report of what the old build did:

- NEW: this worktree at `aed035c`, `target/debug/agent-scaffold`, built by me.
- PRE: `9eeca42` (predates the whole increment), the already-built binary at
  `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/verify-inc3-prefix/target/debug/agent-scaffold`,
  a pre-existing worktree checked out at that exact commit. I did not build this myself, but
  I verified its commit (`git log --oneline -1` in that worktree reads `9eeca42`) before
  trusting its output.

GATES, run on the tree as reviewed: `cargo build` clean; `cargo test` 422 passed, 0 failed,
across nine binaries; `cargo clippy --all-targets -- -D warnings` exit 0; `cargo run --
render docs/plans/agent-scaffold.plan.toml --check` reports `up to date`. No source edit was
made anywhere in this worktree; `git status --short` is empty throughout and at the time of
writing.

---

## Job A: the acceptance check table

Checks 15, 16, 17, 18 and 20 are the ones this round's brief names as commands with expected
exit codes. Each was run against NEW, and, for checks 15, 17, 18 and 20's grep half, also
against PRE to confirm the check actually discriminates rather than passing on both builds.

| Check | Command | Observed (NEW) | Pass/Fail | Judgement: does it establish what it claims |
| --- | --- | --- | --- | --- |
| 15 | (inside a fresh non-instrumented fixture) `validate --source docs/plans/TEMPLATE.plan.toml --workflow` | stderr: `no metrics log at docs/metrics/workflow.jsonl; nothing to validate` then `--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a \`--metrics\` naming this project's log, or record the project's review rounds there`; exit 1 | PASS | YES. PRE on the identical fixture exits 0 with the old skip note (`--workflow has a plan source but the metrics log is missing; skipping the workflow check`) and a stdout `valid` summary. The check genuinely discriminates pre/post. |
| 16 | re-run check 10: `validate --source docs/plans/TEMPLATE.plan.toml` (no `--workflow`), and bare `validate` with no source | stderr: `no metrics log at docs/metrics/workflow.jsonl; nothing to validate`; exit 0 for both | PASS | LIMITED, and this is the gap already recorded and not to be re-raised: the check as written re-exercises only the plain-absent-log input, which was never put at risk by this increment (both PRE and NEW give byte-identical output on it), rather than the unsearchable-ancestor input `Q-55-existsgate` actually turned on. It confirms no regression on the input it tests, but a defective implementation that broke plain `validate` only on the unreadable-ancestor case would still pass this check as written. |
| 17 | borrowed-slug fixture (`triager-runs-only-on-findings`, `complete`) with an empty `docs/metrics/workflow.jsonl`, `validate --source ... --workflow` | stdout/stderr: W3 message naming the step, `... has no round records and no covering waiver ...`; exit 1 | PASS | YES, as a control rather than a discriminator: NEW and PRE are byte-identical here (both exit 1 with the same message), which is exactly what a control should show, the underlying W3 mechanism is untouched by this increment, so the new failure path added by inc3 is additive rather than a replacement of a working check. |
| 18 | `cd docs/plans && validate --source TEMPLATE.plan.toml --workflow` (bare filename, no parent to derive a root from) | stderr: `no metrics log at docs/metrics/workflow.jsonl; nothing to validate` then the same `no round log at ...` problem; exit 1 | PASS | YES. PRE on the identical fixture exits 0 with the old skip note. Accepted cost (i) is confirmed pinned exactly as the spec describes: a silent miss under inc1 alone becomes a hard failure naming the path it looked for under inc3, in neither case reading the project's real log. |
| 20 | rebuild a fixture without `--instrument`, grep `AGENTS.md` for the backstop sentence; then `cargo test` (drift guards) | `AGENTS.md:93` (and the two deployed copies, and `pack/AGENTS.md`) read: "...when instrumentation is on, the deterministic \`validate --workflow\` check is the backstop... and on a project with no round log yet, which every project scaffolded without \`--instrument\` remains, that check exits non-zero reporting that it could not run rather than passing." `the_committed_scaffold_matches_a_fresh_render` and `the_committed_role_prompts_match_a_fresh_render` both pass. | PASS | YES. PRE's fixture carries the old, unqualified sentence with "once built" and no log-scoped qualifier, the qualifier is new to this increment. I independently rebuilt both a non-instrumented and a freshly-`--instrument`ed fixture and ran check 15's command in each: both exit 1, and the sentence's operative rule ("on a project with no round log yet") correctly predicts this for both populations, including the freshly-instrumented one that has not yet run a round. This closes exactly what round 1's `T-2` finding required. |

No check among the five was found to pass vacuously on both PRE and NEW, other than check
16's already-recorded gap.

---

## Job B: delivered change against delivered specification

All eight changed files (`.agents/AGENTS.reference.md`, `AGENTS.md`, `CHANGELOG.md`,
`README.md`, `pack/AGENTS.md`, `src/main.rs`, and the two test files) were accounted for
against the `INC3:` documentation-impact list and the increment bullet:

- `src/main.rs`'s three hunks (the `--workflow` help clause, the `run_validate` doc comment,
  and the `try_exists`-gated match arm) match the increment bullet's "the `_` catch-all ...
  becomes a reported problem" and the documentation-impact block's `run_validate` doc-comment
  item. The extra help-string clause is the one this round's brief pre-clears as already
  ruled in scope by round 1's triage; I read that triage's reasoning (the enumeration would
  otherwise be short by one, and the governing "each item travels with the increment that
  makes it stale" sentence permits a site to travel twice) and I agree with it independently:
  the clause is true on every problem-producing path I exercised, and removing it would leave
  the help text silent about a cause the code now has.
- Both test files' edits (module doc and the `:96-98`-area comment in
  `validate_workflow_toml_source_needs_no_plan.rs`, the rename in
  `metrics_and_ledger_anchor_to_the_plan_source.rs`) match the documentation-impact block's
  explicit instruction to place inc3's new case as a sibling of the two already in that file,
  and to keep accepted cost (i)'s pinning test current with its new exit code.
- `pack/AGENTS.md:93` and its two deployed copies match the `INC3:` block's `WHAT IT MUST
  SAY` / `WHAT IT MUST NOT DO` instructions: the qualifier names the instrumented tier, the
  no-`--instrument` population, and the refusal, without restating `pack/instrument.md`'s
  schema or lengthening the paragraph, and it uses the same "when instrumentation is on"
  phrasing already established at `:61` and `:63`.
- `README.md`'s two hunks match `README.md:210` (named explicitly) and the accepted-cost-(i)
  sentence at `:234` (made stale by this increment specifically, per round 1's `T-5`, and
  fixed by the same governing "travels with the increment that makes it stale" rule).
- `CHANGELOG.md`'s `Changed` addition matches the block's instruction to name the exit-code
  flip and the population it breaks. Its `Added`-bullet edit ("It requires `--plan` and
  reuses" -> "It reuses") is not named by the `INC3:` block; it corrects a claim an *earlier*
  increment's clap relaxation made false, in the same unreleased block this diff already
  touches. Round 1's triage (`T-6`) considered exactly this provenance question and kept it
  in scope on the ground that deferring a three-word deletion costs more than making it. I
  independently re-checked the claim (`ValidateArgs::plan` is `Option<PathBuf>` with no
  `required`, `ValidateArgs::workflow` carries no `requires = "plan"`) and it is true and
  minimal. I do not raise this as new scope creep: it is a pure deletion, it does not
  introduce a false claim, and re-litigating an already-reasoned triage call on the same
  evidence would not change the answer.

SCOPE (the "what this step does not do" list): confirmed by inspection of the diff that no
hunk touches `src/workflow.rs`, `src/next.rs`, `src/plan/source.rs`, or any of the shared
`resolve_metrics_path` / `checked_plan_root` / `is_outside_root` machinery `status` and `next`
also use, every touched line in `src/main.rs` is inside `run_validate`'s own body or its
doc comment, or `ValidateArgs::workflow`'s help string. `git diff --stat main...HEAD` also
confirms none of `docs/plans/agent-scaffold.ledger.md`, `docs/plans/agent-scaffold.plan.toml`,
`docs/plans/agent-scaffold.md`, `docs/metrics/workflow.jsonl`, or anything under
`docs/plans/*.reviews/` appears in the reviewed diff, and no step status or increment
declaration is touched.

USER-FACING TEXT, read end to end: the `--workflow` help, the new error message (both the
`Ok` "no round log at ..." and the `Err` "... could not be checked (...)" forms), the
README `validate` paragraph, the `CHANGELOG` unreleased entries, and the scaffolded
`AGENTS.md` built both with and without `--instrument`. I rebuilt both fixtures independently
and confirmed the backstop sentence differs from the two fixtures only in expected ways
(the `## Instrumentation` section, and nothing else) and that the sentence's stated rule
("on a project with no round log yet ... that check exits non-zero") correctly predicts
check 15's exit code for both. I found nothing a user would get wrong from this text that is
not already recorded as `R2A-4`'s residual (the stale "no metrics log at ..." note printed one
line above the corrected sentence) or `R3A-1`'s residual (the "pass a `--metrics`" advice
being inert when the resolved path is already the project's own unreadable log), both
reproduced identically to their recorded descriptions and neither is new.

---

## Findings

None. I found no new conformance gap, false claim, or scope violation in this round.

Everything I checked either (a) matches the specification and behaves as documented and
tested, or (b) is a residual, an accepted cost, or a settled ruling already recorded by an
earlier round, and reproduces identically to that record. Specifically checked and confirmed
unchanged: `R2A-4` (the stale absent-log note above the corrected `--workflow` sentence,
reproduced on a mode-600 `docs/metrics` fixture), `R3A-1` (the inert `--metrics` remedy clause
on a default-anchored unreadable log, reproduced the same way), accepted costs (i) and (ii)
(reproduced via checks 18 and the symlink case is unchanged from round 3's spot-check, not
re-run in full here since it is inc2's mechanism and untouched by this diff), and the two
round-3 fix-pass commits (`37df2ab`, `aed035c`), each of which is confirmed to be exactly the
one-word/four-word deletion its triage prescribed and nothing more.

The severity ceiling for this round is therefore clean at zero. Per the brief, a clean round
here starts the required two-consecutive-clean-round streak; it does not by itself end the
loop.

## Relitigation and constraints check

Nothing in this file raises or reopens: the four standing residuals (the in-root bound, the
single-anchor `..` case, the earlier increment's rejected-ledger context slot, the
off-convention `--source` surface); accepted costs (i) through (iv); round 1's `ADV-4` or
`SC-3`; round 2's `R2A-4`, `R2B-2`, or `R2B-3`; round 3's `R3A-1` (accepted as residual) or
`R3A-3` (routed to validation-constraints); the pre-existing plain-`validate` inconsistency
(queued to validation-constraints); `Q-55-existsgate`'s declined `try_exists()?` gate change;
or the check-16 gap named in this round's brief. No line-length, prose-wrapping, or
comment-raggedness observation appears anywhere in this file.

FIXTURE HYGIENE: all fixtures built under
`/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/rev-r4-cold/`,
a directory of my own naming; nothing outside it was written or deleted. The one fixture
chmodded to 600 (`r2a4-check/docs/metrics`, to reproduce `R2A-4`/`R3A-1`) was chmodded back to
755 before this file was written, and a closing sweep (`find ... -type d ! -perm -u+rwx` and
`find ... -type f -perm 000`) over the whole scratch directory returns nothing. `TMPDIR` was
outside any git repository for every `cargo test`. No `nix fmt` and no `just scaffold-self`
was run. No source file in this worktree was edited; `git status --short` is empty.
