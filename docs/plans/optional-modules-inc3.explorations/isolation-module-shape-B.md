# Q-41: concrete file shape of the isolation module (increment 3), explorer B

Design exploration for Q-41 (`docs/plans/agent-scaffold.md:120`). The question owner is the shared decision queue; this document supplies the design-space survey and a recommendation so the orchestrator can move Q-41 from `exploring` to `open` and put the concrete options to the human.

## 1. The question

`agent-scaffold scaffold --module isolation` must drop a file set that wires a project to agent-box and agent-images for writer-agent container isolation. The question is which files, with what content, ownership, and render flags, and how they connect the three existing pieces: the isolation rule already in `AGENTS.md`, the agent-box/agent-images external projects, and the preflight step.

What is decided and off-limits:

- The isolation module is POINTER-SCAFFOLDING ONLY. It is a content module that drops config and guidance pointing at the external agent-box and agent-images projects. It does not reimplement isolation and does not give the Rust binary a runtime dependency on either project (`docs/plans/agent-scaffold.md:528-530`, `docs/plans/cross-harness-isolation.explorations/q33-q36-A.md:1-6`, `docs/plans/cross-harness-isolation.explorations/q33-q36-B.md:1-6`).
- The deferred growth path (a `.agents/spawn.toml` role->launch-command map plus a container-wrapper shim) is DEFERRED and must not be built now (`docs/plans/agent-scaffold.md:530`).
- The isolation RULE (worktree-first, capability-tiered: container > worktree > file-safety) already ships in `pack/AGENTS.md:79-85`. The module adds wiring, not the rule. The module must point to the rule and must not duplicate it.
- Byte-identical-when-off: with no `--module isolation` selected, the scaffold output is unchanged. All new files are module-tagged assets; the guidance partial renders into `{{modules}}` only when the module is enabled.

## 2. Required-reading facts that ground the options

These are the interface facts I need to be accurate about, with citations.

**The isolation rule (already shipped, `pack/AGENTS.md:79-85`):** Three tiers in preference order: (1) container via agent-box/agent-images, (2) worktree, (3) file-safety discipline. Read-only agents (reviewers, triager, explorers) need no isolation. The module provides the mechanism for tier 1; the rule is the selection policy that applies regardless of whether the module is installed.

**The worktree lifecycle (already shipped, `pack/AGENTS.md:87-89`):** The orchestrator owns the full worktree lifecycle: creates it, runs the implementer there, reviews in it, merges only on convergence, then removes it. This is the tier-2 path and is already fully described. The isolation module must not restate it.

**Preflight (already shipped, `pack/AGENTS.md:98`):** The orchestrator runs preflight at kickoff and resume, detecting and stating the resolved isolation tier: "container via agent-box / agent-images if wired, else a git worktree, else the file-safety fallback." The module's guidance should tell the orchestrator exactly HOW to do this detection for the container tier, since the core text names what to look for but not how to verify it.

**The `{{modules}}` slot and guidance partial mechanism (`pack/pack.toml:11-14`, `src/manifest.rs:74-101`):** A `[[module]]` entry's `guidance` field names a partial filename in the pack. When the module is enabled, that partial is concatenated into the `{{modules}}` slot at the tail of `pack/AGENTS.md:110`. The partial is read by the tool and rendered into AGENTS.md; it is not an asset dropped as its own file. The `ModuleSpec` schema is `{ name, description, guidance: Option<String>, requires: Vec<String> }`.

**The checks module as the template (`pack/pack.toml:110-157`, `pack/checks-guidance.md`):** The checks module drops five assets: `.agents/checks.toml` (working, verbatim), two ast-grep files (working, verbatim), `.agents/prompts/checks-reviewer.md` (reference, verbatim), and `.agents/hooks/pre-commit` (reference, verbatim, executable). Its guidance partial `pack/checks-guidance.md` is the text rendered into `{{modules}}` when `--module checks` is selected. Working assets are create-if-absent; reference assets are always refreshed. The module has no `requires`.

**agent-images interface (`/home/jessea/Documents/projects/agent-images/README.md:82-167`):** Images are built with `nix build .#<agent>` (e.g. `nix build .#claude-code`), loaded with `podman load < result`. Each image runs as a non-root `agent` user (uid 1000) with `/workspace` as the working directory. Used with agent-box via a global `~/.agent-box.toml` (`workspace_dir`, `base_repo_dir`, `runtime.backend`, `runtime.image`, `runtime.env_passthrough`). Local mode: `ab spawn --local`. Worktree mode (sandboxed, recommended for writer agents): `ab new <repo> -s <session> --git` then `ab spawn -s <session> --git`. There is no per-project agent-box config file; configuration is global per machine.

**The two tiers and their relationship to this module:** The worktree tier is already active wherever the harness supports it (Claude Code's Agent tool's `isolation: "worktree"` option, or a manual `git worktree add` path the orchestrator uses). The container tier requires the user to install agent-box, build and load an agent-images image, and configure `~/.agent-box.toml`. The module's job is to scaffold the guidance and project-local record that makes this setup repeatable and discoverable.

## 3. Design space

Three options, each traced against the user journey: the user runs `agent-scaffold scaffold --module isolation`, then needs to reach a working sandboxed writer agent.

### Option A: Guidance partial only (minimum viable pointer)

The module drops exactly one logical artifact: a guidance partial (`pack/isolation-guidance.md`) that renders into the `{{modules}}` section of AGENTS.md when the module is enabled. No additional files are dropped into the project.

User journey: the user runs the scaffold command, reads the new section in AGENTS.md, follows the prose steps (install agent-box, build and load an image, configure `~/.agent-box.toml`), and at the next session start the orchestrator's preflight detects `ab` on PATH and states the resolved tier. There is no project-local artifact; setup knowledge lives in AGENTS.md's generated section or in each developer's head.

### Option B: Guidance partial plus a project-local pointer doc (working file)

The module drops the guidance partial (as in A) and also a working file `.agents/isolation/README.md` (create-if-absent, verbatim, not rendered). The README is a template the user fills in to record which image the project uses, the `~/.agent-box.toml` snippet for that image, and any project-specific notes (extra env vars, NixOS steps). All setup steps are also in the guidance partial; the README is the project's durable record.

User journey: same as A for the first run. Additionally, the user fills in `.agents/isolation/README.md` with the project's chosen image name and setup notes. New contributors clone the project, read the README, follow its recorded steps, and reach the same container tier without having to rediscover the image choice. The orchestrator at preflight can reference the README explicitly: "check `.agents/isolation/README.md` for the project's recorded image."

### Option C: Guidance partial plus pointer doc plus a Nix snippet (additional working file)

Everything in B, plus an additional working file `.agents/isolation/flake-input.nix.example` (create-if-absent, verbatim) showing the boilerplate for adding agent-images as a flake input and building a custom image with `mkAgentImage`. This targets users who want a project-specific or customized image rather than a pre-built one.

User journey: as B, with an additional file giving Nix users the exact boilerplate to add to their `flake.nix`. Reduces copy-paste research for the custom-image path.

## 4. Trade-offs per option

**Against the Project Principles (from `docs/plans/agent-scaffold.md:18-26`):**

Principle 1 (prefer the cleaner long-term architecture) favors B over A: a project that commits to container isolation should have a project-level record of WHICH image it uses, because that knowledge otherwise lives only in `~/.agent-box.toml` on each developer's machine (per-machine, not per-project, per the agent-images interface). Without a project-local file, a new contributor has no in-repository pointer to the setup steps; the guidance in AGENTS.md is generic prose, not project-specific config. B gives the project a stable home for "we use this image" that travels with the repo. A lacks this.

Principle 2 (minimal by default) favors A for the case where the user just wants the guidance without a working file to maintain. It also cautions against Option C: a Nix snippet is specific to one workflow (Nix users building custom images) and would be unused noise for the majority of users who use a pre-built image via `nix build github:nothingnesses/agent-images#claude-code`. The distinction matters: most users run `nix build .#<agent>` from the agent-images repo (or `nix build github:nothingnesses/agent-images#<agent>`), they do not need `mkAgentImage` in their own flake. Option C adds a file most users will leave as an example and never activate.

Principle 3 (safe on existing projects) is satisfied by all options: working files are create-if-absent, reference files are always refreshed (and there are no reference files in any option), and `--module isolation` selected on a project that already has `.agents/isolation/README.md` leaves the existing file intact.

Principle 4 (idempotent) is satisfied by all options: re-running scaffold refreshes nothing (the README is working, the guidance partial is computed into AGENTS.md which is also working). Nothing accumulates on re-runs.

Principle 7 (reproducible) favors B over A, and is neutral on C. The purpose of the working README is precisely reproducibility: any contributor can clone the repo, read the file, and know exactly which image to build, which backend to use, and which env vars to pass through. Option A achieves none of this because the guidance partial in AGENTS.md is generic (it cannot record WHICH image this project chose). Option C adds one more reproducibility artifact, but for the more advanced custom-image path that most projects do not need at first.

**The two-tier teaching:** All three options use the guidance partial to teach the two-tier choice. The partial points to the `agent-isolation` rule for the selection policy, adds ONLY the container tier's concrete wiring (setup steps, detection commands, invocation commands), and notes that the worktree tier is already active via the rule and requires no additional setup. A user who wants only worktree isolation gets nothing wrong from installing this module; the module adds the container tier without removing the worktree fallback.

**Composition with the checks module:** The isolation and checks modules are orthogonal. The checks module uses worktrees internally for check execution (`agent-scaffold checks` creates a temporary worktree per Q-39), but that is the checks RUNNER's internal mechanism, not the writer-agent isolation the isolation module provides. There is no dependency in either direction. The isolation module should NOT declare `requires = ["checks"]`; a user who wants container isolation for writer agents but not the checks gate should be able to enable isolation alone.

**Composition with the `agent-isolation` rule:** The rule (`pack/AGENTS.md:85`) already says "the isolation mechanism... is an optional module; this rule is the always-applicable selection policy and holds whether or not that module is built, resolving to the file-safety fallback until it is." The isolation module is the mechanism the rule anticipates. The module's guidance must point back to this rule explicitly rather than restating the tier order, so there is one source of truth for the policy (`pack/AGENTS.md:79-85`) and the module carries only the concrete wiring for tier 1.

## 5. Recommendation: Option B

Guidance partial plus one project-local working file.

Reasoning: A produces a generic guidance section in AGENTS.md but no project-level record of which image was chosen, making the setup non-reproducible across contributors (violates Principle 7) and giving the orchestrator no project-specific source to reference at preflight. B adds exactly one working file that fixes this without adding complexity the user does not need immediately. C's Nix-snippet file adds complexity for the uncommon custom-image case and is a Principle 2 (minimal) violation at this stage; it belongs as a future addition if users ask for it.

The key insight: the checks module's `.agents/checks.toml` earns its place as a working file because the tool reads it at runtime and the user edits it to configure their specific checks. The isolation module cannot claim that level of tool integration (no `agent-scaffold isolation` subcommand exists or is planned). But `.agents/isolation/README.md` earns its place on a weaker but legitimate ground: the project-specific image choice (e.g. `claude-code` vs `codex`) is not captured anywhere else in the repository, and without it setup is tribal knowledge. Working file; create-if-absent; user-owned.

### 5a. `[[module]]` entry for `pack/pack.toml`

```toml
[[module]]
name = "isolation"
description = "Pointer setup for container-based writer-agent isolation via agent-box and agent-images. Complements the built-in worktree tier with a higher-isolation container option that sandboxes the filesystem, environment, and process."
guidance = "isolation-guidance.md"
```

No `requires` field: isolation is independent of the checks module.

### 5b. Asset table

| Source (in pack/) | Destination (in project) | Ownership | Render | Executable | Notes | | isolation/README.md | .agents/isolation/README.md | working | false | false | Project-local setup record; user fills in image name and machine config |

One asset, one working file. The guidance partial (`isolation-guidance.md`) is not a dropped asset; it is read by the tool and rendered into `{{modules}}` in AGENTS.md, the same mechanism as `checks-guidance.md` (referenced at `pack/pack.toml:13`, `src/manifest.rs:82-90`).

In `pack/pack.toml`, the asset entry goes after the checks module assets:

```toml
# The isolation module (opt in with --module isolation). Its single asset is
# a user working file: a project-local setup record the user fills in to
# record which image this project uses, making container isolation reproducible
# across contributors (see .agents/isolation/README.md for the schema).
[[asset]]
source = "isolation/README.md"
dest = ".agents/isolation/README.md"
ownership = "working"
module = "isolation"
```

### 5c. Content sketch: `pack/isolation-guidance.md`

This file is never dropped directly. The tool reads it and renders it into the `{{modules}}` slot when `--module isolation` is enabled, following the same mechanism as `pack/checks-guidance.md`.

```markdown
## Writer-agent isolation: container tier setup

The isolation rule (see "Writer isolation (capability-tiered)" above) governs which tier to use at each session: container > worktree > file-safety. The worktree tier is already active wherever the harness supports it (the orchestrator creates and manages worktrees as described under "Worktree lifecycle and merge-back"). This section adds the concrete wiring for the container tier via agent-box and agent-images; once set it up, the preflight step resolves to container automatically.

No extra action is needed for the worktree tier: skip to "Preflight detection" if you are using worktrees only.

### Container tier: what it provides

Container isolation (agent-box plus agent-images) isolates the filesystem, the environment, and applies a process-level security sandbox, more than a git worktree alone. It is the preferred tier per the isolation rule when available. Gitignored files (for example build artifacts, credentials) are not visible to the agent in worktree mode.

### Container tier: setup (one time per machine)

1. Install agent-box: see https://github.com/0xferrous/agent-box.
2. Choose an agent image from https://github.com/nothingnesses/agent-images (for example `claude-code`). Build it with Nix (requires Nix with flakes enabled and Podman or Docker):
```

nix build github:nothingnesses/agent-images#<agent> podman load < result

````
3. Configure `~/.agent-box.toml` (global, per machine):
```toml
workspace_dir = "~/.local/agent-box/workspaces"
base_repo_dir = "~/path/to/your/projects"
[runtime]
backend = "podman"
image = "localhost/agent-images/<agent>:latest"
env_passthrough = ["ANTHROPIC_API_KEY"]
````

Replace `<agent>` with the image name you chose and add any other API keys your agent needs to `env_passthrough`. 4. Record the image name and any project-specific notes in `.agents/isolation/README.md` (created by this module) so contributors can reproduce the setup.

### Running a writer agent under container isolation

Worktree mode (recommended: agent sees committed and tracked files only, not gitignored files):

```
ab new <repo-name> -s <session-name> --git
ab spawn -s <session-name> --git
```

Where `<repo-name>` is the repository directory name under `base_repo_dir` and `<session-name>` is a name you choose for the work session.

### Preflight detection

At each session preflight (see the Preflight section above), the orchestrator should check for container tier availability as follows:

- Run `which ab` to detect agent-box.
- If present, run `podman images` (or `docker images`) and check that the configured image appears.
- If both checks pass, state: "Container tier available via agent-box (image: <configured image>); using container isolation for writer agents."
- If either check fails, state: "Container tier not wired; falling back to worktree isolation" and continue with the worktree tier per the isolation rule.
- Reference `.agents/isolation/README.md` for the project's recorded image name and any project-specific notes.

````

### 5d. Content sketch: `pack/isolation/README.md` (dropped as `.agents/isolation/README.md`, working, verbatim)

This is a template the user fills in. It is a create-if-absent working file; re-scaffolding never overwrites it.

```markdown
# Container isolation setup for this project

This file records the container isolation configuration for this project so any contributor can reproduce the setup. Fill in the sections below after completing the one-time machine setup described in the isolation section of `AGENTS.md`.

References:
- agent-box: https://github.com/0xferrous/agent-box
- agent-images: https://github.com/nothingnesses/agent-images

## Image in use

Image name: <fill in, e.g. claude-code>
Build command: nix build github:nothingnesses/agent-images#<image-name>

## Machine setup summary

After building and loading the image, configure ~/.agent-box.toml on each machine:

```toml
workspace_dir = "~/.local/agent-box/workspaces"
base_repo_dir = "~/path/to/your/projects"
[runtime]
backend = "podman"
image = "localhost/agent-images/<image-name>:latest"
env_passthrough = ["ANTHROPIC_API_KEY"]
````

Add any other API keys or environment variables this project's agent needs.

## Project-specific notes

<!-- Record any project-specific setup steps here, e.g. extra env vars,
NixOS rootless Podman steps, or why a specific image was chosen. -->

```

### 5e. Composition with the checks module

No interaction and no dependency. The checks module (`agent-scaffold checks`) creates its own temporary worktrees for check execution (per Q-39). That is internal to the checks runner and independent of where the orchestrator runs writer agents. A project may use both modules independently or together, and neither module auto-enables the other.

## 6. Adoption journey end to end

After running `agent-scaffold scaffold --module isolation`:

1. AGENTS.md gains the "Writer-agent isolation: container tier setup" section in its `{{modules}}` block (at the tail, after the checks guidance if that module is also enabled).
2. `.agents/isolation/README.md` is created with the template content.
3. The user reads the AGENTS.md section and follows the one-time setup: install agent-box, `nix build github:nothingnesses/agent-images#claude-code`, `podman load < result`, fill in `~/.agent-box.toml`.
4. The user fills in `.agents/isolation/README.md` with the chosen image name and notes. This file is committed so contributors see it.
5. At the next session kickoff, the orchestrator's preflight runs the detection steps (check `which ab`, check `podman images`), confirms the container tier is available, and states this to the human before proceeding.
6. For each writer agent (implementer, planner), the orchestrator uses `ab new <repo> -s <session> --git` then `ab spawn -s <session> --git`, replacing the native Agent tool's worktree isolation for the writer spawn. The reviewers and triager are read-only and need no isolation, per `pack/AGENTS.md:82-84`.

What the user does NOT need to do: no changes to the workflow rules, no changes to `.agents/prompts/`, no changes to the checks configuration. The isolation module touches nothing outside its own files and the AGENTS.md `{{modules}}` slot.

## 7. What NOT to build

**The deferred growth path.** A `.agents/spawn.toml` role->launch-command map and a container-wrapper shim that let the orchestrator actually shell out to agent-box for cross-environment spawning. Both are recorded as deferred in `docs/plans/agent-scaffold.md:530` and must remain deferred. The reason is unchanged: the harness owns spawning; cross-environment agents are only reachable via a Bash shell-out path the native Agent tool cannot provide; a static launch config is a staleness liability; the orchestrator LLM can discover available CLIs at preflight without a scaffolded map. Build the rule (already done via `reviewer-diversity`) and the pointer setup now; build the launch map only after a real multi-CLI proof-of-concept validates it.

**A Nix snippet file.** An example `flake.nix` input or `mkAgentImage` call (Option C above). The majority of users build a pre-built agent-images image and do not author their own; the Nix custom-image path is for the minority who want a customized image. This can be added later as a separate working file or a note in `README.md`, but it does not belong in the initial file set.

**An `agent-scaffold isolation` subcommand.** No runtime integration with agent-box. The tool scaffolds content and then exits; it does not shell out to agent-box at scaffold time or at any other time.

**A reference prompt update to `orchestrator.md`.** The preflight guidance in `pack/AGENTS.md:98` already tells the orchestrator to detect and state the resolved tier. The isolation module's guidance partial adds the HOW (the specific detection commands) for the container tier. The orchestrator prompt itself does not need a new section; the `{{modules}}` guidance in AGENTS.md is the single point where container-tier preflight instructions live.

**A `requires = ["checks"]` dependency.** The isolation and checks modules are orthogonal. Do not auto-enable checks from isolation or vice versa.

**A per-project agent-box config file.** agent-box has no per-project config concept; its configuration is global via `~/.agent-box.toml`. A project-specific TOML config dropped under `.agents/` that no tool reads would be misleading. The `.agents/isolation/README.md` is documentation, clearly named as such.

**Any wiring that requires agent-box at scaffold time.** The Rust binary must still run correctly (and produce identical output from core assets) when agent-box and agent-images are not installed. The new files are inert content until the user installs the external tools.
```
