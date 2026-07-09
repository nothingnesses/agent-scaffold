# agent-scaffold spec

Status: draft, awaiting review. The name `agent-scaffold` is a working name (see OQ-6).

This document plans a tool that scaffolds the agent workflow (front-load context -> structured plan -> iterative and adversarial review -> isolated implementation -> adversarial review) into a project, so the structure does not have to be hand-rolled each time. It follows the same planning format the tool is meant to scaffold.

## Motivations

- Setting up the workflow by hand for each project is repetitive: the same planning-document skeleton, the same guidance/principles, the same reusable prompts, and (when wanted) the same worktree and container isolation.
- The setup should work in two modes: starting a new project, and adding to or editing an existing one. A greenfield-only template does not cover the second mode.
- The default should be minimal. Extra machinery (diagram prompt packs, container isolation, language starters) should be opt-in rather than imposed.
- Guidance should be harness-agnostic: `AGENTS.md` is the canonical file, with any harness-specific file (for example `CLAUDE.md`) reduced to a thin pointer to it.

## Project Principles

These govern how this tool is built.

1. Prefer the cleaner long-term architecture over the smallest diff: prioritise correctness, internal coherence, and maintainability, and when a local fix conflicts with a cleaner design, choose the cleaner design unless a concrete limitation prevents it.
2. Minimal by default. The core does one thing well; everything else is an optional module the user opts into. Adding a module must not complicate the core.
3. Safe on existing projects. Scaffolding into a populated repo must never clobber or silently overwrite existing files. Prefer create-if-absent, namespaced locations, and clearly marked idempotent edits.
4. Idempotent. Running the tool twice produces the same result as running it once, with no duplicated blocks and no drift.
5. Make illegal states unrepresentable. Work out the valid inputs and outcomes first and encode them, rather than admitting bad states and guarding against them at runtime.
6. Ground decisions in evidence. Before an approach is built out, validate it with a small proof-of-concept that builds and produces the expected output. If the candidates are exhausted, raise the impasse for a decision rather than forcing through an unvalidated approach.
7. Reproducible. Prefer the project's existing toolchain conventions (Nix, just, direnv) so a scaffolded project and its checks behave the same on any machine.

Note: the principles the tool *ships* to other projects (the default `AGENTS.md` content) are a separate artifact from these, and which set to ship by default is OQ-5.

## Documentation Protocol

This plan is kept current during the work. Each Implementation Step carries a `Status:` line (`not started`, `in progress`, `blocked on <x>`, `complete`). Resolved Open Questions are removed from that section and folded into the relevant step as the adopted decision plus reasoning, rather than left as a pointer, so the document does not accumulate stale decision history. When no questions remain open, the section says so.

## Open Questions, Decisions, Issues and Blockers

### OQ-1: What form should the tool take?

The tool must serve both new and existing projects, which rules out a plain flake template as the sole mechanism.

Approaches:

- **A. Nix flake app, run via `nix run`, that drops files into the current directory.** Works for new and existing projects (idempotent, create-if-absent, namespaced). Reproducible, no install step, stays in the existing Nix stack. Trade-off: the idempotent create/merge logic (especially for `AGENTS.md`) has to be written carefully; requires the flake to be fetchable.
- **B. Plain script (POSIX sh or Nushell) copied or curled in.** Simplest to read and modify, runs anywhere. Trade-off: not reproducible the way Nix is, manual dependency handling, less aligned with the rest of the toolchain.
- **C. Copier / cookiecutter template.** Parameterised, and `copier update` gives some existing-project support. Trade-off: adds a Python-ecosystem dependency outside the usual Nix/Rust tooling; update semantics can be fiddly.
- **D. Flake template for greenfield plus one of the above for existing.** Two entry points.

Recommendation: A, with a flake template added later as an optional greenfield convenience (a subset of D). Reasoning: A alone satisfies both modes, is a single `nix run` one-liner (the "quick and easy" goal), and is reproducible. The template is a nicety, not a requirement, so it is deferred under the minimal-by-default principle.

### OQ-2: Where do scaffolded files live?

Approaches:

- **A. A namespaced directory** (for example `agents/` or `.agents/`) holds the kit's own assets (plan template, prompt library, shipped principles source), and only a small number of root files (`AGENTS.md`, optionally a thin `CLAUDE.md`) are touched.
- **B. Everything at conventional root locations** (`AGENTS.md`, `docs/plans/`, `prompts/`).

Recommendation: A for the kit's own assets, with `AGENTS.md` at root (create-if-absent) and the plan template written to `docs/plans/` since that is where plans are expected to live. Reasoning: namespacing keeps the kit self-contained and updatable and minimises collision risk on existing repos, while the plan template belongs where plans are actually authored.

### OQ-3: How are existing files handled?

Chiefly `AGENTS.md`. Approaches: create-if-absent only (skip if present); or create-if-absent and, when present, append a clearly marked, idempotent block; or write kit content to an included file that the root `AGENTS.md` references.

Recommendation: create-if-absent, and when present insert a single marked block (for example between `<!-- agent-scaffold:start -->` and `<!-- agent-scaffold:end -->`) that is replaced in place on re-run. Reasoning: satisfies idempotency and safety without forcing the user to hand-merge.

### OQ-4: What is in the minimal core versus optional modules?

Proposed core: `AGENTS.md` (canonical guidance), a planning-document template, and a small prompt library (clarifying-questions, open-questions gate, adversarial-review). Proposed optional modules: diagram prompt pack; container and worktree isolation recipes (integrating agent-box and agent-images); a justfile of recipes; language-specific starters. Recommendation: as proposed. Reasoning: the core is the irreducible workflow; the modules are the "bells and whistles" the user asked to keep optional. Confirm the split.

### OQ-5: Which principles ship in the default AGENTS.md?

The reusable set (make illegal states unrepresentable; parse, don't validate; evidence-grounding via proof-of-concept; explicit failure and absence; prefer clean long-term architecture) versus a smaller starter subset the user edits per project. Recommendation: ship the full reusable set with a short note that they are a starting point to trim per project. Reasoning: it is easier to delete an unwanted principle than to remember a missing one. Open for the user's preference.

### OQ-6: What is the tool named?

`agent-scaffold` is the working name and fits the existing `agent-box` and `agent-images` family. Alternatives: `agent-kit`, `agent-loom`, `agent-workflow`. Recommendation: confirm or replace before the first module is built, since it becomes the repo and command name.

## Implementation Steps

Sequencing is gated on OQ-1; the steps below assume approach A.

### 1. Author the minimal core assets

Status: not started. Write the canonical `AGENTS.md` content, the planning-document template, and the three core prompts, as static files, before any generation logic. This is the reusable substance and can be validated by eye.

### 2. Proof-of-concept the file-dropper

Status: blocked on OQ-1. A minimal flake app that drops the core into an empty directory and into a populated one, demonstrating create-if-absent and the marked-block edit for `AGENTS.md`. Validates the form decision per principle 6 before building further.

### 3. Idempotency and safety pass

Status: not started. Re-run produces no drift; existing files are preserved; the marked block round-trips.

### 4. Optional modules behind flags

Status: not started. Add diagram pack, isolation recipes, justfile, and language starters as opt-in selections, each self-contained.

### 5. Optional greenfield flake template

Status: not started. Expose a `nix flake new` template as a convenience for the new-project case, reusing the same core assets.

## Success Criteria

- One command drops the minimal core into an empty directory and into an existing repository, in both cases leaving a usable `AGENTS.md`, a planning-document template, and the core prompts.
- Running it a second time changes nothing (idempotent) and never overwrites a pre-existing file outside its own marked block.
- The dropped assets are immediately usable to run one pass of the workflow (plan -> review -> implement -> review) without further setup.
- Optional modules can be added without touching or complicating the core.
