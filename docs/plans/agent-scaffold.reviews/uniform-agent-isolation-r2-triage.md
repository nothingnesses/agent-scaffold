# Triage verdicts (round 2): uniform-agent-isolation (Q-61), revised artifact at `856ddf1`

Triager run isolated in its own worktree (`triage/ui2`), independent of the writer and the orchestrator. Each finding was re-verified against the artifact at `856ddf1` (my worktree IS the artifact); the reviewers' write-ups were not trusted on their own. Severity is the four-level `low`/`medium`/`high`/`critical` scale from `AGENTS.md`. Round 1's two high findings (F1 pack carve-out, F2 explorer classification) are confirmed fixed by both round-2 reviewers and are not re-adjudicated here; I spot-checked the fix loci (`AGENTS.md:65,83,89,91`, `pack/isolation-guidance.md:37`) and found the flat contradictions gone and no new contradiction introduced.

## Finding 1 (Reviewer D low #1): file-safety baseline intro still framed writer-only

- Verdict: VALID (as a framing/coherence residual).
- Severity: low (uphold Reviewer D).
- In scope: yes, as a residual of THIS change's goal. Not a required fix.
- Must-fix vs acceptable: ACCEPTABLE residual (not a convergence blocker).
- Verification:
  - `AGENTS.md:75` (unchanged by the diff): "Every writer agent's damage must stay a visible, committed-or-recoverable diff ... This is the always-on baseline; running writers under isolation builds on it rather than replacing it (see Writer isolation below)." Intro is writer-scoped. Confirmed verbatim.
  - The universal isolation rule's tier-3 fallback references this baseline for everyone: `AGENTS.md:87` ("The file-safety discipline above, as the fallback when the harness offers no isolation") and `:89` ("Isolation is the structural upgrade over the file-safety baseline"). So a read-only agent resolving to tier 3 now falls back to a baseline whose header/intro names only writers, while `AGENTS.md:89` distinguishes writer from read-only agents by authority.
  - The operative bullets that actually protect a tier-3 read-only agent's only write are already general: "Recover on interrupt" (`AGENTS.md:81`, "On any agent kill or interrupt, the orchestrator ...") and "Commit before delete" (`AGENTS.md:78`, "a findings file ..."). "Clean tree before a writer" (`AGENTS.md:77`) is writer-scoped but is about protecting already-decided state before spawning a writer, not about a read-only agent's own diff. Confirmed.
- Reasoning: this change universalized who-isolates and newly routes read-only agents (which round 1 had said "need no isolation") to the file-safety fallback tier, whose intro is still scoped to "writer agent" damage. That makes it a genuine residual of the change's own goal, not a pre-existing unrelated gap: before this change a read-only agent did not hit tier 3 at all. But the residual is framing only: the two bullets that carry the operative protection for a tier-3 read-only agent's findings-file write are already general, so there is no behavioural hole, illegal state, or contradiction, only a header/intro that talks about writers while the rule it underpins now applies to everyone. That is why it is low, not medium, and why it does not block convergence. The suggested fix (generalise the `:75` intro from "Every writer agent's damage" / "running writers under isolation builds on it" to "every spawned agent") is a cheap, safe improvement that would fully close the coherence gap; I recommend the orchestrator apply it, but its absence is an acceptable residual, not a blocker.

## Finding 2 (Reviewer D low #2, == Reviewer C low note; deduplicated): residual writer-only framing in `pack/isolation-guidance.md:3,30,37`

- Verdict: VALID (as an incomplete completion of round-1 F1's fix direction).
- Severity: low (uphold both reviewers). This is a single finding; Reviewer C's low note and Reviewer D's Finding 2 are the same locus (`pack/isolation-guidance.md:3,30,37`) and are ruled once here.
- In scope: yes, same locus the change was meant to fully fix. Not a required fix in itself.
- Must-fix vs acceptable: ACCEPTABLE residual (not a convergence blocker), with a recommendation to complete it (see below).
- Verification:
  - `pack/isolation-guidance.md:37`: "`ab spawn --local` is the un-sandboxed alternative ...; prefer the `--git` sandbox for writer agents. The orchestrator states the resolved isolation tier at preflight, so you know before a run whether a writer will run in a container, a worktree, or under the file-safety fallback. ... Every spawned agent, including the reviewers, the triager, and the explorers, runs under the resolved isolation tier ...". The first two sentences single out "writer agents" / "a writer"; the last sentence generalises. Confirmed verbatim.
  - `pack/isolation-guidance.md:3`: "the orchestrator runs writer agents under container isolation". `:30`: "Run a writer under isolation ... spawn the writer against it". Confirmed writer-centric.
  - The retired flat contradiction (round-1: "Read-only agents ... need no isolation and run without a container or a worktree of their own") is GONE from line 37; grep for the carve-out phrasing across the shipped tree returns zero hits (consistent with Reviewer C's residual-sweep).
- Reasoning: the round-1 triage's Finding 1 fix direction was "rewrite `pack/isolation-guidance.md:37` (and soften the writer-only framing at 30/37) to the uniform rule." Only the carve-out sentence was rewritten; the "prefer the `--git` sandbox for writer agents" / "whether a writer will run" clauses at 37 and the writer framing at 3/30 were left. So this is a valid residual of the change's own goal and, specifically, an incomplete part of what round-1 required. It is nonetheless low, not the high that Finding 1 was: the high rating in round 1 rested on the flat contradiction (a shipped file asserting read-only agents need no isolation), which is fully resolved. What remains is soft framing in a module whose heading is "Writer isolation via agent-box and agent-images" and whose commands (`ab new`/`ab spawn`) are writer mechanics, so a writer as the running example is legitimate; the paragraph's own closing sentence now states the universal rule, and the file nowhere claims read-only agents skip isolation. There is no surviving contradiction. A reader wiring a reviewer/triager/explorer spawn could still infer `--git` is a writer-only preference, which is why it is a valid low residual rather than dismissed. Recommendation to the orchestrator: apply the two trivial wording changes ("prefer the `--git` sandbox for writer agents" -> "... for every spawned agent"; "whether a writer will run" -> "whether an agent will run"), completing round-1 F1's parenthetical; optionally soften 3/30 likewise. This is a near-zero-cost completion of a fix round 1 already scoped. Leaving it is defensible (no contradiction, low severity) but finishing it is better; I do not rate it must-fix because the substantive contradiction is gone.

## Finding 3 (Reviewer C low note): writer-centric example framing at `pack/isolation-guidance.md:3,30,37`

- Verdict: DUPLICATE of Finding 2 above (same locus, `pack/isolation-guidance.md:3,30,37`). Ruled there: VALID, low, in scope, acceptable residual with a recommendation to complete the round-1 fix. Not counted as a second finding.

## Finding 4 (Reviewer C out-of-scope note): frozen historical records still carry the old carve-out

- Verdict: VALID-as-observation, OUT OF SCOPE as a required fix. Not a convergence blocker.
- Severity: low.
- In scope: no. These are frozen historical records of a completed step, not the live normative rule.
- Must-fix vs acceptable: ACCEPTABLE (out of scope; must NOT be edited).
- Verification:
  - `docs/plans/agent-scaffold.md:419`: "Read-only agents (reviewers reading, the triager) need no isolation (Principle 2 ...)". This sits inside the `agent-isolation` step detail (`Q-18`, "adopted by the human"), whose Roadmap status is `complete`.
  - `docs/plans/agent-scaffold.steps/agent-isolation.md:5`: identical sidecar text for the same completed step.
  - Both confirmed present at `856ddf1`; both describe the `agent-isolation` step's decision as it stood, not the live `AGENTS.md` rule (which WAS updated).
- Reasoning: this is the same class as round-1 Finding 3 (frozen completed-step historical records), which the round-1 triage ruled valid-as-observation but out of scope, not a blocker. The records describe what the `agent-isolation` step decided at its time (Q-18: read-only agents need no isolation); Q-61 later reversed that policy. Rewriting these records to the new rule would falsify the history of what that step decided, so they should NOT be edited. Reviewer C is correct that this specific pair (`agent-isolation`) was not enumerated in round-1 Finding 3 (which listed the `driver-output-generation` step records and design doc), but it carries the identical class and verdict. Confirmed out of scope, not a blocker.

## Severity summary

- critical: none.
- high: none.
- medium: none.
- low: 2 distinct valid residuals (Finding 1 file-safety baseline framing; Finding 2 == C's low note, residual writer-only framing in `pack/isolation-guidance.md`), plus one out-of-scope historical observation (Finding 4). Reviewer C's low note is a duplicate of Finding 2, not an additional item.

## Round outcome

CLEAN. No must-fix valid findings: all round-2 findings are low, and none is a contradiction, behavioural hole, illegal state, or false durable record. The two round-1 high findings are confirmed fixed, and no new contradiction was introduced.

Advice to the orchestrator (who owns the convergence call): the two low items (Findings 1 and 2) are VALID residuals of this change's own goal, not pre-existing unrelated gaps. Finding 2 is specifically an incomplete part of round-1 F1's already-scoped fix direction ("soften the writer-only framing at 30/37"). Both fixes are trivial, safe, and non-behavioural. I recommend applying them to fully close the change's stated goal (emitted instructions consistently stating uniform isolation) before declaring convergence, but neither blocks convergence on its own, so accepting them as residuals and converging is also defensible. Finding 4 is out of scope and must be left unedited.
