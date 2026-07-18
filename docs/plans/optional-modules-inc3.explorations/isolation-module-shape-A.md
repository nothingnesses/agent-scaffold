# Q-41: the concrete file shape of `optional-modules` increment 3 (the isolation module)

Design exploration, explorer A. Lens: minimalism and safety. This document settles ONLY the file shape of the pointer-scaffolding isolation module that ships now; it does not re-open the direction.

## 1. The question, and what is already off-limits

The question (queue item `Q-41`, `docs/plans/agent-scaffold.md:120`): when a user runs `agent-scaffold scaffold --module isolation`, which files does the bundle drop, what does each contain, what ownership (reference vs working) and rendering does each carry, and how do the files point a project at the external agent-box (`https://github.com/0xferrous/agent-box`) and agent-images (`https://github.com/nothingnesses/agent-images`) projects so that writer agents can run under worktree/container isolation.

Already decided and NOT open here:

- The module is POINTER-SCAFFOLDING only. It scaffolds config/guidance pointing a project at agent-box/agent-images; it is not a real container-integration path and does not reimplement isolation. The tool stays a scaffolder (`docs/plans/agent-scaffold.md:528`; the two cross-harness explorations `docs/plans/cross-harness-isolation.explorations/q33-q36-A.md` and `...-B.md`).
- No runtime dependency on agent-box/agent-images, and the Rust binary never shells out to them. Scaffolded content only.
- The isolation RULE (worktree-first, capability-tiered) already ships in `AGENTS.md` via the `agent-isolation` step: the tier order is container (agent-box/agent-images) > worktree > file-safety fallback (`AGENTS.md:79`-`AGENTS.md:85`), the worktree lifecycle is defined (`AGENTS.md:87`-`AGENTS.md:89`), and `session-preflight` resolves the concrete tier for a session (`AGENTS.md:98`). This module ships the concrete WIRING that the container tier resolves TO; it must not duplicate the rule, only point at it.
- The DEFERRED growth path stays deferred: a `.agents/spawn.toml` role->launch-command map plus an agent-box container-wrapper shim (`docs/plans/agent-scaffold.md:530`). Do not build it now.
- Byte-identical-when-off: with no `--module isolation`, output must be unchanged, so everything new is either a module-tagged asset or a module guidance partial that renders only when the module is enabled (the `{{modules}}` slot is empty otherwise; `manifest.rs:411`-`manifest.rs:440`, `manifest.rs:140`).

The mechanics I am working within (the checks module, increment 2, is the template): a content module is one `[[module]]` entry with a `guidance` partial concatenated into the `{{modules}}` render slot in declaration order (`manifest.rs:411`-`manifest.rs:440`), plus zero or more module-tagged `[[asset]]` rows, each with an `ownership` of `reference` (always rewritten) or `working` (create-if-absent, `manifest.rs:32`-`manifest.rs:39`), an optional `render`, and an optional `executable` bit. The checks module shows the full range: a guidance partial (`pack/pack.toml:11`-`pack/pack.toml:14`, `pack/checks-guidance.md`), a create-if-absent working config (`pack/pack.toml:116`-`pack/pack.toml:120`), seeded working files, a reference role prompt, and an executable reference hook (`pack/pack.toml:110`-`pack/pack.toml:158`).

The upstream interfaces I am pointing at, so claims are grounded:

- agent-images: OCI images built with Nix, `nix build .#<agent>` then `podman load < result`, image name `localhost/agent-images/<agent>:latest` (`agent-images/README.md:87`-`agent-images/README.md:90`). Consumed by agent-box.
- agent-box: a GLOBAL, per-user config at `~/.agent-box.toml` carrying `workspace_dir`, `base_repo_dir`, and a `[runtime]` table (`backend`, `image`, `env_passthrough`) (`agent-images/README.md:133`-`agent-images/README.md:146`); `ab spawn --local` mounts the current directory (`agent-images/README.md:152`-`agent-images/README.md:155`); `ab new <repo> -s <session> --git` then `ab spawn -s <session> --git` runs the sandboxed WORKTREE mode where the agent sees only committed files (`agent-images/README.md:161`-`agent-images/README.md:166`).

## 2. The design space (viable file sets)

The `[[module]] isolation` entry with `guidance = "isolation-guidance.md"` is common to every option (a content module needs its partial to render anything into `{{modules}}`). The options differ only in which module-tagged assets, if any, drop alongside it.

- Option A: guidance-partial-only. One `[[module]] isolation` entry, one guidance partial rendered into `{{modules}}`, and ZERO module-tagged assets. The partial carries the actionable setup inline: the tier mapping (this is what the container tier in `AGENTS.md:79`-`AGENTS.md:85` resolves to), the agent-images build/load commands, the canonical `~/.agent-box.toml` snippet, and the `ab new`/`ab spawn --git` worktree flow, each pointing at the upstream READMEs as the source of truth. Nothing is dropped into the project tree.

- Option B: guidance + a sample agent-box config file. Option A plus a module-tagged create-if-absent working file, for example `.agents/isolation/agent-box.toml.example`, a commented copy of the `~/.agent-box.toml` template.

- Option C: guidance + sample config + a helper script or justfile recipe. Option B plus a module-tagged executable script, for example `.agents/isolation/spawn-writer.sh`, wrapping `ab new`/`ab spawn --git` around a writer launch.

- Option D: guidance + a dropped pointer README working file. Option A plus a module-tagged working doc, for example `.agents/isolation/README.md`, restating the install/enable pointers (explorer B's shape in `docs/plans/cross-harness-isolation.explorations/q33-q36-B.md:38`).

## 3. Trade-offs, judged against the numbered Project Principles

The Project Principles are the plan's own (`docs/plans/agent-scaffold.md:18`-`docs/plans/agent-scaffold.md:24`): 1 cleaner long-term architecture, 2 minimal by default (a module must not complicate the core), 3 safe on existing projects (create-if-absent, namespaced, never clobber), 4 idempotent, 6 ground decisions in evidence, 7 reproducible.

Option A (guidance-partial-only):

- Principle 2 (minimal by default): the lightest possible module footprint. It reuses the exact `{{modules}}` mechanism the checks module already established (`manifest.rs:411`-`manifest.rs:440`), adds no new file to the project tree, and is byte-identical to core when unselected. Strongest on this principle.
- Principle 3 (safe on existing projects): zero dropped files means zero clobber surface. The only thing that changes is the `{{modules}}` block inside the working `AGENTS.md` (create-if-absent) and its refreshed reference copy `.agents/AGENTS.reference.md`, which is exactly how the checks module already behaves. Trivially safe.
- Principle 16 (one source of truth) and Principle 7 (cite sources): by pointing at the upstream READMEs rather than copying their config, it keeps agent-box/agent-images as the authoritative source for their own interfaces. The image name, the `~/.agent-box.toml` shape, and the `ab` flags all live upstream and evolve there; the partial cites them rather than freezing a snapshot.
- Cost/risk: the setup content lives inside `AGENTS.md` rather than in a file a user edits and keeps. But agent-box's config is a GLOBAL, per-user file (`~/.agent-box.toml`), not a project artifact, so there is nothing project-local for the user to own here anyway (see Option B). The partial can (and should) carry the same actionable code blocks a separate file would, exactly as `pack/checks-guidance.md` carries actionable command examples; so no actionable content is lost by not dropping a file.

Option B (guidance + sample `.agent-box.toml`):

- Principle 3 and Principle 2: a create-if-absent working file is safe against clobber, but it earns its place poorly. agent-box reads its config from `~/.agent-box.toml` in the user's HOME (`agent-images/README.md:133`), a machine-level, per-user location. Dropping a project-tracked copy puts a global config in the wrong place: a user who edits `.agents/isolation/agent-box.toml.example` has changed nothing agent-box reads, and the file invites the misconception that the project configures agent-box. That is a category error, not a convenience.
- Principle 16 (one source of truth): the sample duplicates the canonical `~/.agent-box.toml` example that already lives in `agent-images/README.md:133`-`agent-images/README.md:146`, so it drifts as the upstream config shape changes. A guidance partial that points at the README does not.
- The only thing the file adds over inlining the same snippet in the partial is an editable stub, and for a global config that stub belongs in the user's home, which the scaffolder does not and should not write to.

Option C (guidance + config + helper script/recipe):

- Principle 2 and the YAGNI boundary: a `spawn-writer.sh` wrapping `ab new`/`ab spawn --git` is the DEFERRED container-wrapper shim (`docs/plans/agent-scaffold.md:530`) in thin disguise. Building it now contradicts the explicit "stays deferred" decision and the evidence gate (Principle 6): the deferred path is held pending a proof-of-concept and a validated multi-CLI/container user. Rejected on scope.
- An executable script is also the highest-drift artifact of all (it hardcodes `ab` flags and an image name), and the highest-blast-radius (`executable = true`, like the checks hook), for a capability the harness/orchestrator, not a scaffolded script, is meant to drive.

Option D (guidance + dropped pointer README):

- Principle 2 and Principle 1: a `.agents/isolation/README.md` restating the upstream pointers duplicates the guidance partial's own content into a second file, giving two places to maintain the same pointers (drift, against Principle 1) and a second concept for the user to find. The `{{modules}}` slot exists precisely so a module's guidance lands in the one canonical place (`AGENTS.md`) that the orchestrator already reads to resolve the isolation tier at preflight (`AGENTS.md:98`); a parallel README competes with that. Rejected as redundant.

## 4. Recommendation: Option A, guidance-partial-only

Ship the isolation module as a single `[[module]]` entry plus one guidance partial, and drop NO files into the project tree. This is the smallest correct set that genuinely helps a user adopt agent-box/agent-images isolation, and it is the safest.

Reasoning, principle-grounded:

- It is minimal by construction (Principle 2). It adds exactly one guidance partial to the pack and one `[[module]]` row, reuses the `{{modules}}` machinery the checks module already proved, and is byte-identical to core when the module is off. A module that drops nothing cannot complicate the core.
- It is the safest possible shape (Principle 3). With no module-tagged assets, there is no create-if-absent decision, no ownership question, no clobber surface at all beyond the already-safe `AGENTS.md` render path.
- It puts the setup content where the workflow already looks for it (Principle 1, Principle 16). The container tier the isolation rule and the preflight name in the abstract (`AGENTS.md:79`-`AGENTS.md:85`, `AGENTS.md:98`) resolves, concretely, to "install agent-box + an agent-images image, configure `~/.agent-box.toml`, spawn writers via `ab new`/`ab spawn --git`", and that concrete resolution belongs in the same `AGENTS.md` the orchestrator reads at preflight, delivered through `{{modules}}`, rather than in a side file.
- It keeps agent-box/agent-images as the source of truth for their own evolving interfaces (Principle 7, Principle 16), by citing the READMEs instead of vendoring a config or a script that would drift.
- The candidate file that comes closest to earning its place, a sample agent-box config, does not earn it: agent-box's config is a per-user GLOBAL file in HOME, so a project-tracked copy is in the wrong location and duplicates the upstream canonical example. Everything actionable it would contain is carried inline in the partial instead, exactly as `pack/checks-guidance.md` carries command examples.

### Concrete specification of the recommended option

The `[[module]]` entry (in `pack/pack.toml`, alongside the existing `checks` entry at `pack/pack.toml:11`):

```toml
[[module]]
name = "isolation"
description = "Pointer setup wiring a project to the external agent-box and agent-images projects for container/worktree writer-agent isolation. Scaffolds guidance only (the concrete form of the container tier the Writer isolation rule names); it takes no runtime dependency on those projects and reimplements no isolation."
guidance = "isolation-guidance.md"
```

- `requires`: none. Unlike the test modules, which auto-enable `checks`, the isolation module depends on no other module.
- No `[[asset]]` rows tagged `module = "isolation"`. No `[[var]]` rows. Nothing is dropped into the project tree.

The one new file, the guidance partial:

- Path in the pack: `pack/isolation-guidance.md` (a pack source file read by `module_guidance`, `manifest.rs:411`-`manifest.rs:440`; it is NOT itself a dropped `[[asset]]`, exactly like `pack/checks-guidance.md`, so it never lands as its own file in a scaffolded project).
- Ownership: N/A. A guidance partial has no `ownership`; it is concatenated into the `{{modules}}` render slot, which lands inside the working `AGENTS.md` and the reference `.agents/AGENTS.reference.md` (`pack/pack.toml:16`-`pack/pack.toml:31`). It renders only when `--module isolation` is enabled; otherwise `{{modules}}` is empty and output is byte-identical (`manifest.rs:140`, `manifest.rs:411`-`manifest.rs:440`).
- Rendering: the partial contains NO `{{...}}` placeholders. `module_guidance` inserts it as the value of the `{{modules}}` variable and the whole `AGENTS.md` asset is then rendered, so a stray placeholder in the partial could be re-substituted; keeping the partial placeholder-free (as `pack/checks-guidance.md` is) avoids that. Plain prose and fenced code blocks only.
- Executable: N/A (not a dropped file).

Contents sketch of `pack/isolation-guidance.md` (a section that concatenates under the module block, mirroring the shape of `pack/checks-guidance.md`; all ASCII, no hard-wrapping):

- Heading, e.g. `## Writer isolation via agent-box and agent-images`.
- One paragraph stating what the module is: it does not restate the Writer isolation rule (points at that section of `AGENTS.md`); it is the concrete wiring the container tier resolves to when agent-box and an agent-images image are present, and it falls back to worktree, then the file-safety baseline, when they are not, exactly as the rule already promises.
- A short "install the image" block: `nix build .#<agent>` then `podman load < result`, yielding `localhost/agent-images/<agent>:latest`, pointing at `https://github.com/nothingnesses/agent-images` for the image list and `mkAgentImage` customisation.
- A short "configure agent-box" block: create the GLOBAL `~/.agent-box.toml` (stress: this is a per-user file in your home, not a project file, which is why the module scaffolds no config into the repo) with `workspace_dir`, `base_repo_dir`, and `[runtime]` (`backend`, `image`, `env_passthrough`), pointing at `https://github.com/0xferrous/agent-box` and the agent-images README as the canonical example.
- A short "run a writer under isolation" block tying it to the workflow: for the worktree lifecycle the orchestrator already owns (`AGENTS.md:87`), use the sandboxed worktree mode, `ab new <repo> -s <session> --git` then `ab spawn -s <session> --git`, so a writer agent sees only committed files; `ab spawn --local` is the un-sandboxed mount for reference. One line noting the orchestrator states the resolved tier at preflight (`AGENTS.md:98`) and that read-only agents need no isolation.
- A closing line pointing at the deferred growth path by name only ("a role->launch spawn map and a container-wrapper shim are a possible future addition, not part of this module"), so a reader is not surprised, without scaffolding any of it.

Validation (Principle 6): the proof-of-concept is that `scaffold --module isolation` renders the partial into `AGENTS.md`'s `{{modules}}` block, and that with no module selected the scaffold output is byte-identical to today's core (the same invariant `builtin_manifest_lists_the_expected_assets` and the checks-off test already pin, `manifest.rs:576`-`manifest.rs:602`, `manifest.rs:629`-`manifest.rs:672`). Because no asset drops, there is no ownership or clobber behaviour to test beyond the existing `{{modules}}` render path.

## 5. What NOT to build (the YAGNI boundary)

- Do NOT drop a sample `~/.agent-box.toml` (or `.agent-box.toml.example`) into the project. agent-box reads its config from the user's HOME, not the repo (`agent-images/README.md:133`); a project-tracked copy is in the wrong place and duplicates the upstream canonical example (drift, Principle 16). Inline the snippet in the guidance instead.
- Do NOT drop a helper/wrapper script or a justfile recipe (`spawn-writer.sh`, an `ab`-wrapping recipe). That is the DEFERRED container-wrapper shim (`docs/plans/agent-scaffold.md:530`) and it stays deferred pending a proof-of-concept and a validated user (Principle 6). It is also the highest-drift, highest-blast-radius artifact for a capability the orchestrator, not a scaffolded script, drives.
- Do NOT drop a `.agents/spawn.toml` role->launch-command map. Explicitly deferred (`docs/plans/agent-scaffold.md:530`); the always-on diversity DOCS rule that motivated it already shipped separately via `reviewer-diversity`.
- Do NOT drop a `.agents/isolation/README.md` pointer doc. It duplicates the guidance partial's content into a second maintained place (Principle 1) and competes with the one canonical location (`AGENTS.md` via `{{modules}}`) the orchestrator already reads.
- Do NOT add a runtime dependency on agent-box/agent-images, and do NOT have the Rust binary shell out to `ab`, `podman`, or `nix`. The module is inert scaffolded content; the orchestrator, in the harness, acts on it.
- Do NOT restate the Writer isolation rule or the worktree lifecycle in the partial. They live in `AGENTS.md:79`-`AGENTS.md:89`; the partial points at them and supplies only the concrete agent-box/agent-images wiring the container tier resolves to.
