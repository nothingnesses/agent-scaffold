# Work review, round 3, `workflow-enforcement-tier-inc1`, REVIEWER (residue and fix verification)

Reviewer: independent of the implementer, the planner, and the other round-3 reviewer. Worktree `.claude/worktrees/wr3-inc1-a`, branch `wr3/inc1-a`, at `fe54995`, the exact commit under review.

## Summary verdict

The two round-2 fix lanes (`b18a0a8` implementer, folded into `fe54995` planner) landed all fourteen implementer sites and all three planner sites exactly as the round-2 triage prescribed, character for character where the triage supplied exact text, and word-for-word (rewrap only) where a splice re-wrapped a paragraph. No behaviour changed: every changed line in `src/main.rs` is a `///` or `//` comment line (verified by a systematic grep, not by eye), `resolve_metrics_path`, `project_root_of_source` and `default_ledger_path`'s bodies are byte-identical before and after, and the full suite (395 tests) passes with a freshly built, contamination-checked binary. `docs/plans/agent-scaffold.md` was regenerated (`render --check` reports "up to date") and mirrors the sidecar exactly. Neither lane re-seeded: a case-insensitive sweep for every phrase implicated in `W2B-1` through `W2B-4` found no missed twins beyond the ones the round-2 triage already ruled on.

Both disclosed known defects are CONFIRMED VALID:

- **(a)** `src/main.rs`'s `run_validate` doc block (`:806-807`, `:813-814`) falsely claims `--plan` is still clap-required for `--workflow`. Confirmed false by the struct definition, a passing regression test, and an independent manual repro. It predates `workflow-enforcement-tier` by roughly two weeks (introduced 2026-07-19 by `8017a2c`, made stale the same day by `f230f80`'s relaxation) and no inc1 commit ever touches these lines. RULING: valid defect, but NOT a defect against inc1 and does not block inc1's round-3 convergence; recommend a standalone backlog fix (same pattern as `test-tmpdir-repo-assumption`), not a fourth inc1 fix-pass item.
- **(b)** Sidecar `:186`'s "Every field of `ActiveLoop` is derived from the rounds, including ... `context` ..." is confirmed false by `src/next.rs`'s own doc comments (`isolation_tier` echoed from the CLI, `round_cap` from the workflow spec) and by `build_context` (`review_findings`/`triage_findings` built from the task name alone), and is now additionally SELF-CONTRADICTED by the planner's own new bullet at sidecar `:382`, added in this very fix pass, which says the opposite about those same `context` slots. RULING: valid defect, same exhaustiveness class as round 2's `W2B-3`, owned by the PLANNER, and it DOES count against round 3 because it sits in the step's own actively-maintained sidecar and now contradicts text the planner just wrote.

**Round 3 is NOT CLEAN.** One in-scope finding (`W3A-2`, medium, planner-owned) must be fixed before the streak can advance; the streak stays at 0 of 2. `W3A-1` (defect (a)) is recorded but ruled out of inc1's scope and does not itself block convergence.

## Findings table

| id | defect | severity | verdict | owner | blocks inc1 round-3 clean? |
| --- | --- | --- | --- | --- | --- |
| `W3A-1` | known defect (a): `run_validate` doc's false clap-required claims | low | VALID, but NOT a defect against inc1 (pre-existing, untouched by any inc1 commit) | backlog (not inc1) | NO |
| `W3A-2` | known defect (b): sidecar `:186`'s false "every field ... context" exhaustiveness claim, now self-contradicted by `:382` | medium | VALID, fix required | PLANNER | YES |

No residue findings. All seventeen prescribed sites (fourteen implementer, three planner) landed as prescribed; no re-seeding, no orphaned referents, no missed twins.

## `W3A-1` (low): known defect (a), `run_validate`'s false clap-required claims

### The claims, quoted and located

`src/main.rs:806-807` (inside `run_validate`'s doc block, `:791-819`):

```
/// unchanged from the previous increment. With `--workflow` (which still requires
/// `--plan`), the plan status is cross-referenced against the round log: ...
```

`src/main.rs:813-814`, same doc block:

```
/// skipped, the same treatment a missing file gets elsewhere here. `--plan` stays
/// clap-required for now (the relaxation for a TOML-only project is deferred).
```

### Verified false, three independent ways

1. **The struct definition.** `ValidateArgs.plan` (`src/main.rs:432-434`) carries a bare `#[arg(long)]` with no `requires`. The only `requires` in `ValidateArgs` is on `workflow_spec` (`requires = "workflow"`, `:443`). Clap enforces no relationship between `--workflow` and `--plan` at all.

2. **A passing regression test.** `tests/validate_workflow_toml_source_needs_no_plan.rs` exists precisely to pin this: its own doc comment (`:1-5`) states "Regression test for the Inc 6 clap relaxation: `--workflow` no longer `requires` `--plan`". Ran it directly:

```
$ cargo test --quiet --test validate_workflow_toml_source_needs_no_plan
running 2 tests
..
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

3. **A fresh manual repro**, outside any git repository and outside the suite's fixture harness:

```
$ cd /tmp/wr3a-scratch/defect-a-repro && git rev-parse --is-inside-work-tree
fatal: not a git repository (or any parent up to mount point /)
$ .../target/debug/agent-scaffold validate --metrics workflow.jsonl --workflow --source plan.plan.toml
workflow.jsonl: 0 records, valid
plan.plan.toml: 1 steps, 0 questions, valid
plan.plan.toml vs workflow.jsonl: workflow invariants hold
exit=0
```

No `--plan` given, no clap usage error (would be exit 2), and the check runs to completion. Both quoted clauses are false of this tree.

### Predates the increment; no inc1 commit touches it

```
$ git log --oneline --all -S "clap-required for now" -- src/main.rs
8017a2c fix: substrate-correct W5 locator and accurate TOML-swap docs
$ git log --oneline --all -S "still requires" -- src/main.rs
8017a2c fix: substrate-correct W5 locator and accurate TOML-swap docs
$ git log --oneline --all -S 'requires = "plan"' -- src/main.rs
f230f80 feat: relax --workflow --plan requirement for TOML-only projects; ...
88356ad feat(validate): add --workflow enforcement (W3) ...
$ git log -1 --format="%ci %s" 8017a2c
2026-07-19 09:14:12 +0100 fix: substrate-correct W5 locator and accurate TOML-swap docs
$ git log -1 --format="%ci %s" f230f80
2026-07-19 14:00:53 +0100 feat: relax --workflow --plan requirement for TOML-only projects; ...
$ git merge-base --is-ancestor 8017a2c f230f80 && echo "doc written BEFORE the relaxation"
doc written BEFORE the relaxation
$ git log -1 --format="%ci %s" 69c0525
2026-08-01 15:16:58 +0100 docs: start the workflow-enforcement-tier step
```

So the doc text was TRUE when written (2026-07-19 09:14): `--plan` really was clap-required then. The SAME DAY, `f230f80` (14:00) relaxed it and did not update this doc block, a residue miss from that earlier, unrelated increment. `workflow-enforcement-tier` did not start until 2026-08-01, roughly two weeks later. Confirmed no commit in inc1's own range touches these lines:

```
$ git diff 69c0525 fe54995 -- src/main.rs | grep -c "clap-required\|still requires"
0
```

### No twin sites

```
$ grep -rniF "clap-required" src/main.rs tests/... CHANGELOG.md docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md README.md docs/plans/agent-scaffold.md
src/main.rs:814: ... clap-required for now ...
$ grep -rniF "still requires" <same files>
src/main.rs:807: ... which still requires ...
docs/plans/agent-scaffold.md:1976: `--ledger-fragment` still requires `--resume` ... (unrelated flag pair, correct)
```

Exactly one doc block, two clauses, no twin elsewhere. Immediately adjacent, correct text already exists in the SAME file describing the SAME behaviour accurately: `src/main.rs:438`, "A TOML-primary --source needs no --plan (a TOML-only project has no Markdown plan); the Markdown path still needs --plan present." That sentence is true and needs no edit.

### Severity and ownership

LOW. Unlike round 1's `W1A-1` (medium: a false claim of correctness in the doc of the function that produces the exact false green the increment is `risky` about), this misdescribes CLI plumbing with no safety or correctness implication: the actual behaviour is right, is pinned by a regression test, and no other document or code path depends on the false claim. It is closer to round 1's `W1A-3` calibration ("no user-visible defect exists, the behaviour is right, and the doc inaccuracy misleads only about test coverage" -> low), here misleading only about clap mechanics.

NOT INC1'S TO FIX. The false text predates the step's start by about two weeks, was authored by an unrelated, already-shipped increment (`structured-skeleton` Inc 6 lineage), and no inc1 commit touches, was adjacent to, or was derived from it. Applying the same distinction round 1 and round 2's triages already draw between "is this a defect" and "is this a defect against inc1", this does not block inc1's round-3 clean status.

### Minimal fix (for the record, not prescribed for this round)

Delete the false parenthetical at `:806-807`:

```
With `--workflow` (which still requires `--plan`), the plan status is cross-referenced
```
->
```
With `--workflow`, the plan status is cross-referenced
```

and delete the false sentence at `:813-814`:

```
skipped, the same treatment a missing file gets elsewhere here. `--plan` stays
clap-required for now (the relaxation for a TOML-only project is deferred).
```
->
```
skipped, the same treatment a missing file gets elsewhere here.
```

Recommend filing as its own backlog step (parallel to `test-tmpdir-repo-assumption`), not as a fourth item in inc1's residue-fix pass.

## `W3A-2` (medium): known defect (b), sidecar `:186`'s false "every field ... context" claim

### The claim, quoted and located

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:186` (mirrored identically at `docs/plans/agent-scaffold.md:1581`):

```
Every field of `ActiveLoop` is derived from the rounds, including `role`, `prompt`,
`context`, `reminders` and `filled_prompt_summary`, so the block goes as a unit;
suppressing only the `next:` line would leave an instruction assembled from foreign
evidence in the surrounding fields.
```

### Verified false, by the struct's own doc comments

`src/next.rs:151-153`:

```
/// The advisory total-round cap (from the workflow spec); forward guidance, not
/// enforced (consistent with advisory mode).
pub(crate) round_cap: u64,
```

`src/next.rs:156-158`:

```
/// The isolation tier echoed from the CLI (`worktree`/`container`/`file-safety`),
/// or `unknown`.
pub(crate) isolation_tier: String,
```

`src/next.rs:873-900`, `build_context`:

```rust
let review_findings = findings_naming::review_findings_path(context.task, &facts.step);
let triage_findings = findings_naming::triage_findings_path(context.task, &facts.step);
```

`findings_naming::review_findings_path`/`triage_findings_path` build from the task name and step slug, not from any round record.

### A live, independent repro reinforces it

Built a TOML-primary project with an in-progress step and a ZERO-RECORD metrics log, ran `next --json`:

```
$ .../target/debug/agent-scaffold next --source p.plan.toml --metrics workflow.jsonl \
      --isolation-tier container --json
{
  ...
  "active_loop": {
    ...
    "total_rounds": 0,
    "round_cap": 5,
    "isolation_tier": "container",
    "next_instruction": {
      "role": "reviewer",
      "prompt_path": ".agents/prompts/reviewer.md",
      "context": {
        "isolation_tier": "container",
        "ledger": "p.ledger.md",
        "review_findings": "docs/plans/p.reviews/only-step-reviewer-<disambiguator>.md",
        "triage_findings": "docs/plans/p.reviews/only-step-triage.md"
      },
      ...
    }
  }
}
```

Zero round records, yet `round_cap: 5` and `isolation_tier: "container"` (exactly the value passed on the CLI, changes only if the flag changes) both appear, and `context.review_findings`/`context.triage_findings` are populated from the task name (`p`) and step slug (`only-step`) alone. None of these four values has any relationship to round content.

### Self-contradicted by the planner's own new text in this fix pass

Sidecar `:382` (added at `fe54995`, this very fix pass, as the `W2B-3` recorded-consequence bullet):

```
It does not anchor the report paths `next` emits. `review_findings` and
`triage_findings` are built from the task name alone (`src/findings_naming.rs:52-55`,
via `src/next.rs:881-882`) and stay relative to the process working directory, ...
```

`:186` says `context` is "derived from the rounds"; `:382`, written in the same commit by the same planner, says the `review_findings`/`triage_findings` slots inside that same `context` map are "built from the task name alone". These two sentences, seven lines and two hundred lines apart in the same file, now directly contradict each other on the same fields.

### Same exhaustiveness class as `W2B-3`

Round 2's `W2B-3` (`src/main.rs:1282-1286`, "every field of the projected loop ... is derived from those two files") was ruled VALID and MEDIUM on the same reasoning that applies here: "The claim is an exhaustiveness claim ('every field'), which this project has calibration data on as unusually easy to falsify, and its falsity is precisely what would stop a reader noticing a cross-project path in an instruction an agent acts on." `:186` is the sidecar's own unfixed twin of the same over-claim, one level up (the CODE's version was deleted by `W2B-3`'s fix; the SIDECAR's near-identical claim, in a different paragraph making a different argument, was not caught because it neither matched `W2B-3`'s cited text nor was grepped for by that finding's site-count vocabulary, which searched for `every field`/`those two files`/`from the durable files`/`derived from the round log`, not the `:186` paragraph's own wording).

### Whose fix, and does the load-bearing constraint survive narrowing?

PLANNER's. The claim lives entirely in the sidecar the planner lane is already actively maintaining (this exact fix pass edited `:164` and `:166`, seven lines away).

The planner's argument that `:186` is LOAD-BEARING as a design constraint for inc2 ("the block goes as a unit; suppressing only the `next:` line would leave an instruction assembled from foreign evidence") does not depend on EVERY field being round-derived. It depends on ENOUGH of them being round-derived that a reader would still see round-tainted output if only `next:` were blanked. That remains true and does not need "every field of `context`" to be true: `state`, `risk_class`, `consecutive_clean`, `required_streak`, `total_rounds`, `valid_transitions`, and (via `state`) `role`, `prompt_path` and `filled_prompt_summary` ARE genuinely round-derived (confirmed at `src/next.rs`'s doc comments for each), and those are exactly the fields an agent would act on if only `next:` were nulled. The minimal fix narrows the ENUMERATION rather than the CONCLUSION:

```
Every field of `ActiveLoop` is derived from the rounds, including `role`, `prompt`,
`context`, `reminders` and `filled_prompt_summary`, so the block goes as a unit;
```
->
```
Most fields of `ActiveLoop`, including `state`, `role`, `prompt` and
`filled_prompt_summary`, are derived from the rounds (directly, or via `state`); a
few, including `isolation_tier`, `round_cap` and the task-derived `review_findings`/
`triage_findings` slots in `context`, are not. Suppressing only the `next:` line would
still leave the round-derived fields assembled from foreign evidence, so the block
goes as a unit;
```

This is offered as the shape of a fix, not a prescription to copy verbatim; the planner should draft and own the exact wording (this project's own convention, per the round-2 triage, is that only PRESCRIBED text is copied verbatim, and this reviewer does not prescribe).

### Severity

MEDIUM, matching `W2B-3`'s precedent exactly: an exhaustiveness claim, easy to falsify, whose falsity is precisely what would let a reader miss a non-round-derived field leaking through an "everything is safe because it's all round-derived" argument. It does not reach high: nothing here is a regression, the behaviour it discusses is inc2's (unbuilt), and the load-bearing conclusion survives a correct narrowing.

## Enumeration

### All seventeen prescribed sites, verdict each

Implementer (14), all `src/main.rs` unless noted:

1. `:429` (`ValidateArgs.metrics` help), `W2B-1` item 1, DELETION. LANDED AS PRESCRIBED.
2. `:438` (`ValidateArgs.workflow` help), `W2B-1` item 2, REPLACEMENT. LANDED AS PRESCRIBED.
3. `:455` (`StatusArgs.metrics` help), `W2B-1` item 3, DELETION. LANDED AS PRESCRIBED.
4. `:479` (`NextArgs.metrics` help), `W2B-1` item 4, DELETION. LANDED AS PRESCRIBED.
5. `:794` (`run_validate` doc), `W2B-1` item 5, REPLACEMENT. LANDED AS PRESCRIBED.
6. `:824` (`run_validate` in-body comment), `W2B-1` item 6, REPLACEMENT. LANDED AS PRESCRIBED.
7. `:1069` (`run_status` doc, was `:1073-1077`), `W2B-1` item 7, REPLACEMENT. LANDED AS PRESCRIBED; word-level diff confirms zero unintended word changes (only the two prescribed clauses removed).
8. `:1147` (`METRICS_RELATIVE` doc, was `:1152-1154`), `W2B-1` item 8, REPLACEMENT. LANDED AS PRESCRIBED.
9. `:1159` (`project_root_of_source` LEXICAL doc, was `:1165-1168`), `W2B-2` item 1, DELETION. LANDED AS PRESCRIBED; word-level diff confirms the continuation past the splice point ("It also means a `..` component ...") is word-identical, rewrap only.
10. `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:370-372` (acceptance check 9 doc), `W2B-2` item 2, NARROWING. LANDED AS PRESCRIBED; word-level diff confirms the continuation ("The whole stdout is compared ...") is word-identical, rewrap only.
11. `tests/...:393` (assertion message), `W2B-2` item 3, NARROWING. LANDED AS PRESCRIBED.
12. `CHANGELOG.md:22`, `W2B-2` item 4, NARROWING. LANDED AS PRESCRIBED.
13. `:1275` (`run_next` doc, was `:1282-1286`), `W2B-3` item 1, DELETION. LANDED AS PRESCRIBED.
14. `tests/...:14-17` (module doc), `W2B-4`, NARROWING. LANDED AS PRESCRIBED.

Planner (3), `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`:

15. `:164` (refusal paragraph), `W2B-1` item 9, APPEND. LANDED AS PRESCRIBED, verified character for character against the triage's supplied text.
16. `:166` (lexical/canonical split paragraph), `W2B-2` item 5, DELETION. LANDED AS PRESCRIBED, verified character for character.
17. `:382` (new scope bullet), `W2B-3` item 2, APPEND. LANDED AS PRESCRIBED, verified character for character.

Plus the shared regeneration: `docs/plans/agent-scaffold.md` REGENERATED not hand-edited, verified by `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` reporting "up to date" (exit 0), and by diffing the three mirrored spans against the sidecar (identical).

### Claims re-verified

- ZERO BEHAVIOUR CHANGE: `git diff f8f2e09 fe54995 -- src/main.rs | grep -E '^[+-]'` filtered to non-`///`/`//` lines: zero hits. `resolve_metrics_path`, `project_root_of_source`, `default_ledger_path` bodies: byte-identical before/after (`diff` of the extracted line ranges, no output). `tests/...` diff outside doc comments: exactly one line, the assertion FAILURE-MESSAGE string (not test logic).
- ZERO-AUTHORED-WORDS on the two re-wrapped paragraphs: verified with a word-level `difflib` diff (not by eye) for both `src/main.rs`'s LEXICAL doc and the test's acceptance-check-9 doc; in both cases the only word-level difference is exactly the triage's prescribed deletion/insertion, and the continuation past the splice point is word-identical.
- REGENERATION: `render --check` -> "up to date", exit 0. The three edited spans in `docs/plans/agent-scaffold.md` (`:1559`, `:1561`, `:1777`) are byte-identical to the sidecar's `:164`, `:166`, `:382`.
- SUITE: 373 + 5 + 1 + 1 + 9 + 3 + 1 + 2 = 395 passed, 0 failed, `TMPDIR=/tmp/wr3a-scratch` (outside the repo; the three ambient-git-state tests pass under this condition).
- CONTAMINATION TRAP: test binary force-rebuilt after deleting the old one; `strings target/debug/deps/metrics_and_ledger_anchor_to_the_plan_source-bf4905c55850edca | grep -F target/debug/agent-scaffold` reports `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/wr3-inc1-a/target/debug/agent-scaffold`, this worktree's own path.
- CLIPPY: `cargo clippy --all-targets --quiet -- -D warnings`, zero output, zero warnings.
- FILE SCOPE: `git diff --stat f8f2e09 fe54995` touches exactly `CHANGELOG.md`, `docs/plans/agent-scaffold.md`, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, `src/main.rs`, `tests/metrics_and_ledger_anchor_to_the_plan_source.rs`. No stray files.

### Case-insensitive twin sweep (negatives)

Per phrase, restricted to `src/`, `tests/`, `CHANGELOG.md`, `README.md`, `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, `docs/plans/agent-scaffold.md`:

- `every field` (case-insensitive): 4 hits total. 2 are the sidecar `:186` claim and its regenerated mirror (`W3A-2`, above). 2 are unrelated (`src/metrics.rs:1664,1699`, about validator schema field names, not derivation claims). NO MISSED TWIN of `W2B-3`'s deleted code claim.
- `plan's own` / `plans own`: 17 hits (case-insensitive). All either already-fixed sites, test-assertion messages naming which fixture a test expects (not exhaustiveness claims), the correctly-qualified `:1164` LEXICAL continuation ("... only when the `..` does not climb out through one", already conditional), the normative END PROPERTY at `:111` ("must never pair", a goal statement, not a claim of current behaviour), or inc2/inc3 forward-looking vocabulary (`:224`, `:282`, `:326`, out of scope per the tense rule). NO NEW TWIN.
- `the plan's project`: 4 hits. `:164` (already covered), `:218`/`:230` (inc2 JSON-reason vocabulary, forward-looking), `src/main.rs:281` (unrelated: plan-review recommendation text). NO TWIN.
- `belongs to that plan`, `not whichever log the current directory`: 0 hits (both fully deleted, no twin anywhere).
- `resolved FROM THE PLAN`: 4 hits. `tests/...:2` (true, header comment), `README.md:226` (round-2 triage's own "REMOVED SITE", verified TRUE, no edit prescribed and none present), `CHANGELOG.md:22` (already the fixed text), `src/main.rs:1275` (the `W2B-3`-fixed text, true). NO TWIN.
- `byte-identical`: 9 hits. 2 already-fixed sites (`tests/...:393`, sidecar-mirrored acceptance-check-9 text). 5 entirely unrelated (`--module` scaffolding byte-identity in `CHANGELOG.md:18`, `README.md:327`, `src/main.rs:303,2213,2259,2375`, a different feature). 1 is acceptance check 9 itself (`:316`), which round 2's triage explicitly ruled narrow-and-true, do-not-edit; unchanged, correctly so. 1 is `:244`, an unrelated exploration-methodology sentence about a different candidate policy (absent-field defaults, not lexical byte-identity). NO TWIN.
- `byte for byte`: 1 hit, the already-fixed test doc. NO TWIN.
- `still prints the relative paths`, `unchanged and still prints`: 1 hit each, both the same already-fixed `CHANGELOG.md:22` line. NO TWIN.
- `clap-required`, `still requires` (excluding the unrelated `--ledger-fragment still requires --resume`): exactly the two clauses of `W3A-1`, one doc block. NO TWIN elsewhere.

### Negatives (things checked and found NOT to be defects)

- `README.md:226`'s "resolved FROM THE PLAN" sentence: verified still true, still unedited, consistent with round 2's own ruling. Not re-litigated beyond confirming no edit landed there (none was prescribed).
- Round 1's `W1A-2` fix (sidecar `:164`'s "Measured to close both of A's self-found false passes ..." sentence): confirmed intact, unmodified by the `fe54995` append (which only adds text after it, verified by reading the full paragraph before and after).
- Acceptance check 9 (sidecar `:316`, `tests/...:380`): confirmed untouched, as the round-2 triage required.
- The `..` escape behaviour and the divergent-anchor false green: not re-litigated, per scope; only their descriptions were checked, and both remain accurately described post-fix.
- `docs/plans/workflow-enforcement-tier.explorations/`: not opened, not edited.

## Commands run, with output, for the record

```
$ git log --oneline -10                             # confirmed HEAD fe54995, branch wr3/inc1-a
$ git diff f8f2e09 fe54995 -- src/main.rs            # 14 implementer main.rs sites
$ git diff f8f2e09 fe54995 -- tests/... CHANGELOG.md # remaining implementer sites
$ git diff f8f2e09 fe54995 -- docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md
$ git diff f8f2e09 fe54995 -- docs/plans/agent-scaffold.md
$ python3 wdiff2.py                                  # word-level diffs, two rewrapped paragraphs
$ diff <(sed -n '1173,1253p' main_before.rs) <(sed -n '1173,1253p' main_after.rs)  # empty: fn bodies identical
$ cargo build --bin agent-scaffold
$ cargo run --quiet -- render docs/plans/agent-scaffold.plan.toml --check   # "up to date", exit 0
$ rm -rf target/debug/deps/metrics_and_ledger*; TMPDIR=/tmp/wr3a-scratch cargo test --quiet
  # 373+5+1+1+9+3+1+2 = 395 passed, 0 failed
$ strings target/debug/deps/metrics_and_ledger_anchor_to_the_plan_source-bf4905c55850edca | grep -F target/debug/agent-scaffold
  # .../wr3-inc1-a/target/debug/agent-scaffold -- this worktree's own path
$ cargo clippy --all-targets --quiet -- -D warnings  # no output, no warnings
$ cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
  # workflow invariants hold, exit 0 (sanity)
$ cd /tmp/wr3a-scratch/defect-a-repro && git rev-parse --is-inside-work-tree
  # fatal: not a git repository
$ .../agent-scaffold validate --metrics workflow.jsonl --workflow --source plan.plan.toml
  # workflow invariants hold, exit 0, no --plan given (defect (a) repro)
$ cd /tmp/wr3a-scratch/defect-b-repro && .../agent-scaffold next --source p.plan.toml \
      --metrics workflow.jsonl --isolation-tier container --json
  # round_cap:5, isolation_tier:"container", review_findings/triage_findings from task
  # name, all with total_rounds:0 (defect (b) repro)
```
