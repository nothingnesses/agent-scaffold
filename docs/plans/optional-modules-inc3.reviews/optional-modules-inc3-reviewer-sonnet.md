# Review: optional-modules increment 3 (isolation module)

Reviewer: sonnet (documentation quality, accuracy, consistency lens)
Branch: `impl/inc3-isolation`, commit `a644926`, diff `main..impl/inc3-isolation`
Reviewed files: `pack/isolation-guidance.md`, `pack/pack.toml`, `src/manifest.rs`, `CHANGELOG.md`
Ground-truth sources: `/home/jessea/Documents/projects/agent-images/README.md`, `AGENTS.md` (worktree), `docs/plans/agent-scaffold.md` (spec), `docs/plans/optional-modules-inc3.explorations/isolation-module-shape-A.md` (design record)

---

## Findings

### F-1: Preflight detection cue omitted (MEDIUM)

`pack/isolation-guidance.md:35`

The master plan spec (`docs/plans/agent-scaffold.md:530`) lists "a preflight container-tier detection cue (`which ab` + the configured image present)" as a required element of the guidance partial. The guidance partial does not include it. It tells the reader that "the orchestrator states the resolved isolation tier at preflight" but gives no detection mechanism for the container tier. The `AGENTS.md` preflight rule (line 98) tells the orchestrator to detect "what is present" but does not say how to detect whether agent-box is wired; the isolation module's guidance was the intended concrete answer. Without it, an orchestrator following the guidance can state the tier but has no spec-mandated way to determine whether the container tier is available for this project.

The design exploration summary (shape-A.md:101) condenses this to "one line noting the orchestrator states the resolved tier at preflight," which the guidance does satisfy. But the plan document (agent-scaffold.md:530) is the controlling spec and it names the cue explicitly. The implemented line covers the condensed form but not the controlling form.

Impact if unfixed: orchestrators following the rendered `AGENTS.md` have no concrete detection instruction for the container tier from within the module's own guidance, relying entirely on the general preflight rule to infer the method.

---

### F-2: Inline config example `workspace_dir` path diverges from canonical upstream (LOW)

`pack/isolation-guidance.md:17`

The inline example shows `workspace_dir = "~/agent-box/workspaces"`. The canonical example in `agent-images/README.md:136` is `workspace_dir = "~/.local/agent-box/workspaces"` (under `~/.local/`, the XDG-conventional location). The guidance correctly defers to the upstream README as canonical ("See ... the agent-images README for the canonical, up-to-date example of this config"), so a careful reader will not be misled. However, a reader who copies the inline snippet without following the deference link gets a path that differs from the upstream default.

The spec (shape-A.md:101) calls for a "canonical `~/.agent-box.toml` snippet" pointing at the upstream READMEs. Using a path that doesn't match the upstream canonical snippet contradicts the intent of being grounded in the upstream source.

Impact if unfixed: readers who copy the inline snippet verbatim may end up with a non-standard `workspace_dir` and need to reconcile with upstream docs when they later consult them.

---

### F-3: "committed files" understates what is visible in worktree mode (LOW)

`pack/isolation-guidance.md:28`

The guidance says "use the sandboxed worktree mode so a writer agent sees only committed files." The authoritative description in `agent-images/README.md:159` says "the agent only sees committed/tracked files" and confirms that gitignored files are not visible. "Committed" and "committed/tracked" are meaningfully different: tracked but uncommitted changes in the index are visible in the worktree; untracked and gitignored files are not. The guidance's "committed files" would incorrectly suggest that staged but not yet committed changes are hidden, which is not the expected worktree behavior.

Impact if unfixed: a reader implementing preflight scripting or trying to understand what a sandboxed writer can see may misunderstand the scope of the worktree's visibility.

---

### F-4: Docker omitted as a valid backend (LOW)

`pack/isolation-guidance.md:5`, `pack/isolation-guidance.md:21`

The agent-images README (`README.md:65`) lists Podman or Docker as requirements. The guidance only names podman ("load it into podman", `backend = "podman"` in the example) without acknowledging Docker as a valid alternative. A Docker-only user would have to figure out that the build step (`podman load < result`) can be replaced with `docker load < result` and the backend changed to `"docker"`, both of which are documented upstream but not pointed at.

The guidance defers to the upstream READMEs as canonical, which mitigates this. But given Principle 20 (self-contained enough for a reader without prior context), a single parenthetical noting Docker as an alternative would close the gap.

Impact if unfixed: Docker users who follow the guidance literally see commands and config that only apply to Podman.

---

## Clean confirmations

The following items were checked adversarially and found correct.

**Build matches Q-41 human decision.** The implementation is guidance-only: no `[[asset]]` rows tagged `module = "isolation"`, no `[[var]]` rows, zero files dropped into the project tree. The `pack/pack.toml` entry (`name`, `description`, `guidance`, no `requires`) matches the spec at `docs/plans/agent-scaffold.md:530` and the design record `isolation-module-shape-A.md:76`-`84` exactly.

**No restating or contradicting the Writer isolation rule.** The guidance correctly points at "Writer isolation (capability-tiered)" and "Worktree lifecycle and merge-back" by name and says "above," which is accurate: those sections appear at `AGENTS.md:79`-`89`, well above where `{{modules}}` renders. It supplies only the concrete container-tier wiring, not the tier order or worktree lifecycle, which remain authoritative in `AGENTS.md`.

**Rationale for no scaffolded config.** The guidance correctly explains why the module drops no config into the repo ("a project-tracked copy would sit in the wrong place and never be read"), grounded in the fact that agent-box reads from `~/.agent-box.toml` in the user's home, matching `agent-images/README.md:133`.

**Command syntax accuracy.** `nix build .#<agent>`, `podman load < result`, `localhost/agent-images/<agent>:latest`, `ab new <repo> -s <session> --git`, and `ab spawn -s <session> --git` all match the agent-images README (lines 87-90, 162-166). `ab spawn --local` and its description as mounting the current directory also match `README.md:152`-`155`.

**URLs are correct.** `https://github.com/nothingnesses/agent-images` and `https://github.com/0xferrous/agent-box` are the correct upstream URLs as recorded throughout the spec and design documents.

**No over-claiming.** The guidance does not imply the tool does any container setup, and explicitly says "nothing here is generated or executed on your behalf." The statement "nothing here is required to run the workflow" is accurate: `AGENTS.md:85` says the isolation mechanism "resolves to the file-safety fallback until it is" built.

**No restatement of the worktree lifecycle.** The guidance references the lifecycle the "orchestrator already owns" without duplicating it, consistent with the YAGNI boundary in shape-A.md:113 and the one-source-of-truth principle.

**Consistent tone and structure with `checks-guidance.md`.** Both partials open with a heading, follow with a paragraph placing the module in context, and proceed with named topic paragraphs followed by fenced code blocks. The isolation guidance is shorter because its content is simpler (three setup steps vs. four interlocking workflow roles). The tone (present tense, direct) matches.

**CHANGELOG entry is accurate and correctly placed.** It is under `## [Unreleased]` -> `### Added`, describes what was actually built (guidance-only, no dropped files, no runtime dependency), and does not over-claim. New entries are correctly prepended above the older optional-modules entry within the same section.

**ASCII-clean.** No em-dashes, en-dashes, double-hyphen dashes, emoji, or non-ASCII symbols found in any changed file.

**Test coverage is adequate.** `builtin_isolation_module_renders_its_guidance_only_when_selected` verifies the asset-list identity (no dropped files), the empty `{{modules}}` block when no module is selected, and the presence of the heading and both upstream URLs in the rendered guidance. These are the right invariants for a guidance-only module.

---

## Summary

4 findings: 0 critical, 0 high, 1 medium, 3 low. No convention violations (no em-dashes, no non-ASCII, bullet punctuation is N/A since the partial has no bullet lists). The most important finding is F-1: the controlling spec (`docs/plans/agent-scaffold.md:530`) explicitly required a preflight detection cue (`which ab` + configured image present) that the guidance partial does not include, leaving an orchestrator without the concrete detection instruction for the container tier. The remaining three findings are low-severity accuracy notes against the agent-images README ground-truth. The implementation correctly matches the Q-41 human decision (guidance-only, no dropped files), correctly points at the Writer isolation rule without restating it, and does not contradict AGENTS.md.
