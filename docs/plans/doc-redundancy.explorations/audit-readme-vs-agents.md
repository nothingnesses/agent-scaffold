# Doc-redundancy audit: README.md vs AGENTS.md

This is a read-only redundancy audit. It changes nothing and proposes no edits; it enumerates every workflow rule, concept, definition, or description that appears in BOTH `README.md` and `AGENTS.md`, classifies each, and scopes a later human design decision on where the single-source line should sit per instance. It mirrors the Q-44 Phase 1 audit house style (prose findings with `file:line` evidence).

Files read in full: `AGENTS.md` (144 lines), `README.md` (304 lines).

Scope note on the "one source" caveat: root `AGENTS.md` is a generated copy of `pack/AGENTS.md` (scaffold-self), so that pair is ONE logical source and is NOT treated as a redundancy. Every finding below is `README.md` content vs the AGENTS.md content, which is the real question.

## The core tension (frame for the human, not decided here)

Two of the scaffold's own principles pull in opposite directions on this exact question, and every instance below is a point on the line between them:

- Principle 16 (one source of truth): "Keep one authoritative source for each piece of data and derive the rest." This pulls toward stating each workflow rule ONCE, in AGENTS.md, and having README point to it. Every restated rule in README is a place the two can silently diverge when one is edited and the other is not.
- Principle 20 (make documentation self-contained) plus README's actual job (it is the crates.io / GitHub landing page, read by a human deciding whether to adopt the tool, often BEFORE any `AGENTS.md` exists in their target repo): this pulls toward README carrying enough of the workflow to stand alone and sell the tool, without forcing the reader into AGENTS.md first.

The resolution is not global. README is documentation ABOUT the tool that produces AGENTS.md; a landing page that only said "see AGENTS.md" would fail its onboarding job. So a short SUMMARY that defers to AGENTS.md as the authority (type B below) is legitimate and arguably good. The failure mode P16 warns about is a RESTATED RULE that carries a specific, load-bearing value (a cap number, an exemption clause, an exact routing) which can drift from its authority (type A), or that HAS ALREADY drifted (type C). The audit's job is to separate these three so the human can decide per instance; it does not decide them.

## Classification key

- (A) REAL SSOT DRIFT-RISK: the same rule is RESTATED in both such that they can diverge (the Principle 16 failure mode), but the two copies currently agree.
- (B) LEGITIMATE SUMMARY/POINTER: README gives a short entry-point overview and defers to AGENTS.md as the authority; acceptable, arguably good onboarding.
- (C) ALREADY DRIFTED: the two copies say materially DIFFERENT things now (the worst case).

---

## Findings

### R-1: Harness-agnostic canonical-file rule -- classification A (low value)

- `AGENTS.md:3`: "It is harness-agnostic: any harness-specific file (for example `CLAUDE.md`) should point here rather than duplicate it."
- `README.md:12` (Motivations bullet): "Guidance is harness-agnostic. `AGENTS.md` is the canonical file, and any harness-specific file (for example `CLAUDE.md`) should point at it rather than duplicate it."

This is the same normative rule ("a harness-specific file should point rather than duplicate") stated in two places, near-verbatim. It is genuinely restated, not merely summarised, so if the policy ever changed in AGENTS.md the README bullet would silently keep the old wording: that is the P16 drift-risk, hence type A. It is low value because the rule is a one-line design stance, not a load-bearing constant, and README's Motivations section legitimately needs to state the tool's design philosophy to a prospective adopter. Cheapest single-source fix: keep README's bullet as a description of the PROPERTY ("guidance is harness-agnostic; `AGENTS.md` is canonical") and drop the restated directive clause ("should point ... rather than duplicate"), leaving that rule's one home at `AGENTS.md:3`. Alternatively, accept it as a deliberate onboarding restatement; the drift cost here is small.

### R-2: The phase sequence (front-load / plan / review / implement / review) -- classification B

- `AGENTS.md:27-35`: the authoritative numbered Phases 1-5 with full detail.
- `README.md:5` (intro): "front-load context, write a structured plan, review it, implement in small steps, then review the work."
- `README.md:47` (How it's used): "front-load context, draft a plan under `docs/plans/`, review the plan, implement in small steps, then review the work."

README gives a one-line summary of the five phases, twice, with no phase-specific rule that could drift (no cap, no convergence detail, no routing). This is a legitimate onboarding overview and defers to AGENTS.md for the detail. Type B. Sub-note (README-internal, not a README-vs-AGENTS finding): the same phase sequence is stated twice inside README (L5 and L47); harmless, but if one is ever edited for wording the other may lag. Not a P16 violation against AGENTS.md.

### R-3: The roles (orchestrator, planner, reviewers, triager, implementer) -- classification B

- `AGENTS.md:17-25`: the authoritative role definitions, each with its prompt file and responsibilities, plus the separate-triager and writer/read-only distinctions.
- `README.md:47`: "The workflow separates roles (an orchestrator drives it, with a planner, independent reviewers, a separate triager, and an implementer), and `.agents/prompts/` carries one prompt per role."
- `README.md:51` (parenthetical): "an orchestrator drives every phase and keeps the review ledger."

README names the five roles in one sentence and correctly captures the load-bearing emphases (reviewers are "independent", the triager is "separate"). It restates no role's detailed rule. The role set matches AGENTS.md exactly (5 roles; the explorer, which AGENTS.md introduces only inside the exploration mode, is correctly omitted from the summary). Type B. If a role were ever renamed or added in AGENTS.md this summary would need a matching touch, but that is normal summary maintenance, not a divergent-rule risk.

### R-4: Convergence / "loops until it converges" -- classification B (and a POSITIVE finding)

- `AGENTS.md:49-55`: the authoritative convergence rule, including the total-round cap "default five" (L53) and the consecutive-clean-round counts (one for low-risk, two for risky, L52).
- `README.md:51`: "Each review loops until it converges."
- `README.md:66,75` (mermaid diagram edges): "total-round cap reached" -> "Escalate to a human".

This is the instance the human specifically asked about (does README restate the cap NUMBER and risk drifting from AGENTS.md?). It does NOT. README says only "loops until it converges" and the diagram labels the escalation edge "total-round cap reached" WITHOUT the number five and WITHOUT the clean-round counts. The single load-bearing constant (five) lives only at `AGENTS.md:53`. This is the model of a good type-B summary and is worth recording as a positive: README carries the concept (there is a cap; it triggers escalation) without carrying the value that could drift. No fix needed; keep it label-only.

### R-5: Stop condition -- classification B

- `AGENTS.md:35`: "The workflow is done when every step in the plan's Roadmap is complete and an acceptance review confirms the plan's Success Criteria."
- `README.md:51`: "the work stops only once every step is done and an acceptance review confirms the Success Criteria."

Near-identical restatement of the stop condition. It carries no drift-prone constant (it names the Roadmap/Success-Criteria concepts, both of which live in the plan, not as numbers). Low drift risk; a reworded stop condition in AGENTS.md would leave README lagging in wording only. Type B (borderline A on the "restated near-verbatim" test, but there is no specific value to diverge, so B).

### R-6: Escalation is a request for a decision, not a stop -- classification B

- `AGENTS.md:35`: "Escalating to a human is not a stop: it is a request for a decision on an impasse, after which the orchestrator applies the decision and resumes the workflow where it paused."
- `README.md:51`: "Escalating to a human is a request for a decision (the workflow resumes at the paused review after it), not a stop."

Same rule, restated in one sentence, currently in agreement. Carries no constant. README even sharpens it correctly ("resumes at the paused review"). Type B; same borderline-A note as R-5 (near-verbatim but no divergent value).

### R-7: Human requests / intake, trivial vs non-trivial routing -- classification B

- `AGENTS.md:37-39`: the authoritative intake rule, including the exact trivial test (local, reversible, changes neither Success Criteria nor Roadmap scope, raises no new open question) and the non-trivial-routes-through-the-planner rule.
- `README.md:51`: "a human may add or change requests at any time, which are assessed at intake and, when non-trivial, re-enter through the plan."
- `README.md:84-87` (mermaid): "Human adds or changes requests" -> "Intake: assess and advise (human decides routing)" -> non-trivial to `plan`, trivial to "Orchestrator folds it in directly" -> "resume at the roadmap-steps gate".

README summarises intake correctly and, importantly, does NOT restate the four-part trivial test (which would be the drift-prone part). The one place the diagram adds detail beyond the prose is the trivial-fold resume point, "resume at the roadmap-steps gate" (`README.md:87`); AGENTS.md L39 says a trivial request "may be folded in directly" without naming the resume point, so the diagram elaborates rather than contradicts. Type B. See R-9 for the diagram as a whole.

### R-8: Acceptance is a single pass that loops shortfalls back to planning/implementation -- classification B (minor imprecision noted)

- `AGENTS.md:33`: acceptance shortfall "goes back to planning (add or revise steps) or implementation."
- `README.md:81-82` (mermaid): "Success Criteria met?" -> "no: shortfall to planning or implementation" -> arrow drawn to the `plan` node only.

The diagram's EDGE LABEL correctly says "planning or implementation", but the edge is drawn only to the `plan` node (there is no drawn arrow from `adec` to an implementation node). This is a minor visual imprecision, not a material contradiction, since the label preserves both destinations. Type B, flagged so a human deciding on the diagram knows the edge under-draws the "or implementation" branch that the prose and the label both state.

### R-9: The workflow mermaid diagram as a whole -- classification B (largest drift SURFACE)

- `AGENTS.md:13-100`: the authoritative prose workflow (phases, convergence, escalation, acceptance loop, intake).
- `README.md:53-88`: the mermaid flowchart encoding the same workflow visually.

The diagram is a legitimate and valuable onboarding summary: a reader grasps the whole loop at a glance. It restates no drift-prone constant (see R-4: no cap number, no clean-round counts). It is classified B, but recorded separately because it is the SINGLE LARGEST drift surface in README: it visually encodes phases (R-2), the convergence decision (R-4), escalation (R-4/R-6), the acceptance loop (R-8), and intake routing (R-7) all at once, so ANY structural change to the workflow in AGENTS.md (a new phase, a changed routing, a renamed decision gate) requires a matching diagram edit or the diagram drifts. It currently agrees with AGENTS.md except for the R-8 under-drawn edge. There is no cheap single-source fix (a diagram cannot "point to" prose), so the honest options for the human are: (a) accept the diagram as a deliberately maintained derived view and add a one-line note that AGENTS.md is authoritative on any conflict; or (b) if maintenance cost is judged too high, thin the diagram toward the phase skeleton only. Recommend (a): the diagram's onboarding value is high and it has stayed accurate; the mitigation is an explicit "AGENTS.md is authoritative" pointer, not deletion.

### R-10: "Getting started, for the human" -- classification B (exemplary pointer)

- `AGENTS.md:5-11`: the full "Getting started, for the human" section.
- `README.md:46`: "To then start a task, see the 'Getting started, for the human' section of the scaffolded `AGENTS.md`, which points you at the kickoff prompt to copy and explains your ongoing part."

This is the model type-B: README does NOT restate the kickoff/pause/resume mechanics; it names the section and defers. No drift risk (a pure pointer). Recorded as the exemplar of the pattern the A/C instances should move toward. No fix.

### R-11: `validate --workflow` convergence check -- classification C (borderline, the closest thing to real drift)

- `AGENTS.md:139`: W3 is convergence-OR-waiver: "a `complete` step with no round records is exempt when a `step`-unit waiver covers it, and an increment whose peak `consecutive_clean` falls short of its risk class's required streak is exempt when an `increment`-unit waiver names that increment's `task`."
- `README.md:187`: "every Roadmap step marked `complete` must have converging round records in the log, so a step marked done without its review loop (or that never reached the clean-round streak its risk class requires) is caught."

README states the W3 rule as an ABSOLUTE ("every ... `complete` [step] must have converging round records"), but AGENTS.md L139 qualifies it with the WAIVER exemption (a complete step or short-streak increment can be exempt when a waiver covers it). README omits the waiver escape hatch entirely. This is materially different: a reader trusting README would conclude there is no legitimate way to mark a step complete without converging records, which the waiver mechanism contradicts. It is classified C (already drifted) rather than A because the two texts currently say different things, but it is BORDERLINE: README is user-facing CLI documentation and omitting the waiver nuance is a defensible simplification of what `validate --workflow` does at a high level, not a wrong statement of the common case. Cheapest single-source fix: add a short qualifier to README, e.g. "must have converging round records (or a recorded waiver)", so the absolute is no longer stated; or explicitly frame the sentence as the common case ("normally must have ...") and defer the exemption detail to AGENTS.md. This is the highest-value finding to put in front of the human because it is the only instance where README's text is currently at odds with the authority.

### R-12: The metrics log path `docs/metrics/workflow.jsonl` -- not a redundancy (shared data pointer)

- `AGENTS.md:131,133,139`: the path and the record schema.
- `README.md:190`: "Validate the default metrics log (docs/metrics/workflow.jsonl)."

Both cite the same path. This is not a restated RULE; it is a reference to a shared data location whose single source is the file itself (and the tool's default). No drift risk beyond a path rename, which would break the tool regardless. Recorded for completeness as a non-finding.

### Non-duplications worth recording (README correctly does NOT copy these)

Three of the heaviest AGENTS.md sections are NOT duplicated in README, which is the correct P16 outcome and bounds the audit:

- The numbered Principles list (`AGENTS.md:104-127`, 22 items) is NOT enumerated in README. README treats principles only as a SELECTION concept for the tool (`README.md:146-169`, `--principles`), never restating the principle texts. Good.
- Writer isolation tiers (`AGENTS.md:79-85`, container / worktree / file-safety) are NOT described in README at all. Good.
- The instrumentation record types and their fields (`AGENTS.md:133-139`) are NOT restated in README; README only documents the `validate`/`status` CLI surface (`README.md:184-208`), which is complementary (user-facing command behaviour) rather than a copy of the schema. The one point of overlap is R-11.

---

## Summary

Counts by classification:

- Type A (real SSOT drift-risk, currently agreeing): 1 -> R-1.
- Type B (legitimate summary/pointer): 9 -> R-2, R-3, R-4, R-5, R-6, R-7, R-8, R-9, R-10. (R-5 and R-6 are borderline A on the "near-verbatim restatement" test but carry no divergent constant.)
- Type C (already drifted, materially different): 1 -> R-11 (borderline; defensible simplification, but README states an absolute the waiver mechanism contradicts).
- Non-findings recorded for completeness: R-12 (shared data path) plus the three correct non-duplications (principles list, isolation tiers, instrumentation schema).

Highest-value single-source fixes (for the human to decide, not decided here):

1. R-11 (type C, top priority): README `L187` states W3 as an absolute ("every `complete` step must have converging round records") while AGENTS.md `L139` qualifies it with the waiver exemption. This is the only place README's text is currently at odds with the authority. Cheapest fix: add "(or a recorded waiver)" or reframe as the common case.
2. R-9 (type B, largest drift surface): the mermaid diagram (`L53-88`) visually encodes most of the workflow at once, so it is the biggest maintenance liability even though it currently agrees. Recommend an explicit "AGENTS.md is authoritative on any conflict" note rather than deletion, and fixing the R-8 under-drawn "or implementation" edge.
3. R-1 (type A, low value): the harness-agnostic "point rather than duplicate" directive is restated near-verbatim at README `L12` and AGENTS.md `L3`. Cheapest fix: README's Motivations bullet describes the property and drops the restated directive clause.
4. R-4 positive finding (keep as-is): README already avoids restating the total-round cap number "five"; the diagram labels the edge "total-round cap reached" without the value. This is the pattern the other instances should follow; do not "improve" it by adding the number.
5. R-2 sub-note (README-internal): the phase sequence is stated twice inside README (`L5` and `L47`); minor internal redundancy, not a README-vs-AGENTS violation, but a candidate to consolidate if README is edited.
