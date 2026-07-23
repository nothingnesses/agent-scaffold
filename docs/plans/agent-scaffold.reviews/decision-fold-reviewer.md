# Review: Q-60 / Q-62 decision fold

Reviewer lens: fidelity to the two decisions, receipt / plan integrity, scope, and prose. Reviewed adversarially against branch `plan/decision-fold-q60-q62`, commit `fbe4f65654a2a43c8320fc695a2098e73f4e16eb`, base main `291f7a16df27d7c19da848b647efb93d250cc183`.

## Verdict

No valid findings. The fold records both decisions faithfully, the receipts are valid and member-of-options, the append is append-only, question-status and step integrity hold, scope is limited to the allowed files, prose is ASCII-only with no dash substitutes or AI filler, and the validators are green.

## What I checked

### Fidelity to the decisions

- Q-60 -> `single-source-recommendation-rule`. The appended `ask` sentence and the sidecar both record the single-source-rendered decision accurately: restate the recommendation-in-options rule (recommendation plus trade-offs plus reasoning judged against the plan's principles by name), NOT as a bare pointer and NOT as a hand-copied duplicate, with the rule living in ONE canonical source rendered into both the human-input-contract paragraph and the Preflight, following the `ISOLATION_POLICY_FRAGMENT` / `{{isolation_policy}}` byte-guarded pattern. The human's general directive (always restate, render duplicates from a single source to prevent drift) is captured. Both the `ask` and the sidecar mark it DEFERRED (a build owed later: a new rendered fragment, its substitution slot, its byte-guard test, plus the Preflight edit); it is not overstated as already built.
- Q-62 -> `driver-isolation-reminder-scope`. The appended `ask` sentence and the sidecar record option (a), WIDEN the `next` driver's isolation reminder to fire at `AwaitingFirstReview` and `AwaitingReviewers`, widen `spawns_writer` (or add a sibling predicate) in `src/next.rs`, and flip the pin test `the_reviewer_states_carry_no_isolation_reminder`. It is recorded as chosen over (b) keep-writer-only and (c) split-the-reminder, and both places state it is a deferred `src/next.rs` code change plus pin-test flip, not done in this fold pass.

### Receipt integrity

Both appended `type:"decision"` records validate:

- Q-60: `q_id` = "Q-60"; `task` = "single-source-recommendation-rule" (equals the `folded_into` step slug); `options` is a non-empty 4-member array; `recommendation` = "Yes, add a pointer"; `chosen` = "Other: always restate, rendering the duplicate from a single canonical source to prevent drift", which IS one of the four `options` members (the "Other:" option is present verbatim). Member-of-options: confirmed.
- Q-62: `q_id` = "Q-62"; `task` = "driver-isolation-reminder-scope" (equals the `folded_into` step slug); `options` is a non-empty 3-member array; `recommendation` = "(a) Widen to all spawn states"; `chosen` = "(a) Widen to all spawn states", which IS an `options` member. Member-of-options: confirmed.

### Append-only

The `docs/metrics/workflow.jsonl` diff adds exactly two lines at the end (after record 170) and modifies no existing line. No `-` lines in that file's diff. Record count goes 170 -> 172, consistent with two appends.

### Question-status and step integrity

- Q-60: `status = "decided"`, `folded_into = "single-source-recommendation-rule"`, `receipt = "Q-60"`. The step `single-source-recommendation-rule` exists with that exact slug, `status = "deferred"`, order 76, and `[step.provenance]` `decisions = ["Q-60"]`.
- Q-62: `status = "decided"`, `folded_into = "driver-isolation-reminder-scope"`, `receipt = "Q-62"`. The step `driver-isolation-reminder-scope` exists with that exact slug, `status = "deferred"`, order 77, and `[step.provenance]` `decisions = ["Q-62"]`.

Each `folded_into` points at a real new step slug and each `receipt` matches its own `q_id`.

### Scope

Only the allowed files changed: `docs/plans/agent-scaffold.plan.toml`, the two new sidecars under `docs/plans/agent-scaffold.steps/`, the generated `docs/plans/agent-scaffold.md`, and `docs/metrics/workflow.jsonl`. No pack, src, or ledger file changed.

### Prose

The added text is pure ASCII (checked the added diff lines for any byte outside 0x00-0x7F: none). No em-dashes, en-dashes, or `--` dash substitutes in the added lines. No AI filler patterns of concern. The generated `.md` status line updates consistently (75 -> 77 steps, 14 -> 16 deferred, 7 -> 5 open questions); byte-for-byte `.md` fidelity is covered by `render --check` below rather than hand-checked.

### Validator output

Run from this worktree against the folded docs (then restored):

- `validate --source docs/plans/agent-scaffold.plan.toml --workflow`:
  - `docs/metrics/workflow.jsonl: 172 records, valid`
  - `docs/plans/agent-scaffold.plan.toml: 77 steps, 62 questions, valid`
  - `docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold`
- `render --check docs/plans/agent-scaffold.plan.toml`: `up to date`

Expected 77 steps, 62 questions, 172 records, invariants hold, render up to date: all confirmed. The worktree was restored to HEAD after the checks, so the only committed change is this findings file.
