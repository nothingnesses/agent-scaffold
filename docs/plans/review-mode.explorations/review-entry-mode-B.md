# Q-35 review entry mode: design exploration B

The human has locked the direction: `review` is a THIRD entry mode of the one workflow (alongside `implement` via `kickoff` and `explore` via `explore.md`), not a separate workflow. It reuses the review machinery wholesale, has no implement phase, and terminates in a findings REPORT to the human with a clean handoff to a `kickoff` task. This document resolves the concrete shape and recommends one design, judged against the numbered Project Principles (`agent-scaffold.md` "## Project Principles", 1-7).

The single most load-bearing observation, which drives every decision below: the implement run already ends in an ACCEPTANCE phase (AGENTS.md phase 5), and acceptance is defined as "a single reviewers-then-triager pass, not the consecutive-clean convergence loop: it does not require clean rounds and does not run its own round loop or cap." That is exactly what a standalone review is. So `review` is not new machinery; it is the acceptance pass promoted to a first-class ENTRY point, entered directly by a human instead of being reached as the terminal of an implement run. Everything else follows from refusing to build a second copy of that pass (Principle 2 minimal, and the reuse-not-duplicate / single-source-of-truth constraints the task weights most).

## Target: how the human specifies what is reviewed

Reuse the reviewer prompt's existing convention verbatim. `pack/prompts/reviewer.md` already tells reviewers "use the before and after commit hashes (or the diff range) you were given," and `triager.md` mirrors it. The review entry mode specifies the target in exactly those terms, so no reviewer/triager wording has to learn a new target vocabulary:

- A DIFF between two refs: the human writes a range, e.g. `<before>..<after>` (or a pair of hashes). This is the literal input the reviewers already expect. Use case: "review what changed on this branch," "review this PR range."
- The WHOLE CURRENT STATE: the degenerate case where there is no "before." The human names a single ref (default `HEAD`) and, optionally, a path scope (e.g. `src/metrics.rs` or `src/`). The orchestrator briefs reviewers to judge the tree AS IT STANDS at that ref rather than a delta. This is the "whole-codebase acceptance review" LATER job (see Subsumption below), now just `review` with target = current state and no path scope.

The scope is a bound, not decoration: it is the primary defence against a whole-codebase review sprawling without end (see Adversarial). The orchestrator passes the resolved target string to every reviewer and the triager unchanged, so all agents in the round judge the same target (single source of truth for "what is under review," Principle 5: the target is stated once and handed down, never re-derived per agent).

## Criteria: what anchors the review, and how reviewers are briefed

Three criteria sources, which COMPOSE rather than being mutually exclusive, listed strongest anchor first:

1. A plan's Success Criteria. The human points at `docs/plans/<x>.md`; the orchestrator briefs reviewers to review the target against that plan's Success Criteria, exactly as the acceptance phase does. This is the tightest anchor and makes `review` against a plan bit-for-bit the acceptance pass.
2. Ad-hoc constraints. The human writes explicit conditions in the review prompt ("must not allocate on the hot path," "public API must stay backward compatible"). The orchestrator hands these to reviewers as the review contract for this round.
3. Open-ended correctness and quality. The default when neither of the above is given: review against `AGENTS.md` and the project's numbered Principles for correctness, edge cases, and consistency.

The critical adversarial point (a review with no plan and no criteria: what anchors severity?) is already resolved by the existing reviewer contract and needs NO new mechanism. `reviewer.md` rates each finding on "an absolute rating of the finding's impact if left unfixed, not a ranking relative to the other findings," on the four-level `low`/`medium`/`high`/`critical` scale, and instructs reviewers to "read AGENTS.md and the plan, so you review against the project's current principles." AGENTS.md and its numbered Principles are ALWAYS present, so there is always an anchor even with zero plan and zero ad-hoc constraints: the severity floor is "impact against the project's principles and correctness," never "how far from a plan." Criterion 3 is therefore never empty. The only new obligation is that the report must STATE which criteria anchored it (see Report terminal), so the human can calibrate how to read the severities.

## review.md prompt outline (mirror kickoff.md and explore.md)

A thin trigger, same register as the other two user-prompts: it points at AGENTS.md and states the target and criteria; it does NOT restate the workflow. Draft:

```
# Review prompt

Copy this, fill in the bracketed parts, and paste it to the agent to ask for a
code review that ends in a findings report, not an implementation. It triggers
the review entry mode defined in `AGENTS.md`; like the kickoff and explore
prompts, it is a thin trigger and does not restate the workflow. The agent
reviews the target you name against the criteria you give, and returns a
findings report for you to act on. It does NOT fix anything.

---

Act as the orchestrator described in `.agents/prompts/orchestrator.md`. Read
`AGENTS.md` first (the workflow and the project's principles), then, if this
review is against a plan, the plan under `docs/plans/`.

I want a review, not an implementation. Do not fix anything; produce a findings
report and stop.

Target: [either a diff range `<before>..<after>` (or two commit hashes), or
"the current state at <ref, default HEAD>", optionally scoped to <paths>].

Criteria: [one or more of: "the Success Criteria in docs/plans/<x>.md"; these
constraints: <list>; open-ended correctness and quality against AGENTS.md and
the project principles].

[Optional: how deep to go, for example how many independent reviewers and which
lenses or models; any context or where the relevant code lives.]

Run the review entry mode as `AGENTS.md` defines it: brief independent
reviewers on this target and these criteria, have a separate triager adjudicate
their findings, and write me a findings report (with a suggested kickoff
handoff for any valid findings). This is a single reviewers-then-triager pass,
the same pass as the acceptance phase; there is no implement phase.
```

Registration, mirroring `explore.md` exactly: add `[[asset]]` `source = "user-prompts/review.md"`, `dest = ".agents/user-prompts/review.md"` to `pack/pack.toml`, and add `".agents/user-prompts/review.md"` to the asset-list assertion in `src/manifest.rs`. Add a pointer in AGENTS.md "Getting started, for the human" ("To ask the agent to review a diff or the current codebase and report findings without changing anything, use `.agents/user-prompts/review.md`").

## Report terminal: the human-facing findings report

Format. A single committed Markdown file, `docs/plans/<task>.review-report.md`, distilled from the triaged findings. It is the durable deliverable of the run (contrast the per-reviewer findings files and the ledger, which are transient and deleted). Sections:

- Header: the target (the exact ref or diff range and any path scope) and the criteria that anchored the review (which of the three sources, named explicitly), so a reader knows what the severities are measured against.
- Valid findings: each with its triager-confirmed severity, the file-and-line or step evidence, and a one-line description. Grouped by severity, highest first.
- Dismissed findings (brief): what was raised and dismissed, and why, so the human sees the review's coverage and does not re-raise settled points. Any high-or-critical dismissal carries its second-triager re-check outcome (the existing backstop applies to this pass too).
- Bottom line: an explicit "no blocking issues found" when clean, or a ranked list of what to fix.
- Suggested kickoff handoff: a ready-to-paste block (see Kickoff handoff below).

Single-pass, NOT convergence. The report is produced from ONE reviewers-then-triager pass, identical to the acceptance phase, for a structural reason: convergence (consecutive clean rounds) presupposes a WRITER that changes the artifact between rounds so a later round reviews something new. Review mode spawns no writer, so the target is static; a second round would only re-sample fresh reviewers against byte-identical input. That is not convergence (there is no streak to build on unchanged input, and no fix to verify); it is just more sampling, which is better spent as MORE PARALLEL REVIEWERS in the one pass (different lenses and models) than as sequential rounds that add latency without changing the semantics. So: breadth via parallel reviewers in a single pass, not depth via rounds. This directly kills the "unbounded review that never converges" failure mode: the pass cannot loop because it has no loop.

Durability. The report is COMMITTED as the run's output (Principle 3/4 durability, and File-safety "git is the recovery substrate"). This is the answer to "reports rot because they are not committed": unlike findings files and the ledger, the report is not deleted at task close; it is the artifact the human keeps and the kickoff handoff references by path. It lives beside its plan (or, for a plan-less review, under `docs/plans/` named for the review task) so it travels across machines and sessions.

## Ledger and round-log decision

Does a review-only run (which implements nothing) create a ledger and write round records, and does it count toward metrics? Decision:

- Ledger: YES, but minimal. The orchestrator copies `LEDGER.template.md` to `docs/plans/<task>.ledger.md` at task start, as for any run, and records ONE round record (what was reviewed: the target and criteria; the reviewers and separate triager with their findings-file paths; the verdicts; the outcome). This keeps single-source-of-truth: the round's narrative record has exactly one home, and a long whole-codebase review survives a compaction mid-pass via the ledger's RESUME STATE (the orchestrator can be re-spawned and know which reviewers already ran). There is no consecutive-clean streak to track (single pass), so the ledger stays short. It is DELETED (committed deletion) at task close, because the durable output is the report, not the ledger. This matches how the ledger already relates to its plan: transient working state, deleted when the task closes.
- Round log (`docs/metrics/workflow.jsonl`, only when `--instrument` is on): YES, one `round` record, so a standalone review's reviewer productivity is captured for calibration like any other pass. But this record needs a PHASE, and the metrics `Phase` enum today is `plan_review` / `work_review` / `acceptance` (`src/metrics.rs`, `pack/instrument.md`). This is the one code change the feature needs, mirroring exploration-mode's "mostly docs plus one code line": add a `Review` variant (`phase: "review"`) to the enum, its VARIANTS list, the doc in `instrument.md`, and the metrics tests. Rationale for a new value rather than reusing `acceptance`: a standalone review and an implement-run acceptance are different populations (different provenance, and a review may target a diff or an arbitrary ref with no plan), so tagging both `acceptance` conflates them and poisons any calibration that reads phase (Principle 6, ground decisions in evidence: do not merge two distinct signals into one label; Principle 5, do not represent a review round as an acceptance round when it is not one). The record contributes nothing to the CONVERGENCE-constant calibration (no streak, single pass), exactly as the acceptance phase's single-pass records already do not; it contributes reviewer-productivity data.

Single-source-of-truth holds throughout: the narrative round record lives in the ledger (always present, committed), the JSONL is the optional mechanical mirror (present only under `--instrument`), and the report is the distilled human deliverable. Three artifacts, three distinct purposes, no restatement between them (the report references findings by their file paths, the ledger references the plan, the JSONL is machine-only).

## Kickoff handoff: valid findings into a follow-on implement task without re-reviewing

The report ends with a ready-to-paste kickoff block, e.g.:

```
To act on these findings, paste `.agents/user-prompts/kickoff.md` with:
  Task: Fix findings F1, F3, F5 in docs/plans/<task>.review-report.md
        (the high/medium items; F2, F4 accepted as residual risk).
```

The handoff is clean precisely because the findings are ALREADY triaged-valid. The follow-on implement run does NOT re-review them: it treats the committed report as the task input (the substrate its planner folds into a plan or the implementer works from directly for a trivial fix), implements the fixes, and its OWN work-review phase reviews the FIXES (new code), not the original findings. So there is no double review: the review-mode pass established that the findings are valid, and the implement run establishes that the fixes are correct. These are two different artifacts under review (the original target vs the fix diff), which is why no re-litigation rule is even needed across the boundary. The report being committed is what makes the handoff durable: the kickoff task references it by path, so the implement run reads the exact triaged findings rather than a remembered summary.

## Subsumption of the acceptance review, and relation to the implement run's acceptance phase

This is the reuse-not-duplicate core of the design. There is ONE piece of machinery: a single reviewers-then-triager pass against stated criteria. It has two ENTRY points and two TERMINALS:

- As the implement run's phase 5 (acceptance): entered automatically when no pending steps remain; criteria are the plan's Success Criteria; terminal feeds each valid shortfall BACK into that run's planning or implementation, verified by a later acceptance pass.
- As the `review` entry mode: entered directly by a human via `review.md`; criteria are whatever the human named (a plan's Success Criteria, ad-hoc constraints, or open-ended); terminal is a REPORT to the human plus a kickoff handoff, and the run stops.

They are the SAME pass with different entry and terminal, which is exactly the "one workflow, several entry modes" frame the human locked. AGENTS.md should state review mode BY REFERENCE to the acceptance phase ("the same single reviewers-then-triager pass as the acceptance phase (phase 5), entered directly by a human and terminating in a report rather than feeding shortfalls back into an implement run"), not restate the pass. This is what prevents silent duplication: there is one description of the pass, and review mode points at it.

The "whole-codebase acceptance review" noted as a LATER job in `agent-scaffold.md` is deleted as a separate job and folded here: it is `review` with target = current state (no diff, optional whole-tree scope) and criteria = the plan's Success Criteria or open-ended quality. No separate whole-codebase reviewer is built. The plan's Status/Roadmap text that carries that LATER job is updated to say "subsumed by the `review` entry mode (Q-35)."

Pack vs code changes, summarised:

- Pack (docs only): new `pack/user-prompts/review.md`; a "Review entry mode" paragraph in AGENTS.md (mirroring the exploration paragraph at line 45) plus a short "Review report" terminal description (mirroring "Design explorations"); a "Getting started" pointer; small generalizations in `reviewer.md`/`triager.md` so "the plan" degrades gracefully to "the criteria you were given" when no plan is in play (one clause each, so a plan-less review does not read as missing an artifact); `pack.toml` asset registration.
- Code: add `review.md` to the asset-list test in `src/manifest.rs` (a test edit); add the `Review` phase variant to `src/metrics.rs` and its tests and the `pack/instrument.md` doc (the one runtime-code change).
- Plan: mark Q-35 decided/folded; delete the standalone whole-codebase-review LATER job as subsumed.

## Adversarial failure modes and mitigations

- No plan, no criteria: what anchors severity? Mitigated by the existing reviewer contract: AGENTS.md and the numbered Principles are always present and the four-level scale is an ABSOLUTE impact rating, so criterion 3 (open-ended correctness/quality) is never empty. The report must name which criteria anchored it, so the human reads severities correctly.
- Unbounded whole-codebase review that never converges. Mitigated structurally: the pass is single-pass by construction (no writer, so nothing changes between rounds, so there is no convergence loop to run and nothing to fail to converge). Scope is bounded by the target's ref and path scope. Depth is bought with more parallel reviewers, not more rounds.
- A review mode that silently duplicates the acceptance phase. Mitigated by defining review mode AS the acceptance pass entered standalone, referenced not restated; one description of the pass exists in AGENTS.md.
- Findings reports that rot because they are not committed. Mitigated by making the report the run's committed durable deliverable (kept, not deleted, unlike findings files and the ledger), which the kickoff handoff references by path.
- Scope creep where review mode starts implementing fixes. Mitigated by review.md and AGENTS.md stating explicitly that review mode has NO implement phase and spawns NO implementer or planner (only reviewers and a triager, both read-only); the terminal is the report; fixing is a separate kickoff run. The read-only roster is the enforcement: with no writer role spawned, there is no agent that CAN implement (Principle 5, make the illegal state, a review that edits code, unrepresentable by not staffing a writer).

## Recommendation

Build `review` as the acceptance pass promoted to a first-class entry mode: a single reviewers-then-triager pass over a human-named target (a diff range or the current state, in the reviewers' existing before/after-or-range vocabulary) against human-named criteria (a plan's Success Criteria, ad-hoc constraints, or the always-present open-ended default anchored on the Principles), terminating in a committed `docs/plans/<task>.review-report.md` with a ready-to-paste kickoff handoff. It creates a minimal one-round ledger (deleted at close, the report being the durable output) and, under `--instrument`, writes one round-log record tagged with a NEW `review` phase. The whole-codebase acceptance-review LATER job is subsumed. The footprint is a new docs-only user-prompt plus AGENTS.md/prompt wording, and exactly one runtime-code line class: the `review` phase enum value. This maximises reuse (Principle 2), keeps one description of the review pass (single-source-of-truth), and keeps the illegal "review that implements" unrepresentable by staffing only read-only roles (Principle 5).

## Steelman against this recommendation

The strongest case against my own design attacks TWO choices.

First, the new `review` phase enum value. A reasonable explorer would argue this is unnecessary code for a metrics distinction nobody has asked to calibrate: the acceptance phase is ALREADY a single-pass, non-convergence-contributing record, so a standalone review is data-shaped identically, and reusing `acceptance` would make the feature PURELY docs (zero runtime-code change), a stronger fit for Principle 2 (minimal by default; adding a mode must not complicate the core) and for the "mostly docs" spirit exploration-mode set. The counter I gave (populations differ, conflation poisons calibration) is speculative until `workflow-calibration` actually runs and finds it needs to split them; Principle 6 (ground decisions in evidence) arguably cuts AGAINST adding the enum value now, since we have no evidence the split matters, and YAGNI says add it when calibration demands it. This is the single point on which I expect a reasonable explorer to land differently, and it is genuinely close: if the reviewer of this exploration prefers zero code change, reuse `acceptance` and note in `instrument.md` that acceptance records may originate from a standalone review, deferring the phase split to `workflow-calibration`. I lean to the new value because a data label is cheap to add now and expensive to disambiguate retroactively (old records tagged `acceptance` cannot later be re-attributed), but I concede the evidence-grounding principle points the other way.

Second, single-pass. The steelman here is that a high-stakes WHOLE-CODEBASE review is precisely the case where one pass is too weak: fresh reviewers are sampled each round, and the convergence machinery exists BECAUSE one clean round is weak evidence (AGENTS.md Convergence), so a whole-codebase review of a large static target arguably wants multiple independent reviewer ROUNDS (more total sampling) even though nothing changes between them, treating "rounds" as pure breadth. My design routes that breadth into parallel reviewers within one pass instead, which is latency-cheaper but caps the sampling at whatever fan-out the harness runs at once. If a harness cannot fan out widely, sequential rounds would sample more. My rebuttal: sequential rounds on an unchanged artifact have no defined stopping rule here (the consecutive-clean semantics are about a CHANGING artifact), so they reintroduce the non-termination risk the single-pass design removes; better to make "depth" an explicit reviewer-count knob in review.md (already in the optional line) than to resurrect a round loop with no convergence meaning. But an explorer who weights thoroughness of a whole-codebase audit over termination-simplicity could reasonably argue for an opt-in bounded multi-round review (say, a human-set N passes) for the highest-stakes target, and that is a defensible alternative to my strict single-pass.
