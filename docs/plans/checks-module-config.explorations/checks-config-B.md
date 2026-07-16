# optional-modules increment 2 (deterministic-checks): config layout and wiring, exploration B

Explorer B. Independent design pass for the deterministic-checks module's CONFIG LAYOUT and WIRING, owed at the start of `optional-modules` increment 2 once `Q-26` decided option (a): a declarative `.agents/checks.toml` with `[[check]]` entries (a command, a kind, a check-mode command), ast-grep first-party, pluggable, harness-agnostic, no runtime tool dependency. The `--module` machinery (increment 1) is already built and shipped: the hybrid schema (asset `module = "<name>"` tags plus a `[[module]]` metadata section), the repeatable `--module` flag, and the dangling-tag validation all exist in `src/manifest.rs`, and the byte-identity invariant is enforced by tests in `src/main.rs`. This document decides the `[[check]]` schema, ownership and seeding, the AGENTS.md injection slot (the genuinely-open question), the checks-reviewer role and its wiring, the optional pre-commit hook, and ast-grep-plus-plug-in parity, judged against the numbered Project Principles. It is one explorer's take, to be synthesised against the other's.

I read the increment-1 machinery (`src/manifest.rs` `load`, the `AssetSpec`/`ModuleSpec`/`VarSpec` structs, the reserved-builtin handling), the `{{instrument}}` render slot and its byte-identity tests (`src/main.rs` `instrument_off_omits_the_block_and_on_includes_it`, `rendered_agents_ends_with_a_single_trailing_newline`), the `render()` trailing-newline normalisation in `src/manifest.rs`, `pack/pack.toml`, `pack/AGENTS.md` (the Phases list, the work-review phase, the writer-isolation and file-safety rules), `pack/prompts/{orchestrator,reviewer,implementer}.md`, and both test-module explorations (`test-driven-and-mutation-A.md`, `-B.md`) whose `kind = "test"` and `kind = "mutation"` needs this schema must reserve room for. The recommendations are grounded in those, not invented.

## 1. The `[[check]]` schema and the TOML

### 1.1 Fields

A check is a pure declaration of HOW to run one deterministic check. WHEN it runs, with WHAT expected outcome (pass vs fail), and against which artifact are decided by the WORKFLOW (the implementer's verify step, the checks-reviewer, the phase the orchestrator spawns it in), NOT by this file. That separation is the load-bearing design choice and it is what keeps the config harness-agnostic and single-sourced (Principle 1). Fields:

- `name` (required, string, unique across the file): the check's id. Used in findings ("check `nixfmt` failed"), in the runner, and to reference a check in guidance. Uniqueness makes findings unambiguous (Principle 5, make illegal states unrepresentable: a duplicate name is a load-time error, not a runtime ambiguity).
- `kind` (required, enum): one of `lint`, `format`, `test`, `mutation`. The kind alone determines the execution semantics (detect-only vs auto-apply-plus-verify vs run-and-expect-pass vs mutate). Encoding the behaviour on a closed enum rather than on free-form booleans (`applies = true`, `detects = true`) makes the incoherent combinations unrepresentable (Principle 5): a check cannot be both apply-only and detect-only.
- `command` (required, string): the primary command, run from the repo root via the shell. Its meaning per kind: lint = the detector (non-zero exit or structured output means findings); format = the APPLY command (rewrites files in place); test = the RUN command (expect exit 0 for the green gate); mutation = the mutate-and-report command (surviving mutants are findings).
- `check` (optional, string): the secondary, NON-MUTATING command. Its meaning per kind: format = the DRY-RUN / check-mode detector (exit non-zero if files are unformatted, so the reviewer can verify formatting WITHOUT applying); test = the COMPILE-ONLY command (build the tests without running them, so the workflow can tell "compiles and fails" from "does not compile"); lint and mutation do not use it (lint's `command` is already non-mutating; a mutation run is its own check). One field name across all kinds (not `check` for format and `compile` for test) is deliberate: it keeps the schema and the runner uniform, and the meaning-by-kind is documented once (Principle 1). This reconciles the two test-module explorations, which named this field `check` (B) and `compile` (A) for the test case; `check` is the single name.
- `paths` (optional, array of globs): scopes the check to matching files. For lint/format it limits what is scanned or formatted; for test it additionally scopes the test-driven module's frozen-tests tripwire (the git-diff over test paths). Absent means the check's own default scope (usually the whole tree).
- `budget` (optional, string duration, mutation only): a wall-clock cap. A timeout yields a PARTIAL result (mutants tested so far, the rest reported untested), never a hard failure, so an expensive gate stays affordable and never blocks a step on an unbounded run.

That is the whole schema. There is NO field for the pass/fail expectation, no `expect = "fail"`, no red/green flag, and no per-check severity. The red/green test expectation is a PHASE PARAMETER the orchestrator hands the checks-reviewer ("evaluate the test check under the red profile"), not a config field, exactly as both test-module explorations require; putting it in checks.toml would couple a pure command declaration to workflow semantics and force per-phase overrides that drift (Principle 1). Severity is assigned by the checks-reviewer when it writes the finding (the same four-level scale the LLM reviewers use), not pinned in config, so the triager adjudicates deterministic and LLM findings on one scale.

### 1.2 How lint (detect) and format (auto-apply plus check-mode) are modelled and run

The lint-versus-format distinction is entirely carried by `kind`, and it maps onto the three consumers of the config:

- A `kind = "lint"` check is DETECT-ONLY. It is never applied. The checks-reviewer runs its `command` in the work-review phase; a non-zero exit (or structured findings output) becomes a finding. The implementer never runs it as a mutating step (a linter that also auto-fixes is still declared as a lint whose `command` only reports; if a project wants auto-fix-on-format semantics for a tool like `ruff --fix`, that is a separate `kind = "format"` entry pointing at the fix command, with a `check` pointing at the report-only mode).
- A `kind = "format"` check is APPLY-plus-VERIFY. The implementer runs its `command` (apply) during its verify step, scoped to the files it changed (see 7.2 on the reformat-untouched-files hazard); this generalises today's hardcoded `nix fmt`. The checks-reviewer runs its `check` (dry-run) read-only in the work-review phase; a non-zero `check` exit means "the implementer left unformatted files", raised as a finding. If a format entry has no `check` command, the reviewer MUST NOT run `command` (that would modify files and break the reviewer's read-only contract); it records a degraded finding ("format check `X` has no check-mode command; cannot verify without applying") so the gap is visible rather than silently skipped.

### 1.3 The seeded checks.toml (ast-grep and a formatter pre-populated, reserved kinds shipped as commented example rows)

```toml
# .agents/checks.toml
#
# Declarative deterministic checks the agent workflow runs over your code output.
# This file is YOURS: created only if absent, never overwritten by re-running the
# scaffolder. Add, edit, or remove entries freely.
#
# Each [[check]] declares HOW to run one check. WHEN it runs and WHAT exit code is
# expected are decided by the workflow, not here: this file is a pure command
# declaration, so the same entry serves the implementer's verify step, the
# checks-reviewer, and the optional pre-commit hook without duplication.
#
# Fields:
#   name    (required, unique) id used in findings and guidance.
#   kind    (required)         lint | format | test | mutation.
#   command (required)         the primary command, run from the repo root.
#   check   (optional)         the non-mutating secondary command:
#                                format -> dry-run detector (exit != 0 if unformatted)
#                                test   -> compile-only (build tests, do not run)
#   paths   (optional)         globs scoping the check.
#   budget  (optional)         mutation only: a wall-clock cap (partial on timeout).

# -- lint: detect problems, never modify files. Run by the checks-reviewer. --
# ast-grep is the first-party linter. Rules live under .agents/ast-grep/rules/
# (see sgconfig.yml, also dropped by this module). Other linters plug in
# identically: add a [[check]] with kind = "lint" and their command.
[[check]]
name    = "ast-grep"
kind    = "lint"
command = "ast-grep scan"
paths   = ["src/"]

# -- format: normalise files. Auto-applied by the implementer, verified in --
# -- check-mode by the checks-reviewer. Replace with your project's formatter. --
[[check]]
name    = "format"
kind    = "format"
command = "nix fmt"
check   = "nix fmt -- --check"

# -- Reserved kinds. The checks module owns the whole checks.toml schema and --
# -- reserves these so the test-driven and mutation modules plug straight in. --
# -- They are inert until uncommented; leaving them commented changes nothing. --

# test: run a suite and expect exit 0 (the green gate). The optional `check`
# compiles the tests only, so the workflow can distinguish "compiles and fails"
# from "does not compile" (the test-driven module's red gate needs this). The
# red-vs-green expectation is NOT a field here; it is a phase parameter the
# orchestrator passes the checks-reviewer.
# [[check]]
# name    = "test"
# kind    = "test"
# command = "cargo test --locked"          # run: expect exit 0 (green gate)
# check   = "cargo test --no-run --locked" # compile-only: expect exit 0
# paths   = ["tests/", "src/**/*_test.rs"]  # scopes the frozen-tests tripwire

# mutation: mutate the code and confirm the tests kill the mutants. Expensive;
# scope it to the diff and time-box it. Consumed by the mutation module.
# [[check]]
# name    = "mutation"
# kind    = "mutation"
# command = "cargo mutants --in-diff HEAD"  # surviving mutants -> findings
# budget  = "10m"                            # timeout -> partial result, not a fail
```

The two reserved kinds ship commented, as inline schema documentation, so a reader learns the full shape (Principle 1: one place documents the schema) and adding the test-driven or mutation module is one uncomment away. Commented rows are inert whether or not those later modules are selected, so they perturb nothing (Principle 2).

## 2. Ownership, seeding, and byte-identity

### 2.1 checks.toml is a module-tagged, create-if-absent user working file

In `pack.toml`:

```toml
[[module]]
name        = "checks"
description = "Deterministic lint/format checks over code output, with a checks-reviewer and an optional pre-commit hook."
guidance    = "guidance/checks.md"   # see section 3

[[asset]]
source     = "checks/checks.toml"
dest       = ".agents/checks.toml"
ownership  = "working"               # create-if-absent, never clobbered (Principle 3)
module     = "checks"

[[asset]]
source     = "checks/sgconfig.yml"
dest       = "sgconfig.yml"
ownership  = "working"               # ast-grep's config lives at the repo root
module     = "checks"

[[asset]]
source     = "checks/ast-grep/rules/example.yml"
dest       = ".agents/ast-grep/rules/example.yml"
ownership  = "reference"             # a scaffolded example rule, tool-owned
module     = "checks"

[[asset]]
source     = "checks/run-checks.sh"
dest       = ".agents/checks/run-checks.sh"
ownership  = "reference"             # the single execution engine (section 5/6)
module     = "checks"

[[asset]]
source     = "prompts/checks-reviewer.md"
dest       = ".agents/prompts/checks-reviewer.md"
ownership  = "reference"
module     = "checks"
```

Every asset carries `module = "checks"`, so with no `--module checks` none of them drop and the scaffold output is byte-identical to today's core (Principle 2, and the invariant is already enforced by the increment-1 loader: an unselected module's tagged assets are skipped). checks.toml itself is `ownership = "working"`: it is a user file the tool creates only if absent and NEVER overwrites (Principle 3), so a project that has hand-edited its checks (added ruff, removed the ast-grep example) keeps its edits across re-runs, and running the scaffolder twice is a no-op on it (Principle 4). The pre-populated ast-grep and formatter rows are a STARTING POINT, not a tool-managed block: the tool has no claim on the file after it is created.

### 2.2 How a project extends it

A project adds a `[[check]]` with its tool's command and a kind. That is the entire extension mechanism, and it is why ast-grep is not privileged: `ruff`, `eslint`, `clippy`, `gofmt`, `prettier` are each one `[[check]]` (see section 6). The seeded ast-grep and format rows can be edited or deleted; the reserved test/mutation rows uncommented. Because the file is user-owned, the tool never reconciles or migrates it; the schema is forward-compatible (the runner and the reviewer ignore unknown kinds gracefully, treating an unrecognised kind as "run `command`, report non-zero", so a future kind does not break an older runner).

### 2.3 Byte-identity when off

Two things must stay byte-identical when `--module checks` is absent: the dropped file SET (handled by the module tags above, already load-tested) and the rendered AGENTS.md (handled by the injection slot, section 3). No core asset gains a checks-specific fragment; the checks-reviewer is a new file, not an edit to `orchestrator.md` or `reviewer.md`; checks.toml, sgconfig.yml, the runner, the rule, and the guidance partial are all new, module-tagged files. Nothing in the core changes shape (Principle 2).

## 3. The injection-slot decision (the genuinely-open question)

The problem: a module must add workflow guidance to the SINGLE rendered AGENTS.md, but AGENTS.md is one asset and cannot be module-tagged per fragment. Two candidate mechanisms were named:

- A `{{modules}}`/`{{instrument}}`-style render slot: a reserved builtin variable in AGENTS.md that the tool substitutes with the concatenated guidance partials of the enabled modules, empty when none.
- A generic increment-1 hook: a permanent, module-agnostic sentence in core AGENTS.md telling the orchestrator "when a module is selected it drops its guidance under `.agents/modules/`; read what is present", plus each module dropping a `.agents/modules/<name>.md` file.

### 3.1 Decision: the `{{modules}}` render slot, shipped as the FIRST deliverable of increment 2 (generic, reusable)

I recommend the render slot, and I recommend building it as a GENERIC mechanism at the head of increment 2 (not retrofitting increment 1, which is already shipped, and not making it checks-specific). Reasons:

- It reuses a mechanism that already exists and is already byte-identity-tested. `{{instrument}}` is a reserved builtin resolved in `build_assets` (`src/main.rs`), substituted by `render()` in `src/manifest.rs`, whose trailing-newline normalisation (`format!("{}\n", out.trim_end())`) already guarantees an empty substitution leaves no blank-line drift. `{{modules}}` is the identical pattern with a different source: instead of reading one `instrument.md` gated on a bool, it concatenates the guidance partials of the selected modules (empty string when none). Inventing nothing beyond a second builtin (Principle 1).
- It keeps ONE rendered AGENTS.md as the single source of workflow guidance, with no second discovery step. The generic-hook alternative needs both a permanent core sentence AND the orchestrator to actually go read `.agents/modules/*.md` at the right moment; a render slot inlines the guidance where it belongs so the orchestrator reads it in the one file it already reads first. Fewer moving parts, no "did the agent remember to read the module dir" failure.
- It provably preserves the Principle 2 byte-identity invariant (see 3.3).

Mechanism concretely:

- Add a `guidance` field to `[[module]]` (see the ModuleSpec in `src/manifest.rs`, which today holds only `name` and `description`): an optional path to the module's guidance partial within the pack. Declaring the partial on the module (rather than by an implicit `modules/<name>.md` convention) single-sources it and lets the loader validate it (a `guidance` pointing at a missing file is a pack-authoring error, caught at load, Principle 5; the same spirit as the existing dangling-module-tag validation).
- `{{modules}}` becomes a reserved builtin (like `{{instrument}}`; a pack may not declare it, reusing the existing reserved-variable rejection in `src/manifest.rs`). `build_assets` computes it: for each selected module, in `[[module]]` declaration order (deterministic, so the rendered output is stable regardless of `--module` flag order), read its `guidance` file and concatenate with a single blank-line separator; empty string when no module is selected or no selected module declares guidance.
- Place `{{modules}}` at the TAIL of AGENTS.md, after `{{instrument}}` (which is currently the last line). The checks module's `guidance/checks.md` partial is a self-contained section with its own `##` heading, so appending it at the tail reads cleanly.

### 3.2 Increment-1 retrofit or checks deliverable

It is a CHECKS deliverable, but a GENERIC one. Increment 1 is already built and shipped without it and there is no reason to reopen it. The checks module is the first module that needs to inject guidance (its checks-reviewer role and its work-review wiring must reach the orchestrator via AGENTS.md), so the slot lands as the first task of increment 2. But it is not checks-specific: the `guidance` field on `[[module]]` and the `{{modules}}` builtin are module-agnostic, so when test-driven and mutation are built they inherit the slot and each just declares its own `guidance` partial. Both test-module explorations independently concluded the checks increment must deliver this slot (A calls it "a further reason the checks module is the substrate"; B names it a shared prerequisite); this design makes it an explicit named deliverable, which resolves B's stated risk that it might otherwise land ad hoc.

### 3.3 Proof it keeps byte-identity

With no `--module`, `{{modules}}` resolves to the empty string. AGENTS.md ends `...\n\n{{principles}}\n\n{{instrument}}\n{{modules}}\n` (the two trailing slots). `render()` does the substitutions then `format!("{}\n", out.trim_end())`, so every trailing blank line and both empty slots are trimmed and exactly one newline is appended. The result is byte-for-byte the pre-modules output: the same reasoning the existing `rendered_agents_ends_with_a_single_trailing_newline` test relies on for `{{instrument}}`. The invariant is locked by adding a sibling test:

```rust
#[test]
fn modules_off_omits_guidance_and_keeps_byte_identity() {
    // build with modules = &[]  -> {{modules}} renders empty, trims clean.
    // assert AGENTS.md does not contain any module guidance heading,
    // does not contain the literal "{{modules}}",
    // ends with exactly one newline (no trailing blank line),
    // and equals the output built before the slot existed (a golden compare).
}
```

Because the slot lives at the tail alongside `{{instrument}}`, an empty substitution cannot leave a blank line in the MIDDLE of the document (which `trim_end` would not fix); a mid-document slot is the one placement that would break byte-identity, and this design avoids it by construction. This is the decisive reason the slot goes at the tail rather than adjacent to the work-review phase text where the guidance is conceptually relevant.

## 4. The checks-reviewer role and its wiring

### 4.1 The role prompt (outline)

`prompts/checks-reviewer.md`, dropped to `.agents/prompts/checks-reviewer.md`, module-tagged, `ownership = "reference"`. Outline:

- Identity: "You are the deterministic checks reviewer. You are READ-ONLY: you run configured check commands and report what they find; you never modify code, never apply formatters, never fix findings." (The read-only contract mirrors `reviewer.md` and is what lets the orchestrator spawn it without isolation, per the writer-isolation rule in AGENTS.md.)
- Inputs it is handed: the before/after commit hashes or diff range of the artifact under review; its assigned findings-file path under `docs/plans/<task>.reviews/`; and any PHASE PARAMETER (for example, from the test-driven module, "evaluate the `test` check under the red profile: expect `check` to exit 0 and `command` to exit non-zero"). Absent a phase parameter it uses the default profile: lint detects, format is verified in check-mode, test expects pass, mutation reports survivors.
- Procedure: read `.agents/checks.toml`; run the checks via the single runner (`.agents/checks/run-checks.sh detect lint format`, section 5), which encodes the per-kind semantics (lint = run `command`; format = run `check`, never `command`; test = run `check` then `command` under the handed profile; mutation = run `command`, diff-scoped, under `budget`). Running through the runner rather than re-deriving the semantics in the prompt keeps a single execution path shared with the implementer and the hook (Principle 1).
- Output: write findings to the assigned findings file in the SAME finding schema the LLM reviewers use (one entry per finding: a severity on the `low`/`medium`/`high`/`critical` scale, plus concrete evidence: the check name, the exact command, its exit code, and the offending `file:line` from the tool's output). A clean run states so explicitly, like a clean LLM review. The reply may summarise; the file is the record. This is verbatim the findings-file contract in `reviewer.md`, so the triager reads a checks finding identically to an LLM finding.
- Line-length and prose-wrapping are never findings (the standing convention), inherited from the reviewer contract.

### 4.2 How the orchestrator spawns it, and how it stays off the core

The orchestrator learns to spawn the checks-reviewer from the `guidance/checks.md` partial injected into AGENTS.md (which the orchestrator reads first), NOT from an edit to `orchestrator.md`. The partial says: "When the checks module is active, in the work-review phase (phase 4) the orchestrator ALSO spawns the checks-reviewer (`.agents/prompts/checks-reviewer.md`) alongside the LLM reviewers, handing it the same diff range and a distinct findings-file path (`<step>-checks.md`) per the findings-file naming rule. It is read-only, so it needs no isolation. Its findings go to the triager with the LLM reviewers' findings." Because this reaches the orchestrator through AGENTS.md and not through a core-prompt edit, the core `orchestrator.md` and `reviewer.md` stay byte-identical when the module is off (Principle 2). It runs in phase 4 (the code-output review), not phase 3 (plan review), because the checks gate is over CODE.

### 4.3 How deterministic findings merge with LLM findings for the triager

They do not need merging; they land in the same place. Each reviewer (LLM or checks) writes its own findings file under `docs/plans/<task>.reviews/` with a distinct name (`<step>-<role>-<disambiguator>.md` for LLM reviewers, `<step>-checks.md` for the checks-reviewer), exactly as the orchestrator already assigns distinct paths so parallel writers never collide. The triager reads all the round's findings files and adjudicates every finding, LLM and deterministic, on the one severity scale: fix, or suppress/accept with reasoning. A deterministic finding is not auto-valid: the triager may accept a lint finding the project chooses to suppress (the fix is then a user edit to checks.toml or an ast-grep rule ignore, since both are user-owned), or rule a formatter's complaint about a generated file out of scope. This reuses the reviewer -> triager -> implementer pipeline wholesale (Principle 1: no new adjudication machinery; the same "verify with a tool, judge with a human/agent" split as the metrics validator, Principle 5, Principle 6). The implementer fixes the valid ones in the next round; the format ones it fixes by running the apply command, the lint ones by changing code.

### 4.4 The implementer's auto-apply step

The formatter auto-apply is a WRITER action, generalising today's hardcoded `nix fmt`. The `implementer.md` core prompt is NOT edited (Principle 2); the `guidance/checks.md` partial adds the instruction: "In your verify step, before recording the diff, apply the `kind = "format"` checks over the files you changed (`.agents/checks/run-checks.sh apply format`), which formats only your changed paths, honouring the existing 'format only the files you changed' rule." The reviewer then catches an implementer that skipped this via the format check-mode (a non-zero `check` becomes a finding), which is the whole point of the check-mode: it deterministically verifies the code is formatted without the reviewer touching a file.

## 5. The pre-commit hook

### 5.1 Opt-in mechanism

A `--with-precommit-hook` flag on the `scaffold` verb, valid ONLY together with `--module checks` (requesting the hook without the module is a usage error: `clap` `requires` on the module, or an explicit check that errors and writes nothing, Principle 5). The flag also connects to the deferred TUI module pane as a per-module option there, but the flag is the buildable mechanism for increment 2. It is opt-in and secondary by construction: the module is fully functional without it (the review path is authoritative), and the flag only adds a mechanical backstop.

### 5.2 What it runs and how it stays secondary

The hook runs the FAST, non-mutating checks only: the lint `command`s and the format `check`s (`.agents/checks/run-checks.sh detect lint format`). It never runs tests or mutation (too slow for a commit) and never applies a formatter (a commit hook that rewrites files under the committer is surprising and unsafe). A non-zero result blocks the commit with a message naming the failing check. It is SECONDARY to the review path: it can be bypassed (`git commit --no-verify`), it does not replace the checks-reviewer (a bypass still gets caught in work-review), and the guidance states plainly that the review path is the authoritative gate and the hook is a convenience backstop. This keeps the review-integrated path primary and the hook a fast local echo of it.

### 5.3 Dropping a hook into a repo safely (Principle 3, Principle 4)

The hook logic is NOT `.git/hooks/pre-commit` directly. The module drops `.agents/hooks/pre-commit` (a 3-line script that execs the runner: `exec .agents/checks/run-checks.sh detect lint format`), which is a tool-owned reference asset (idempotent to re-drop). INSTALLING it into `.git/hooks/` is create-if-absent:

- If `.git/hooks/pre-commit` does not exist, the tool symlinks or copies `.agents/hooks/pre-commit` to it.
- If `.git/hooks/pre-commit` already exists (the user or another tool owns it), the tool NEVER clobbers it (Principle 3). It leaves the existing hook and prints one line of guidance: "add `. .agents/hooks/pre-commit` (or `.agents/checks/run-checks.sh detect lint format`) to your existing `.git/hooks/pre-commit`." A future refinement could append a marker-delimited idempotent block to the existing hook, but the safe default is to not touch a hook the tool did not write.
- Re-running the scaffolder is a no-op on an already-installed hook (Principle 4): the reference `.agents/hooks/pre-commit` re-drops identically, and the install step sees the target present and does nothing.

### 5.4 Safety on a repo without the tools installed (harness-agnosticism, no runtime tool dependency)

A contributor who clones the repo without ast-grep (or the formatter) installed must not have their commits hard-blocked by a missing binary, and the hook must not depend on `agent-scaffold` being installed (the `Q-26` "no runtime tool dependency" decision). Two mechanisms:

- The runner (section 6) is pure POSIX `sh` plus `awk` (universally present), NOT `agent-scaffold`. It parses checks.toml itself, so the scaffolded project runs its checks with zero dependency on this tool. This is the direct implementation of "harness-agnostic, no runtime tool dependency".
- The runner treats a MISSING check binary as a warn-and-skip, not a block: if a check's command is not found on `PATH`, it prints "check `X` skipped: `ast-grep` not installed" and continues, exiting zero for that check. So a contributor lacking a linter can still commit; the check is simply not enforced locally for them, and the review path (where the tools ARE present, in the workflow's environment) remains the real gate. This is the honest trade-off: better a hook that degrades to a warning than one that blocks a clone that lacks an optional tool. The workflow's own environment (Nix/direnv per Principle 7) has the tools, so the authoritative gate is unaffected.

## 6. ast-grep first-party and identical plug-in

### 6.1 ast-grep scaffolding

ast-grep is first-party in exactly two senses, neither of which touches the schema:

- It is the pre-populated lint row in the seeded checks.toml (`command = "ast-grep scan"`).
- The module drops ast-grep's own config so a rule set exists out of the box: `sgconfig.yml` at the repo root (create-if-absent working file, since ast-grep looks for it there) pointing at `.agents/ast-grep/rules/`, plus one example rule `.agents/ast-grep/rules/example.yml` (a reference asset, e.g. a rule flagging a banned pattern) that documents the rule format so a project writes its own. A project adds rules by dropping `.yml` files under that directory; ast-grep discovers them via `sgconfig.yml`. This is ast-grep's NATIVE mechanism, used as-is; the checks module does not wrap or reinvent it.

### 6.2 How other tools plug in identically

Every other tool is one `[[check]]`, with the schema privileging nothing:

```toml
[[check]]                                   [[check]]
name = "ruff"                               name = "clippy"
kind = "lint"                               kind = "lint"
command = "ruff check ."                    command = "cargo clippy --all-targets -- -D warnings"

[[check]]                                   [[check]]
name = "eslint"                             name = "prettier"
kind = "lint"                               kind = "format"
command = "eslint ."                        command = "prettier --write ."
                                            check   = "prettier --check ."
```

The runner runs `command` (or `check`) and interprets the exit code; it knows nothing tool-specific. The schema has ZERO ast-grep-specific fields, which is the direct rebuttal to "a schema so ast-grep-specific that ruff/eslint/clippy do not fit": the only ast-grep-specific artifacts are `sgconfig.yml` and the example rule, and those are ast-grep's OWN config that a non-ast-grep project simply ignores (or deletes). A language with no ast-grep grammar drops the ast-grep row and adds its native linter; nothing degrades.

## 7. Adversarial failure modes and mitigations

7.1 The tool overwrites checks.toml and clobbers a user's edits. Mitigated by `ownership = "working"` (create-if-absent, never overwritten, Principle 3) and idempotent re-runs (Principle 4). The seeded rows are a starting point the tool has no further claim on; `--force` (the existing overwrite flag) is the only path that would replace it, and that is the user's explicit choice.

7.2 A formatter reformats files the change did not touch. Mitigated by scoping the implementer's apply to the CHANGED files only (`run-checks.sh apply format` intersects each format check's `paths` with the changed-file set, or invokes the formatter on the specific changed paths), which is exactly the "format only the files you changed" rule already in `implementer.md`. Where a formatter cannot be path-scoped, the implementer runs it and reverts stray reformatting of untouched files, and the orchestrator's existing incidental-reformatting duty catches residual drift. The checks-reviewer, being read-only, runs only the `check` (dry-run) command and never writes, so IT cannot reformat anything.

7.3 A hook fails on a repo without the tool installed. Mitigated by the runner's warn-and-skip on a missing binary (5.4) and by the runner being POSIX `sh`+`awk` with no dependency on `agent-scaffold`. A missing linter degrades the local hook to a warning, never a block; the authoritative gate is the review path in the tooled environment.

7.4 An injection slot that breaks the byte-identical invariant. Mitigated by placing `{{modules}}` at the TAIL alongside `{{instrument}}`, where `render()`'s `trim_end` provably collapses an empty substitution (3.3), and by locking it with a golden byte-compare test. A mid-document slot would break it; this design forbids that placement.

7.5 A schema so ast-grep-specific other tools do not fit. Mitigated by a schema with zero tool-specific fields (section 6): kind/command/check/paths/budget are generic, and ast-grep's specificity lives only in its own `sgconfig.yml`, which non-ast-grep projects ignore.

7.6 The hook clobbers a user's existing `.git/hooks/pre-commit`. Mitigated by create-if-absent install: an existing hook is never touched; the tool prints one line of guidance instead (5.3).

7.7 The checks-reviewer, meant to be read-only, mutates the tree (e.g. runs a format `command` instead of `check`). Mitigated at two layers: the runner's `detect` mode only ever runs `check` for format kinds (never `command`), and the role prompt states the read-only contract explicitly. A format entry missing a `check` is reported as a degraded finding, not silently applied (1.2).

7.8 checks.toml and the pre-commit hook drift (the hook runs a stale command list). Mitigated by the runner parsing checks.toml LIVE at commit time; the hook is a 3-line exec of the runner, carrying no inlined command list, so it cannot drift from the config (single source, Principle 1). This is the decisive reason for a runner script over a hook with inlined commands (see the steelman, section 9).

## 8. Recommendation

Build increment 2 as: (1) a GENERIC `{{modules}}` render slot at the tail of AGENTS.md, driven by an optional `guidance` field on `[[module]]`, computed as a reserved builtin concatenating enabled modules' partials in declaration order, empty and byte-identical when none, delivered first because checks is the first module to need it and test-driven/mutation inherit it. (2) A `checks` module dropping `.agents/checks.toml` (create-if-absent working file, seeded with an ast-grep lint row and a format row, with commented `test` and `mutation` rows reserving the full schema), `sgconfig.yml` plus an example ast-grep rule, a POSIX `sh`+`awk` runner `.agents/checks/run-checks.sh` as the single execution engine, a read-only `checks-reviewer` role prompt, and a `guidance/checks.md` partial. (3) The `[[check]]` schema: `name`, `kind` (lint/format/test/mutation), `command`, optional `check` (format dry-run / test compile-only), optional `paths`, optional `budget` (mutation), with the red/green expectation kept OUT as a phase parameter. (4) The checks-reviewer spawned in phase 4 by orchestrator guidance injected via AGENTS.md (no core-prompt edit), writing findings in the shared schema to `<step>-checks.md` for the same triager. (5) The implementer auto-applying format checks over its changed files in its verify step, via the runner. (6) An opt-in `--with-precommit-hook` flag (requires `--module checks`) that install the runner as a create-if-absent `.git/hooks/pre-commit` running lint-detect and format-check only, degrading to warn-and-skip on missing tools, secondary to the review path.

This is the cleanest long-term architecture (Principle 1: one config file, one schema, one execution engine shared by implementer/reviewer/hook, no drift), it does not touch the core when off (Principle 2: every asset module-tagged, the render slot byte-identity-proven and tested), it is safe on existing repos (Principle 3: create-if-absent checks.toml and hook), idempotent (Principle 4: re-runs are no-ops on user files and the hook), makes illegal states unrepresentable (Principle 5: kind is a closed enum, duplicate names and dangling guidance are load errors, hook-without-module is a usage error), grounds the deterministic gate in a tool rather than an LLM (Principle 6), and stays reproducible and harness-agnostic (Principle 7: the runner is POSIX `sh`+`awk` with no dependency on this tool). Trade-off accepted: it ships a runner script (an `awk` TOML reader) rather than having agents read checks.toml directly; I judge the single execution path and the drift-free hook worth that asset (see the steelman).

## 9. Steelman against my own recommendation

The weakest joint is the RUNNER SCRIPT (`run-checks.sh`) as the single execution engine. The strongest case against it:

The runner ships an `awk`-based TOML parser into every scaffolded project. That is a nontrivial, fragile asset for a config format (TOML) that has real edge cases (multi-line strings, quoting, inline tables, comments mid-value) an ad-hoc `awk` parser will get subtly wrong, and when it does the failure is a mis-run check, which is worse than no check because it looks like it ran. A simpler design drops the runner entirely: the checks-reviewer and the implementer are LLM agents that read `.agents/checks.toml` DIRECTLY (they parse TOML natively and reliably) and run the commands themselves, with the per-kind semantics stated in `guidance/checks.md` prose; and the pre-commit hook is GENERATED at scaffold time with the current lint/format-check commands inlined, carrying a comment "regenerate if you edit checks.toml". This removes the fragile parser, removes a shipped asset, and leans on the agents' existing ability to read config. Under this view my "single execution engine" is over-engineering: the three consumers do not actually need to share code, they need to share the CONFIG (checks.toml), which they already do; the runner adds a second thing to keep correct for no capability gain.

The cost of that alternative, and why I still reject it: the inlined-command hook DRIFTS from checks.toml the moment a user edits their checks (the exact single-source violation, Principle 1, that this module exists to prevent for the lint/format commands), and "regenerate the hook" is a manual step users forget, so the local gate silently diverges from the reviewed gate. And moving the per-kind semantics (lint = detect, format = apply-vs-check, test = compile-then-run under a profile) into PROSE that three different agents each re-interpret invites exactly the "agent did the wrong thing" nondeterminism the checks module is meant to eliminate (Principle 6). The runner encodes those semantics once, deterministically. I keep the runner but concede two things narrow the gap: the `awk` parser must be kept deliberately SIMPLE, which means the checks.toml schema must stay a FLAT array of tables with single-line string values (no inline tables, no multi-line strings), a constraint the schema in section 1 already satisfies and which should be stated as a hard schema rule; and if the `awk` parser proves fragile in the increment-2 proof-of-concept (Principle 6: validate before building out), the fallback is the agents-read-toml-directly path with an inlined-and-regenerated hook, accepting the drift risk as the lesser evil. That is the single point a reasonable explorer might decide the other way: whether the shared runner's drift-free single-sourcing is worth shipping an `awk` TOML reader, or whether agents-read-config-directly plus an inlined hook is the more minimal and less fragile design.
