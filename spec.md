# agent-scaffold spec

Status: draft, revised after review round 1. Name confirmed as `agent-scaffold`.

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

Refinement (review round 2): a hard requirement is that the tool runs from any working directory and can output to any target directory (defaulting to the current directory with the namespaced layout), selected by an `--output-dir` flag. This rules out a template-only mechanism (a flake template can only initialise the current directory) and settles the shape as "a runnable command that takes an output directory," which is A or B, not C or a bare template. Whether that command is packaged as a Nix flake app (A) or a plain script (B) is a smaller, reversible decision that can be settled at proof-of-concept time per principle 6, rather than up front. Leaning A for distribution and reproducibility, but a script proof-of-concept is the fastest way to validate the dropper logic first.

### OQ-3: Ownership and update model (broadened from existing-file handling)

Once a file is scaffolded, who owns it, and what happens on re-run? This subsumes the earlier, narrower question of how pre-existing files are handled.

Approaches:

- **A. Vendored, one-shot.** The tool copies assets in once and never touches them again. The user owns everything and edits freely. No update path: template improvements are picked up by re-scaffolding into a scratch location and diffing by hand.
- **B. Vendored with managed blocks.** Whole files belong to the user, except a clearly marked region (for example `<!-- agent-scaffold:start -->` ... `end`) that the tool owns and replaces in place on re-run. Only works for formats that can carry markers.
- **C. Vendored with opt-in update (3-way merge).** The tool records what it generated (a manifest), and an explicit `update` re-applies template changes via a 3-way merge (base = last generated, ours = user edits, theirs = new template), copier-style. Most powerful, most complex, can conflict.
- **D. Referenced, non-vendored.** Assets stay in the tool or a package and the project references them (symlink or include). Always current, but the user cannot edit freely without breaking the link.

Decision (review round 2): approach A, vendored and one-shot, at whole-file granularity. By default the tool creates files that do not exist and never modifies files that do (create-if-absent), so it can never step on the user's edits. An opt-in `--force` (off by default) overwrites existing files with the pack version. This is simpler and more predictable than both the marked-block co-tenancy (B) and the 3-way-merge update (C), and it matches the user's preference for a plain override option over a merge.

Consequence for `AGENTS.md`: since the tool will not edit an existing file, the shipped guidance always also lives in the namespaced directory (as a reference copy). On a new project the root `AGENTS.md` is created with the guidance; on a project that already has one, the root file is left untouched and the user merges from the reference copy (or passes `--force` to overwrite). B (a marked, auto-updated block in `AGENTS.md`) and C (opt-in merge `update`) are recorded as possible later enhancements, both off the critical path. D (non-vendored reference) stays rejected because it removes the editability the user wants.

Open sub-question: for an existing `AGENTS.md`, is "leave it and drop a reference copy alongside" sufficient, or is the marked-block augmentation (B) wanted sooner because merging by hand is friction?

### OQ-5: Which principles ship in the default AGENTS.md?

Shipping posture (decided): ship a broad reusable set rather than a minimal one. Refinement (review round 2): rather than shipping a fixed set the user edits after the fact, the tool offers an interactive selection at scaffold time (see OQ-8) with a sane default subset pre-selected, from the full pool below. So the pool is the menu, the pre-selected defaults are the sane starting point, and the user checks or unchecks per project. Still open: which principles belong in the pre-selected default subset (marked later, once the pool is pruned). These are the principles the tool ships to other projects, distinct from this project's own Project Principles above.

Data and type modelling:
- Make illegal states unrepresentable.
- Parse, don't validate (reject malformed input at the boundary and turn it into a precise type).
- Make failure and absence explicit (Result and Option; no nulls, exceptions, or silent defaults).
- Keep pattern matches total; avoid a catch-all where a missing case would be a bug.
- Prefer immutability; keep mutation local and explicit.
- Avoid primitive obsession; wrap meaningful values in newtypes.
- One source of truth; derive rather than duplicate.
- Encode invariants in types; fall back to a checked boundary or documented invariant only when the type system cannot express them.

Architecture:
- Prefer the cleaner long-term architecture over the smallest diff.
- Separate mechanism from policy.
- Functional core, imperative shell: push effects to the edges.
- Prefer composition over inheritance.
- Small modules with clear boundaries and explicit contracts.
- Depend on abstractions at boundaries (dependency inversion).
- YAGNI: do not build for speculative futures.
- KISS: prefer the simplest thing that works.
- Prefer duplication over the wrong abstraction; extract only on the third repeat.
- Design for deletion and replaceability.
- Least privilege and least authority.
- Principle of least astonishment.
- Make operations idempotent where they may be retried.

Correctness and quality:
- Correctness before performance; avoid premature optimisation, but ask the cheap scaling questions early.
- Tests must actually exercise the code they claim to.
- Reach for property-based tests when the input space is large.
- Prefer compile-time enforcement, then runtime checks, then convention, then prose, in that order.
- Fail fast and loudly; no silent failure.
- Handle errors where you can act on them.
- Make the common case easy and the wrong case hard.

Agent process:
- Ask clarifying questions before forging ahead; always give recommendations and the reasoning behind them.
- Surface open questions, decisions, and blockers before implementing.
- Ground decisions in evidence; validate an approach with a small proof-of-concept before building it out, and if the candidates are exhausted, raise the impasse rather than forcing an unvalidated approach through.
- Keep changes small and reviewable.
- Have independent or adversarial review before accepting work.
- Verify, do not trust: run it and test it rather than asserting success.
- Cite sources (file and line) rather than asserting from memory.
- Match the existing conventions of the codebase.
- No silent scope expansion: do what was asked and flag what was not.
- Leave durable notes that survive context compaction.
- Prefer reversible steps; make risky or irreversible steps explicit and confirm them.

Documentation:
- Document the why, not the what.
- Treat types and tests as documentation.
- Include runnable examples with assertions.
- Keep docs next to the code and make stale docs loud (for example compile-time doc checks).

Security:
- Validate and parse untrusted input at the boundary.
- Never trust external input.
- Sandbox or isolate untrusted execution (directly relevant to agents).
- Keep secrets out of code and logs.

### OQ-8: Selection UI for principles and modules

At scaffold time, how does the user choose which principles (OQ-5) and which optional modules to include?

Approaches:

- **A. Flags and config only.** Non-interactive: `--principles default|all|none|<list>`, `--modules <list>`, plus an optional config file. Scriptable and idempotent, but the user must know the names.
- **B. Interactive multi-select with sane defaults, plus the non-interactive path.** In a terminal, present a checkbox selector (for example via gum, fzf, or whiptail) for principles and modules with the sane defaults pre-checked; when not attached to a terminal, or when flags or a config are supplied, run non-interactively.

Decision (review round 2): B. The user asked for a pick-from-the-list UI that ships sane defaults, and an interactive multi-select over the OQ-5 pool and the module set delivers exactly that. Requirement: interactivity is a convenience layer, never the only path; a non-interactive route (flags or a written config) and a recorded record of what was selected must always exist, so the tool stays scriptable, reproducible, and idempotent (principles 2 and 4). The picker dependency (gum, fzf, whiptail, or a small built-in) is deferred to proof-of-concept time. One UI covers both principles and modules.

## Implementation Steps

Sequencing is gated on OQ-1 (form: flake app or script, to be settled at proof-of-concept) and the OQ-3 sub-question (existing `AGENTS.md`). Template source is decided (OQ-7 approach B): a built-in default pack plus optional `--template <path-or-flake-ref>`.

Folded decisions: the kit's own assets (plan template, prompt library, shipped-principles source) live in a namespaced directory by default, with an `--output-dir` override so the tool can run from any directory and write to any target; `AGENTS.md` is written at root and the plan template under `docs/plans/`. Ownership is vendored one-shot: create-if-absent by default, `--force` to overwrite (OQ-3). The optional module set is confirmed: a diagram prompt pack, container and worktree isolation (integrating agent-box and agent-images), a justfile of recipes, and language starters.

### 1. Author the minimal core assets

Status: not started. Write the canonical `AGENTS.md` content, the planning-document template, and the three core prompts (clarifying-questions, open-questions gate, adversarial-review) as static files, before any generation logic. Blocked in part on OQ-5 (which principles are pre-selected by default).

### 2. Proof-of-concept the file-dropper

Status: blocked on OQ-1 (form) and the OQ-3 sub-question. A minimal command (likely a script first) that drops the core into an empty directory and into a populated one, writing kit assets to the namespaced directory under `--output-dir`, creating absent files and leaving existing files untouched unless `--force` is given. Validates the form and ownership decisions per principle 6 before building further.

### 3. Idempotency and safety pass

Status: not started. Re-run changes nothing; pre-existing files are preserved unless `--force`; the `--output-dir` override works from any working directory.

### 4. Selection UI

Status: not started. Interactive multi-select for principles and modules with sane defaults pre-checked (OQ-8 B), plus the non-interactive flag and config path and a recorded selection.

### 5. Optional modules

Status: not started. Package the confirmed modules as opt-in selections, each self-contained, none complicating the core.

### 6. Bring-your-own template support

Status: not started (OQ-7 B decided). Support `--template <path-or-flake-ref>` with a small manifest and minimal named-variable substitution; the built-in agent-workflow pack is the default.

### 7. Optional greenfield flake template

Status: not started. Expose a `nix flake new` template as a convenience for the new-project case, reusing the same core assets.

### 8. Optional later enhancements

Status: not started. Marked-block augmentation of an existing `AGENTS.md` (OQ-3 B) and an opt-in merge `update` command (OQ-3 C), if the create-or-overwrite model proves too blunt in practice.

## Success Criteria

- One command, run from any working directory and targeting any `--output-dir`, drops the minimal core (a usable `AGENTS.md`, a planning-document template, and the core prompts) into both an empty directory and an existing repository.
- By default it creates only absent files and never modifies existing ones; `--force` is required to overwrite; a default re-run therefore reverts nothing.
- The user can choose which principles and modules to include, interactively with sane defaults or non-interactively via flags or config.
- The dropped assets are immediately usable to run one pass of the workflow (plan -> review -> implement -> review) without further setup.
- Optional modules can be added without touching or complicating the core.
- The tool can scaffold from a user-supplied template pack, not only the built-in one.
