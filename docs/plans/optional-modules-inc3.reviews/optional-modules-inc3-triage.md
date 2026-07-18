# Triage: optional-modules increment 3 (isolation module)

Triager verdicts on the two reviewers' findings for branch `impl/inc3-isolation` (tip `a644926`, merge-base `b4b7051`). Read-only adjudication; no fixes applied here.

Inputs adjudicated:
- opus (correctness/completeness): F1 preflight cue omitted [low], F2 terminology overlap [low].
- sonnet (docs-quality): F-1 preflight cue omitted [medium], F-2 workspace_dir path diverges [low], F-3 "committed files" vs "committed/tracked" [low], F-4 Docker omitted [low].

Ground truth used: `docs/plans/agent-scaffold.md:530` (the human-decided `Q-41` file shape, controlling spec), `AGENTS.md:79-98` (Writer isolation rule + Preflight rule), `/home/jessea/Documents/projects/agent-images/README.md`, and the rendered partial `pack/isolation-guidance.md`.

---

## V1. Preflight container-tier detection cue omitted (dedup of opus F1 + sonnet F-1)

Verdict: VALID. Final severity: LOW. Must-fix: YES.

Dedup: opus F1 and sonnet F-1 are the same finding at different severities (opus low, sonnet medium). Setting absolute severity myself: LOW. Sonnet's medium overstates the impact. The concrete cue (`which ab` on PATH plus the configured agent-images image present) is small and near-obvious, and AGENTS.md:98 already directs the orchestrator to "detect and state the writer-isolation modes and tools actually available," so no reader is misled or blocked; the harm if left unfixed is confined to a missing spec element, not a functional or accuracy defect. Opus's low is the correct absolute rating.

Why still must-fix despite low severity: the controlling spec (`agent-scaffold.md:530`, human-decided 2026-07-18) explicitly enumerates the elements the partial "supplies" and lists "a preflight container-tier detection cue (`which ab` + the configured image present)" as one of them. The partial's only preflight sentence (`pack/isolation-guidance.md:35`) states the resolved tier but gives no detection cue. This is a genuine deviation from a human-decided spec, not an acceptable deferral: the AGENTS.md Preflight rule owns detection generically and defers the tier policy to the Writer-isolation rule "rather than re-defining it," so the concrete per-tool container cue was intended to live in the module's own guidance (this partial), which is exactly where it is missing. Must-fix status is orthogonal to severity: the deviation is trivially closable and the spec is controlling.

What a fix does: append one short clause to the preflight sentence giving the concrete container-tier detection cue (the presence of `ab`/agent-box on PATH plus the configured agent-images image being loaded), so the partial supplies the specced cue rather than leaning entirely on the general AGENTS.md rule. One sentence, no structural change.

## V2. Terminology overlap: agent-box `--git` "worktree mode" vs tier-2 "worktree isolation" (opus F2)

Verdict: VALID. Final severity: LOW. Must-fix: NO (optional polish).

The partial (`isolation-guidance.md:28,31-33`) calls `ab ... --git` "the sandboxed worktree mode," which is agent-box's own upstream term (README section "Worktree Mode (Sandboxed)", README:157). The residual risk is that a reader conflates it with AGENTS.md tier-2 "Worktree isolation" (`AGENTS.md:82`), a bare git worktree with no container, when agent-box `--git` is actually still tier-1 container isolation that additionally uses a worktree for file visibility. Paragraph 1 of the partial (line 3) keeps the container and worktree tiers distinct, so this is a mild clarity risk, not a contradiction, and the wording is faithful to upstream. A one-clause disambiguation would improve it, but the current text is not inaccurate, so this does not block convergence.

## V3. Inline `workspace_dir` path diverges from upstream example (sonnet F-2)

Verdict: VALID. Final severity: LOW. Must-fix: NO (optional cosmetic alignment).

The inline example shows `workspace_dir = "~/agent-box/workspaces"` (`isolation-guidance.md:17`); the README example uses `~/.local/agent-box/workspaces` (README:136). This is not inaccurate or misleading: `workspace_dir` is user-chosen config, neither path is an enforced default, and the partial explicitly labels its snippet illustrative and points at the README as canonical (line 26). Aligning the inline example to the upstream XDG-conventional path is a free consistency win the implementer may batch in, but because the current value is a legitimate user path and the partial already defers to canonical, it is not a defect that must be fixed.

## V4. "committed files" vs "committed/tracked files" (sonnet F-3)

Verdict: VALID. Final severity: LOW. Must-fix: NO (optional precision alignment). Reviewer rationale partly corrected.

The partial says a writer "sees only committed files" (`isolation-guidance.md:28`); the authoritative README says "committed/tracked files" (README:159). Aligning to the ground-truth wording is a cheap precision improvement, so the finding is valid at low. However, sonnet's stated rationale, that "committed" wrongly implies staged-but-uncommitted changes are hidden, is itself imprecise: `ab new --git` creates a fresh git worktree checked out to the committed state, so staged/uncommitted changes in the main tree are not carried into the worktree anyway. The practical visibility boundary the reader cares about (gitignored/untracked files like `result` are not visible) is already correctly conveyed. So this is a wording-fidelity nit, not a correctness error, and does not block convergence.

## V5. Docker omitted as a valid backend (sonnet F-4)

Verdict: VALID. Final severity: LOW. Must-fix: NO (optional self-containment improvement).

The README lists "Podman or Docker" as the runtime requirement (README:65) and shows `docker load` as an inline alternative (README:89). The partial names only podman (`isolation-guidance.md:5,9,21`). This is not inaccurate: the partial mirrors upstream's own podman-forward default (README Quick Start and `backend = "podman"` config example both lead with podman), and it defers to the canonical READMEs. A one-word parenthetical "(or docker)" on the load step and the backend value would improve self-containment per Principle 20, but a Docker user is not misled given the deference link, so this is optional.

---

## Roll-up

- Findings after dedup: 5 (the two reviewers' top finding merged into V1).
- Valid: 5 of 5. Invalid: 0.
- Severities (final, absolute): all LOW. Sonnet's F-1 medium was corrected down to low on dedup; opus's low ratings confirmed; the three remaining sonnet lows confirmed at low.
- No high/critical findings, and none elevated, so no second-triager backstop applies.
- Must-fix before convergence: 1 (V1, the preflight container-tier detection cue). It is a genuine deviation from the human-decided controlling spec (`agent-scaffold.md:530`) that the AGENTS.md Preflight rule does not fully cover, and it is closable with one clause.
- The other 4 (V2-V5) are optional low polish: the guidance is not inaccurate or misleading against the agent-images README ground truth, and each defers to the canonical upstream docs. The implementer may batch the cheap alignments (V3 path, V4 wording, V5 docker parenthetical, V2 one-clause disambiguation) into the same edit, but none of them block convergence.

Recommendation: this low-risk artifact needs one fix round for V1 only; V2-V5 are optional and may be folded into that round. After the cue is added, the normal one-clean-round convergence rule for a low-risk artifact is satisfied.
