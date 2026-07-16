# Q-26 / optional-modules increment 2: the deterministic-checks module config layout and wiring (exploration A)

Scope: the concrete config layout and wiring for the deterministic-checks module, `optional-modules` increment 2. Q-26 is already decided in principle (a declarative `.agents/checks.toml` with `[[check]]` entries; ast-grep first-party; other tools plug in; one source of truth, pluggable, harness-agnostic, no tool dependency). Q-25 is decided (formatters auto-apply at the implementer's verify step; a read-only checks-reviewer runs linters and check-mode formatters into a findings file the triager adjudicates; an optional pre-commit hook as a secondary backstop). This document settles the exact `[[check]]` schema, ownership and seeding, the AGENTS.md guidance-injection mechanism, the checks-reviewer role and its wiring, the pre-commit hook opt-in, and the ast-grep first-party plus plug-in story. It is design notes, not an implementation, and it edits nothing outside itself.

Grounding read: the built-in pack (`pack/pack.toml`, `pack/AGENTS.md`, `pack/instrument.md`, the role prompts under `pack/prompts/`), the manifest module and variable machinery in `src/manifest.rs` (the `module` tag on assets and variables, the `[[module]]` metadata section, `RESERVED_VARS`, and `render` which normalises to a single trailing newline via `format!("{}\n", out.trim_end())`), the `{{instrument}}` builtin-variable path in `src/main.rs` (`build_assets`, and the byte-identity tests `instrument_off_omits_the_block_and_on_includes_it` and `rendered_agents_ends_with_a_single_trailing_newline`), and both test-modules explorations (`test-driven-and-mutation-A.md`, `-B.md`) whose `kind = "test"` / `kind = "mutation"` needs and shared injection-slot request this design must accommodate.

One decision drives the rest: the checks module owns ONE config schema (checks.toml) and ONE new read-only role (checks-reviewer), and it reuses the existing findings/triage/convergence pipeline and the existing `{{instrument}}`-style render mechanism wholesale. Every later module (test-driven, mutation) contributes rows to the one checks.toml and reuses the one role; nothing is duplicated (Principle 1, Principle 2).

---

## 1. The `[[check]]` schema and the TOML sketch

The checks module OWNS the full checks.toml schema, including the kinds only later modules activate (Principle 1: one source for the schema, no drift). Increment 2 ACTIVATES `kind = "lint"` and `kind = "format"`; it RESERVES `kind = "test"` and `kind = "mutation"` and ships them as commented example rows (inline schema documentation), so the schema is defined once here and the test-driven / mutation modules add no schema, only guidance to uncomment and fill a row.

Fields (a single flat `[[check]]` table array; unknown keys tolerated so older/newer readers coexist, matching the manifest's existing forward-compatibility posture):

- `name` (required): a stable identifier for the check, shown in findings ("check `ast-grep` reported ..."), and the handle a project or a later module references. Making it required buys legible findings for free.
- `kind` (required): one of `lint`, `format`, `test`, `mutation`. This is the ONLY dimension the workflow branches on; it decides who runs the check and how (below).
- `command` (required): the canonical action for the check. For `lint` it DETECTS (read-only). For `format` it APPLIES (rewrites files). For `test` it RUNS the suite. For `mutation` it runs the mutation tool.
- `check` (optional): a NON-mutating verify variant that the read-only checks-reviewer runs without touching the tree. For `format` it is the dry-run that exits non-zero when the tree is not formatted (so the reviewer can confirm formatting was applied without applying it). For `test` it is the compile-only command (exit 0 means "it builds", which lets the red/green gate distinguish "compiles and fails" from "does not compile"). For `lint` it is omitted, because `command` is already read-only (the reviewer runs `command`). For `mutation` it is omitted (the mutation run IS the check).
- `paths` (optional): an array of globs scoping the check, and, for `kind = "test"`, scoping the frozen-tests tripwire the test-driven module relies on.
- `budget` (optional, `mutation` only): a wall-clock cap such as `"10m"`; a timeout yields a partial result, not a hard failure.
- `threshold` (optional, `mutation` only): the maximum surviving mutants tolerated before a finding is raised.

How lint versus format are distinguished and run, precisely (this is the load-bearing part of Q-25):

- `kind = "format"`: the IMPLEMENTER auto-applies it. At the implementer's verify step, for every `format` check it runs `command` (which rewrites files), generalising today's hardcoded `nix fmt`. This is a WRITER action. The read-only checks-reviewer then runs the SAME check's `check` (the dry-run) and treats a non-zero exit as a finding: this catches an implementer who did not format, WITHOUT the reviewer mutating anything. So a formatter is applied by the writer and verified-in-check-mode by the reviewer, exactly the split Q-25 fixed.
- `kind = "lint"`: the IMPLEMENTER does not "apply" it (linters detect). The checks-reviewer runs `command` (the read-only detector) and turns each reported violation into a finding. The implementer's verify step MAY run the linters to self-correct before handoff, but the authoritative gate is the checks-reviewer's detect pass, so a skipped self-check is still caught.

The red/green expected outcome is NOT a checks.toml field. checks.toml stays a pure declaration of COMMANDS (Principle 1, harness-agnostic). Whether a `test` check is expected to pass (green) or to fail-by-assertion (red) is a PHASE parameter the orchestrator hands the checks-reviewer when it spawns it, per the test-driven module's guidance; the checks-reviewer prompt itself stays generic (it runs the check it is told and compares to the expected outcome it is handed, knowing nothing test-specific). This keeps the file a stable list of commands and keeps the workflow semantics in the workflow prompts, which is where the two later modules put them.

The seeded `.agents/checks.toml` (created if absent; see section 2):

```toml
# .agents/checks.toml
# Deterministic checks for this project. Scaffolded once (create-if-absent);
# this file is yours to edit. Each [[check]] declares a command to run. The
# workflow's checks-reviewer runs the read-only checks and the implementer
# auto-applies formatters at its verify step. This file is a PURE declaration of
# commands: it never encodes WHEN a check runs or with what expected outcome
# (that is a phase/workflow concern), so a test that must "fail first" is not a
# field here.
#
# Fields:
#   name      (required) stable id, shown in findings.
#   kind      (required) one of: lint | format | test | mutation.
#   command   (required) the action. lint: detect (read-only). format: APPLY
#             (rewrites files). test: run the suite. mutation: run the tool.
#   check     (optional) a NON-mutating verify variant the read-only checks-
#             reviewer runs. format: dry-run, exits non-zero if unformatted.
#             test: compile-only, exit 0 => "it builds". lint: omit (command is
#             already read-only). mutation: omit (the run is the check).
#   paths     (optional) globs scoping the check (and, for test, the
#             frozen-tests tripwire).
#   budget    (optional, mutation only) wall-clock cap, e.g. "10m".
#   threshold (optional, mutation only) max surviving mutants before a finding.

# -- First-party linter: ast-grep (structural lint). Rules live under
#    .agents/checks/ast-grep/rules/. ast-grep is privileged only in that this
#    module seeds it; it runs as an ordinary command, no differently from a
#    tool you add below. --
[[check]]
name = "ast-grep"
kind = "lint"
command = "ast-grep scan --config .agents/checks/ast-grep/sgconfig.yml"

# -- Formatter. The implementer runs `command` at verify to APPLY formatting;
#    the checks-reviewer runs `check` (dry-run) to confirm the tree is
#    formatted. Replace `nix fmt` with your project's formatter. --
[[check]]
name = "format"
kind = "format"
command = "nix fmt"
check   = "nix fmt -- --check"

# -- Plug in ANY other linter or formatter identically: add a [[check]] row.
#    A language with no ast-grep grammar plugs in ruff / eslint / clippy the
#    same way ast-grep is wired above. --
# [[check]]
# name = "clippy"
# kind = "lint"
# command = "cargo clippy --all-targets -- -D warnings"

# -- Reserved kinds. Inert in this module; the test-driven and mutation modules
#    tell you to uncomment and fill these. Shown here as schema documentation. --
# [[check]]
# name    = "test"
# kind    = "test"
# command = "cargo test --locked"          # green: expect exit 0 (all pass).
# check   = "cargo test --no-run --locked" # compile-only: exit 0 lets the red/green
#                                          # gate tell "compiles and fails" from
#                                          # "does not compile".
# paths   = ["tests/", "src/**/*.rs"]      # scopes the frozen-tests tripwire.
#
# [[check]]
# name      = "mutation"
# kind      = "mutation"
# command   = "cargo mutants --in-diff HEAD~1" # diff-scoped; language-specific.
# budget    = "10m"
# threshold = 0
```

Note the schema uses ONE `check` field with a per-kind meaning ("the non-mutating verify variant") rather than separate `compile` / `dry_run` fields. This matches the Q-26 wording ("a command, a kind, and a check-mode command") and keeps the row shape uniform: two commands at most, one the action, one the read-only verify. Explorer A of the test-modules pass called this second field `compile` for test entries and Explorer B called it `check` uniformly; I take the uniform `check` because it is one field the checks-reviewer always knows how to treat (run it read-only, compare exit to the handed-in expected outcome), and it does not special-case the test kind in the schema.

---

## 2. Ownership, seeding, and byte-identity

`.agents/checks.toml` is a USER WORKING FILE, `ownership = "working"`, create-if-absent (Principle 3). A project curates its own checks, so the module must never clobber an existing checks.toml. The manifest already models this distinction (`ownership = "working"` versus `"reference"`), so the checks module simply tags the checks.toml asset `working` and `module = "checks"`. Running the scaffolder twice leaves an edited checks.toml untouched (Principle 4).

How it is seeded: when `--module checks` is selected and checks.toml is absent, the module drops the seed above, PRE-POPULATED with two live rows (the ast-grep linter and a formatter) and the commented `clippy`, `test`, and `mutation` examples as inline schema documentation. The two live rows give a working gate out of the box (ast-grep runs once the seeded rule exists; the formatter row uses `nix fmt`, this project's convention per Principle 7, and the comment tells the user to swap it). The commented rows are inert whether or not the test-driven / mutation modules are ever added, so they perturb nothing (Principle 2). A project ADDS entries by editing checks.toml and appending a `[[check]]` row, which is the whole plug-in mechanism.

The full asset set the checks module drops (every one tagged `module = "checks"`, so all are absent unless the module is selected):

- `.agents/checks.toml` (ownership `working`, create-if-absent): the seed above.
- `.agents/checks/ast-grep/sgconfig.yml` (ownership `working`, create-if-absent): the ast-grep config pointing at the rules dir.
- `.agents/checks/ast-grep/rules/no-ascii-escape.yml` (ownership `working`, create-if-absent): one seeded example rule (for example, forbidding non-ASCII bytes in source, generalising the project's own ASCII-clean convention) so ast-grep runs and demonstrates the rule format.
- `.agents/prompts/checks-reviewer.md` (ownership `reference`): the read-only checks-reviewer role prompt (section 4).
- `.agents/hooks/pre-commit` (ownership `reference`): the inert, opt-in pre-commit hook script (section 5).
- `pack/modules/checks.md` (a pack SOURCE, not a dropped asset): the module guidance partial concatenated into the rendered AGENTS.md via the `{{modules}}` slot (section 3). It is single-sourced exactly as `pack/instrument.md` is: read at build time, substituted, never dropped as its own file.

The `[[module]]` metadata section gains one entry:

```toml
[[module]]
name = "checks"
description = "Deterministic lint/format checks gate over the workflow's code output (ast-grep first-party; other linters/formatters plug in via .agents/checks.toml)."
```

Byte-identity when the module is off (Principle 2): every dropped asset above is `module = "checks"`-tagged, so with no `--module checks` NONE of them drop, and the loader's output is unchanged. The only core file touched is `pack/AGENTS.md`, which gains a `{{modules}}` slot that renders to the EMPTY string when no module is selected; `render`'s `format!("{}\n", out.trim_end())` collapses the empty slot so the rendered AGENTS.md is byte-for-byte identical to today's (this is exactly the `{{instrument}}` precedent, already guarded by `instrument_off_omits_the_block_and_on_includes_it` and `rendered_agents_ends_with_a_single_trailing_newline`). So the no-module scaffold stays byte-identical, and the byte-identity invariant is provable by the same test the instrument slot already ships.

---

## 3. The AGENTS.md guidance-injection slot (the decision both test-modules explorers requested)

Decision: a `{{modules}}` RENDER SLOT, computed exactly like `{{instrument}}`, and it is a CHECKS-INCREMENT deliverable (not an increment-1 retrofit). I reject the generic `.agents/modules/` hook (the strongest alternative, taken by test-modules Explorer B) for the reasons below.

Mechanism. `pack/AGENTS.md` gets a `{{modules}}` line (placed after `{{instrument}}`, so both trailing slots collapse when empty). `src/main.rs::build_assets` computes a `modules` builtin variable the same way it computes `instrument`: for each selected module, read `modules/<name>.md` from the pack source (empty if the file is absent, so a pure-content module that ships no guidance is fine), concatenate them, and insert the result as the `modules` builtin. Add `"modules"` to `RESERVED_VARS` beside `"principles"` and `"instrument"`. The concatenation order is the module DECLARATION order in the `[[module]]` section, NOT the `--module` flag order, so `--module a --module b` and `--module b --module a` render an identical AGENTS.md (Principle 4, idempotent/reproducible). The checks module's `modules/checks.md` partial opens with a `## Deterministic checks` heading and carries: the implementer's verify-step formatter-apply duty, the checks-reviewer's role and when the orchestrator spawns it, how checks findings compose with the LLM reviewers' and the triager's, and a pointer to `.agents/checks.toml` and `.agents/prompts/checks-reviewer.md`.

Why this over the generic `.agents/modules/` hook. Both preserve byte-identity, but they preserve it differently, and that difference decides it:

- The `{{modules}}` slot preserves byte-identity of the RENDERED core output: with no module the slot is empty and the AGENTS.md bytes are unchanged from today. Nothing new appears in a no-module scaffold at all.
- The generic-hook alternative requires a PERMANENT, always-present sentence in core AGENTS.md ("when a module is selected it drops guidance under `.agents/modules/`; read what is present") plus, per Explorer B, a matching sentence in the orchestrator prompt. That sentence ships in EVERY scaffold, module or not, so it changes the core rendered text that a no-module user sees, forever. Principle 2 says adding a module must not change core behaviour when unused; a slot that renders empty honours that literally, whereas a permanent hook sentence adds standing core text for a feature the user did not opt into.

On Principle 1 the slot also wins: it reuses the proven `{{instrument}}` builtin-variable path (one small addition mirroring code that already exists and is already byte-identity-tested), and it single-sources each module's guidance in one partial that is concatenated in, with no second copy and no discovery/glob step at agent runtime. The generic hook needs the orchestrator to glob `.agents/modules/` and read whatever is there, which is more moving parts for less coherence (the guidance is split across N dropped files the orchestrator must find, versus one rendered AGENTS.md every role already reads first).

This slot is FIRST needed here. Q-25 requires the checks module to add "workflow guidance wiring it in", and there is no way to add per-fragment guidance to the single rendered AGENTS.md without either a slot or a hook; the checks module is the first module that must inject guidance (increment 1 shipped only the machinery and could serve pure-content modules that just drop files). So the `{{modules}}` slot is built as part of increment 2, and by the time test-driven and mutation are built it already exists and they each just add a `modules/<name>.md` partial. That is the shared injection mechanism both test-modules explorers asked to be settled; it is settled as the `{{modules}}` slot.

Roles are injected DIFFERENTLY from guidance, and cleanly: a module's role prompt (here `checks-reviewer.md`) is an ordinary `module = "checks"`-tagged asset dropped under `.agents/prompts/`, exactly like the core role prompts. The `{{modules}}` guidance tells the orchestrator when to spawn it; the prompt file itself is a normal dropped asset. So guidance goes through the slot and roles go through the asset mechanism, and neither touches a core file's rendered bytes when the module is off.

---

## 4. The checks-reviewer role and its wiring

The role prompt `.agents/prompts/checks-reviewer.md` (outline, in the same register as the existing `reviewer.md` / `triager.md`):

- Identity: you are a read-only deterministic checks reviewer. You RUN the checks declared in `.agents/checks.toml` and REPORT what they find. You do not fix anything and you do not adjudicate (the implementer fixes; the triager adjudicates).
- Inputs the orchestrator hands you: the diff range (before/after commit hashes) under review; WHICH check kinds to run this round (for a normal work-review: `lint` and `format`; the test-driven and mutation modules hand you `test` and `mutation` when active); and the EXPECTED OUTCOME per phase (default: lint clean, format clean; the test-driven module hands you the red or green expectation for a `test` check). Read AGENTS.md (including the injected checks guidance) and checks.toml first.
- Procedure: for each applicable `[[check]]`, run its READ-ONLY variant and never mutate the tree. For `lint` run `command`. For `format` run `check` (the dry-run) and NEVER `command` (which would rewrite files). For `test` run `check` (compile) then `command`, comparing exit codes to the handed-in red/green expectation. Capture exit code and tool output.
- Findings: a check whose actual outcome differs from the expected outcome is a finding. Cite the check `name`, the exact command, the exit code, and the offending file and line from the tool's own output (deterministic evidence, not prose). Rate severity by impact on the same four-level scale the LLM reviewers use. Write findings to the findings-file path the orchestrator assigned under `docs/plans/<task>.reviews/`, one entry per finding, same convention as `reviewer.md`; if everything passed, say so in the file. Your reply may summarise; the file is the record.

Wiring into the work-review phase. The orchestrator spawns the checks-reviewer as ONE reviewer alongside the LLM reviewers in the existing work-review round, handing it the diff range and the kinds/expected-outcome for the phase. Its findings file sits beside the LLM reviewers' files under `docs/plans/<task>.reviews/`. The orchestrator itself never runs a check or produces a finding (its role stays clean: it spawns the checks-reviewer like any reviewer, exactly the rationale Q-25 records). The separate triager then reads ALL findings files (LLM and checks) and adjudicates uniformly: a deterministic finding carries tool evidence (exit code plus tool output) so it is near-always valid, but the triager still rules (it may accept or suppress a known false-positive lint with recorded reasoning, and route a suppression rule rather than a code fix), and the high/critical dismissal-recheck backstop applies to a dismissed deterministic finding just as to an LLM one. Valid findings route to the implementer, who fixes them; the loop re-verifies under the normal convergence rule. So deterministic findings compose with LLM findings through the SAME findings/triage/convergence pipeline with zero new machinery (Principle 1, Principle 2): the checks-reviewer is just another reviewer whose findings happen to be tool-generated.

This is the direct answer to "how do deterministic findings compose": they are ordinary findings-file entries adjudicated by the same independent triager, differing only in that their evidence is a tool exit code rather than a reviewer's argument, which makes them the easiest findings to adjudicate, not a parallel track.

---

## 5. The optional pre-commit hook opt-in

The hook is SECONDARY to the review-integrated path and OPT-IN. The module drops an INERT hook script at `.agents/hooks/pre-commit` (namespaced, not installed into `.git/hooks/`, so it changes no git behaviour and cannot clobber an existing hook, Principle 3). Two opt-in activations, in ascending convenience:

- Documented one-liner: the checks guidance shows the user how to activate it, for example `ln -s ../../.agents/hooks/pre-commit .git/hooks/pre-commit` (a symlink so it tracks edits), guarded by a note to check for an existing hook first.
- A `--with-precommit-hook` flag on the `scaffold` verb that INSTALLS the hook create-if-absent: it writes `.git/hooks/pre-commit` only if none exists, and if one exists it prints the manual instructions instead of clobbering (Principle 3). The flag is meaningful only with `--module checks`; requesting it without the module is a usage error (Principle 5, do not admit the incoherent state). (Whether this is a bare flag or, later, a checkbox in the deferred TUI module pane is a surface detail; the flag is the increment-2 form.)

What the hook runs: the `lint` detect commands and the `format` `check` (dry-run) commands from checks.toml over the staged tree, failing the commit on any violation. It does NOT run `test` or `mutation` (too slow for a commit hook) unless the user adds them deliberately.

How the hook stays single-sourced without a workflow tool dependency. The hook must not inline the commands (that would drift from checks.toml, against Principle 1), so it DELEGATES to a runner that reads checks.toml. The recommended runner is `agent-scaffold checks --staged` (a thin subcommand that reads checks.toml and runs the read-only lint/format checks over the staged files), which is drift-free and single-sourced, and is consistent with the already-planned direction of running `agent-scaffold validate --workflow` from the checks pre-commit hook (Q-27). The important line: harness-agnosticism forbids the AGENT WORKFLOW from depending on the binary (and it does not: the checks-reviewer is an LLM agent that reads checks.toml and runs the raw commands itself, and checks.toml is readable and runnable by any human or agent without the tool). A LOCAL git pre-commit hook is a developer-machine convenience the developer explicitly opted into, on a machine that already has the tool installed to scaffold; letting THAT use the binary does not make the workflow depend on it. Fallback for a project that wants zero binary use even in the hook: delegate to `just check` when the justfile module is present, or list the commands in the hook and accept the documented drift risk. I recommend the `agent-scaffold checks --staged` runner as primary because drift-free single-sourcing (Principle 1) outweighs adding a small read-only subcommand, and I flag the binary-at-commit-time as the one trade-off, scoped so the LLM workflow path stays binary-free.

---

## 6. ast-grep as first-party, and the identical plug-in story

ast-grep is first-party at SCAFFOLD TIME only, never at workflow RUNTIME. The module seeds ast-grep's own config and one example rule so a project has a working structural linter immediately:

- `.agents/checks/ast-grep/sgconfig.yml`: the ast-grep project config, pointing `ruleDirs` at `.agents/checks/ast-grep/rules/`.
- `.agents/checks/ast-grep/rules/no-ascii-escape.yml`: one seeded example rule demonstrating the rule format (for example, forbidding non-ASCII source bytes, generalising the project's ASCII-clean convention).
- A single checks.toml row: `name = "ast-grep"`, `kind = "lint"`, `command = "ast-grep scan --config .agents/checks/ast-grep/sgconfig.yml"`.

The decisive point for harness-agnosticism: ast-grep is registered EXACTLY like any other tool, as a `[[check]]` row naming a command. There is nothing ast-grep-special in how the workflow RUNS it; the checks-reviewer just runs the command in the row. ast-grep is privileged only in that this module pre-seeds its rules scaffold and its row, whereas any other tool the user adds themselves. A language with no ast-grep grammar plugs in `ruff`, `eslint`, `clippy`, or a formatter identically: append a `[[check]]` row with the tool's command (and a `check` dry-run for a formatter). So the extensibility story is uniform: one row per tool, ast-grep included, and the workflow-runtime mechanism (the checks-reviewer running commands from checks.toml) knows nothing about which tool it is running. That is precisely the "one source of truth, pluggable, harness-agnostic, no tool dependency" the Q-26 decision asked for.

The seeded ast-grep config and rule are `ownership = "working"` (create-if-absent), because a project curates its own rules; running twice never clobbers an edited rules dir (Principle 3, Principle 4).

---

## 7. Recommendation

Ship the checks module (`--module checks`) as: one user-owned, create-if-absent `.agents/checks.toml` whose `[[check]]` schema is `{ name, kind, command, check?, paths?, budget?, threshold? }` with `kind` in `{lint, format, test, mutation}` (lint/format active, test/mutation reserved as commented example rows the checks module owns as the single schema source); formatters auto-applied by the implementer at verify (`command`) and verified in dry-run by the read-only checks-reviewer (`check`), linters detected by the checks-reviewer (`command`); a seeded first-party ast-grep config plus rule and a formatter row, with every other tool plugging in as an identical `[[check]]` row; a new read-only `.agents/prompts/checks-reviewer.md` role the orchestrator spawns as one reviewer in the work-review round, whose tool-evidenced findings the independent triager adjudicates alongside the LLM reviewers' through the existing findings/triage/convergence pipeline; module guidance injected via a NEW `{{modules}}` render slot in core AGENTS.md computed exactly like `{{instrument}}` (empty when no module -> byte-identical core, concatenated in `[[module]]`-declaration order when modules are selected), delivered as part of increment 2; and an inert, opt-in `.agents/hooks/pre-commit` backstop activated by a `--with-precommit-hook` flag (create-if-absent) or a documented one-liner, delegating to `agent-scaffold checks --staged` to stay single-sourced. Add a `requires` field to `[[module]]` here (first used by the pre-commit sub-option and by the later test-driven -> checks and mutation -> test-driven dependencies) so module dependencies are single-sourced in the manifest.

Why this design against the principles. Principle 1: one schema in one file, one role, one injection mechanism, no duplication and no drift, and the red/green semantics kept OUT of the config (a phase parameter) so the config is a pure command list. Principle 2: every asset is module-tagged so the no-module scaffold drops nothing, and the `{{modules}}` slot renders empty to keep AGENTS.md byte-identical, provable by the existing instrument byte-identity tests. Principle 3: checks.toml and the ast-grep rules are create-if-absent user working files, and the pre-commit hook never clobbers an existing hook. Principle 4: declaration-order concatenation and create-if-absent seeding make repeated runs idempotent. Principle 5: the pre-commit flag is meaningless without the checks module (a usage error, not an admitted bad state), and `requires` auto-resolution keeps dependent modules from being selected incoherently. Principle 6 / the verify-don't-trust ethos: deterministic tools establish the mechanical facts (lint clean, formatted, compiles-and-fails) and the LLM triager judges only genuine judgment. Harness-agnosticism: the workflow runs raw commands from checks.toml with no binary, and the only binary use is the opt-in local pre-commit hook, a developer convenience outside the workflow path.

The riskiest assumption: that a single `check` field with a per-kind meaning ("the read-only verify variant") is expressive enough for every current and reserved kind without a third command or a per-kind sub-schema. It holds for lint (omit), format (dry-run), test (compile), and mutation (omit) as sketched, but a future kind that needs BOTH a compile step AND a separate dry-run, or a check whose verify variant is not a single command, would strain it. I judge the risk low because the four kinds cover the decided roadmap and unknown keys are tolerated (a later kind can add its own optional field without breaking readers), but if a kind needs two distinct verify commands the schema grows a field rather than reinterpreting `check`, and that is the first place this design would bend.

---

## 8. Rejected alternatives

- The generic `.agents/modules/` hook for guidance injection (the strongest alternative; test-modules Explorer B's choice). A permanent module-agnostic sentence in core AGENTS.md (and the orchestrator prompt) tells the orchestrator to read whatever guidance is dropped under `.agents/modules/`, and each module drops its guidance there as a normal asset. Rejected in favour of the `{{modules}}` slot: the hook adds STANDING core text present in every scaffold whether or not a module is used (a weaker reading of Principle 2 than a slot that renders empty), and it splits guidance across N discovered files with a runtime glob step, versus one rendered AGENTS.md every role already reads first (Principle 1). The slot also reuses the already-tested `{{instrument}}` path, inventing almost nothing. The hook's one genuine merit (guidance as fully self-contained dropped files) is not worth the always-present core text and the discovery step.

- Encoding the red/green expected outcome in checks.toml (a per-check `expect = "fail"` field, or a duplicate "red" copy of the test check). Rejected: it couples the pure command declaration to workflow phase semantics and needs per-phase overrides, drifting the config. The expected outcome is a phase parameter the orchestrator hands the checks-reviewer; checks.toml stays a harness-agnostic command list (Principle 1). This is also what keeps the checks-reviewer prompt generic and reusable by the test-driven and mutation modules unchanged.

- Separate `compile` and `dry_run` fields instead of one `check`. Rejected: it special-cases the test kind in the schema and adds a field most rows never use, for no expressiveness the single `check` lacks across the decided kinds. The uniform `check` (the read-only verify variant) is one field the checks-reviewer always handles the same way. (This is the exact seam the riskiest-assumption note watches; if a kind ever needs both, the schema adds a field rather than retrofitting `check`.)

- A tool-owned reference checks.toml (ownership `reference`) rewritten on every scaffold. Rejected: it violates create-if-absent and would clobber a project's curated checks (Principle 3), and it fights idempotence (Principle 4). checks.toml must be a user working file; the tool seeds it once and never again.

- ast-grep as a special-cased first-party integration in the tool (a dedicated `ast-grep` code path, native `sgconfig.yml`-plus-guidance rather than a checks.toml row). Rejected (this was Q-26's declined option c): it makes ast-grep privileged at RUNTIME and makes non-ast-grep tools second-class, breaking the uniform "one row per tool" plug-in story and the harness-agnostic "just run the command" model. ast-grep is first-party only in being pre-seeded; at runtime it is one row like any other.

- A shell pre-commit hook that inlines the check commands (no delegation). Rejected: the inlined commands drift from checks.toml the moment the user edits a check (Principle 1). The hook delegates to a runner that reads checks.toml so there is one source; the accepted cost is the runner (recommended: `agent-scaffold checks --staged`), scoped to the opt-in local hook so the LLM workflow path stays binary-free.

- Auto-installing the pre-commit hook into `.git/hooks/` by default. Rejected: it clobbers an existing hook and changes git behaviour a user did not ask for (Principle 3). The hook drops inert and namespaced; activation is explicit (a flag that installs create-if-absent, or a documented one-liner), and it stays strictly secondary to the review-integrated checks-reviewer, which is the authoritative gate.
