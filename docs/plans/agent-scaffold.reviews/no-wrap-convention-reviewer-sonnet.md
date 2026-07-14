# Review: no-wrap-convention (Q-22)

Reviewer: convention-and-tooling lens (Sonnet) Commit range: `30dc348..b4d8b24` Artifact risk: low (convention doc + config change, fully reversible)

## Conventions assessed

### 1. `pack/AGENTS.md` "Prose formatting" paragraph

Placed at line 84 of `pack/AGENTS.md`, after the "Checkpoint and resuming after context loss" subsection and immediately before `## Principles`. This is the end of the Workflow section, before Principles - the position the plan specifies. The paragraph is coherent with the surrounding text (it closes out the workflow-instruction block). "Line length is never a review finding" appears exactly once in this file. No issues.

### 2. Reviewer and triager statement consistency

- `pack/prompts/reviewer.md` line 11: "Line length and prose line-wrapping are never findings: the project does not hard-wrap prose and a formatter owns wrapping, so do not raise or comment on them."
- `pack/prompts/triager.md` line 5 (inline): "A finding about line length or prose line-wrapping is never valid: the project does not hard-wrap prose and a formatter owns wrapping, so dismiss any such finding."
- `pack/AGENTS.md` line 84: "Line length is never a review finding, so reviewers and triagers do not raise or act on it."

All three state the same prohibition with role-appropriate framing: the reviewer will not raise it; the triager will dismiss it; AGENTS.md is the master statement. Each is stated once in its document. No contradictions. The reviewer adds "or comment on them" (a reasonable role-specific tightening not present in AGENTS.md), which is not a contradiction.

### 3. `.prettierrc.json` placement

File exists at repo root (`/home/jessea/Documents/projects/agent-scaffold/.prettierrc.json`). It is not under `pack/`, not listed in `pack/pack.toml` as an asset, and not referenced in `src/manifest.rs`. Dev-tooling placement is correct; it will not ship to scaffolded projects.

---

## Findings

### F1 - `scaffold-self` runs a repo-wide formatter rather than scoping to generated files

**Severity: low**

`justfile` line 42 (`{{ direnv_prefix }} nix fmt`) runs treefmt on the entire repository every time a developer runs `just scaffold-self`. The recipe's stated purpose is to "Regenerate the project's own scaffolded assets" (`AGENTS.md` and `.agents/`). The formatter step will also reformat the plan, ledger, README, CHANGELOG, and any other markdown that happens to be formatter-dirty at the time - producing a diff that mixes scaffold regeneration with collateral reformat of unrelated files.

Evidence: `dc5f69b` adds `nix fmt` to scaffold-self; `nix fmt` invokes treefmt, which runs prettier over all `*.md` in the repo, not only the files cargo run generated.

Against Principle 2 (minimal): the recipe does more than its name and purpose advertise. Against Principle 1 (clean architecture): a developer expecting a focused scaffold diff gets a wider diff that is harder to review in isolation.

Mitigating factors: the formatter is idempotent, the commit history shows nix fmt was just run repo-wide to bring everything to a clean state, and in steady state the collateral diff will be zero. The scope mismatch is nonetheless real and is not covered by the Q-22 decision text, which decided "the orchestrator runs the repo-wide nix fmt reflow as a separate mechanical commit" - a one-time act, not a permanent addition to scaffold-self.

Fix candidate: scope the format step to the files cargo run generates (AGENTS.md, the .agents/ tree), or split scaffold-self into two recipes (regenerate and fmt), or document that scaffold-self intentionally formats the whole repo so the scope is at least stated.

### F2 - `scaffold-self` behavior change landed in a style commit

**Severity: low**

The justfile addition (`{{ direnv_prefix }} nix fmt`) is a functional change to recipe behavior. It landed in commit `dc5f69b`, whose subject is "style: reflow all markdown to proseWrap=never; scaffold-self formats output". The conventional commit `style` prefix is conventionally reserved for formatting-only changes (no behavior change). A justfile recipe that now invokes an additional tool is a behavior change; it fits `feat` or `chore`, not `style`.

Evidence: `git show dc5f69b --stat` lists `justfile` alongside 28 reformatted markdown files. The feature commit `4fa5e74` (feat) added the convention and .prettierrc.json but not the justfile change.

Impact: commit history is slightly misleading; a bisect looking for "what made scaffold-self run nix fmt" would look for a feat or chore commit and could overlook this style commit. Low impact on correctness.

---

## No findings at medium, high, or critical severity.

All decided scope items for `no-wrap-convention` are present: the convention paragraph in `pack/AGENTS.md`, the reviewer and triager rule, `.prettierrc.json` at repo root (not a pack asset), and the one-time repo-wide reflow as a separate mechanical commit (`dc5f69b`). Nothing decided is missing. The only done-but-not-decided item is the permanent addition of `nix fmt` to scaffold-self (captured in F1 above).
