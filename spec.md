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

Note: the principles the tool _ships_ to other projects (the default `AGENTS.md` content) are a separate artifact from these, and which set to ship by default is OQ-5.

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

Decision (review round 3): A. The tool is a Rust binary (clap-based CLI, see OQ-8) packaged as a Nix flake app and run via `nix run`. This resolves the earlier flake-app-versus-script ambiguity toward a Rust binary, which is consistent with the CLI and TUI tooling in OQ-8 and keeps the door open to a script-only proof-of-concept purely to de-risk the drop logic before the Rust implementation. Reversible if it proves wrong.

Refinement (review round 4): the tool must not require Nix at runtime. Nix is used only for the development environment (a reproducible dev shell for contributors). The tool ships as a standalone Rust binary that end users install and run without Nix: via `cargo install`, a prebuilt release binary, or, for Nix users only and as a convenience, a flake app. Impacts: distribution needs a non-Nix path (crates.io and prebuilt binaries built for common targets in CI), which is slightly more release engineering than a single flake; the bring-your-own template fetch must not depend on Nix fetchers (support a local path and a git URL, with a flake-ref only as an optional extra); and the optional isolation module (agent-box, agent-images) keeps its Nix and container dependencies, so it is the one part not available Nix-free, which is acceptable because it is opt-in. No real downside for a CLI of this kind: `Cargo.lock` gives adequate build reproducibility, and dropping the hard Nix dependency widens the audience to anyone with a Rust toolchain or a prebuilt binary.

### OQ-3: Ownership and update model (broadened from existing-file handling)

Once a file is scaffolded, who owns it, and what happens on re-run? This subsumes the earlier, narrower question of how pre-existing files are handled.

Approaches:

- **A. Vendored, one-shot.** The tool copies assets in once and never touches them again. The user owns everything and edits freely. No update path: template improvements are picked up by re-scaffolding into a scratch location and diffing by hand.
- **B. Vendored with managed blocks.** Whole files belong to the user, except a clearly marked region (for example `<!-- agent-scaffold:start -->` ... `end`) that the tool owns and replaces in place on re-run. Only works for formats that can carry markers.
- **C. Vendored with opt-in update (3-way merge).** The tool records what it generated (a manifest), and an explicit `update` re-applies template changes via a 3-way merge (base = last generated, ours = user edits, theirs = new template), copier-style. Most powerful, most complex, can conflict.
- **D. Referenced, non-vendored.** Assets stay in the tool or a package and the project references them (symlink or include). Always current, but the user cannot edit freely without breaking the link.

Decision (review round 2): approach A, vendored and one-shot, at whole-file granularity. By default the tool creates files that do not exist and never modifies files that do (create-if-absent), so it can never step on the user's edits. An opt-in `--force` (off by default) overwrites existing files with the pack version. This is simpler and more predictable than both the marked-block co-tenancy (B) and the 3-way-merge update (C), and it matches the user's preference for a plain override option over a merge.

Consequence for `AGENTS.md`: since the tool will not edit an existing file, the shipped guidance always also lives in the namespaced directory (as a reference copy). On a new project the root `AGENTS.md` is created with the guidance; on a project that already has one, the root file is left untouched and the user merges from the reference copy (or passes `--force` to overwrite). B (a marked, auto-updated block in `AGENTS.md`) and C (opt-in merge `update`) are recorded as possible later enhancements, both off the critical path. D (non-vendored reference) stays rejected because it removes the editability the user wants.

Resolved sub-question (review round 3): "leave it and drop a reference copy alongside" is sufficient. This settles a two-tier ownership model:

- **Tool-owned reference assets** in the namespaced directory (the pristine copies of the shipped guidance, prompts, plan template, and principle data) are always written and safe to overwrite on every run, so they stay current. The user reads and copies from these rather than editing them in place.
- **User working files** (the root `AGENTS.md`, plans authored under `docs/plans/`) are create-if-absent, with `--force` to overwrite.

For an existing `AGENTS.md`: the root file is left untouched, and the namespaced reference copy is refreshed for the user to merge from. The marked-block augmentation (B) and the merge `update` (C) remain optional later enhancements, not needed for this model.

### OQ-5: Which principles ship in the default AGENTS.md?

Shipping posture (decided): ship a broad reusable set rather than a minimal one. Refinement (review round 2): rather than shipping a fixed set the user edits after the fact, the tool offers an interactive selection at scaffold time (see OQ-8) with a sane default subset pre-selected, from the full pool below. So the pool is the menu, the pre-selected defaults are the sane starting point, and the user checks or unchecks per project. Still open: which principles belong in the pre-selected default subset (marked later, once the pool is pruned). These are the principles the tool ships to other projects, distinct from this project's own Project Principles above.

Data model (review round 3, decided): each principle is structured data, not a bare line, so the tool can carry richer meaning and render a chosen amount of it. This is worth doing: it powers the selection UI's help text (the user sees why a principle exists before choosing it), lets the output verbosity vary, and lets bring-your-own packs (OQ-7) ship their own principle data in the same shape. Proposed schema per principle:

- `id`: stable slug (for example `make-illegal-states-unrepresentable`), used in flags, config, and the selection record.
- `tags`: an array of labels, since a principle can belong to several. Two conventional namespaces: category tags (`data-modelling`, `architecture`, `correctness`, `agent-process`, `documentation`, `security`) for grouping and display, and applicability tags (`universal`, `static-types`, `fp`, `oop`) for adapting defaults to the project type. A principle carries as many as apply (for example sandbox-or-isolate carries `security` and `agent-process`).
- `name`: short imperative title.
- `summary`: one sentence.
- `rationale`: as rich as needed to convey the idea, but not verbose (why to adopt it and what it prevents).
- `default_selected`: whether it is pre-checked in the sane-default set.
- `default_order`: integer sort key (spaced by 10) for the output list; selected principles render as a numbered list in this order, and the TUI can reorder them per project.
- optional `references`: links or citations; optional `related`: ids of related principles.

The tool renders the selected subset as a numbered list ordered by `default_order`, at a chosen verbosity: `name` only, `name` plus `summary` (proposed default), or `full` (name, summary, rationale, references), via a `--principle-detail` flag or the UI. Format: an external data file (recommend TOML for readability and easy authoring by bring-your-own users, with serde on the Rust side; RON or JSON are alternatives), bundled for the default pack and loadable from a pack for BYO. The same schema pattern extends to modules, so the selection UI can show a description and rationale for each optional module too. Still open: the exact field set and the data format.

Default set (review round 5, decided): adopt the recommendation. Drop KISS and "match existing conventions" from the defaults (both stay in the pool), tag every principle with `applicability` (`universal`, `static-types`, `fp`, or `oop`) so the default set adapts to the project, and trim the defaults to a smaller high-leverage subset. On a statically-typed project the defaults are the universal set plus the top typed principles; on a dynamically-typed project only the universal ones apply.

Applicability tags (finalised per principle in the data file, and a principle may carry more than one): data and type modelling principles carry `static-types` (several also `fp`); agent-process, documentation, and security principles carry `universal`, except "types and tests as documentation" which is `static-types`; correctness principles carry `universal` except the compile-time-enforcement hierarchy (`static-types`); architecture is mixed, with functional core imperative shell tagged `fp`, composition over inheritance `oop`, and the rest `universal`.

Proposed trimmed default set (about 21; the three static-types items apply only to statically-typed projects, the rest are universal):

- Data and type modelling (static-types): make illegal states unrepresentable; parse, don't validate; make failure and absence explicit.
- Architecture (universal): prefer the cleaner long-term architecture over the smallest diff; least privilege and least authority.
- Correctness and quality (universal): correctness before performance; tests must actually exercise the code they claim to; fail fast and loudly.
- Agent process (universal): ask clarifying questions first; surface open questions before implementing; ground decisions in evidence with a proof-of-concept; keep changes small and reviewable; have independent or adversarial review before accepting work; verify, do not trust; cite sources; no silent scope expansion; leave durable notes that survive compaction.
- Documentation (universal): document the why, not the what.
- Security (universal): validate and parse untrusted input at the boundary; never trust external input; keep secrets out of code and logs.

On a dynamically-typed project the three static-types items drop, leaving eighteen universal defaults.

Close calls left out of the default but reasonable to pull back in: prefer reversible steps, and handle errors where you can act on them. Sandbox or isolate untrusted execution is deliberately not a default because it relies on external tools (containers, sandboxes) to be effective, so it would be dead advice on a project without them; it stays opt-in, carried by the isolation module.

Boy scout rule: added to the pool but deliberately not a default, because for agents it pulls against the higher-priority "no silent scope expansion" (it invites touching adjacent code unasked). Best left opt-in.

Still open: confirm the trimmed default list and any close-call additions; the exact per-principle `applicability` tags are finalised in the data file.

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
- Leave code a little better than you found it (the boy scout rule).

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

Decision (review round 2): B. The user asked for a pick-from-the-list UI that ships sane defaults, and an interactive multi-select over the OQ-5 pool and the module set delivers exactly that. Requirement: interactivity is a convenience layer, never the only path; a non-interactive route (flags or a written config) and a recorded record of what was selected must always exist, so the tool stays scriptable, reproducible, and idempotent (principles 2 and 4). One UI covers both principles and modules.

Tooling decision (review round 3): expose both a CLI and a TUI. The CLI (the non-interactive path) uses `clap`, the standard Rust choice. The TUI (the interactive path) uses `ratatui`, which is the modern, maintained Rust TUI library (the successor to `tui-rs`); `iocraft` is a newer declarative, React-style alternative worth a look but far less mature, and `cursive` is a higher-level retained-mode option. Decision (review round 6): `clap` for the CLI and `ratatui` for the TUI, committing to `ratatui` up front rather than starting with a prompt library such as `inquire`. A prompt library would be less code for a bare multi-select, but the user wants a genuine TUI, and `ratatui` is the maintained standard for that. Choosing `clap` and `ratatui` confirms the tool is implemented in Rust, consistent with OQ-1.

TUI requirements (review round 7): the principle selector must let the user (a) toggle whether a principle is included, (b) move the cursor to select the currently active included principle, and (c) reorder that active included principle up and down. The output list is a numbered list; reordering overrides the `default_order` sort for that project. The reorder behaviour applies to principles; modules are toggle-only. Implementation is deferred: pause before building the TUI and check in with the user first.

## Implementation Steps

Form is decided (OQ-1): a standalone Rust binary (clap CLI plus a Rust TUI) that runs without Nix, distributed via `cargo install` or prebuilt binaries, with an optional flake app for Nix users; Nix is used only for the development environment. Template source is decided (OQ-7 approach B): a built-in default pack plus optional `--template <ref>`. Ownership is decided (OQ-3): two-tier, with tool-owned namespaced reference assets always refreshed and user working files create-if-absent (`--force` to overwrite).

Folded decisions: the kit's own reference assets (plan template, prompt library, principle data) live in a namespaced directory by default, always refreshed, with an `--output-dir` override so the tool can run from any directory and write to any target; the root `AGENTS.md` is create-if-absent and the plan template is authored under `docs/plans/`. The optional module set is confirmed: a diagram prompt pack, container and worktree isolation (integrating agent-box and agent-images), a justfile of recipes, and language starters.

### 1. Author the minimal core assets and principle data

Status: complete. Wrote the canonical `AGENTS.md` guidance (with a `{{principles}}` placeholder), the plan-document template, the three core prompts, and the principle data as structured TOML (48 principles, 21 default), parsed and tested. Content policy: the shipped prompts and guidance stay principle-agnostic and defer to the user-selected principle set in `AGENTS.md`, rather than baking in specific opinions a user may not have chosen (the open-questions gate was corrected to follow this).

### 2. Proof-of-concept the file-dropper

Status: complete. The Rust binary drops the asset set under `--output-dir`, refreshing tool-owned reference assets under `.agents/`, creating absent working files (root `AGENTS.md`, `docs/plans/TEMPLATE.md`), and leaving existing working files untouched unless `--force` is given. `AGENTS.md` is generated by rendering the selected principles into the pack guidance template. Covered by unit tests.

### 3. Idempotency and safety pass

Status: complete. Re-run refreshes reference assets and skips existing working files (tests cover create, refresh-versus-skip, and `--force` overwrite); `--output-dir` targets any directory.

### 4. Selection UI

Status: in progress. The non-interactive path is done: `--principles default|all|none|<ids>` selects the set and `--principle-detail name|summary|full` sets the rendering, both feeding generation and `--list-principles`. The interactive TUI (ratatui: toggle inclusion, select the active included principle, reorder it) is not started, and is the agreed pause point for a check-in with the user before implementation.

### 5. Optional modules

Status: not started. Package the confirmed modules as opt-in selections, each self-contained, none complicating the core.

### 6. Bring-your-own template support

Status: not started (OQ-7 B decided). Support `--template <ref>`, where `ref` is a local path or a git URL (a Nix flake-ref is an optional extra for Nix users, not required, and the fetch must not depend on Nix), with a small manifest and minimal named-variable substitution; the built-in agent-workflow pack is the default.

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
