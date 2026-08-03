# workflow-enforcement-tier-inc2, isolated reviewer, SPECIFICATION-CONFORMANCE lens

Artifact under review: `git diff main..HEAD` at commit `1543325` (branch
`review/inc2-rev-conformance`), five files: `src/main.rs`, `src/next.rs`,
`tests/unsafe_pairings_are_refused_and_omitted.rs`, `README.md`, `CHANGELOG.md`.

Specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (389 lines,
read in full).

Build and suite, run first-hand before any behavioural claim below (`TMPDIR` pointed
outside the repo per the spec's own instruction at line 311):

```
cargo build                                   -> clean
cargo test  (TMPDIR=<scratch outside repo>)   -> 378 + 5 + 1 + 1 + 9 + 3 + 13 + 1 + 2 = 413 passed, 0 failed
cargo clippy --all-targets -- -D warnings     -> clean
cargo run -- render docs/plans/agent-scaffold.plan.toml --check -> "up to date"
cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow
  -> docs/metrics/workflow.jsonl: 262 records, valid
     docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
     docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
  (no-regression case, check 9)
```

## REQUIRED RULING: is the containment predicate one test or two?

**Ruling: mechanically ONE primitive reused at two different arities, not a
re-implementation, and the one place the two arities disagree is the already-recorded,
explicitly out-of-scope IN-ROOT BOUND surfacing through the two-anchor mechanism rather
than the single-root one. Not a new defect.**

### What the code actually does

`next`, `validate --workflow` and `status` all call `checked_plan_root` (`src/main.rs:1308`)
to get ONE canonical root (the TOML `--source` when it is TOML-primary, else the Markdown
`--plan`; `None` when neither resolves), then test containment of the metrics path and the
ledger path against that ONE root via `is_outside_root` (`src/main.rs:1351`). This is a
plain containment test: is the artifact outside root R.

`status --resume` (`src/main.rs:run_resume`, using `resume_roots` at `src/main.rs:1421`)
reads no plan at all, so there is no single checked plan to root on. `resume_roots` computes
UP TO TWO canonical roots, one per anchor that is given AND canonicalises (`--source` and/or
`--plan`), and `run_resume` (`src/main.rs:1479-1489`) then calls the SAME `is_outside_root`
primitive once per supplied root, treating the ledger as unsafe if it is outside ANY of
them (i.e., it must be inside EVERY supplied root). The doc comment on `resume_roots`
states this is "expressed by requiring the ledger to be under EVERY anchor's root rather
than by comparing the roots to each other" as an intentional stand-in for the spec's "must
resolve to the SAME root" (workflow-enforcement-tier.md:182).

So: `is_outside_root` and `canonical_project_root` are the ONE shared mechanism (One
source of truth is upheld at the function level, matching workflow-enforcement-tier.md:179).
What differs across surfaces is only how many roots get fed into it and where those roots
come from, which follows directly from resume being the one surface that reads no plan
(workflow-enforcement-tier.md:163, :182). This is exactly the "supplies a root... rather
than being re-implemented per surface" framing the spec itself uses for this case, and the
code matches that framing at the mechanism level.

### Where "under every supplied root" and "the same root" coincide, and where they do not

For two DISJOINT roots (the normal case: two unrelated projects), "the artifact must be
inside every supplied root" and "the two roots must be equal" produce IDENTICAL verdicts:
no real path can be inside two disjoint directory trees at once, so the multi-root test
rejects the pairing exactly when the roots differ, matching a literal equality test. This
is the case every acceptance-check fixture in `tests/unsafe_pairings_are_refused_and_omitted.rs`
uses (sibling `home`/`away`, `alpha`/`beta`, all top-level siblings under one scratch root),
and I confirmed it first-hand:

```
$ agent-scaffold status --resume --source <alpha's source> --plan <beta's plan>
the ledger <alpha's ledger path> is not under the plan's project root <beta's root>
exit=0
```

(matches `resume_omits_the_default_ledger_under_a_divergent_pairing`, passing in the suite.)

For NESTED roots (one anchor's project sits inside the other's directory tree), the two
tests diverge. I constructed this directly rather than reasoning about it in the abstract:

```
outer/docs/plans/y.md              (a project's own Markdown plan)
outer/docs/plans/y.ledger.md       ("OUTER's own resume state")
outer/inner/docs/plans/x.plan.toml (a nested, unrelated TOML-primary project)
outer/inner/docs/plans/x.ledger.md ("INNER's own resume state, belongs to the nested project only")

$ agent-scaffold status --resume --source outer/inner/docs/plans/x.plan.toml --plan outer/docs/plans/y.md
## RESUME STATE

INNER's own resume state, belongs to the nested project only.
exit=0
```

`root(--source)` is `outer/inner` and `root(--plan)` is `outer`: two DIFFERENT roots, so
per the spec's literal text ("must resolve to the SAME root or the block is omitted") this
pairing should have been rejected. It was not, because the ledger (anchored on `--source`
per `default_ledger_path`'s source-first order) sits inside `outer/inner`, which is itself a
subdirectory of `outer`, so the ledger is trivially "under every supplied root" even though
the two roots are not the same root.

### Why this is not a new finding

I checked whether the identical gap reaches `next` and `validate` (which use the
single-root test, not `resume_roots`), to see whether this is confined to the two-anchor
mechanism or is a broader hole in the shared primitive. I constructed the analogous case for
`next` (an inner, markdown-primary `--source` anchoring the metrics log and ledger, paired
with an outer `--plan` that is the checked plan) and reproduced the SAME shape of leak: the
inner project's converged round (for a slug matching the outer plan's in-progress step) and
its `## RESUME STATE` block both surface under the outer plan's projection, unflagged:

```
$ agent-scaffold next --source outer/inner/docs/plans/p.plan.toml --plan outer/docs/plans/p.md
ACTIVE LOOP
  shared-step / shared-step-inc1  in progress -> mark-step-complete
  state: converged
  ...
  next: mark the step complete, re-render, and commit
  ...
RESUME STATE (verbatim from the ledger):
## RESUME STATE

INNER's own resume state, must not leak into outer's next projection.
exit=0
```

This is the single-root mechanism failing too, for the same underlying reason: the inner
project's log and ledger sit at their OWN conventional paths, and both happen to lie inside
the checked (outer) plan's root subtree, so a pure containment test cannot distinguish "this
artifact belongs to the plan I am checking" from "this artifact belongs to some other,
nested project that happens to live in the same subtree." That is verbatim the definition
of THE IN-ROOT BOUND named in the specification ("CONTAINMENT REFUSES ONLY WHAT LIES OUTSIDE
THE CHECKED PLAN'S ROOT SUBTREE, so every foreign artifact inside that subtree is invisible
to it: a log copied to this plan's own `docs/metrics/`, and equally a NESTED project's own
log and ledger at their own conventional paths") and is explicitly out of scope for this
review ("Recorded and not closed by explicit human decision").

Both of my constructions are instances of that recorded bound: the `--source`/`--plan`
anchoring is merely HOW the tool ends up pointing at the nested project's artifacts; the
reason neither instance is caught is the same reason a manually copied log is not caught,
which the specification has already measured, accepted, and queued (the log half to the
project-identity step; the ledger half explicitly "has NO OWNER in this plan today,
recorded here rather than scheduled", workflow-enforcement-tier.md:269). I am therefore NOT
filing this as a new finding (CON-N), per the explicit instruction to treat the IN-ROOT
BOUND as out of scope. I am recording the evidence here because the task requires a ruling
regardless of whether it becomes a finding, and because no previous round had demonstrated
that the bound reaches BOTH the single-root mechanism (`next`/`validate`/`status`) and the
two-anchor mechanism (`status --resume`) via the SAME nested-layout shape; a future reader
closing the IN-ROOT BOUND should know both mechanisms need the fix, not just one.

**Conclusion:** the two rules are one predicate, applied at two different arities because
one surface has a checked plan and one does not. That is a faithful, non-buggy
implementation of what `Q-55-resumepairing` asked for, and the residual gap it inherits is
the pre-existing, already-excluded IN-ROOT BOUND rather than a new divergence introduced by
treating the two surfaces differently.

## Findings

None. After sweeping every region below, including adversarial construction (nested
project layouts, typo'd anchors, precedence-rule combinations, symlinked plans and logs,
`..` escapes, the divergent `--source`/`--plan` pairing on all four surfaces), I found no
place where the code disagrees with the specification within this increment's scope. Severity
counts: critical 0, high 0, medium 0, low 0.

This is not a rubber stamp: the required ruling above involved constructing and running two
scenarios not in the existing suite, both of which reproduced the shape of a real leak, and
both of which resolved to the explicitly out-of-scope IN-ROOT BOUND on inspection rather than
to a new conformance defect.

## Regions swept, and what was checked and found conformant

1. **"The exact behaviour, per surface" (line 177).** `validate --workflow` refuses (problem
   pushed, non-zero exit) computed before the four-arm match (`src/main.rs:989-1006`,
   confirmed by running check 11 and check 13b live: exit 1, no "workflow invariants hold").
   `status` omits the metrics half only, plan half unchanged, exit 0
   (`src/main.rs:1141-1163`, confirmed live and by `status_omits_only_the_unpairable_part`).
   `status --resume` omits the whole block with a note, exit 0
   (`src/main.rs:run_resume`, confirmed live and by
   `resume_omits_the_default_ledger_under_a_divergent_pairing`). `next` omits the metrics
   line, the WHOLE `ACTIVE LOOP` block as a unit (state/streak/rounds/next/role/prompt/
   summary all absent, not just the action line), and the resume echo, each with its own
   note, exit 0 (`src/main.rs:1518-1550`, `src/next.rs:render_human`; confirmed live and by
   `next_withholds_the_whole_loop_on_an_unpairable_log`, which asserts all seven field
   prefixes absent). The refusal message names the checked plan, the resolved log, the
   derived root, and THREE remedies verbatim matching the spec's required third member
   ("or correct the `--source` and `--plan` pair", `src/main.rs:998-1002` against
   workflow-enforcement-tier.md:157); confirmed by running check 11 directly.

2. **"The field shape and the value vocabulary" (line 210).** All three enums
   (`MetricsAbsentReason`, `NoActiveLoopReason`, `ResumeStateAbsentReason`,
   `src/next.rs`) carry `#[serde(rename_all = "kebab-case")]` and their variant spellings
   match the specification's wire tokens character-for-character (`log-absent`,
   `log-not-this-project`; `no-plan-steps`, `all-steps-terminal`,
   `metrics-not-this-project`; `ledger-absent`, `no-resume-section`,
   `ledger-not-this-project`), verified against workflow-enforcement-tier.md:216-229 and by
   running `next --json`/`status --json` live. `metrics_absent_reason` sits beside `metrics`
   on both `NextProjection` (`src/next.rs`) and `status`'s `Projection` (`src/main.rs`);
   `resume_state_absent_reason` sits beside `resume_state`. Every reason field is `Some`
   exactly when its sibling is `None` in every code path I traced (`run_status`, `run_next`,
   `next::project`). THE PRECEDENCE RULE (unsafe wins over absent, line 231): confirmed
   live for both `next` and `status` with an explicit `--metrics` naming a nonexistent path
   outside the root (`log-not-this-project`, not `log-absent`), matching acceptance check
   14f's fourth run. THE CORRELATION RULE (line 233): `next::project`'s
   `steps_leave_no_loop` helper is exhaustive against `select_active_loop`'s own terminal
   condition (`StepPhase::is_terminal` covers exactly the four phases not covered by
   `is_pending`/`InProgress`), so `metrics-not-this-project` is reported only when the
   loop's absence really is metrics-derived, pinned by
   `a_terminal_plan_reports_the_step_cause_not_the_log_cause`.

3. **The four falsified doc comments (lines 199-202) and the pre-existing `active_loop`
   mismatch (line 204).** All five diffed and checked word-for-word against the spec's
   required corrections: `no_active_loop_reason`'s comment now states it is serialised and
   why; `NextProjection`'s own comment's enumeration now includes the unpairable case;
   `status`'s `Projection` comment was changed in the SAME diff (not left to drift from the
   cross-reference); `resume_state`'s comment now names three causes. `active_loop`'s
   comment was reconciled to "no steps, every step terminal, or a round log this tool
   cannot vouch for" (`src/next.rs`), removing the false "every pending step blocked"
   claim WITHOUT adding a blocked-steps variant, exactly as line 204 requires; confirmed
   also by the dedicated test `a_blocked_pending_step_still_yields_a_loop_so_it_is_not_a_no_loop_reason`.

4. **Line 206's conventions.** No `skip_serializing_if` anywhere in `src/next.rs` or
   `src/main.rs` (grepped). The three new reason fields serialise as explicit `null` in the
   correct-run case, confirmed live (`next --json`/`status --json` against this
   repository's own plan) and by `a_correct_run_serialises_the_new_reasons_as_null`, and the
   `GOLDEN_JSON`/`GOLDEN_HUMAN` byte-compare tests pass, so the diff is exactly the added
   fields. The two caller-assembled "note" fields (`metrics_absent_note`,
   `resume_state_absent_note`) remain `#[serde(skip)]`, consistent with "THE ENUM IS THE
   MACHINE VALUE ONLY" (line 212): they are rendering plumbing for `next`'s
   internally-computed projection, not part of the documented JSON contract, and are not
   claimed to be.

5. **"Documentation impact" (line 343) against `README.md`/`CHANGELOG.md`.** `README.md:210`
   gains the refusal as a named failure mode with a worked example; `:228` gains the
   omitted-part paragraph, explicitly states all three of `status`/`next`/`status --resume`
   still exit 0, and separates that from the validator's new refusal in the same release
   (matching the spec's explicit worry about a reader conflating the two). The `--json`
   paragraph and example were added. `CHANGELOG.md` gained ONE entry under `Added` (the
   serialised reasons, naming every kebab-case token) and ONE separate entry under
   `Changed` (the refusal/omission behaviour, its two accepted-cost manifestations, and the
   line 263 requirement that a deliberate out-of-root `--metrics` now breaks); this is the
   "likely two entries rather than one" split the spec anticipates (line 363), and the
   `Changed` entry does distinguish the validator's exit-non-zero behaviour from the
   projections' exit-0 behaviour within its text.

6. **The mechanism paragraphs (line 143).** The lexical/canonical split is intact:
   `resolve_metrics_path`/`default_ledger_path` remain purely lexical (unchanged by this
   diff), while `canonical_project_root` explicitly canonicalises and its doc comment states
   the split must not be collapsed and why (`src/main.rs:1279-1288`). The root is taken from
   the plan the surface actually reads (`checked_plan_root`, using `toml_primary` exactly as
   each surface already branches on it), not from the metrics/ledger anchor, confirmed live
   against the divergent `--source`/`--plan` pairing (check 13b) on both `validate` and
   `next`, including the typo'd-`--source` sub-case. The refusal message's three-remedy
   shape is confirmed in section 1 above.

All 20 numbered acceptance checks in the specification (plus 13b, 14b-14h, 19, 19b) that
apply to inc2 are pinned in `tests/unsafe_pairings_are_refused_and_omitted.rs`, all 13 tests
in that file pass, and I additionally reproduced checks 9, 11, 13b, 14b, 14c, 14e, 14f, 19
live against the actual binary (not just the suite) as the evidence standard requires,
plus the two adversarial nested-layout constructions discussed in the required ruling above.
