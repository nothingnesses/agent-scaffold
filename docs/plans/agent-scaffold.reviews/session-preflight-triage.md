# session-preflight triage

Triager: independent (separate from producer, both reviewers, and the orchestrator).
Artifact: `session-preflight` pack-doc change. Branch `impl/session-preflight` (`6eadf66`), diff `1d76f1e..HEAD`.
Reviewer files adjudicated: `session-preflight-reviewer-opus.md`, `session-preflight-reviewer-sonnet.md`.

Judged against the step's REAL scope, which is the MAIN copy of `docs/plans/agent-scaffold.md`, not the worktree's stale copy. Verified the base-currency issue directly (see verdict T1). Both reviewers ran mechanical checks clean (`just scaffold-self` no drift, `just test` 95 pass, `just clippy` clean, rendered copies match `pack/` sources); I did not re-run them.

Dedup map: opus F1 and sonnet F2 are the same scope-creep finding (adjudicated once as T1). All others are distinct.

---

## T1 (was opus-F1 + sonnet-F2): "scope creep" - worktree-lifecycle and merge-back paragraphs not in the step spec

Verdict on the raised finding: DISMISSED (not scope creep). Reclassified into a distinct, VALID model-gap finding (see T1b).

Reasoning. Both reviewers read the WORKTREE's copy of `docs/plans/agent-scaffold.md`, which is based on `1d76f1e` and is stale. On MAIN the step detail was already extended to authorize exactly this content, by two human-directed orchestrator commits:

- `23c1eaa` ("docs: record worktree merge-back model in session-preflight scope") adds a "Scope addition (human-decided, 2026-07-16)" paragraph to the `session-preflight` step authorizing the WORKTREE LIFECYCLE and MERGE-BACK model in `pack/AGENTS.md` (orchestrator owns the worktree lifecycle; review-in-worktree, merge-on-convergence; main only ever carries reviewed, converged steps).
- `b94cb6e` ("docs: refine merge-back model (authoring vs integration)") refines that same paragraph to authorize the CLEAN-merge-is-integration vs AUTHORED-conflict-resolution-is-implementation distinction, matching commit `6eadf66` in the branch.

So both flagged paragraphs (`46940e7` worktree lifecycle, `6eadf66` authoring-vs-integration) ARE plan-authorized on main. Principle 8 (no silent scope expansion) is not violated: the expansion was human-directed and recorded in the plan. The reviewers' verdict is a false positive caused entirely by the stale base, not by the artifact.

Owner: none (nothing to fix in the artifact for this specific verdict).

## T1b (new, derived from T1): worktree base goes stale relative to main's orchestrator-owned edits, so a worktree reviewer judges against a stale plan

Verdict: VALID. Severity: medium.

Reasoning. T1 exposes a real gap in the workflow model, not just a reviewer mistake. Orchestrator-owned artifacts (the plan and the ledger) are edited on MAIN, while product changes live in the WORKTREE branch. The worktree branch's base can therefore fall behind main whenever the orchestrator records a plan/ledger edit during the step (here, the two scope-addition commits landed on main after the worktree branched). A reviewer or triager reading the plan from the worktree then judges the change against a stale spec and manufactures false scope-creep findings, exactly what happened. This is a structural defect in the review-in-worktree model the step itself is documenting; it is medium because it will recur on every step whose scope is amended mid-flight and it silently corrupts review verdicts (wasted rounds, wrong dismissals).

Recommended action (both parts):
- (a) Add to the orchestrator's worktree duties: before spawning reviewers/the triager against a worktree branch (and before any fix round), rebase the worktree branch onto current main so the plan and ledger the reviewers read are current. A rebased branch stays fast-forwardable, so this is consistent with the merge-back model already in the paragraph.
- (b) Document that base-currency rule in the "Worktree lifecycle and merge-back" paragraph in `pack/AGENTS.md` (and mirrors), stating that the orchestrator keeps the worktree branch rebased on current main so reviewers judge against the current plan/ledger, since those orchestrator-owned artifacts are authored on main, not in the worktree.

Owner: isolated implementer in the worktree.

## T2 (sonnet-F1): findings-file location is unspecified for a worktree review

Verdict: VALID. Severity: low (reviewer rated medium; lowered).

Reasoning. The new worktree-lifecycle paragraph says reviewers and the triager "read the branch under review" but never says where they WRITE their findings files, and the existing "Findings files" rule (`AGENTS.md:61`) predates the worktree model, so the cross-tree case is genuinely open. The correct answer is the main repo's `docs/plans/<task>.reviews/`: findings must be readable by the orchestrator and other agents without tree-switching, and must survive `git worktree remove`. Lowered to low (not medium) because the existing Findings-files rule already says the orchestrator assigns each reviewer/triager its exact file path, so in practice the orchestrator's briefing resolves it; this is a documentation completeness gap in the new section, not a live ambiguity that blocks a run. Same root cause family as T1b (the worktree model interacting with main-owned artifacts).

Recommended action: add one sentence to the "Worktree lifecycle and merge-back" paragraph in `pack/AGENTS.md` (and mirrors): reviewers and the triager write their findings files to the MAIN repo's `docs/plans/<task>.reviews/` (not inside the worktree), consistent with the orchestrator owning those artifacts on main, so they are readable across trees and outlive the worktree.

Owner: isolated implementer in the worktree.

## T3 (opus-F3): clean-merge path does not state the fallback when post-merge tests fail

Verdict: VALID. Severity: low.

Reasoning. The authoring-vs-integration paragraph requires the orchestrator check that "the tests pass on the main tree after the merge" but does not say what happens when that check fails, i.e. a textually clean merge that is semantically broken. The routing is inferable (treat it like authored conflict resolution) but left implicit, a genuine small logical gap. Low because the check itself is present.

Recommended action: add a clause to the clean-merge case: if the post-merge test check fails, the orchestrator undoes the merge so main stays clean (reset/revert to pre-merge), then routes the fix to an isolated implementer in the worktree (the same path as authored conflict resolution) and re-reviews before retrying the fast-forward. A clean merge is only integration WHEN it is mechanically correct AND tests pass; otherwise it becomes implementation.

Owner: isolated implementer in the worktree.

## T4 (sonnet-F4): abandoned-worktree clean-up path is unspecified

Verdict: VALID. Severity: low.

Reasoning. The lifecycle documents only the happy path (create -> implement -> review -> converge -> merge -> remove). It says nothing about a step abandoned before convergence (plan revised to drop the step, or the human cancels), leaving the worktree orphaned with no documented disposition. Real gap, low severity (rare, and a stranded worktree is recoverable, not damaging).

Recommended action: add one sentence: if a step is abandoned before convergence, the orchestrator removes the worktree and deletes its branch without merging (nothing unreviewed reaches main), under the same commit-before-delete discipline as other workflow-managed artifacts.

Owner: isolated implementer in the worktree.

## T5 (sonnet-F3): AGENTS.md Preflight rule does not state ordering relative to state reconstruction on resume

Verdict: VALID. Severity: low.

Reasoning. The AGENTS.md Preflight paragraph lists the resume trigger ("whenever a session is resumed ... after a compaction or a pause") without saying the preflight runs AFTER state reconstruction. `resume.md` orders them correctly ("Reconstruct your state ... then run the preflight ... before doing any work"), but a reader of AGENTS.md alone would not know. The ordering matters: detecting the isolation tier and confirming adherence is more precise once the orchestrator has read the plan's Status line and knows where the work stands. Low.

Recommended action: in the AGENTS.md Preflight paragraph (and mirrors), qualify the resume trigger as running after reconstructing state per the "Checkpoint and resuming after context loss" section, so the ordering is explicit in AGENTS.md, not only in the resume pointer.

Owner: isolated implementer in the worktree.

## T6 (opus-F2): "worktree-first" phrasing vs the container-first tier order

Verdict: VALID. Severity: low.

Reasoning. The standing directive is phrased "isolated (worktree-first) agent" in the Preflight paragraph (`pack/AGENTS.md`) and in `pack/prompts/orchestrator.md`, next to a tier order that is container-first (container, then worktree, then fallback). A reader can momentarily read a contradiction. It is defensible: the same sentence resolves the order ("container via agent-box / agent-images if wired, else a git worktree, else the file-safety fallback"), and the phrasing mirrors the plan's own "worktree-first (where possible)" language, so worktree is the realistic top tier only because containers are not currently wired (`Q-33`). Low, optional clarifier; non-blocking.

Recommended action (optional): change "worktree-first" to "worktree-first where containers are not wired" (or similar) in the Preflight paragraph and `orchestrator.md`, to remove the momentary tension. Skip-able if the implementer prefers to keep it aligned with the plan's wording.

Owner: isolated implementer in the worktree.

## T7 (sonnet-F6): orchestrator.md restates the tiers while AGENTS.md says reference, do not restate

Verdict: VALID. Severity: nit.

Reasoning. AGENTS.md's Preflight item (2) says the orchestrator should detect and state the tier "referencing that rule rather than restating the tiers," while `orchestrator.md`'s new paragraph restates the full tier list inline. As sonnet notes, these serve different purposes (AGENTS.md governs the human-facing preflight OUTPUT; orchestrator.md gives the agent its detection context) and are not contradictory, but an agent reading both notices the tension. Nit.

Recommended action: scope the "referencing that rule rather than restating the tiers" clause in AGENTS.md to the human-facing preflight output (for example "in that stated output, referencing that rule rather than restating the tiers"), so it plainly does not forbid the role prompt from carrying the tier list as detection context.

Owner: isolated implementer in the worktree.

## T8 (sonnet-F5): "defined there" in resume.md is ambiguous

Verdict: VALID. Severity: nit.

Reasoning. `resume.md` says "run the preflight defined there," where "there" refers back to `AGENTS.md` two clauses earlier; `kickoff.md` uses the explicit "run the preflight defined in `AGENTS.md`." Instructions agents must follow should be explicit. Trivial.

Recommended action: change "defined there" to "defined in `AGENTS.md`" in `pack/user-prompts/resume.md` and its `.agents/` mirror.

Owner: isolated implementer in the worktree.

---

## D1 (directed addition, apply regardless, not a reviewer finding): authored conflict resolution must go through review AND a separate FOCUSED triager before the fast-forward

Status: human-decided, apply regardless of review. Severity/priority: medium (correctness of the merge model).

The authoring-vs-integration paragraph currently says an authored conflict resolution "is then reviewed like any writer output (the review loop above)." Make explicit that the resolution goes through a reviewer AND a separate triager (per the always-separate-triager rule), and that this is a FOCUSED review of just the conflict resolution (the newly authored merge content), not a full re-review of the already-converged step, before the orchestrator fast-forwards. This keeps the separate-triager guarantee on the one piece of content no reviewer has seen, without paying for a redundant re-review of work that already converged.

Recommended action: extend the authored-conflict-resolution sentence in the "Worktree lifecycle and merge-back" second paragraph (`pack/AGENTS.md` and mirrors) accordingly.

Owner: isolated implementer in the worktree.

---

## Severity summary

- Critical: 0.
- High: 0.
- Medium: 1 valid (T1b base-currency / rebase-before-review) + D1 directed addition (medium priority).
- Low: 5 valid (T2 findings-file location, T3 post-merge test fallback, T4 abandoned-worktree cleanup, T5 preflight ordering on resume, T6 worktree-first wording).
- Nit: 2 valid (T7 reference-vs-restate scope, T8 "defined there").
- Dismissed: 1 (T1 scope creep, both reviewers) - refuted by main plan commits `23c1eaa` and `b94cb6e`; reclassified into the valid T1b model gap.
