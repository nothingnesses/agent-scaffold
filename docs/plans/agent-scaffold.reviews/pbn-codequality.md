# PBN code-quality review: phase -> principle-names map in `next`

Lens: code-quality / mechanical. Scope: `git diff main HEAD -- src/next.rs`
(commit a4817c9, "feat: project per-phase principles by name via a phase->names
map in next (Q-64)").

## Build / test / lint / validator lines

- `cargo test`: `test result: ok. 348 passed; 0 failed; 0 ignored` (lib), plus
  all integration bins green (1, 3, 1, 2 passed). `cargo test next::`:
  `28 passed; 0 failed; 320 filtered out`.
- `cargo clippy --all-targets`: `Finished dev profile`; 0 warning/error lines.
- `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml`:
  `docs/plans/agent-scaffold.plan.toml: 85 steps, 65 questions, valid`.
- `cargo run -- validate --workflow --source docs/plans/agent-scaffold.plan.toml`:
  `workflow invariants hold` (the bare `--workflow <path>` form the charter cites
  is rejected by clap; `--workflow` is a flag paired with `--source`).
- `cargo run -- render --check docs/plans/agent-scaffold.plan.toml`:
  `up to date` (unaffected by this diff).

## Findings

### CQ-1 (INFO) hardcoded "recommendation" noun in a now-generic projection

- Location: `src/next.rs:369` (the `format!` in `projected_principle_reminders`).
- Problem: the projected line reads `Ground the recommendation in the Project
  Principle "..."`. The noun "recommendation" is inherited verbatim from the old
  escalate-only projection. The function is now generic over `state` and its
  doc-comment describes a general "actor" grounding "its decision", but the
  emitted string still asserts a "recommendation" specifically. It fits both
  states mapped today (ReadyToPlan's planner gives a recommendation; Escalate's
  human decision is a recommendation), so there is no defect now. It is a latent
  wording mismatch: if a future phase that is not recommendation-shaped is added
  to `phase_principle_names`, the projected text will misdescribe it.
- Fix: none required now. If the map grows to a non-recommendation phase, replace
  "the recommendation" with phase-neutral wording (e.g. "Ground this decision in
  ...") or template the noun per state. Recorded as an observation, not a blocker.

## Checks that passed (evidence)

- Exhaustive match, no catch-all: `phase_principle_names` (`src/next.rs:76-92`)
  matches ReadyToPlan and Escalate explicitly, then the remaining seven variants
  (Blocked, AwaitingFirstReview, AwaitingFixes, AwaitingReviewers, Converged,
  RiskClassConflict, Done) in one `&[]` arm. That is all 9 `LoopState` variants
  (`src/next.rs:187-195`) with no `_ =>` arm, so a newly added state fails to
  compile rather than silently getting no principle. This is the safe design.
- Idiomatic projection: `projected_principle_reminders` (`src/next.rs:361-375`)
  is a clean `iter().filter_map(find).map(format!).collect()`; the `Vec<String>`
  allocation is fine on this non-hot path. Call site is an unconditional
  `principle_reminders.extend(projected_principle_reminders(state, ...))`
  (`src/next.rs:837`), correctly replacing the old `if state == Escalate` guard;
  an unmapped state extends with an empty Vec (no-op).
- No stale references: `grep -rn "ESCALATE_PRINCIPLE_NAME\|projected_principle_reminder\b" src/`
  returns nothing. The old const and the singular fn name are fully removed, not
  orphaned. No dead code left.
- Doc-comments accurate: the map fn doc (`src/next.rs:66-75`), the
  `principle_reminders` field doc (`src/next.rs:175-178`), the `base_reminders`
  doc (`src/next.rs:306-311`), the projection fn doc (`src/next.rs:352-360`), and
  the `build_instruction` assembly doc (`src/next.rs:815-821`) all describe the
  phase-map behavior and name `phase_principle_names` /
  `projected_principle_reminders`. No stale reference to the removed const or the
  old singular fn name remains.
- Tests genuinely assert:
  - `the_ready_to_plan_reminder_projects_the_grounding_principle_by_name`
    (`src/next.rs:1563-1583`) asserts `state == ReadyToPlan`, `.expect(...)` on
    finding the reminder, and asserts both the real text and `plan principle 6`.
  - `an_unmapped_state_projects_no_principle` (`src/next.rs:1585-1603`) asserts
    `state == AwaitingFixes` and a true negative (`!...any(contains)`) even though
    the plan carries the grounding principle. Genuine negative control.
  - The escalate test (`src/next.rs:1533-1560`) and the degrade test
    (`src/next.rs:1606-1624`) are repointed to the test-local const, unchanged in
    strength. No assertion weakened to force a pass.
  - Test-local const `GROUNDING_PRINCIPLE_NAME` (`src/next.rs:1529`) equals the
    production map value "Ground decisions in evidence"; repointing is sound.
- House rules on the diff: added lines are tab-indented (a `^\+ ` space-indent
  grep on the diff returns nothing); no non-ASCII on added lines (a `[^\x00-\x7F]`
  grep on `^+` lines returns nothing) so no em/en dashes, double-hyphen dashes,
  emoji, or unicode arrows/math; the diff adds no `#[allow]`/`#[expect]`
  attributes, so that rule is N/A.

## Verdict

1 INFO observation (CQ-1), no blockers. The diff is idiomatic, exhaustive,
fully cleaned up of the removed const/fn, and covered by genuine tests.
