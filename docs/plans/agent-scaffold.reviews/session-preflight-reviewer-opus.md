# session-preflight review (reviewer: opus, lens: correctness + consistency)

Branch `impl/session-preflight` (`6eadf66`), diff `1d76f1e..HEAD`. Read against the worktree at `.claude/worktrees/session-preflight`.

## Mechanical checks (all pass)

- `just scaffold-self` then `git status --short`: empty. The rendered `AGENTS.md` and `.agents/AGENTS.reference.md` are in sync with the `pack/` sources; the three prompt/user-prompt edits also render clean.
- `just test`: 95 passed, 0 failed.
- `just clippy`: clean, no warnings.
- Rendered copies match sources: `AGENTS.md` == `pack/AGENTS.md` and `.agents/AGENTS.reference.md` == `pack/AGENTS.md` for the three added paragraphs (identical text in the diff).

## Findings

### Finding 1 (medium): Two of the three commits add content beyond the `session-preflight` step's stated scope; the plan step detail was not updated to cover them.

Evidence: the plan step detail (`docs/plans/agent-scaffold.md:588`) scopes this step precisely: "Build as a pack-doc change: a preflight section in `pack/AGENTS.md`, a pointer from `pack/user-prompts/kickoff.md` and `pack/user-prompts/resume.md`, and the orchestrator's duty in `pack/prompts/orchestrator.md`." That is exactly commit `bedefb3` (preflight section + two pointers + orchestrator duty).

The branch adds two more commits that the step detail does not describe:
- `46940e7` adds the "Worktree lifecycle and merge-back" paragraph (`pack/AGENTS.md:83`).
- `6eadf66` adds the "authoring-versus-integration" merge-distinction paragraph (`pack/AGENTS.md:85`).

Neither paragraph is authorized by any Roadmap step. The nearest related step, `agent-isolation` (`docs/plans/agent-scaffold.md:434`, status `complete` at line 137), defines the tiering rule but says nothing about worktree lifecycle, merge-back, or conflict-resolution routing; its outcome note (line 432 region) does not cover it. So this is new normative workflow content folded silently into `session-preflight` without the plan recording it. The step's own detail still describes only the preflight, so the plan documentation and the shipped work diverge. There is a genuine gap being filled (post-isolation the workflow never said what happens to the worktree), but per the project's own "structure over disposition" discipline the fill should be a recorded step (or an amendment to `agent-isolation` / a new step) rather than an undocumented rider. Recommend either updating the `session-preflight` step detail to state that it also adds the worktree-lifecycle and merge-back model, or splitting those two paragraphs into their own recorded step.

### Finding 2 (low): "worktree-first" standing-directive phrasing sits in mild tension with the container-first tier order it points at.

Evidence: the preflight paragraph (`pack/AGENTS.md:94`) and `pack/prompts/orchestrator.md:13` both phrase the standing directive as "writer work always runs in a separate, isolated (worktree-first) agent". The Writer isolation rule it references (`pack/AGENTS.md:75-79`) orders the tiers container-first (tier 1 container, tier 2 worktree, tier 3 fallback). A reader who hits "worktree-first" next to a container-first tier order can read a contradiction. The text is internally saved because the same sentence resolves the order to "container via agent-box / agent-images if wired, else a git worktree, else the file-safety fallback", i.e. worktree is the realistic top tier only because containers are not currently wired (`Q-33`, still exploring, `docs/plans/agent-scaffold.md:112`). The phrasing also faithfully mirrors the plan's own language (`docs/plans/agent-scaffold.md:588` uses "worktree-first"), so it is defensible. Flagging as low because it is a wording tension, not a logic error; consider "isolated (worktree-first where containers are not wired)" or similar if clarity matters.

### Finding 3 (low): The clean-merge path does not say what happens if post-merge tests fail.

Evidence: the merge paragraph (`pack/AGENTS.md:85`) makes the clean merge the orchestrator's direct integration job and requires it check "the tests pass on the main tree after the merge". This is the safeguard against a conflict-free-but-semantically-broken merge. But the paragraph does not state the fallback when that post-merge test check fails (a semantic conflict passed the textual merge). Presumably it routes back to an isolated implementer like authored conflict resolution, but the text leaves it implicit. Small logical gap; low severity because the check itself is present and the routing is inferable.

## Coherence checks that passed (no finding)

- Preflight's three actions (restate disciplines / detect-and-state isolation tier / confirm with human) match the plan step (`docs/plans/agent-scaffold.md:588`) and the two pointers. The disciplines it restates (review loop and convergence, separate-triager rule, findings and exploration files, file-safety rules, checkpoint and queue-push cadence) all resolve to existing sections (Convergence `:47`, triager-independence, findings-files/design-explorations, File safety `:67`, Checkpoints/Checkpoint-and-resuming). No dangling reference.
- Pointers are thin and consistent. `kickoff.md` and `resume.md` both list the same three actions in the same order as AGENTS.md and defer the substance to it ("the preflight defined in `AGENTS.md`" / "defined there"); neither restates the tiers. `orchestrator.md` restates a little more (the standing directive), which is normal for a role prompt and it also cross-references "the Preflight rule in `AGENTS.md`".
- Triggers are consistent across all four files: before the workflow starts (kickoff) and on resume after a compaction or a pause (resume). `resume.md`'s header wording "after a compaction or a lost session" is the same intent as AGENTS.md's "after a compaction or a pause".
- The authoring-vs-integration merge model is stated correctly: clean merge (fast-forward or conflict-free replay) = orchestrator integration, same category as committing the plan and the ledger, checked for mechanical correctness only; authored conflict resolution = isolated implementer rebases and resolves in the worktree, reviewed via the normal review loop, then orchestrator fast-forwards. The rebase-then-fast-forward sequencing is internally consistent (a rebased branch is fast-forwardable). No logical hole in the core distinction.
- The convergence counts restated in the worktree-lifecycle paragraph ("one clean round for a low-risk artifact, two for a risky one") carry an explicit "see Convergence above" cross-reference, so this is a pointer, not a competing definition. Not a double-definition.
- `validate --workflow` is referenced as "once built" / "the backstop", consistent with `workflow-invariants` being not-started in the plan. Acceptance review, human-input contract, and separate-triager references all resolve to existing defined sections.
- Step is still `next` (not prematurely marked complete): `docs/plans/agent-scaffold.md:157` and the step detail's leading "Next." at line 588.

## Severity summary

- Critical: none.
- High: none.
- Medium: 1 (Finding 1, scope creep / plan step detail not updated for the worktree-lifecycle and merge-back paragraphs).
- Low: 2 (Findings 2 and 3).
