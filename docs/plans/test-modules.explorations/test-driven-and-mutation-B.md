# Q-30 (test-driven) and Q-31 (mutation-testing) modules: design notes B

Explorer B. Independent design pass owed for `Q-30` and `Q-31`, scheduled after the deterministic-checks module (`optional-modules` increment 2). This document follows the human-input contract written to a file: options, trade-offs judged against the numbered Project Principles, a single recommendation with reasoning, and an explicit "what not to build". It is one explorer's take, to be synthesised against the other explorer's.

I read the `optional-modules` design decisions (the hybrid `[[module]]` schema, the `--module` flag, the checks module's execution model with the read-only `checks-reviewer` running linters and check-mode formatters into a findings file the triager adjudicates), the whole Workflow section of `pack/AGENTS.md` (phases, convergence, findings files, writer isolation, checkpoints), the existing role prompts, and the `Q-30`/`Q-31` queue items. The recommendations below are grounded in those, not invented.

A note on one load-bearing assumption. The module machinery (increment 1) must add ONE generic, permanent hook to core `AGENTS.md` and `orchestrator.md`: a module-agnostic sentence saying "when an optional module is selected it drops its own guidance under `.agents/modules/` and its own role prompts under `.agents/prompts/`; read the ones present." That sentence ships in the baseline whether or not any module is selected, so the byte-identical invariant (no `--module` => output identical to today's core) still holds. Every module below (checks, test-driven, mutation) is then PURELY ADDITIVE FILES: it never edits a core prompt or a core `AGENTS.md` fragment, because a single rendered `AGENTS.md` asset cannot be module-tagged per fragment and per-module core edits would break byte-identity. I treat this generic hook as an increment-1 prerequisite for both modules here. If increment 1 did not add it, the checks module already needs it (it adds the `checks-reviewer` role and "workflow guidance wiring it in"), so this is a shared dependency, not new cost I am introducing.

---

## 1. Q-30: the test-driven module

### 1.1 Recommendation in one paragraph

Ship ONE module, `--module test-driven`, that HARD-DEPENDS on `--module checks` (selecting test-driven auto-selects checks; see 4.3). It is discipline-plus-one-role, not a config module: it adds ZERO checks config and reuses the checks module's `kind = "test"` capability for the green gate. Concretely it drops (a) one new WRITER role prompt `.agents/prompts/test-author.md`, separate from the implementer; (b) one guidance addendum `.agents/modules/test-driven.md` carrying the planner test-spec directive, the inserted test-authoring phase, the two gate profiles, the red/green verification procedure, and the interface-stub rules; and (c) one ast-grep lint rule (a `kind = "lint"` row example in the checks module's `.agents/checks.toml`) forbidding non-panicking stub bodies in a test-artifact commit. The per-step implement loop (phase 4) splits into a test-authoring sub-phase (red gate) then a code sub-phase (green gate), each its own reviewed artifact with its own gate profile. That is the "roughly doubles the reviewed-artifact count" the human anticipated.

### 1.2 Module decomposition: one module, discipline-plus-role, not a discipline/tooling split into two `--module` names

The queue item floats separating "the discipline (planner test-spec, test-first ordering)" from "the tooling (test execution as a `kind = "test"` entry)". I read that as a CONCEPTUAL separation of concerns, not a proposal for two independently-selectable `--module` names. There is no user who wants the test-first discipline while refusing the ability to actually run tests, nor vice versa, so two flags would be a false choice that only adds a way to select an incoherent state (against Principle 5, make illegal states unrepresentable). One module name, internally realised as discipline assets (prompts, guidance) that CONSUME the checks module's already-existing test-running capability.

The key single-sourcing move (Principle 1, single source of truth; Principle 2, no duplication of the checks machinery): the command that runs the tests lives in exactly one place, the checks module's `.agents/checks.toml`, as a `kind = "test"` entry. test-driven does NOT ship its own test-runner, its own config file, or its own copy of that command. It ships the DISCIPLINE that uses it. This is why test-driven depends on checks: the green gate is defined as "the `checks-reviewer` runs the `kind = "test"` entries and they all pass", which is a checks-module capability. Adding test-driven therefore adds no new gate-execution machinery at all; it adds a phase ordering and one writer role.

Rejected alternative (the strongest one): a STANDALONE test-driven module with its own minimal built-in test command, usable without adopting the checks module. This is genuinely attractive because it decouples two optional modules and lets a project take test-first discipline without the whole ast-grep/checks.toml apparatus. I reject it because it duplicates the test command and its check-mode semantics into a second config surface that will drift from checks.toml (against Principle 1), and it forks the gate-execution path (two ways to run tests, against Principle 2's "adding a module must not complicate the core"). The dependency on checks is the cost of single-sourcing, and I judge single-sourcing worth it. This is the single most defensible point of disagreement (see 6.1); an explorer who weights module independence over config single-sourcing would reach the opposite call.

### 1.3 test-author as a role SEPARATE from the implementer

The test artifact and the code artifact must be written by DIFFERENT agents. The whole workflow rests on "no agent grades its own work" (Workflow, first paragraph). A test authored to fail-first is an independent SPECIFICATION the code must satisfy; if the same agent writes the test and then the code, it can (subconsciously or lazily) write both to match, so the test stops being an independent check and becomes a restatement of whatever the implementer chose to do. Separating the roles mirrors reviewer-separate-from-producer and is the same reasoning as triager-independence. So I recommend a distinct `test-author` writer role, spawned like the implementer, its artifact reviewed independently. This is the direct cause of the "roughly doubles reviewed artifacts" cost, and it is the point, not an accident.

The test-author is a WRITER (it changes files: the test files plus the interface stubs), so the file-safety and writer-isolation rules already in AGENTS.md apply to it unchanged (worktree-first where the harness offers it). No new isolation machinery.

### 1.4 The two gate profiles, and how "must fail first" is verified without the checks gate flagging it as broken

There are two artifacts per step, reviewed in two consecutive loops with two gate profiles:

- TEST ARTIFACT (red phase). The test-author writes the tests plus the interface stubs they call (signatures and types with `unimplemented!()` / `todo!()` bodies). Gate profile: (1) COMPILES (the stubs make the tests build); (2) FAILS BY ASSERTION at runtime, not at compile time (every new test fails because the stub panics or the assertion is unmet, not because a symbol is missing); (3) COVERS the plan's enumerated test-spec cases for this step (one real assertion per listed input/expected-output, property, and end-to-end flow); (4) NON-VACUOUS (no `assert!(true)`, no assertion that a stub equals itself).
- CODE ARTIFACT (green phase). The implementer fills the stub bodies. Gate profile: (1) compiles; (2) lints clean (the checks module's `kind = "lint"` entries via the `checks-reviewer`); (3) ALL pre-authored tests pass (the checks module's `kind = "test"` entries via the `checks-reviewer`).

The critical mechanic is how the RED requirement ("tests must fail first") is verified without the checks gate flagging the failing tests as broken. The resolution is PHASE-SCOPING the `kind = "test"` gate, and it needs no new config field:

- The `checks-reviewer` runs the `kind = "test"` entries as a PASS gate (expect exit success) ONLY when reviewing the CODE artifact (green phase). It is never spawned against the test artifact. So the checks gate never sees the intentionally-failing red-phase tests, and never flags them.
- The RED gate over the test artifact is a NORMAL reviewer (the existing `reviewer.md` role) carrying the red-phase lens from `.agents/modules/test-driven.md`. That reviewer runs the same test command from checks.toml but with the INVERTED expectation: it asserts the tests FAIL by assertion. Concretely, for a compiled language it runs the `check` field first (compile-only, e.g. `cargo build --tests --locked`, must succeed => stubs are present) and then the `command` field (e.g. `cargo test`, must report assertion failures, not compile/link errors => the tests are real and reach the stub). A test that compiles and fails-by-assertion passes the red gate; one that fails to compile, or that PASSES, fails the red gate.

The single source of truth is preserved because the checks.toml entry stores HOW to run the tests exactly once (`command` = run tests, `check` = compile tests). The GATE PROFILE (green expects success, red expects assertion-failure) is a phase property carried by the workflow prompts, applied by phase; it is not stored per-check. This is the honest answer to "how do the two gate profiles get written into AGENTS.md and prompts": they live in `.agents/modules/test-driven.md` (the module addendum) and in the `test-author.md` and (red-lens) `reviewer` context the orchestrator hands out, keyed by which sub-phase is running. The core AGENTS.md is untouched beyond the generic module hook.

Note this reuses a check field shape the checks module already has. The checks decision says a `[[check]]` names a command, a kind, and a check-mode command. For `kind = "lint"` and `kind = "format"` the check-mode command is the dry-run detector. For `kind = "test"` I read the same two fields as: `command` = run the suite (the green gate and the thing the red reviewer inverts), `check` = compile-only (the "does it build" half of both gates). No new field is invented; the `kind = "test"` row is a natural instance of the existing schema. Sketch:

```toml
# .agents/checks.toml (owned by the CHECKS module; test-driven ships NO config)
[[check]]
kind = "test"
command = "cargo test --locked"          # green gate: expect success. red gate: red reviewer runs this and expects assertion-failure.
check   = "cargo build --tests --locked" # compile gate: expect success in both phases (stubs must build).
```

### 1.5 Interface-stub mechanics (so tests compile pre-implementation) and the trivially-green risk

The interface stubs are authored as PART OF THE TEST ARTIFACT, by the test-author, not as a separate preceding step. Folding stubs into the test artifact keeps the extra reviewed artifacts at TWO per step (test, code), not three; a separate stub sub-step would triple the count for little gain. A stub is a signature and type with an `unimplemented!()` / `todo!()` body. The implementer, in the green phase, replaces the bodies (and may refine signatures, which the reviewer checks against the stubbed interface the tests depend on).

Adversarial case: a stub that returns a hardcoded value making a test trivially green. This is caught twice. First by the RED GATE itself: a stub that makes any new test PASS fails the red gate (the red gate requires the tests to fail-by-assertion). A trivially-satisfying stub therefore cannot get past the red review. Second by a SHIPPED ast-grep lint: test-driven's guidance registers a `kind = "lint"` rule (in the checks module's checks.toml) that forbids stub bodies other than `unimplemented!()` / `todo!()` / an explicit panic in the commit that constitutes the test artifact. Two independent guards, both deterministic (Principle 6, verify with a tool).

### 1.6 Where the phase and gate sit in the implement/review/converge loop

Phase 4 of the workflow ("Implement and review, step by step") becomes, per Roadmap step, WHEN test-driven is active:

- 4a. Plan check. The step's plan detail must carry a TEST-SPEC (inputs/expected outputs, properties, edge cases, end-to-end flows). This is a PLANNER duty added by the module guidance and by the plan-template test-spec slot (see 3.1); if a step lacks it, the orchestrator routes back to the planner before authoring tests. Grounding the test artifact in the plan is what makes non-vacuity checkable in 4b.
- 4b. Test-authoring (red). The orchestrator spawns a `test-author` to write the failing tests plus interface stubs for the step. Then the RED review loop: reviewers (with the red-phase lens) plus a separate triager, converging under the normal convergence rule (one clean round low-risk, two risky). Red gate profile per 1.4. On convergence the test artifact is fixed and committed.
- 4c. Implementation (green). The orchestrator spawns the `implementer` to fill the stub bodies until the pre-authored tests pass. Then the GREEN review loop: the LLM reviewers plus the `checks-reviewer` (which runs `kind = "lint"`, `kind = "format"` check-mode, and now `kind = "test"` as a pass gate), then the separate triager, converging normally. Green gate profile per 1.4.
- 4d. Mark the step complete, move on. (If the mutation module is also active, the mutation gate fires here, once, at green convergence; see section 2.)

This reuses phase 4's existing structure (implementer -> reviewers -> triager -> convergence) verbatim, twice, with different producers and gate lenses. No new phase is added to the workflow spine; the module SPLITS phase 4's single implement-review cycle into two, which is why it composes cleanly.

### 1.7 What is scaffolded vs pure discipline (Principle 2, byte-identity)

Scaffolded ASSETS (all module-tagged `test-driven`, dropped only under `--module test-driven`):

- `.agents/prompts/test-author.md` (new writer role prompt).
- `.agents/modules/test-driven.md` (guidance: the phase-4 split, the two gate profiles, red/green verification, stub rules, and the planner test-spec directive).
- A plan-template addition for the test-spec is the one subtlety: the plan template (`docs/plans/TEMPLATE.md`) is a single core asset and cannot be module-tagged per fragment, same problem as AGENTS.md. Resolution: do NOT edit the core template. Instead the module guidance instructs the planner to add a "Test spec" subsection to each step's detail when the module is active. The plan template stays byte-identical; the test-spec is discipline the guidance imposes, not a template edit. (Strongest alternative: ship a `plan-template.test-driven.md` overlay asset the planner merges; rejected as more machinery than a guidance sentence buys.)

PURE DISCIPLINE (no asset, lives only in the guidance): the planner test-spec content, the test-first ordering, the risk-scaling of ceremony, the red-phase reviewer lens (handed as context, not a new prompt file, since it is a lens on the existing reviewer).

Config CONTRIBUTIONS to the checks module (NOT a new file; example rows the guidance tells the user to add to the checks-owned checks.toml): the `kind = "test"` row (1.4) and the stub-body `kind = "lint"` rule (1.5). Because checks.toml is a user working file (create-if-absent, Principle 3), test-driven cannot safely drop a second copy; it documents the canonical rows and, when both modules are selected at scaffold time, the checks module's checks.toml template is pre-populated with a commented `kind = "test"` example (the checks module already supports arbitrary kinds, so a commented test example belongs in its template regardless). This keeps config single-sourced in checks.toml with zero test-driven config files.

Byte-identity (Principle 2) holds: with no `--module`, none of the above drops, and the only always-present text is the generic module hook from increment 1. Output identical to today's core.

---

## 2. Q-31: the mutation-testing module

### 2.1 Recommendation in one paragraph

Ship ONE module, `--module mutation`, that HARD-DEPENDS on `--module test-driven` (and therefore on checks). It adds a `kind = "mutation"` capability to the checks config surface and ONE read-only role, `mutation-reviewer`. Crucially it does NOT run inside the per-round green loop (that would run the suite hundreds of times per review round and make the loop unusable, the "so slow it is never run" failure). It runs ONCE per step, as a CONVERGENCE GATE fired AFTER the code artifact's green loop converges (phase 4d). A surviving mutant is a finding meaning "your tests are vacuous here"; it routes back to the test-author to add a killing test, then re-implement/re-verify. It is risk-scaled, diff-scoped, and time-budgeted, and it degrades gracefully to a recorded skip when no mutation tool exists for the language.

### 2.2 Does it extend the checks-reviewer pipeline, or introduce its own role/phase? Its own role, at a distinct gate.

Mutation must NOT be folded into the `checks-reviewer`, because the checks-reviewer runs every green-review round and mutation is orders of magnitude too slow for that (each mutant re-runs the suite). Putting it there guarantees the "never run" failure. So mutation gets its OWN read-only role, `mutation-reviewer`, and its own GATE POSITION: once per step, at green convergence, not once per round. This is the conceptual hierarchy the queue names (checks -> test-driven -> mutation, each verifying the layer below): mutation adversarially verifies the TESTS, so it fires after the tests are green and the code is converged, adjudicating whether the converged tests are actually strong.

The `mutation-reviewer` is read-only (it runs the mutation tool and writes surviving mutants as findings to a findings file, per the existing findings-file convention), so it needs no isolation (Principle 2, read-only agents have no blast radius). It slots into the existing findings -> triager -> fix pipeline with no new convergence machinery: a surviving-mutant finding is triaged like any other (valid => add a test; invalid => e.g. the mutant is equivalent/unreachable, dismissed with reasoning), and a valid one routes to the test-author (add the killing test) then the implementer (if the added test now fails, which it should not if the code is correct). This reuses the reviewer/triager/findings/convergence machinery wholesale; the only new pieces are one role prompt and one `kind`.

### 2.3 Config shape (single-sourced in checks.toml)

```toml
# .agents/checks.toml (owned by CHECKS; mutation ships NO separate config file)
[[check]]
kind = "mutation"
command = "cargo mutants --in-diff <range> --timeout 120 --no-shuffle"
# no check-mode field needed; the mutation run IS the check.
```

The command is single-sourced with the other checks. Different languages plug in their tool the same way checks plugs in linters: `cargo mutants` (Rust), `mutmut` (Python), Stryker (JS/TS). This is the identical extensibility story as the checks module's "a language with no ast-grep grammar plugs in ruff/eslint/clippy".

### 2.4 The slowness and the risk-scaling

Four mitigations, all needed:

- ONCE PER STEP, not per round (2.2). The dominant cost reduction: mutation runs a single time per step at green convergence, not on every review round.
- DIFF-SCOPED. The command uses `--in-diff` (or the tool's equivalent) so it mutates only the functions the step changed, not the whole codebase. Mutation cost scales with mutated surface; scoping to the step's diff bounds it to the step's size.
- RISK-SCALED. The convergence rule already CLASSIFIES each artifact as low-risk or risky/high-blast-radius and records that classification in the ledger. The mutation gate REUSES that same classification: run mutation only for steps whose artifact is classified risky (security/safety/data/money-sensitive, widely depended on, hard to roll back). Low-risk steps skip mutation by policy. This is discipline in the guidance, not new machinery; it reuses the existing risk classification verbatim.
- TIME-BUDGETED. The command carries a `--timeout` per mutant and the guidance sets a whole-run budget; if the run exceeds budget it is recorded as a partial/skipped gate with the residual risk explicitly accepted (the convergence rule already has an accept-residual-risk path). Better a recorded, human-visible skip than a silent one or a blocked loop.

### 2.5 Language with no mutation tool

If the language has no mutation tool, there is simply no `kind = "mutation"` row (nothing to plug in). The mutation gate is then NOT RUNNABLE and the orchestrator records "mutation gate: not configured for this language, skipped" in the ledger, rather than blocking the step. Same graceful-degradation pattern as the checks module (plug in a tool, or the check is absent). Selecting `--module mutation` for such a project still drops the guidance and a commented example row, so the day a tool appears it is one uncomment away; until then the module is inert beyond documentation. This is the honest handling of "a language with no mutation tool": the module degrades to a no-op skip, it does not fail.

### 2.6 What is scaffolded (Principle 2)

Assets (module-tagged `mutation`, dropped only under `--module mutation`):

- `.agents/prompts/mutation-reviewer.md` (new read-only role prompt: run the configured `kind = "mutation"` entries, diff-scoped, write surviving mutants as findings, do not fix).
- `.agents/modules/mutation.md` (guidance: the once-per-step convergence-gate position, risk-scaling reusing the ledger classification, the surviving-mutant-as-finding routing to the test-author, the timeout/skip degradation).

Config contribution (to the checks-owned checks.toml, documented, pre-populated commented when both modules selected): the `kind = "mutation"` row.

No new phase, no new convergence loop, no new config file. Byte-identity holds by the same argument as 1.7.

---

## 3. Composition (with checks, and with each other)

### 3.1 The dependency chain and how config and roles are shared without duplication

- checks OWNS `.agents/checks.toml` and the `checks-reviewer` role. It supports arbitrary `kind`s.
- test-driven CONSUMES `kind = "test"` (green gate via checks-reviewer) and contributes a stub `kind = "lint"` rule and the `kind = "test"` example row to the checks-owned config. It ADDS the `test-author` writer role. It ADDS no config file and no gate-execution role.
- mutation CONSUMES a `kind = "mutation"` row (its own gate, its own `mutation-reviewer` role, once per step) and contributes that row to the checks-owned config. It ADDS no config file.

So there is exactly ONE config file (checks.toml, owned by checks) and the check COMMANDS are single-sourced there (Principle 1). The ROLE machinery is shared: reviewer, triager, findings files, convergence, ledger are reused unchanged by all three; the only NEW roles across both modules are one writer (test-author) and one read-only reviewer (mutation-reviewer). Nothing in the checks module's config or role machinery is duplicated (the direct answer to task item 1).

### 3.2 The gate stack, per step, all three modules active

1. Test-authoring (red): test-author writes failing tests + stubs; red review loop (reviewers with red lens + triager); RED gate = compiles + fails-by-assertion + covers plan test-spec + non-vacuous + stub-body lint.
2. Implementation (green): implementer fills bodies; green review loop (LLM reviewers + checks-reviewer running lint/format/`kind=test` + triager); GREEN gate = compiles + lints clean + formatted + all tests pass.
3. Mutation (convergence gate, risky steps only): after green convergence, mutation-reviewer runs `kind = "mutation"` diff-scoped once; surviving mutants -> findings -> triage -> back to test-author. Then re-verify green. Then mark step complete.

Each layer verifies the one below: checks verifies the code is well-formed and tests pass; test-driven verifies the code meets an independent, pre-authored spec; mutation verifies that spec is actually strong. That is exactly the human's stated hierarchy, realised without any module reimplementing another's machinery.

---

## 4. Opt-in, risk-scaling, and sequencing

### 4.1 Sequencing (confirm, with a challenge considered)

CONFIRM the human's schedule: build after the checks module (optional-modules increment 2). Dependencies are hard and directional: test-driven's green gate IS a checks capability (`kind = "test"` run by the checks-reviewer), and mutation layers on test-driven. So the only coherent order is checks -> test-driven -> mutation, each its own reviewed increment/step. I CHALLENGED whether test-driven could ship before checks by carrying its own minimal test runner (which would let it land independently); I reject that in 1.2 because it duplicates the test command and forks the gate path. So the dependency is real and the sequencing stands: these are a later step (or two steps: `test-driven-module` then `mutation-module`), after `optional-modules` fully lands, not sub-increments of it.

### 4.2 Two steps or one?

Recommend TWO Roadmap steps, `test-driven-module` then `mutation-module`, each its own review artifact, because mutation hard-depends on test-driven and is optional-on-optional: a project may adopt test-driven and never adopt mutation. Bundling them would force the reviewed increment to carry both, against "small, reviewable changes". The design pass is bundled (this document), the build is two steps.

### 4.3 Opt-in mechanism and dependency resolution (Principle 5)

Opt-in is the `--module` flag from increment 1: `--module test-driven`, `--module mutation`, repeatable. Dependencies are resolved at PARSE TIME so an incoherent selection is never representable (Principle 5):

- `--module mutation` auto-selects test-driven and checks; `--module test-driven` auto-selects checks. The resolver prints the implied additions to stderr ("`--module mutation` also enables test-driven, checks") so it is visible, not silent.
- The alternative (error out and make the human list all three) is more surprising and more typing for no safety gain, since the dependency is total and unambiguous. Auto-imply with a printed note is the make-illegal-states-unrepresentable choice: you cannot end up with mutation-without-test-driven.
- The `[[module]]` metadata section (the hybrid schema from `Q-25`) is where each module declares its `name`, `description`, and (a small addition) its `requires` list, so the dependency is single-sourced in the manifest and validated there, not hardcoded in the resolver. An unknown `--module` still errors and writes nothing, as decided.

### 4.4 Risk-scaling within an opted-in project

Two levels, both reusing existing mechanisms, no new machinery:

- Scaffold-time: the human opts the project in per module. Whole-project granularity.
- Per-step: the module guidance risk-scales the CEREMONY using the convergence rule's existing artifact risk classification. For a LOW-risk step the guidance permits collapsing the test-first ceremony (the orchestrator may fold red+green into a lighter cycle, or, for a trivial step, skip the separate test-author) exactly as "match the ceremony to the stakes" already licenses. For a RISKY step the full separation applies and the mutation gate fires. So risk-scaling is discipline layered on the ceremony-matching rule already in AGENTS.md, not a new flag.

---

## 5. Failure modes and mitigations

- Vacuous test that passes BOTH gates. A test that fails red (stub panics) but only asserts something trivial (e.g. that the function was called, not what it returned) passes red AND green. Mitigations, in depth order: (1) the red-gate reviewer checks each test against the plan's enumerated test-spec (4a/1.6), so a listed input/expected-output with no real assertion is a red finding; (2) the mutation gate (Q-31) is the adversarial backstop that catches exactly the vacuous-but-passing test, because a vacuous test kills no mutants. Honest limitation: WITHOUT the mutation module, a determined-vacuous test grounded in a weak plan spec can pass; that residual is precisely why Q-31 exists and why risky steps should enable it.
- Interface stub that makes tests trivially green. Caught by the red gate (a stub that makes a test pass fails the red gate) AND by the shipped stub-body lint (`unimplemented!()`/`todo!()`/panic only). Two deterministic guards (1.5).
- Mutation so slow it is never run. Mitigated by once-per-step (not per-round), diff-scoping, risk-scaling to risky steps only, and a time budget with a recorded accept-residual-risk skip on overrun (2.4). The default posture is "run on risky steps, scoped to the diff", which is small enough to actually run.
- Language with no mutation tool. Graceful degradation to a recorded skip, not a failure; the module is inert-but-documented until a tool is plugged in (2.5).
- test-author and implementer collude (same agent writes test then code). Prevented by making them SEPARATE roles/agents (1.3), the same independence principle as reviewer-vs-producer.
- checks gate false-flags the intentionally-failing red-phase tests. Prevented by phase-scoping: the checks-reviewer runs `kind = "test"` as a pass gate only in the green phase, never against the test artifact; the red gate is a separate reviewer with an inverted expectation (1.4).
- Red gate passes on a COMPILE failure mistaken for an assertion failure. Prevented by the two-step red check: run `check` (compile) and require SUCCESS, then run `command` and require assertion-failure; a compile/link error fails the first step, so "fails to compile" cannot masquerade as "fails first" (1.4).
- Config drift between test-driven/mutation and checks. Prevented by single-sourcing all check commands in the one checks-owned checks.toml; the higher modules contribute rows and consume them, they do not keep their own copies (3.1, Principle 1).
- Stub interface diverges from what the implementer builds. The green-phase reviewer checks the code against the stubbed interface the tests depend on; a signature change that breaks the pre-authored tests fails the green gate, forcing either the code to match or a triaged, reviewed interface revision routed through the test-author.

---

## 6. Steelman against my own recommendations

### 6.1 The single strongest objection: test-driven should NOT hard-depend on checks

My recommendation single-sources the test command in the checks-owned checks.toml and so forces `--module test-driven` to pull in `--module checks`. The strongest counter: test-first discipline and running a test command are UNIVERSAL and cheap, whereas the checks module carries an opinionated apparatus (ast-grep as first-party linter, a checks.toml the project must curate, a checks-reviewer role). Coupling test-driven to all of that means a team that wants "write the failing test first, then the code" has to also adopt a lint/format governance module they did not ask for. A standalone test-driven with a one-line built-in test command (`cargo test`, `pytest`, detected or configured with a single value) would be MORE minimal-by-default (Principle 2) for the common case, and the "duplication" I object to is one command string, not a whole config surface. Under this view the clean architecture is: test-driven owns a single `test_command` value, checks (if also selected) can additionally run it, and the two share via the manifest rather than one depending on the other. A reasonable explorer could rank module independence and minimal-by-default above config single-sourcing and land here. My rebuttal stands on Principle 1 (one source for the test command, run identically by the green gate and inverted by the red gate; two sources WILL drift once the command grows flags like `--locked`, workspace filters, or feature sets) and Principle 2's "adding a module must not COMPLICATE the core" (a second gate-execution path is core complication), but I acknowledge this is the closest call in the design and the place the two explorers are most likely to diverge.

### 6.2 Secondary objections I weighed and rejected

- Against a separate test-author role (1.3): it doubles the writer agents and cost per step, and with the red gate plus a mutation backstop, a solo implementer writing tests-then-code might be adequate. Rebuttal: the independence is the core value (the test as an independent spec), and the cost is bounded by risk-scaling (collapse for low-risk steps, 4.4). But for a project that never enables mutation and works mostly on low-risk steps, the separate role earns less than it costs, and folding test-author into the implementer for those steps is defensible.
- Against mutation as a once-per-step CONVERGENCE gate rather than per-round (2.2): with aggressive `--in-diff` scoping the per-run cost may be small enough to run every green round, which would give faster feedback on a vacuous test instead of deferring it to convergence. Rebuttal: even scoped, mutation re-runs the suite per mutant, so per-round is the "never run" risk; once-per-step at convergence is the safe default. An explorer with evidence that scoped mutation is cheap on a given stack could justify per-round, but that is a calibration finding (Principle 6, ground it in measured data via the metrics log), not a default.
- Against folding stubs into the test artifact (1.5) rather than a separate stub sub-step: a separate stub step would let the interface be reviewed on its own before tests are written against it. Rebuttal: it triples the reviewed artifacts per step for a boundary the green-phase reviewer already polices; not worth it.

## What not to build (YAGNI boundary)

- No second `--module` name splitting discipline from tooling; one `test-driven` name (1.2).
- No test-driven-owned config file and no test-driven-owned test runner; consume checks.toml (1.2, 3.1).
- No new workflow PHASE and no new convergence loop; split phase 4 and add one convergence-position gate for mutation (1.6, 2.2).
- No per-fragment editing of core AGENTS.md or the plan template; additive files plus one generic increment-1 hook (opening note, 1.7).
- No mutation inside the per-round checks-reviewer; once per step, risk-scaled, diff-scoped (2.2, 2.4).
- No hard failure when a language lacks a mutation tool; degrade to a recorded skip (2.5).
- No TUI module pane work here; opt-in is the existing `--module` flag (4.3), the pane stays deferred as in Q-25.
