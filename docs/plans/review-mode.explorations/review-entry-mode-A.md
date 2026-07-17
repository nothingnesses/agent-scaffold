# Q-35 design: the `review` entry mode

The direction is fixed: `review` is a third entry mode of the one workflow (alongside `implement` via `kickoff` and `explore` via `explore.md`), not a separate workflow, because it reuses the review machinery wholesale. This note settles the concrete shape: how the human names the target and criteria, the `review.md` prompt, the report terminal, whether a review run writes the ledger and round log, the kickoff handoff, and exactly what code changes versus what is pure pack content.

The governing principle throughout is reuse-not-duplicate and one source of truth per concept (Project Principles 1 and 2, and the plan's standing "extend the existing validator/projection family, single source of truth per region" architecture). The strong version of that here: the reviewer and triager role prompts do NOT change. They are already entry-mode-agnostic (they review "this work" against the plan and the project principles, given a target as before/after hashes or a diff range, and write findings files). Everything specific to review mode lives in the orchestrator's briefing and the new `review.md` trigger, so the review role and the acceptance/plan/work review roles stay a single artifact each.

## Target design (what to review)

Two targets, and they map exactly onto what the orchestrator already hands reviewers today, so no new mechanism is needed:

- Whole current codebase state. The target is the entire working tree at `HEAD` (or the tree as it currently stands), with no baseline. This is the "whole-codebase acceptance review" LATER job, now an instance of this mode.
- A diff between two refs. The human gives two commit hashes, branches, or tags, or a diff range (`<ref-A>..<ref-B>`). This is the reviewer prompt's existing "before and after commit hashes (or the diff range)" convention verbatim.

The `review.md` prompt carries a single TARGET field where the human states one of these in plain words ("the whole codebase as it currently stands", or "the diff between `v0.0.1` and `main`", or "commits `abc123..def456`"). The orchestrator resolves that field into the same brief it already gives reviewers in every other phase: a diff range or commit pair for a diff target, or "the whole tree at `HEAD`, no baseline" for a whole-codebase target. The reviewer prompt already tolerates both: it names before/after hashes and a diff range for the diff case, and "reconstruct the change set from the repository history" plus "review this work on its own terms, investigate it yourself" covers the no-baseline whole-tree case. The one seam is that `reviewer.md` is phrased around a change set, so for a whole-codebase target the orchestrator's per-reviewer briefing states explicitly "the target is the whole tree, there is no before-diff." I recommend carrying that in the briefing rather than editing `reviewer.md`, to keep the reviewer role entry-mode-agnostic (see Rejected alternatives for the edit-the-prompt option).

## Criteria design (what to review against)

Three criteria sources, which compose rather than exclude:

- (a) A plan's Success Criteria. The human names an existing plan under `docs/plans/`; the orchestrator points each reviewer at that plan's Success Criteria section by path. This is already native to the reviewer prompt ("read the plan, so you review against the project's current principles... inconsistent with the plan").
- (b) Ad-hoc human-supplied constraints or conditions. The human writes the specific conditions to check ("no blocking calls on the async path", "every public function is documented", "the migration is reversible"). The orchestrator passes these to each reviewer verbatim, exactly as it passes a diff range today. This is the only genuinely new context a reviewer receives, and it needs no prompt change because it is just more briefing.
- (c) Open-ended correctness and quality. The default when neither (a) nor (b) is given: reviewers apply their standing adversarial correctness/quality lens plus the project's principles from `AGENTS.md`, which they already read.

These compose: the human can name a plan AND add ad-hoc constraints; open-ended (c) is always in force underneath as the floor (the reviewer's assume-issues default and the `AGENTS.md` principles), because the reviewer prompt is written that way already. The `review.md` prompt has a single CRITERIA field that accepts any mix: "against the Success Criteria in `docs/plans/foo.md`", "against these constraints: ...", or left blank for open-ended. The orchestrator briefs each reviewer with the resolved criteria set (plan Success Criteria by pointer, ad-hoc constraints verbatim, principles always), the same way it resolves the target.

## `review.md` user prompt outline

Mirror `kickoff.md` and `explore.md`: a thin trigger that points the agent at `AGENTS.md` and `orchestrator.md` and does not restate the workflow. Outline:

```
# Review prompt

Copy this, fill in the bracketed parts, and paste it to the agent to ask for a
code review and a findings report, rather than to start building or to explore.
It triggers the review entry mode defined in `AGENTS.md`; like the kickoff and
explore prompts, it is a thin trigger and does not restate the workflow. The
agent reviews the target you name against the criteria you give, adjudicates the
findings, and returns a findings report for you to act on. It implements nothing.

---

Act as the orchestrator described in `.agents/prompts/orchestrator.md`. Read
`AGENTS.md` first (the workflow and the project's principles), then, if this
review is against a plan, that plan under `docs/plans/`.

I want a review and a findings report, not an implementation.

Target to review: [the whole codebase as it currently stands, OR the diff
between two refs: commit hashes, branches, or tags, or a `<ref-A>..<ref-B>`
range].

Criteria to review against: [name a plan whose Success Criteria to check, and/or
list specific constraints or conditions to check; leave blank for open-ended
correctness and quality].

[Optional: depth and lenses, for example how many reviewers and which models,
and whether to treat the target as risky (security-, data-, or money-sensitive)
so the review is briefed accordingly.]

Run the review entry mode as `AGENTS.md` defines it: resolve the target and
criteria, run one reviewers-then-triager pass (with the high/critical
dismissal re-check), and give me a findings report grouped by severity with
evidence, plus which findings are worth turning into a kickoff task. Do not
implement any fix.
```

## Report terminal

Single-pass, not a convergence loop. Review mode implements nothing, so the artifact under review is frozen: there is no fix, therefore no revised artifact to re-review, therefore nothing for the consecutive-clean streak to measure (the streak measures "the artifact stopped producing new findings after fixes"). So the terminal is exactly the acceptance review's shape: one reviewers-then-triager pass, no required clean rounds, no round loop, no cap. The high/critical dismissal-recheck backstop STILL applies, and matters more here than anywhere else: it is a per-dismissal guard, not a convergence device, and since there is no second round to catch a wrongly-waved-away critical, the second-triager recheck of a dismissed high/critical finding is the only guard against a stochastic triager burying a real critical. Keep it.

The deliverable is a report the orchestrator synthesises from the reviewers' findings files and the triager's verdict files (both already written under `docs/plans/<task>.reviews/` by the unchanged roles). The report is the durable output and is KEPT (unlike ordinary findings files, which are committed-then-deleted at task close; this parallels "unless the exploration is worth keeping as a durable design record" for explorations). It lives at `docs/plans/<task>.reviews/report.md`, keyed by the review run's task slug (the human or orchestrator picks a slug, for example `review-2026-07-16` or a descriptive name; `docs/plans/<task>.*` is already the workflow's task-keyed artifact root, not strictly a plans directory, as `.reviews/` and `.explorations/` show).

The report is self-contained so it outlives the raw findings files (which follow the normal commit-before-delete cleanup, referenced through git history like the ledger references findings). Format:

```
# Review report: <task slug>

- Target: <whole codebase at HEAD | diff <ref-A>..<ref-B>>.
- Criteria: <plan Success Criteria path | ad-hoc constraints | open-ended>.
- Reviewers: <roles/models>. Triager: <who>. Date: <date>.
- Overall: <one-line assessment, e.g. "3 valid findings, 1 high; the migration
  path is the main risk">.

## Valid findings (triager-confirmed), by severity

### Critical / High
- [<severity>] <one-line title>. Evidence: <file>:<line> (or step). <what is
  wrong and why it matters>. Suggested follow-up: <kickoff-ready task
  statement>.

### Medium / Low
- ... (same shape)

## Dismissed findings
- <one line each>: reviewer raised, triager dismissed, why. (High/critical
  dismissals note the second-triager recheck result.)

## Recommended follow-ups (kickoff handoff)
- <finding or group> -> paste as the Task in `.agents/user-prompts/kickoff.md`;
  target files: <paths>.
```

Evidence points at `file:line` in the reviewed code, never at a findings file, so the report stands alone after the findings files are cleaned up. The report is a file, not the ledger: the ledger is transient round-state deleted at task close, whereas the report is the kept deliverable.

## Ledger and round-log decision

Ledger: yes, a review run reuses the ledger, recording its single pass, exactly as the built-in acceptance phase records its single pass. Reasoning: the acceptance review is also single-pass and is still recorded as a round in the ledger, so review mode is consistent with it (reuse-not-duplicate, Principle 2, no review-only record type invented). The ledger during the run is the transient working record and the resume anchor (its RESUME STATE lets a review interrupted between spawning reviewers and finishing triage rebuild after a compaction); the report is the durable deliverable synthesised from it. The review run copies `.agents/LEDGER.template.md` to `docs/plans/<task>.ledger.md` at start like any task, records the one review round (target, criteria, risk class, the reviewers and separate triager with their findings-file paths, the verdicts, any dismissal-recheck), and deletes the ledger at task close under commit-before-delete, keeping only the report. A review run may run without a plan document (criteria (b)/(c) need none); in that case its only durable anchor during the run is the ledger, and after the run, the report.

Round log (`docs/metrics/workflow.jsonl`): when instrumentation is on, a review run appends ONE `round` record for its pass (plus any `dismissal_recheck` records), matching the acceptance phase. The record's value is reviewer-productivity calibration (the `reviewers` array: `raw_findings` / `valid_findings` per reviewer and model), which is useful regardless of mode. It does NOT feed the required-clean-rounds or cap calibration, because a single-pass review has no convergence signal (`consecutive_clean` is 0 or 1 and `risk_class` does not drive a loop here, the same vacuity the acceptance phase already has). The metrics consumer separates review-mode rounds by their `phase` value. This is the one place a new `phase` value is warranted: add `review` to the `Phase` enum so review-mode data is cleanly separable from an implement run's acceptance data, rather than overloading `acceptance` (see code impact and Rejected alternatives). So: review mode counts toward metrics for reviewer productivity, and is explicitly excluded from convergence-constant calibration by its distinct phase.

## Kickoff handoff

Clean, because the report is self-contained. The report's "Recommended follow-ups" section frames each triager-valid finding (or a group) as a kickoff-ready task statement with its target files. To act, the human copies that statement into the Task field of `.agents/user-prompts/kickoff.md`, names the files as context, and starts a normal implement run. The review run and the follow-on implement run are separate tasks with separate slugs and ledgers; the kept report is the bridge between them. No new machinery: the handoff is just "paste a report finding into kickoff." If the reviewed code has a plan, the human may alternatively route a valid finding into that plan's Open Questions via a normal interrupt, so the fix enters durably through the plan; but the kickoff-from-report path is the primary one, since a review run does not assume a plan exists.

## Relationship to the acceptance review and the whole-codebase LATER job

- The whole-codebase acceptance-review LATER job becomes a plain instance of this mode: TARGET = whole codebase at `HEAD`, CRITERIA = open-ended (or the project's own principles / a named plan). It needs no separate mechanism; it is `review.md` with the whole-codebase target. This subsumes the LATER job.
- The built-in acceptance phase (phase 5 of an implement run) stays exactly where it is: it is the internal terminal of implement mode, gating "is this task done against its Success Criteria." Review mode is the human-invoked analog with the same single-pass reviewers-then-triager machinery, but a report terminal (a deliverable to the human) instead of a done/not-done gate. Seen together: an implement run's acceptance review is the special internal case (target = the run's changes, criteria = this plan's Success Criteria, terminal = done/not-done); review mode generalises it to any human-chosen target and criteria and reports out. They share the single-pass shape and the dismissal-recheck backstop, which is the reuse that justifies making review an entry mode rather than a workflow.

## Code versus content impact

Almost entirely pack content; the real code change is two lines plus a test and a manifest-list update.

Pure pack content:

- New `pack/user-prompts/review.md` (the trigger above).
- `pack/AGENTS.md`: add a "review is a fourth entry mode" paragraph in the entry-modes area (after the design-space exploration paragraph, mirroring how Socratic and exploration modes were added: reuses intake and the review roles, adds no new phase or role); a pointer in "Getting started, for the human" ("to ask for a review and a findings report, use `.agents/user-prompts/review.md`"); and a short "Review reports" artifact convention paragraph parallel to "Design explorations" and "Findings files" (single-pass, report kept at `docs/plans/<task>.reviews/report.md`, raw findings files under normal cleanup, ledger recorded like the acceptance pass).
- `pack/prompts/orchestrator.md`: add the review entry-mode drive (resolve target and criteria from `review.md`, run one reviewers-then-triager pass with the dismissal recheck, synthesise the report, offer the kickoff handoff, no implement phase).
- `pack/prompts/reviewer.md` and `pack/prompts/triager.md`: NO change. This is the single-source-of-truth win; they are already entry-mode-agnostic and the orchestrator carries target and criteria in its briefing.
- `pack/instrument.md`: add `review` to the `phase` enumeration in the `type: "round"` bullet.

Actual Rust code:

- `src/metrics.rs`: add `Review => "review"` to the `Phase` `enum_field!`, and update the one test that asserts the phase error message lists the accepted variants (around the `phase value not one of [...]` assertion). This is the only logic change.
- `src/manifest.rs`: add `".agents/user-prompts/review.md"` to the expected asset list in the manifest test, and add the corresponding `[[asset]]` entry (source `user-prompts/review.md`, dest `.agents/user-prompts/review.md`) to `pack/pack.toml`. Mechanical, matching how `explore.md` is registered.

So the mode is one enum variant of real logic, one pack asset with its registration and asset-list test, and prose additions to three pack docs. Nothing in the review or triager roles forks. That is the reuse-not-duplicate result the direction was chosen for (Principles 1 and 2).

## Recommendation

Ship review as the fourth entry mode with: a TARGET field that is either the whole tree at `HEAD` or a diff of two refs, resolved by the orchestrator into the existing reviewer brief; a CRITERIA field that composes a named plan's Success Criteria, ad-hoc constraints, and an always-present open-ended/principles floor; a thin `review.md` trigger mirroring kickoff/explore; a single-pass reviewers-then-triager terminal (keeping the high/critical dismissal recheck) that produces a kept, self-contained report at `docs/plans/<task>.reviews/report.md`; a reused ledger recording the one pass (deleted at close, like any task) and, under `--instrument`, one `round` record with a new `review` phase for reviewer-productivity calibration only; and a kickoff-from-report handoff. The reviewer and triager prompts stay untouched; the only Rust change is the `review` phase enum variant plus its test and the `review.md` manifest/pack registration.

## Rejected alternatives

- Reuse the `acceptance` phase value for review-mode rounds to avoid any code change. Rejected: it conflates human-invoked review runs with an implement run's internal acceptance gate in the calibration data, so a query that wants required-clean-rounds evidence from real acceptance passes would have to filter review runs out by task slug, which is fragile. The `review` variant is a one-line addition and the round log is explicitly designed to grow (Principle 1, cleaner architecture over the smallest diff).
- Edit `reviewer.md` to name a whole-codebase target explicitly. Rejected: it would tie the reviewer role to entry-mode knowledge it does not need, when the orchestrator's per-reviewer briefing already carries the target in every phase. Keeping the reviewer role entry-mode-agnostic is the single-source-of-truth reason review can be a mode and not a workflow (Principle 2).
- Run review as a convergence loop (consecutive clean rounds, cap) like a work review. Rejected: review implements nothing, so the artifact never changes between rounds and the streak measures nothing; re-running fresh reviewers on the identical frozen target would only add cost and stochastic churn. Single-pass, matching acceptance, is the correct terminal, with the dismissal recheck as the tail guard.
- Put the report in the ledger (or the plan's Open Questions) instead of its own kept file. Rejected: the ledger is transient round-state deleted at task close, so the deliverable would be destroyed with it; Open Questions is the human-decision queue for plan-changing decisions, not a findings sink (the workflow already forbids putting individual findings there). A kept `report.md` parallels the durable-design-record exception for explorations and keeps the deliverable self-contained after findings-file cleanup.
- Make review a separate workflow with its own roles. Rejected by the human's decision and by reuse-not-duplicate: it would fork the reviewer, triager, findings-file, dismissal-recheck, ledger, and round-log machinery that review needs wholesale, violating Principles 1 and 2. The entire justification for an entry mode is that these already exist and review is just a different entry (a human trigger naming target and criteria) and a different terminal (a report, no implement phase) over the same roles and artifacts.

## What not to build (YAGNI boundary)

- No new reviewer or triager prompt, and no review-only findings schema: reuse the existing ones.
- No convergence loop, cap, or required-clean-rounds logic for review: it is single-pass.
- No automatic conversion of findings into plan steps or kickoff runs: the handoff is a human paste of a self-contained report finding; automating it is a separate future call if ever wanted.
- No `status`/`validate` projection work specific to review beyond the `review` phase value already accepted by the round-log schema; the existing validator covers the new phase once the enum variant is added.
