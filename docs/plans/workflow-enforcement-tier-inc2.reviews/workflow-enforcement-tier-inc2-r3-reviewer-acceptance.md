# `workflow-enforcement-tier-inc2`, work review ROUND 3, ISOLATED REVIEWER, ACCEPTANCE RE-RUN LENS

ARTIFACT. `git diff main..HEAD` at commit `a7e05c3` ("fix: root containment on an anchor that does not exist"), reviewed in the worktree `.claude/worktrees/r3-acceptance`. Binary: `target/release/agent-scaffold` built fresh from this commit (`cargo build --release`, clean). `cargo test --release` (with `TMPDIR` pointed outside any git repository, per the specification's own warning) is also clean: 416 passed, 0 failed, across 9 binaries. Specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`.

SCOPE. Inc2 owns acceptance checks 11, 13b, 14b, 14c, 14e, 14g, 19 and 19b. Every fixture below was built from scratch with shell/heredocs against the specification's own description, not by calling the suite's Rust helper functions (`tests/unsafe_pairings_are_refused_and_omitted.rs`), though that file's fixture SHAPES (the minimal TOML/Markdown plan schema, the round-record schema) were read for reference on what a schema-valid fixture looks like. Two human decisions landed since the last full run of this set: `Q-55-emptyroot` (partially resolve a nonexistent anchor via `resolve_for_containment`, sited in `resume_roots`) and `Q-55-emptyrootsite` (confirming the implementer's departure from the literal prescribed byte location). Both are exercised below.

## PART 1: ACCEPTANCE CHECK TABLE

All commands were run with the exit code captured directly (no `|| true`). "Root" below abbreviates the fixture's own project root as printed by the tool; full paths are given in the narrative under each check.

| Check | Sub-run | Command (fixture summarised) | Exit | Verdict |
| --- | --- | --- | --- | --- |
| 11 | single | `validate --source <fix>/docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl --workflow`, from the worktree root, fixture step `triager-runs-only-on-findings` at `complete` | 1 | PASS |
| 13b | run 1 | `validate --source A/docs/plans/p.plan.toml --plan B/docs/plans/p.md --workflow` (A markdown-primary + this repo's own log copied in; B markdown Roadmap carrying the borrowed slug at `complete`) | 1 | PASS |
| 13b | run 2 | same as run 1 but `--source` misspelled (`pXXX.plan.toml`, does not exist) | 1 | PASS |
| 13b | run 3 | `validate --source docs/plans/agent-scaffold.plan.toml --plan docs/plans/agent-scaffold.md --workflow`, this repo against itself (no-regression control) | 0 | PASS |
| 14b | single | `next --source <fix>/docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl`, fixture step at `in-progress` | 0 | PASS |
| 14c | run 1 | `status --source <fix>/docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl` | 0 | PASS |
| 14c | run 2 | `status --resume --source <fix>/...TEMPLATE.plan.toml --ledger-fragment <outside>/foreign.ledger.md` (fragment exists, outside root) | 0 | PASS |
| 14c | run 3 | `status --resume --source A/docs/plans/p.plan.toml --plan B/docs/plans/p.md` (default ledger, no `--ledger-fragment`; A carries a `## RESUME STATE` block) | 0 | PASS |
| 14e | part 1 | `next --source <fix>/...TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl --json` (re-run of 14b) | 0 | PASS |
| 14e | part 2 | `status --source <fix>/...TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl --json` (re-run of 14c run 1) | 0 | PASS |
| 14g | run 1 | `next --source <fix>/...TEMPLATE.plan.toml --json`, no ledger file at all | 0 | PASS |
| 14g | run 2 | same, ledger file present with no `## RESUME STATE` heading | 0 | PASS |
| 14g | run 3 | `--ledger-fragment <outside>/foreign.ledger.md` (exists, outside root) | 0 | PASS |
| 14g | run 4 | `--ledger-fragment <outside>/nonexistent.ledger.md` (outside root AND missing) | 0 | PASS |
| 14g | run 5 ("fourth run" in the spec's own text) | 13b's A/B2 pairing under `next` (B2's step at `in-progress`, no explicit `--metrics`/`--ledger-fragment`), human text, `--json`, and `status --json` on the same pairing | 0 | PASS |
| 19 | layout 1 | `<root>/docs/plans` a symlink to a sibling directory; `validate --workflow`, `status`, `next` | 1 / 0 / 0 | PASS |
| 19 | layout 2 | `<root>/docs/metrics` a symlink to a sibling directory; same three commands | 1 / 0 / 0 | PASS |
| 19b | single | plan at `<root>/notes/p.md`, `x.plan.toml` markdown-primary at `<root>/docs/plans`, log at `<root>/docs/metrics/workflow.jsonl`; `validate --workflow`, `status`, `next`, `status --resume` in both `primary` spellings | 1 / 0 / 0 / 0 | PASS |

**RESULT: 8/8 assigned checks pass, all 18 sub-runs pass. No acceptance-check regression found against the current binary.**

## PART 2: NARRATIVE PER CHECK, WITH EVIDENCE

### Check 11

Fixture: `agent-scaffold scaffold --output-dir <fix> --write --force --principles default` (confirmed 30 files, `ls <fix>/docs` prints only `plans`), then the single step's `slug` set to `triager-runs-only-on-findings` and `status` to `complete`. Run from the worktree root so the relative `--metrics docs/metrics/workflow.jsonl` resolves to this repo's own log, which genuinely carries a converged (`consecutive_clean: 1`, `low_risk`, cap 1) round for that slug, making this the sharpened false-pass reproduction the specification names.

```
$ agent-scaffold validate --source <fix>/docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl --workflow
exit: 1
stderr: --workflow would join <fix>/docs/plans/TEMPLATE.plan.toml against docs/metrics/workflow.jsonl,
        which is not under the plan's project root <fix>; pass a `--metrics` under that root, run
        against the plan's own log, or correct the `--source` and `--plan` pair
```

Non-zero exit, names both paths (the plan, the metrics path as given) and the derived root. Matches specification line 323 exactly.

### Check 13b

Fixture A: `A/docs/plans/p.plan.toml` declaring `primary = "markdown"`, plus `A/docs/metrics/workflow.jsonl` = a verbatim copy of this repository's own 270-record log (confirmed to carry the converged round for `triager-runs-only-on-findings`). Fixture B: `B/docs/plans/p.md`, a minimal schema-valid Markdown plan with a two-column Roadmap table and a matching `### \`triager-runs-only-on-findings\`: The only step` Step Detail heading, status `complete`.

Run 1, from `/tmp` (neither fixture's own directory):

```
$ agent-scaffold validate --source A/docs/plans/p.plan.toml --plan B/docs/plans/p.md --workflow
exit: 1
stderr: --workflow would join B/docs/plans/p.md against A/docs/metrics/workflow.jsonl, which is not
        under the plan's project root B; pass a `--metrics` under that root, run against the plan's
        own log, or correct the `--source` and `--plan` pair
```

Names B's plan (the one actually checked, since `A` is markdown-primary so `toml_primary` is false and the Markdown `--plan` is what gets read), A's log (the anchor-derived default, since `--source` still wins the anchor race even though it is not the checked plan), and B's root. This is precisely the case the specification says an anchor-rooted predicate cannot reach, and it is refused.

Run 2, `--source` misspelled to a nonexistent `pXXX.plan.toml`:

```
exit: 1
stderr: no source plan at A/docs/plans/pXXX.plan.toml; nothing to validate
        --workflow would join B/docs/plans/p.md against A/docs/metrics/workflow.jsonl, ... project root B; ...
```

Identical refusal. The metrics anchor is still lexically derived from the typo'd `--source` path (never re-read), matching the specification's account of this sub-case.

Run 3, no-regression control, this repository checking itself (TOML-primary `--source`, its own generated `--plan`):

```
$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --plan docs/plans/agent-scaffold.md --workflow
exit: 0
stdout: docs/metrics/workflow.jsonl: 270 records, valid
        docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
        docs/plans/agent-scaffold.md: generated projection of a TOML-primary source; skipping the Markdown plan validator
        docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

All three runs pass.

### Check 14b

Same shape of fixture as check 11 but `status = "in-progress"` (not `complete`), per the specification's explicit precondition, so the trap at (out-of-scope) check 14d cannot be mistaken for a pass here.

```
$ agent-scaffold next --source <fix>/docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl
exit: 0
stdout: task: TEMPLATE
        source: <fix>/docs/plans/TEMPLATE.plan.toml
        metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's
                 project root <fix>, so its records cannot be paired with this plan

        no active review loop (the round log docs/metrics/workflow.jsonl is not under the plan's
                 project root <fix>, so its records cannot be paired with this plan)
```

No `state:`, `streak:`, `rounds:`, `next:`, `role:`, `prompt:` or `summary:` line anywhere in the output; no record count; a reason naming the resolved log and the derived root stands in their place; exit 0. Matches specification line 328.

### Check 14c

Fresh fixture (fresh `scaffold`, default template, unmodified step). Run 1 (metrics omission):

```
$ agent-scaffold status --source <fix>/docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl
exit: 0
stdout: plan: 1 steps (1 not started); 0 open-questions items
        metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's
                 project root <fix>, so its records cannot be paired with this plan
```

Run 2 (explicit `--ledger-fragment` outside root, file exists with a foreign `## RESUME STATE` block):

```
$ agent-scaffold status --resume --source <fix>/...TEMPLATE.plan.toml --ledger-fragment <outside>/foreign.ledger.md
exit: 0
stdout: the ledger <outside>/foreign.ledger.md is not under the plan's project root <fix>; nothing to resume
```

No line of the foreign block leaked.

Run 3 (default ledger, no `--ledger-fragment`, check 13b's A/B pairing, A carrying `A/docs/plans/p.ledger.md` with a `## RESUME STATE` block reading "FIXTURE A'S OWN RESUME STATE."):

```
$ agent-scaffold status --resume --source A/docs/plans/p.plan.toml --plan B/docs/plans/p.md
exit: 0
stdout: the ledger A/docs/plans/p.ledger.md is not under the plan's project root B; nothing to resume
```

A's block content never appears. All three runs pass.

### Check 14e

Re-run of 14b with `--json`:

```
{
  "task": "TEMPLATE",
  "source": "<fix>/docs/plans/TEMPLATE.plan.toml",
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  "active_loop": null,
  "resume_state": null,
  "resume_state_absent_reason": "ledger-absent",
  "no_active_loop_reason": "metrics-not-this-project"
}
exit: 0
```

Re-run of 14c run 1 with `--json`:

```
{
  "plan": { "steps": [ { "slug": "example-step", "status": "not started" } ], "open_questions": [] },
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project"
}
exit: 0
```

Both match specification line 331 exactly (`"active_loop": null` with `"no_active_loop_reason": "metrics-not-this-project"`; `"metrics": null` with `"metrics_absent_reason": "log-not-this-project"` on both commands).

### Check 14g

Fresh fixture. Run 1, no ledger file: `resume_state_absent_reason: "ledger-absent"`. Run 2, a ledger file present with a heading that is not `## RESUME STATE`: `resume_state_absent_reason: "no-resume-section"`. Run 3, `--ledger-fragment` naming a file that EXISTS outside the root with its own `## RESUME STATE` block: `resume_state_absent_reason: "ledger-not-this-project"`, and the foreign block's content does not appear anywhere in the output (grepped for zero matches). Run 4, the same fragment path but pointed at a file that does not exist, still outside the root: `resume_state_absent_reason: "ledger-not-this-project"` again, not `"ledger-absent"`, confirming the precedence rule.

Run 5, the "fourth run" the specification adds on top of those four, which pins the default-ledger half of `Q-55-endproperty`: check 13b's A/B2 pairing under `next` with no explicit `--metrics` or `--ledger-fragment` at all (B2 is B with its step re-labelled `in progress` instead of `complete`, same slug, same Step Detail heading):

```
$ agent-scaffold next --source A/docs/plans/p.plan.toml --plan B2/docs/plans/p.md --json
{
  "task": "p", "source": "B2/docs/plans/p.md",
  "metrics": null, "metrics_absent_reason": "log-not-this-project",
  "active_loop": null, "resume_state": null,
  "resume_state_absent_reason": "ledger-not-this-project",
  "no_active_loop_reason": "metrics-not-this-project"
}
exit: 0

$ agent-scaffold next --source A/docs/plans/p.plan.toml --plan B2/docs/plans/p.md
(human text: no `state:`, `streak:`, `rounds:`, or `next:` line anywhere; the metrics-unavailable
 reason and the ledger-rejection note both appear, naming A's log and A's ledger and B2's root)
exit: 0

$ agent-scaffold status --source A/docs/plans/p.plan.toml --plan B2/docs/plans/p.md --json
{ "plan": {...}, "metrics": null, "metrics_absent_reason": "log-not-this-project" }
exit: 0
```

All five sub-runs pass; the resume vocabulary and the metrics vocabulary each separate their causes correctly across every configuration the specification names.

### Check 19

Layout 1: `<root>/docs/plans` is a symlink to a disjoint sibling directory (`plans-target`), containing a TOML-primary `p.plan.toml`; `<root>/docs/metrics/workflow.jsonl` is a real (non-symlinked) file at its conventional path.

```
$ agent-scaffold validate --source <root>/docs/plans/p.plan.toml --workflow
exit: 1
stderr: --workflow would join <root>/docs/plans/p.plan.toml against <root>/docs/metrics/workflow.jsonl,
        which is not under the plan's project root <plans-target>; ...

$ agent-scaffold status --source <root>/docs/plans/p.plan.toml
exit: 0; metrics: unavailable, ... project root <plans-target> ...

$ agent-scaffold next --source <root>/docs/plans/p.plan.toml
exit: 0; same omission, no ACTIVE LOOP block.
```

Layout 2: `<root>/docs/metrics` is a symlink to a disjoint sibling directory (`metrics-target`); `<root>/docs/plans/p.plan.toml` is real and at its conventional path.

```
$ agent-scaffold validate --source <root>/docs/plans/p.plan.toml --workflow
exit: 1; refusal naming the plan, the log, and root <root> (the canonicalised metrics leaf resolves
        outside the plan's own canonical root because the leaf itself is the symlink).

$ status / next on the same pair: exit 0, same omission pattern.
```

Both layouts refuse under `validate --workflow` and omit under `status`/`next`, exactly as accepted cost (ii) is pinned to do. Per the task's instruction, this is a PASS (the cost is pinned, not a defect), not a finding.

### Check 19b

Fixture: `<root>/docs/plans/x.plan.toml` (markdown-primary, one placeholder step), `<root>/notes/p.md` (a real Markdown Roadmap+Step Detail with its own step), `<root>/docs/metrics/workflow.jsonl` (a real log, copied from this repo).

```
$ agent-scaffold validate --source <root>/docs/plans/x.plan.toml --plan <root>/notes/p.md --workflow
exit: 1
stderr: --workflow would join <root>/notes/p.md against <root>/docs/metrics/workflow.jsonl, which is
        not under the plan's project root <root>/notes; ...

$ status / next on the same pair: exit 0, metrics half omitted with the same reason.
```

`status --resume` on the default ledger (`<root>/docs/plans/x.ledger.md`, seeded with its own `## RESUME STATE` block) with `x.plan.toml` declaring `primary = "markdown"`:

```
exit: 0
stdout: the ledger <root>/docs/plans/x.ledger.md is not under the plan's project root <root>/notes; nothing to resume
```

Re-run with `x.plan.toml`'s `primary` field switched to `"toml"` in place, otherwise identical: byte-identical output, same omission, same exit code. `run_resume`'s own root derivation (`resume_roots`, called unconditionally) never consults `toml_primary`, so both spellings agree, as the specification requires ("`status --resume` omits its block in EITHER `primary` spelling").

Both refusal and both omissions, in both `primary` spellings, pass.

## PART 3: THE HELD-BACK CLAUSE, CONSTRUCTED

The task brief asks whether the round-2/round-3 fix pass was right NOT to write FV-2's prescribed clause "note that `status --resume` is the surface that can hold two roots." Constructed directly: with BOTH `--source` and `--plan` supplied and NEITHER resolving to a checked plan (a `--source` that does not canonicalise and a `--plan` that also does not canonicalise, in two directories with different real parents), `containment_roots` falls through to `resume_roots`, which (post `Q-55-emptyroot`) now supplies a root for EVERY supplied anchor regardless of existence, so `next` and `status` (not just `status --resume`) can hold two DIFFERENT roots simultaneously:

```
$ agent-scaffold next --source <alpha>/docs/plans/nonexistent.plan.toml \
    --plan <beta>/notes/nonexistent.md --metrics <alpha>/docs/metrics/workflow.jsonl --json
{
  "task": "nonexistent", "source": "no plan source",
  "metrics": null, "metrics_absent_reason": "log-not-this-project",
  ...
}
exit: 0
```

The metrics file genuinely lives under `<alpha>`'s own derived root (the `--source` anchor's own directory) yet is still omitted, because it is NOT under `<beta>/notes`'s root too, and `containment_roots` requires the artifact under EVERY supplied anchor's root. This is `next`, not `status --resume`, holding two roots and rejecting on the intersection. The prescribed clause, had it been written, would now be FALSE (it singles out `status --resume` as uniquely able to hold two roots, which is no longer true post-fix), so the implementer's judgement to omit it is CORRECT. The general phrasing actually shipped ("every `--source` or `--plan` you gave yields one and the artifact must be under all of them", README.md:236 and CHANGELOG.md:23) already covers the two-or-more-roots case for all three surfaces without singling any one out, and is accurate.

A companion construction confirms the complementary, non-leaking half of the same fix: a SINGLE nonexistent anchor (no second anchor) whose own directory holds its own log reads that log NORMALLY (not omitted), because the sole derived root IS the anchor's own directory:

```
$ agent-scaffold next --source <alpha>/docs/plans/nonexistent.plan.toml --json
{ "task": "nonexistent", "source": "no plan source", "metrics": { "records": 1 }, "metrics_absent_reason": null, ... }
stderr: note: --source <alpha>/docs/plans/nonexistent.plan.toml does not exist
exit: 0
```

This is the `Q-55-emptyroot` fix's own load-bearing property working correctly: a typo'd anchor is reported (the `note:`) but its own directory's evidence is not thrown away.

## PART 4: CLAIM-ACCURACY FINDINGS

Checked: `README.md:236` and `CHANGELOG.md:23` (both edited by the same fix commit, `a7e05c3`, per its own commit message); the doc comments on `resume_roots`, `containment_roots`, `canonical_project_root`, `checked_plan_root`, `note_missing_anchors`; the `--workflow` help string; the stderr note text; and the test names/doc comments the round-2 style fix pass touched (`a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted`, `a_surface_that_reads_no_plan_is_supplied_a_root`'s doc comment).

RESULT: all of the above are TRUE of the current build EXCEPT ONE, filed below.

VERIFIED TRUE, with evidence:

- README.md:236 / CHANGELOG.md:23: the "reads a plan" qualifier (R2C-1's fix) is present in both files and is accurate: `validate --workflow` in the no-plan-read configuration never reaches containment (three constructed runs, all exit 1 on the unrelated `--workflow requested but no plan source resolved` ground, byte-identical whether the named log is in-root or out-of-root). The "which is always so for `status --resume`... whenever neither..." condition (FV-2's fix) is accurate and not scoped to one example.
- The README/CHANGELOG worked examples were re-run verbatim (substituting a real path for the illustrative `/elsewhere`) and match the commented expected shape exactly: the `validate --workflow` refusal example (README.md:225-231) and the `status --json` omission example (README.md:252-259), the latter reproduced character-for-character including the populated (non-null) `"plan"` object.
- `note_missing_anchors` (src/main.rs, doc comment above the function of that name): every clause verified, including "one line on stderr... so it never contaminates `--json` on stdout" (confirmed: every `--json` run with a missing anchor produced clean JSON on stdout and the note exclusively on stderr) and "`source: no plan source` prints identically for 'no plan was asked for' and 'the plan you named is not there'" (confirmed byte-identical across both triggers).
- `containment_roots`'s doc comment: accurately describes the post-`Q-55-emptyroot` behaviour ("Every case where an anchor IS supplied yields a root, including an anchor that does not exist"), confirmed by construction (Part 3).
- `checked_plan_root`'s doc comment: accurate, no claim about its `None` case's downstream effect that could be falsified.
- `resume_roots`'s doc comment (the fully rewritten one at `a7e05c3`): every clause checked against the current 6-line function body; all true.
- The `--workflow` CLI help string (`ValidateArgs::workflow`): accurate, matches observed behaviour on checks 11 and 13b.
- Test name `a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted` (tests/unsafe_pairings_are_refused_and_omitted.rs:539): matches its body, which asserts both the loud refusal and the quiet omission.
- Test doc comment on `a_surface_that_reads_no_plan_is_supplied_a_root` (tests/unsafe_pairings_are_refused_and_omitted.rs:568-575): the rewritten closing clause ("the two LEDGER readers... agree with each other and the two LOG readers... agree with each other... `status` without `--resume` has no ledger field at all") is confirmed true by reading `struct Projection` (src/main.rs:569-578), which indeed carries no ledger-related field.

### R3ACC-1: LOW. `canonical_project_root`'s doc comment overclaims the "no plan is read" case; it is stale relative to the `Q-55-emptyroot` fix and contradicts its own sibling comment three lines below.

CLAIM (src/main.rs, doc comment immediately above `fn canonical_project_root`, first paragraph, unchanged since the increment's original commit `3d0b967` and not touched by the `a7e05c3` fix): "The canonical project root of a plan a surface actually READS, or `None` when that plan does not exist (nothing was read, so there is no root and the containment predicate below does not fire)."

Read literally, this claims: whenever the plan a surface reads does not exist (equivalently, `checked_plan_root` returns `None`), there is no root and the containment predicate does not fire. That was true of the PRE-FIX build (the exact bug `Q-55-emptyroot`/G-EMPTYROOT closed), where an anchor that failed to canonicalise was silently dropped and, with no second anchor, the containment vector went empty and every quantifier over it was vacuous. It is FALSE of the current build for every case where at least one of `--source`/`--plan` is supplied: `containment_roots` falls through to `resume_roots`, whose own doc comment (added/rewritten by the SAME increment, three lines below this one at the same file) states the opposite for the identical condition: "Where NO plan is read, `checked_plan_root` has nothing to derive from, so the rule SUPPLIES a root from the anchors instead" (src/main.rs, `containment_roots`'s doc comment), and `resume_roots`'s own doc comment states "Every case where an anchor IS supplied yields a root, including an anchor that does not exist."

The two adjacent doc comments in the same file, on the same underlying condition ("that plan does not exist" / "no plan is read"), directly contradict each other: one says no root and no firing predicate, the other says a root is supplied from the anchors and the predicate is exactly what fires there. `README.md:236` and `CHANGELOG.md:23`, edited by the very same fix commit, got the scoping right by narrowing their equivalent sentence specifically to "with NEITHER anchor" (both `--source` and `--plan` absent), which IS the one case where no root is genuinely derived; `canonical_project_root`'s parenthetical was left with the wider, now-false "that plan does not exist" scoping.

EVIDENCE. Constructed directly (dimensions varied: single anchor vs. two anchors; anchor exists vs. does not; artifact inside vs. outside the anchor's own derived root):

```
$ agent-scaffold next --source <alpha>/docs/plans/nonexistent.plan.toml \
    --metrics <beta>/foreign.jsonl --json
```

Here `<alpha>/docs/plans/nonexistent.plan.toml` does not exist and no `--plan` was given at all, so by the comment's own words "that plan does not exist" and "nothing was read". Observed:

```
exit: 0
{
  "task": "nonexistent", "source": "no plan source",
  "metrics": null, "metrics_absent_reason": "log-not-this-project",
  ...
}
stderr: note: --source <alpha>/docs/plans/nonexistent.plan.toml does not exist
```

The containment predicate fired (the foreign `--metrics` was rejected, not read), directly contradicting "the containment predicate below does not fire." The correctly-scoped complementary case (Part 3's second construction, a single nonexistent anchor whose OWN directory's log is requested) shows the predicate does NOT reject the anchor's own artifact, but that is the predicate firing and passing, not failing to fire; the comment's claim is about non-firing, not about a favourable outcome.

`file:line`: src/main.rs, `canonical_project_root`'s doc comment (the paragraph immediately preceding `fn canonical_project_root`), contradicted by `containment_roots`'s doc comment and by `resume_roots`'s doc comment in the same file, and by the runtime behaviour of `containment_roots` -> `resume_roots` demonstrated above.

SEVERITY REASONING. No behaviour is wrong: every acceptance check in Part 1 passes, and the mechanism correctly refuses/omits exactly where it must. This is purely a documentation/mental-model defect of the same class and severity as round 2's R2C-1 and FV-2 (a claim whose scope is wider than the behaviour it describes, immediately beside correctly-scoped sibling text), which were both held at `low` on the same reasoning ("no behaviour is wrong... only into an inaccurate mental model"). Rated `low` on the same precedent.

DIMENSIONS VARIED FOR THIS FINDING, stated per the evidence standard: anchor count (one vs. two anchors supplied), anchor existence (exists vs. does not), and artifact placement relative to the anchor's own root (inside vs. outside). Not varied: the `toml_primary` selection on the checked-plan side (irrelevant here, since the defect is specifically about the `checked_plan_root == None` fallback path, which is reached identically regardless of which selection produced the `None`). This is a documentation finding, not a mechanism defect, so no discriminating control against the in-root bound applies; the in-root bound is a mechanism question about nested vs. disjoint artifact placement, and this finding is unaffected by that dimension either way (confirmed: the construction above used a disjoint layout, and the same contradiction in the doc comment's wording holds independent of nesting, since the comment's own text draws no such distinction).

## PART 5: SUMMARY

- 8/8 assigned acceptance checks pass (18/18 sub-runs), against the current binary at commit `a7e05c3`. No check regressed since the last full run.
- `cargo test --release`: 416/416 passed. No regression in the suite.
- Claim accuracy: 1 finding, `low` (R3ACC-1), a stale doc-comment parenthetical on `canonical_project_root` that overclaims relative to its own sibling comments and to current behaviour. No `medium`, `high`, or `critical` findings from either job.
- The task's specific question ("post-fix, can `status` and `next` hold TWO roots? Construct it") is answered YES, constructed in Part 3, and the implementer's decision to omit the FV-2-prescribed clause naming `status --resume` as uniquely able to hold two roots was CORRECT: that clause would now be false, and the phrasing actually shipped avoids the false claim without needing it.
