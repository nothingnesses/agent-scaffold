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

### OQ-3: Ownership and update model (broadened from existing-file handling)

Once a file is scaffolded, who owns it, and what happens on re-run? This subsumes the earlier, narrower question of how pre-existing files are handled.

Approaches:

- **A. Vendored, one-shot.** The tool copies assets in once and never touches them again. The user owns everything and edits freely. No update path: template improvements are picked up by re-scaffolding into a scratch location and diffing by hand.
- **B. Vendored with managed blocks.** Whole files belong to the user, except a clearly marked region (for example `<!-- agent-scaffold:start -->` ... `end`) that the tool owns and replaces in place on re-run. Only works for formats that can carry markers.
- **C. Vendored with opt-in update (3-way merge).** The tool records what it generated (a manifest), and an explicit `update` re-applies template changes via a 3-way merge (base = last generated, ours = user edits, theirs = new template), copier-style. Most powerful, most complex, can conflict.
- **D. Referenced, non-vendored.** Assets stay in the tool or a package and the project references them (symlink or include). Always current, but the user cannot edit freely without breaking the link.

Recommendation: A by default (maximum freedom, zero interference, which directly answers the "will the tool step on my edits" concern), plus B for the few root files the tool must co-inhabit (chiefly `AGENTS.md` when it already exists). Offer C later as an explicit opt-in `update` command for users who want to pull template improvements. Reasoning: hands-off is the safe default and the common case; the marked block covers the one unavoidable co-tenancy; a merge-based update is real value but is advanced and must not complicate the core. D is rejected as a default because it removes the editability the user explicitly wants, though it could be an optional mode later.

### OQ-5: Which principles ship in the default AGENTS.md?

Shipping posture (decided): ship a broad reusable set with a note that they are a starting point to trim per project, since deleting an unwanted principle is easier than noticing a missing one. Still open: which principles make the cut. Candidate pool below, grouped, for pruning. These are the principles the tool ships to other projects, distinct from this project's own Project Principles above.

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

### OQ-7: Template source, built-in versus bring-your-own

Should the content the tool scaffolds be fixed (our agent-workflow pack), or can the user point the tool at their own template set?

Approaches:

- **A. Built-in only.** The tool ships one opinionated pack (the agent workflow). Simplest. Trade-off: not reusable for the user's other scaffolding needs.
- **B. Built-in default plus bring-your-own.** The tool ships the agent-workflow pack as the default, and `--template <path-or-flake-ref>` points it at an alternative. A pack is a plain directory of assets plus a small manifest (what to drop where, and each asset's ownership mode from OQ-3). Substitution stays minimal (simple named-variable replacement), deliberately not a full templating language.
- **C. Bring-your-own first (generic engine).** The tool is a general scaffolding engine and the agent workflow is merely the bundled default pack.

Recommendation: B. Reasoning: it separates mechanism (drop and merge) from policy (the pack), which is cleaner and lets the same tool scaffold the user's own project types cheaply, while stopping short of rebuilding copier or cookiecutter (C's real risk, since a full generic templating engine is a much larger undertaking with mature incumbents). Keeping substitution minimal is what holds B back from sliding into C. B also fits OQ-1 approach A: a pack referenced as a path or flake-ref is exactly what `nix run` can fetch. Flagged for discussion, per the request to talk this through before settling OQ-3.

## Implementation Steps

Sequencing is gated on OQ-1, OQ-3, and OQ-7; the steps below assume OQ-1 approach A.

Folded decisions from review round 1: the kit's own assets (plan template, prompt library, shipped-principles source) live in a namespaced directory by default, with an optional output-directory override; `AGENTS.md` is written at root and the plan template under `docs/plans/`. The optional module set is confirmed: a diagram prompt pack, container and worktree isolation (integrating agent-box and agent-images), a justfile of recipes, and language starters.

### 1. Author the minimal core assets

Status: not started. Write the canonical `AGENTS.md` content, the planning-document template, and the three core prompts (clarifying-questions, open-questions gate, adversarial-review) as static files, before any generation logic. This is the reusable substance and can be validated by eye. Blocked in part on OQ-5 (which principles to ship).

### 2. Proof-of-concept the file-dropper

Status: blocked on OQ-1, OQ-3, OQ-7. A minimal flake app that drops the core into an empty directory and into a populated one, writing kit assets to the namespaced directory, creating `AGENTS.md` if absent and editing its marked block if present. Validates the form and ownership decisions per principle 6 before building further.

### 3. Idempotency and safety pass

Status: not started. Re-run produces no drift; pre-existing files are preserved; the marked block round-trips; the optional output-directory override works.

### 4. Optional modules behind flags

Status: not started. Add the confirmed modules as opt-in selections, each self-contained, none complicating the core.

### 5. Bring-your-own template support

Status: blocked on OQ-7. If B is adopted, support `--template <path-or-flake-ref>` with a small manifest and minimal named-variable substitution.

### 6. Optional greenfield flake template

Status: not started. Expose a `nix flake new` template as a convenience for the new-project case, reusing the same core assets.

### 7. Optional opt-in update command

Status: blocked on OQ-3. If C (3-way merge update) is adopted as an advanced mode, record a generation manifest and implement `update`.

## Success Criteria

- One command drops the minimal core into an empty directory and into an existing repository, in both cases leaving a usable `AGENTS.md`, a planning-document template, and the core prompts.
- Running it a second time changes nothing (idempotent) and never overwrites a pre-existing file outside its own marked block.
- Vendored assets can be edited freely, and a default re-run does not revert those edits.
- The dropped assets are immediately usable to run one pass of the workflow (plan -> review -> implement -> review) without further setup.
- Optional modules can be added without touching or complicating the core.
- (If OQ-7 B is adopted) the tool can scaffold from a user-supplied template pack, not only the built-in one.
