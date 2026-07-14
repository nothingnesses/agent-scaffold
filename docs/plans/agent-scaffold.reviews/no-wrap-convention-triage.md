# Triage: no-wrap-convention (Q-22), round 1

Triager: separate agent, independent of both the producer/orchestrator (per the always-separate rule). Commit range: `30dc348..b4d8b24`. Reflow: `dc5f69b`. Convention-add: `4fa5e74`. Blockquote-safety reword: `81aa4bd`. Reviewers adjudicated: opus (reflow-integrity lens, reported CLEAN) and sonnet (convention/tooling lens, two low findings). Artifact classification (recorded at loop open, ledger): RISKY / high-blast-radius -> two consecutive clean rounds required to converge.

Principle citations below refer to the plan's own Project Principles 1-7 (plan lines 18-24), not the 22 shipped principles. Sonnet's "Principle 1 (clean architecture)" and "Principle 2 (minimal)" already map correctly onto the plan's Principle 1 (prefer the cleaner long-term architecture) and Principle 2 (minimal by default), so no citation normalisation was needed.

## Opus CLEAN verdict: upheld

I independently spot-checked opus's evidence rather than taking it on trust:

- Word-diff `30dc348..dc5f69b` over the pack markdown: only `pack/AGENTS.md`, `pack/prompts/reviewer.md`, and `pack/prompts/triager.md` show an added token block (one each, the intentional convention text); `orchestrator.md`, `planner.md`, `implementer.md`, `plan-template.md` show zero added tokens. Matches opus point 1.
- Structural-marker counts (headings, fences, blockquotes, list/table markers) are identical across the reflow commit for the shipped prompts (`reviewer.md` 1==1, `triager.md` 1==1, `AGENTS.md` 27==27). Matches opus point 4.
- Generated-copy consistency at the step tip: all seven `.agents/prompts/*.md`, all three `.agents/user-prompts/*.md`, and `docs/plans/TEMPLATE.md` match their `pack/` sources byte-for-byte; root `AGENTS.md` equals `.agents/AGENTS.reference.md`; the reference differs from `pack/AGENTS.md` only by the `{{principles}}` placeholder expansion. Matches opus point 7.

The CLEAN verdict is sound. The reflow changed wrapping only; no shipped prose meaning was altered and no markdown structure was broken. I did not re-run `nix fmt` idempotency (the implementer and plan record it as idempotent, incl. the one-time `--no-cache` cache-artifact note); that is a formatter-run claim, not a diff claim, and is out of the reflow-integrity scope I re-checked.

## Findings

### F1 - scaffold-self runs a whole-repo formatter, not scoped to generated files

Verdict: VALID. Severity: low (confirmed; sonnet rated low).

Reasoning. The observation is factually correct: `justfile` line 42 runs `nix fmt` (treefmt over the whole repo), while the recipe's name and purpose are to regenerate the project's own scaffolded assets. The recipe comment compounds this: it says "so format the output afterwards," which reads as formatting the generated render, but the command actually formats every markdown file in the repo. That is a real coherence/accuracy defect (plan Principle 1: internal coherence; the comment does not say what the command does), and it is a done-but-not-decided item: the Q-22 decision text authorised the orchestrator's one-time repo-wide reflow as a separate mechanical commit, not a permanent whole-repo formatter in the dogfooding recipe.

Why the impact is low, not higher. treefmt is idempotent, so in steady state (repo already formatter-clean) the whole-repo run produces zero collateral diff; the only dirty files are the ones the render just produced, which is exactly what a developer running `scaffold-self` wants formatted. A collateral reformat can only appear if the tree already holds unrelated formatter-dirty markdown, which is visible in the diff before commit and fully reversible.

Recommended fix (accept-with-rationale; do NOT rescope). Keep the whole-repo `nix fmt` and record that it is intentional, correcting the misleading comment: state that `scaffold-self` runs the repo-wide formatter (relying on treefmt idempotency, so the steady-state collateral diff is zero) rather than implying it formats only the render output. A one-line plan note that the permanent `nix fmt` in `scaffold-self` is the intended fixed-point step (superseding the "one-time only" reading of Q-22) closes the done-but-not-decided gap.

I explicitly reject sonnet's alternative fix of scoping the format step to the generated dest paths: that would duplicate the manifest's dest-path list (`pack/pack.toml`) inside the justfile, creating a second source of truth that drifts when an asset is added or moved. That is worse under plan Principle 1 (cleaner long-term architecture / one source of truth), and treefmt has no native per-file scoping that avoids the duplication. Whole-repo tree-hygiene formatting is already the orchestrator's remit under AGENTS.md ("leaves incidental reformatting to the orchestrator"); the "format only your own files" rule binds implementer sub-agents, not a developer-run maintenance recipe, so a repo-maintenance recipe doing whole-repo `nix fmt` is consistent with the workflow model, not a violation of it.

### F2 - scaffold-self behaviour change landed under a `style:` prefix

Verdict: VALID. Severity: low (confirmed; sonnet rated low).

Reasoning. The convention mismatch is real: `style:` is conventionally formatting-only, and adding `nix fmt` to a justfile recipe is a behavioural change that fits `chore`/`build`/`feat`. It is a low-value nit, and three mitigations cut against acting on it: (a) the commit subject itself names the behavioural change ("...; scaffold-self formats output"), so a reader or bisecter scanning subjects sees it, weakening sonnet's "a bisect would overlook it" argument; (b) the commit body spells out the recipe change; (c) it is an already-made, unpushed, local commit, and rewriting local history for a prefix label is churn out of proportion to the benefit.

Recommended resolution (accept the residual risk; do not rewrite history). Record it as an accepted low-severity convention nit. Per the convergence rule, a consciously accepted residual risk does not block convergence. If these commits are reworked before pushing for any independent reason, prefer splitting the justfile change into its own `chore:`/`build:` commit, or reprefixing; do not rewrite otherwise-settled local history solely for this.

## Round outcome

NEW VALID FINDINGS (round not clean). F1 is a valid low finding requiring a small action (correct the recipe comment / add the one-line plan note; the code change itself stays). F2 is a valid low finding resolved by accepting its residual risk, which does not itself block convergence, but the round is non-clean regardless because F1 is unaddressed.

No finding at high or critical severity was raised or dismissed, so the dismissed-high/critical backstop re-check does not gate this round.

Convergence state: RISKY artifact, two consecutive clean rounds required. This round is new-valid, so the consecutive-clean streak is 0 of 2. After F1 is addressed (and F2's acceptance recorded in the ledger), a fresh review round is needed; two consecutive clean rounds are still required to converge.
