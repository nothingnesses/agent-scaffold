# Review: validation-constraints-inc1, round 1, lens: shipped promises vs shipped behaviour

Reviewer worktree: `.claude/worktrees/rev-inc1-contract`, branch `review/inc1-contract`.
Artifact: `git diff main..HEAD` (touches `src/workflow.rs`, `src/plan/source.rs`, `CHANGELOG.md`,
`pack/instrument.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`).

Summary: no defect found in what the diff asserts to a reader. Everything the diff claims
(the three rule-text copies, the touched doc comments, the CHANGELOG entry's substance, the
error messages, and the `leading_slug` comment left alone) checked out against the code and
against runnable evidence. One placement question (Fixed vs. amending Added) is a genuine
judgement call, ruled on below; I record it as a low-severity advisory finding as well so the
triager has it on record, not because the content is wrong.

## Findings

### W1C-1 (low): the CHANGELOG's "could not be marked `complete`" overstates a mechanical fact

Claim: `CHANGELOG.md`'s new `### Fixed` entry says the old rule meant "the step carrying it
could not be marked `complete`." Nothing in the code mechanically prevents writing
`status = "complete"` in the plan TOML; there is no git hook, CI config, or parser check tying
step-status assignment to `validate --workflow`'s exit code (checked: no `.github/workflows`
in this repo, and the shipped pre-commit hook module, `pack/hooks`, gates `lint`/`format`
checks, not `--workflow`). What the old rule actually did was make `validate --workflow` report
a problem (not "workflow invariants hold") for such a step.

Mitigating: this is the established rhetorical convention of this exact CHANGELOG section
already (for example the `### Changed` entry at `CHANGELOG.md:23`, "Asking for the workflow
check and getting exit 0 ... is a false green"), and it matches `AGENTS.md`'s own framing of
`validate --workflow` as "the backstop that the required reviewed rounds happened before a step
is marked complete." Read against that convention, "could not be marked complete" means
"could not be marked complete and have the deterministic gate agree," which is how this
project already talks about step completion elsewhere. I am not confident this rises above
`low`, and I would not block on it, but it is worth the triager's eyes since it is a factual
overstatement taken at face value.

Evidence: `CHANGELOG.md:32`; `AGENTS.md:93` ("the deterministic `validate --workflow` check is
the backstop..."); `find . -iname "*.yml" -o -iname "*.yaml"` under the repo root finds no CI
config; `pack/hooks` and `tests/scaffold_precommit_hook.rs` show the shipped pre-commit hook
runs the `checks` module (lint/format), not `--workflow`.

## Verification performed (all passed, no findings)

1. **Three-copy byte consistency.** `pack/instrument.md` (15 lines) is byte-identical to the
   corresponding fragment in both generated copies:
   ```
   diff pack/instrument.md <(sed -n '137,151p' AGENTS.md)              # no output
   diff pack/instrument.md <(sed -n '137,151p' .agents/AGENTS.reference.md)  # no output
   ```
   Confirmed both diffs are empty.

2. **Drift-guard demonstration (revert one file, keep the other two current).** Built a
   scratch copy of `HEAD` (`git archive HEAD | tar -x -C <scratch>`, outside the worktree, per
   the fixture-safety rule) at
   `/tmp/claude-1000/.../scratchpad/rev-inc1-contract-driftcheck`, then reverted only the
   waiver bullet in that copy's `AGENTS.md` back to the pre-fix wording ("the increment's
   leading slug equals the step"), leaving `pack/instrument.md` and
   `.agents/AGENTS.reference.md` at the new wording. Ran:
   ```
   cargo test agents_md_drift
   ```
   Result:
   ```
   test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... FAILED
   thread '...the_committed_scaffold_matches_a_fresh_render' panicked at src/agents_md_drift.rs:402:9:
   assertion `left == right` failed: root AGENTS.md has drifted from a fresh pack render
   (ignoring prettier wrapping); run `just scaffold-self`
   test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 377 filtered out
   ```
   The other three `agents_md_drift` tests still pass. So the guard does what the diff and the
   plan's acceptance item 7b need it to do for a *partial* edit across the three copies: it
   fails loudly and names the fix. (It does not, by its own documented COVERAGE, catch prose
   that states a rule the code no longer implements when all three copies drift together in
   the same wrong direction, which is acceptance item 7b's job, run separately below.)

3. **Acceptance item 7b, run directly** (the sidecar's own fixed-string check that the shipped
   rule text matches the shipped behaviour):
   ```
   grep -c -F "the increment's leading slug equals the step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md
   # pack/instrument.md:0  AGENTS.md:0  .agents/AGENTS.reference.md:0
   grep -c -F "the round log must join that increment to that step" pack/instrument.md AGENTS.md .agents/AGENTS.reference.md
   # pack/instrument.md:1  AGENTS.md:1  .agents/AGENTS.reference.md:1
   ```
   Matches the plan's own acceptance criterion exactly.

4. **Doc comments the diff added or changed** (`waiver_covers_round`'s doc comment, the
   rewritten `w5_problems` doc comment, the `w3_problems` "Exempt this increment iff..."
   inline comment, `step_attribution`'s doc comment, `src/plan/source.rs`'s new clause on the
   declared-increment membership check): read each against the code beside it. All accurate;
   each documents a *why* or a *non-obvious relation* (Principle 19: e.g. why the predicate
   takes the round rather than a caller-supplied step/increment pair, so a caller cannot
   collapse the comparison into comparing a value with itself), not a restatement of the *what*.

5. **Mutation test of the doc comment's own claim.** `waiver_covers_round`'s doc comment
   (`src/workflow.rs:411-425`) asserts: "It takes the round rather than a step slug and an
   increment id the caller supplies ... which is the mutation acceptance check 4b exists to
   catch." Verified by actually applying that mutation, in a second scratch copy
   (`/tmp/claude-1000/.../scratchpad/rev-inc1-contract-mutation`, never in the reviewed
   worktree): changed
   ```rust
   && waiver.step == round_step_slug(round)
   ```
   to
   ```rust
   let _ = round_step_slug(round);
   ...
   && waiver.step == waiver.step
   ```
   (drops the round's step axis, comparing `waiver.step` to itself). Ran
   `cargo test --bin agent-scaffold workflow::`:
   ```
   test result: FAILED. 57 passed; 4 failed
   failures:
       workflow::tests::a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment
       workflow::tests::check_workflow_toml_w5_refuses_an_increment_the_log_joins_to_another_step
       workflow::tests::w5_flags_an_increment_waiver_whose_step_does_not_own_its_increment
       workflow::tests::w5_names_every_step_the_log_attributes_a_waived_increment_to
   ```
   The doc's claim holds, and as a bonus this also confirms the CHANGELOG's claim that "W3 and
   W5 ... consult ONE predicate ... and cannot drift": the failing set includes a W3 test
   (`a_mis_scoped_increment_waiver_does_not_exempt_a_short_streak_increment`, which calls
   `w3_problems`) alongside three W5 tests, so the shared predicate is genuinely shared, not
   duplicated per-check.

6. **Error messages.** Read both new W5 refusal strings
   (`src/workflow.rs:619` and `:624-629`) against what they assert:
   - "...which has no `type:\"round\"` records, so the round log attributes it to no step":
     names no step at all, so it cannot assert a false ownership fact (the class of prior
     defect the task pointed at, where a message asserted an ownership fact that need not be
     true). Confirmed by the test `w5_flags_an_increment_waiver_whose_increment_has_no_round_records`.
   - "...but the round log attributes increment `{}` to {}" (via `step_attribution`): names
     only steps a round record actually joins the increment to, and handles the multi-owner
     case (tested by `w5_names_every_step_the_log_attributes_a_waived_increment_to`, a fixture
     with two owning steps). Both messages are self-contained and state a fact the records
     carry rather than a fact derived from the id's shape.

7. **`leading_slug`'s doc comment, left unchanged, ruled correct to leave.** Its claim ("This
   shim remains only for pre-migration records that omit the structured id") is scoped to the
   round/escalation migration story, not to an exhaustive inventory of every caller in the
   file. Checked all real call sites of `leading_slug(` in non-test code:
   `src/workflow.rs:120` (`round_step_slug`, on `round.task`) and `:135` (`escalation_step_slug`,
   on `escalation.task`), both of which operate on a record's `task`, matching the comment. The
   diff *removes* the one caller that was outside that scope (the old W5 code's
   `leading_slug(increment)` on a plan-authored waiver token, not a round/escalation record),
   so the comment's narrow claim is, if anything, more accurate after this diff than before it,
   not less. **Ruling: correct to leave alone.**

8. **CHANGELOG placement ruling (Fixed vs. amending the existing Added entry).** Checked the
   full history of `CHANGELOG.md`:
   ```
   git log -p --all -- CHANGELOG.md | grep -n "^+### Fixed\|^-### Fixed\|^### Fixed"
   # one hit: the line this diff adds
   ```
   This is the first `### Fixed` subsection ever committed to this file, for a check
   (`type:"waiver"` + W5) that was itself introduced under the same `## [Unreleased]` section
   (`CHANGELOG.md:13`) and has never shipped in a tagged release (only `[0.0.1]` is released,
   and it predates W5 entirely). A prior review already measured and recorded this same fact
   when weighing a related question (ledger, "verified the CHANGELOG precedent through the
   full git history (a `### Fixed` subsection has never existed in that file)"), and a related
   decision (`Q-55-changelog`) chose NO ENTRY over adding a `Fixed` entry for an in-cycle
   correction, on the ground that the underlying claim never shipped falsely. The existing
   Added bullet at `CHANGELOG.md:13` is mechanism-agnostic ("an increment-unit waiver's `step`
   owns its `increment`") and was never false, so amending it costs one clause, not a rewrite.
   Weighed against Principle 16 (one source of truth): keeping both a Fixed entry and an Added
   entry that describe the same never-released check risks exactly the two-sources-for-one-fact
   problem the principle warns against, since a future edit to one description is not
   guaranteed to reach the other. **Ruling: this reads better as an amendment to the existing
   Added bullet (naming the round-log mechanism and the narrowing in one added clause) than as
   a new `Fixed` entry, given this project has never used `Fixed` before and the check being
   fixed has never shipped.** That said, the content of the current Fixed entry is itself
   accurate (see W1C-1's caveat aside) and the narrowing is real and worth a sentence somewhere;
   this is a placement call, not a correctness defect, so I am not filing it above `low`.

9. **Other shipped surfaces.** Grepped `README.md`, `.agents/prompts/`, `pack/`, and
   `src/main.rs` (including `validate --help` output, run against the built release binary) for
   the retired wording or any restatement of the ownership mechanism: none found (the help text
   for `--workflow` and `--metrics` describes what the check does structurally, not how
   ownership is decided, so it needed no change and got none). Confirmed no test fixture
   outside the known-stale, explicitly out-of-scope sidecar/`agent-scaffold.md` pair quotes the
   retired wording as an expected string.

10. **Live-plan regression check** (the CHANGELOG's "No waiver committed to this project's own
    plan is affected"). Built the release binary from this branch and ran:
    ```
    ./target/release/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
    docs/metrics/workflow.jsonl: 318 records, valid
    docs/plans/agent-scaffold.plan.toml: 96 steps, 70 questions, valid
    docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
    ```
    Exit 0. Confirms the claim directly rather than by reading the sidecar's acceptance item 2.

11. **Build health.** `cargo build --release`, `cargo test` (full suite, all green, including
    `agents_md_drift`'s two tests and every `workflow::` unit test), and
    `cargo clippy --all-targets -- -D warnings` all clean on the unmodified worktree.

12. **ASCII check** on the diff:
    `git diff main..HEAD -- <the six touched files> | LC_ALL=C grep -nP '[^\t\x20-\x7e]'`
    reports no matches.

## Process note

I initially ran the mutation-test edit (item 5 above) directly in this reviewer worktree by
mistake, which violates the "do not edit anything except the findings file" rule. Caught it
immediately via `git diff --stat`, ran `git checkout -- src/workflow.rs` to revert before
committing or running anything else against it, confirmed `git status` clean, then redid the
mutation in a scratch copy under `/tmp/claude-1000/.../scratchpad/` as I should have from the
start. The worktree was never left in a modified state at any point I stopped to look at it,
and nothing was committed while it was dirty.
