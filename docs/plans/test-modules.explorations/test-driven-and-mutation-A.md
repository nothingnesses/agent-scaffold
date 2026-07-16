# Q-30 (test-driven) and Q-31 (mutation) module design: exploration A

Scope: a single recommended design for both modules, resolving the open questions each raises, judged against the numbered Project Principles. This is design notes, not an implementation. It assumes the deterministic-checks module (Q-25/Q-26, `optional-modules` increment 2) as already-designed: a user-owned declarative `.agents/checks.toml` with `[[check]]` entries (each a `command`, a `kind` like lint/format, and a check-mode command), a read-only `checks-reviewer` role spawned in the work-review phase that runs linters (detect) and formatters (check mode) into a findings file the triager adjudicates, formatters auto-applied at the implementer's verify step, and an optional pre-commit hook.

One framing decision drives everything below: the deterministic-checks module is the shared substrate, and the two new modules add DISCIPLINE and ADVERSARIAL VERIFICATION on top of it without duplicating any of its machinery. The conceptual hierarchy the intake states (checks runs lint/format/test -> test-driven -> mutation, each layer verifying the one below) maps directly onto module dependencies.

## 1. Q-30: the test-driven module

### 1.1 Module boundary: discipline is the module, execution belongs to checks

Recommendation: Q-30 ships as ONE module named `test-driven` whose pack content is PURE DISCIPLINE (planner test-spec expansion, the pre-implementation test-authoring phase, the red gate profile, the interface-stub sub-step). It owns NO test-execution machinery and NO config file of its own. Test EXECUTION is a `kind = "test"` entry in the checks module's `.agents/checks.toml`, run by the existing `checks-reviewer`. The module therefore REQUIRES the checks module and composes with it rather than duplicating it.

The clean split falls on a natural seam already present in the stated hierarchy: "checks module (runs lint/format/test)". Running a test command and expecting it to pass is an ordinary deterministic check, indistinguishable in mechanism from running a linter and expecting exit 0. So test EXECUTION in the ordinary green sense (tests must pass as part of the code-output gate) belongs to the CHECKS module from increment 2: a project may declare a `kind = "test"` check and have the checks-reviewer run it in the work-review phase like any other check, with no test-driven module present at all. That subset is useful on its own (a fast "the suite still passes" gate) and it is where the runner, the config schema, the role, and the findings pipeline all live.

What `test-driven` adds on top is exactly the parts that are NOT execution: the planner specifies test cases per step; tests are authored and reviewed BEFORE the code; the test artifact is gated RED (must fail) rather than green; and an interface stub lets the tests compile before the code exists. None of that is a new runner or a new config; all of it is prompt and AGENTS.md content plus a workflow reordering. This is the boundary the intake gestured at ("separate the DISCIPLINE from the TOOLING"), made precise: the tooling is not merely reused, it is OWNED by checks and only referenced by test-driven.

Why one module and not two (a discipline module plus a separate tooling module): the tooling half does not exist as a separable thing to package, because it is already the checks module. Splitting `test-driven` further would create a module that drops nothing (all execution is checks) and a module that is the discipline (all of test-driven). So the discipline IS the module. This honours Principle 1 (the cleaner architecture: no empty module, no second config) and Principle 2 (test-driven adds only opt-in discipline; the core and the checks module are untouched when it is off).

Dependency handling: add an optional `requires = ["<module>"]` field to the `[[module]]` metadata section (the small section increment 1 already introduces for name/description/validation). `test-driven` declares `requires = ["checks"]`. When `--module test-driven` is passed, the loader takes the transitive closure of `requires` and enables `checks` too, then reports the resolved set (for example: "modules: checks (required by test-driven), test-driven"). Auto-enabling rather than erroring makes the illegal state (test-driven on, no deterministic test execution beneath it) unrepresentable rather than a runtime guard (Principle 5). It is also single-sourced: the dependency lives on the module's own metadata, not in prose the user must remember. Erroring ("--module test-driven requires --module checks") is defensible and more explicit, but it pushes a mechanical fact onto the user for no benefit; auto-enable is the decisive choice, with the resolved set printed so nothing is enabled invisibly.

Why the dependency is real and not just convenient: the entire justification for the module over plain review is deterministic verification ("verify with a tool, do not trust the LLM", the ethos behind Principle 5 and Principle 6). Without the checks module's runner and checks-reviewer, "the tests pass" and "the tests fail" would be LLM claims, which defeats the purpose. So there is no meaningful pure-discipline-without-execution configuration to support, and the hard dependency costs nothing.

### 1.2 Workflow shape: two artifacts, two gate profiles, one changed expectation

The existing phase 4 ("Implement and review, step by step") becomes, for a test-driven step, a two-artifact sequence. Each artifact runs the normal review-then-triage convergence loop; the only thing that changes between them is the EXPECTED OUTCOME of the test check.

Phase 4 for a test-driven step:

1. Stub-and-tests (writer: the implementer; produces the TEST ARTIFACT). The implementer creates the interface stub (just enough types and signatures for the tests to compile, bodies left `unimplemented!()`/`todo!()` or a trivially-wrong typed return) and the tests drawn from the planner's per-step test-spec. Stub and tests together are one reviewable artifact.
2. Review the test artifact under the RED gate profile (checks-reviewer plus LLM reviewers, then triager, then the convergence loop). Converge before proceeding.
3. Implement (writer: the implementer; produces the CODE ARTIFACT). The implementer fills the stub bodies until the pre-authored tests pass. Tests are frozen (see the tripwire below): the implementer does not weaken a test to make it pass.
4. Review the code artifact under the GREEN gate profile (the ordinary work-review, now including the test check expecting pass). Converge.
5. Mark the step complete in the Roadmap.

The two gate profiles, stated precisely, are what resolves the "tests must FAIL first but the checks gate wants green" tension. Only ONE check's expected outcome differs between them; everything else is green in both:

- RED profile (test artifact): lint = green on the test and stub files, format = green (tests are well-formed, lint-clean, formatted; only their RESULT is red), test = the test check must BUILD (compile) and its run must FAIL. Concretely, using the two-command test entry in 1.3: the compile command exits 0 AND the run command exits non-zero.
- GREEN profile (code artifact): lint = green, format = green, test = the run command exits 0 (all pass).

So the checks gate is never contradicted: lint and format are always green, and the test check simply carries a different expected exit under the red profile. The red-versus-green choice is a PHASE parameter the orchestrator passes to the checks-reviewer when it spawns it ("evaluate the test check under the red profile per the test-driven guidance"), NOT a field in checks.toml. Keeping it out of checks.toml matters: checks.toml stays a pure declaration of commands (single source, harness-agnostic, Principle 1); the red/green semantics live in the test-driven workflow guidance, which is where the discipline belongs, and the checks-reviewer prompt stays generic (it runs the checks it is told and compares against the expected outcome it is handed, knowing nothing test-specific).

How "tests must fail first" is verified deterministically. The deterministic signal is: the test check's COMPILE command exits 0 (the stub plus tests build) and its RUN command exits non-zero (the tests fail). That cleanly separates a genuine red (compiles, then fails) from the failure mode that would otherwise masquerade as red (does not even compile). It does NOT, by exit code alone, separate "fails by assertion against absent behaviour" from "fails by panic in an `unimplemented!()` stub" or "fails because the harness crashed in setup". That residual judgment (the failure is for the RIGHT reason, and the test is not vacuous, and it covers the planner's cases) is delegated to the LLM test-reviewer reading the tests against the plan's test-spec. This is the same division the checks module already uses: the deterministic tool establishes the mechanical fact (compiles and fails), the reviewer judges meaning. It is honest about where determinism ends, and it is the single riskiest assumption in the design (see the closing summary).

The frozen-tests tripwire. In the code phase the implementer must not gut the tests to force green. To make that inspectable rather than trusted, the orchestrator diffs the test-file paths between the converged test-artifact commit and the code-artifact commit. A non-empty test-file diff during implementation is raised as a finding for the triager to adjudicate (was a test genuinely wrong, or was it weakened to pass?), not a free edit. This is a deterministic tripwire (a `git diff` over declared test paths), in the same "verify with a tool" spirit as the metrics validator (Principle 5, Principle 6). Test paths are declared once, either as a glob on the `kind = "test"` check entry or in the module guidance; where a language mixes tests and code in one file, the tripwire degrades to "reviewer confirms the assertions were not weakened", the same graceful fallback as elsewhere.

Interface-stub mechanics. Purpose: in statically-typed languages the tests cannot compile until the symbols they call exist, so a stub is authored first to give them something to bind to. Contents: types, function and method signatures, and module structure only; bodies are inert (`unimplemented!()`/`todo!()`, or a trivially-wrong typed return so the test runs to its assertion and fails there rather than panicking, which yields a cleaner red). It must contain NO real logic; the test-reviewer confirms the stub is inert, and the red gate independently confirms the tests fail (so an accidentally-correct stub is caught). In dynamically-typed languages (Python, JavaScript) the stub sub-step is often unnecessary because the tests can import missing-or-partial symbols; the module guidance makes the stub conditional ("add stubs only as far as the tests need to load or compile"). The stub is authored as the first writer action of step 1 and reviewed as part of the test artifact, not as its own reviewed artifact, so it does not add a third gate.

### 1.3 Config and tooling surface

What a project declares to enable test execution: a `kind = "test"` entry in the checks module's `.agents/checks.toml` (a user-owned, create-if-absent working file, Principle 3):

```
[[check]]
kind = "test"
command = "cargo test"          # green profile: expect exit 0 (all pass)
compile = "cargo test --no-run" # optional; expect exit 0 to distinguish "compiles and fails" from "does not compile", enabling the deterministic red gate
paths = ["tests/", "src/**/*_test.rs"]  # optional; scopes the frozen-tests tripwire
```

Ownership of this schema: checks.toml is one file with one schema and must be single-sourced (Principle 1), so the CHECKS module owns the full schema, including `kind = "test"` and its optional `compile` and `paths` keys. The checks module reserves them from increment 2 even though only test-driven activates the `compile`/red use; the base `kind = "test"` (run-and-expect-pass) is useful to the checks module alone. The checks module's shipped checks.toml should carry commented example entries for each recognized kind (lint, format, test) as inline schema documentation; this is the checks module's own concern (showing how to add checks) and is inert whether or not test-driven is on, so it does not perturb anything (Principle 2).

What the tool actually SCAFFOLDS for test-driven (all discipline, no runner, no config file of its own):

- A module guidance partial (source `modules/test-driven.md`), injected into the rendered `AGENTS.md` via a module slot (see below). It describes: the two-artifact / two-gate-profile model, the red/green expected outcomes, the interface-stub rule, the frozen-tests tripwire, and the per-step risk gating.
- Planner guidance: when the module is on, the planner adds a "Test specification" subsection to each step's Step Details (input -> expected output cases, properties/invariants, end-to-end flows, edge and error cases, and which are automated versus manual). This is authored by the planner per the module guide; it does NOT edit the dropped plan TEMPLATE.md, so the core template stays byte-identical when the module is off (Principle 2).
- No new role prompt. The test author and the implementer are the SAME role (the implementer): a writer making a small reviewable change. The test-artifact-versus-code-artifact distinction is a phase-and-profile distinction, not a role distinction, so no `test-author` role is added (Principle 2, minimal roles). The read-only side reuses the checks-reviewer as-is; the red profile is passed to it at spawn time.
- Guidance to add the `kind = "test"` entry (the tool does not write the entry itself because the test command is language-specific and checks.toml is user-owned; Principle 3).

The AGENTS.md injection mechanism. Modules that must add workflow guidance to the shared `AGENTS.md` reuse the EXISTING `{{instrument}}` pattern rather than inventing a parallel one. `AGENTS.md` already renders an optional `{{instrument}}` variable that is `instrument.md`'s content when instrumentation is on and the empty string otherwise, and the code base already asserts byte-identity of the no-instrument output (see `src/main.rs`: `instrument_off_omits_the_block_and_on_includes_it`, and the "AGENTS.md must not end with a blank line when --instrument is off" assertion). A module-guidance slot (a `{{modules}}` variable that concatenates the guidance partials of the enabled modules, empty when none) works identically and preserves the Principle 2 invariant (no module selected -> byte-identical core). This slot is FIRST needed by the CHECKS module in increment 2 (Q-25 already requires "workflow guidance wiring it in", which must land in AGENTS.md), so by the time test-driven is built the slot exists and test-driven simply contributes another partial to it. That is a further reason the checks module is the substrate.

### 1.4 Opt-in and risk-scaling

Opt-in: `--module test-driven` (which auto-enables `checks` via `requires`, reported in the resolved set). No new selection machinery beyond the `--module` flag increment 1 already provides.

Risk-scaling of the roughly-doubled artifact count: apply the two-artifact treatment PER STEP, matched to the step's risk class, rather than forcing it on every step. This reuses the "match the ceremony to the stakes" rule already in AGENTS.md (trivial changes collapse roles; risky ones keep them) and the convergence rule's existing low-risk/risky classification. With the module on, test-first is the DEFAULT for non-trivial steps; a trivial or low-risk step may implement directly with a recorded reason (exactly as roles collapse today), and the planner marks each step's test-spec as "automated per test-first" or "no test artifact: trivial, reason ...". So cost tracks risk: the doubling lands only where a defect is costly, keeping the module minimal in effect (Principle 2) without a separate knob. The human sets the default cadence at kickoff (test-first every non-trivial step, or only steps at/above a stated risk threshold), the same way the review cadence is set today.

### 1.5 Q-30 recommendation and reasoning

Build `test-driven` as a discipline-only content module that requires `checks`, drops a single AGENTS.md guidance partial plus planner guidance, adds no role and no config file, reuses the checks-reviewer under a red gate profile for the test artifact and a green profile for the code artifact, gates test execution through a `kind = "test"` entry in checks.toml (schema owned by checks), and enforces test-freezing with a git-diff tripwire. It doubles reviewed artifacts only for steps whose risk warrants it. This is the cleanest long-term architecture (Principle 1: no duplicated runner or config, no redundant role), it does not touch the core or the checks module when off (Principle 2, byte-identical via the same `{{instrument}}`-style slot), it keeps deterministic facts deterministic and LLM judgment scoped to genuine judgment (Principle 5, Principle 6), and it is safe on existing repos (Principle 3: checks.toml stays create-if-absent and user-owned).

## 2. Q-31: the mutation-testing module

### 2.1 Module boundary: reuse the checks-reviewer, add a once-per-step gate

Recommendation: `mutation` is a content module that REQUIRES `test-driven` (transitively `checks`). It adds a `kind = "mutation"` check and a once-per-step MUTATION GATE placed after the code artifact converges. It introduces NO new role and NO new config file: it reuses the checks-reviewer (which already "runs deterministic checks and writes findings") and the triager, and it reuses checks.toml. What is new is a PHASE POSITION (the gate) and the routing of its findings.

Why reuse the checks-reviewer rather than a `mutation-reviewer` role: running a mutation tool and collecting surviving mutants is a deterministic check that produces findings, mechanically the same as running a linter. A surviving mutant (a code mutation no test caught) is a finding with evidence (the mutant diff and the line). This is precisely the checks-reviewer's job; a separate role would duplicate the pipeline for no gain (Principle 2, minimal roles; Principle 1, no parallel machinery). The checks-reviewer is spawned to run the `kind = "mutation"` checks at the gate, exactly as it is spawned to run lint/format/test in the work-review round; only WHICH kinds and WHEN differ.

Why a separate gate rather than folding mutation into the code-review round: mutation is expensive and slow (it runs the suite once per mutant), so running it inside a loop that may execute several rounds would multiply an already-heavy cost. So the mutation gate fires ONCE per step, after the code artifact has converged green, not on every review round.

### 2.2 Workflow shape: the gate closes the hierarchy

After a test-driven step's code artifact converges (2.1 above, step 4 of phase 4), and only for a step gated for mutation (2.4), the orchestrator spawns the checks-reviewer to run the `kind = "mutation"` checks over the step's changed code. Each surviving mutant is a finding written to the findings file; the triager adjudicates (is this a real test gap, or an equivalent/uninteresting mutant to accept?). A valid surviving-mutant finding routes back to the TEST ARTIFACT (add or strengthen a test to kill the mutant), which re-converges under the red-then-green sequence (the new test fails first, then passes once already-implemented code covers it, so the red step here is a focused mini-loop on the added test), after which the mutation gate re-runs on the changed scope. This closes the stated hierarchy: mutation verifies the tests, the tests verify the code, the checks verify formatting and linting. Each layer adversarially checks the one below, the same escalation of "do not trust, verify" that runs through the whole tool.

Placing mutation findings against the TEST artifact (not the code) is the important routing choice: a surviving mutant is evidence the TESTS are weak, not that the code is wrong (the code already passed its green gate), so the fix is a stronger test. This keeps the module's meaning honest (it is adversarial verification of the tests themselves, per the intake) and reuses the existing test-authoring loop rather than adding a new fix path.

### 2.3 Config and tooling surface

What a project declares to enable mutation: a `kind = "mutation"` entry in `.agents/checks.toml`:

```
[[check]]
kind = "mutation"
command = "cargo mutants --in-diff HEAD~1"  # language-specific; cargo-mutants / mutmut / Stryker
budget = "10m"      # optional: a time cap; a timeout yields a partial result, not a hard fail
threshold = 0       # optional: max surviving mutants tolerated before a finding is raised
```

The tool is language-specific and far less universal than a test runner (cargo-mutants for Rust, mutmut for Python, Stryker for JS/TS), so the module scaffolds only GUIDANCE plus a commented example entry; the user fills the real command (Principle 3, user-owned checks.toml). The `kind = "mutation"` value is recognized the same way any check kind is (checks.toml admits arbitrary kinds; the checks-reviewer runs the command and reports per the module's expected outcome); the mutation module's guidance documents the kind and the gate, so no schema change beyond what checks already tolerates is required.

Cost bounding is a first-class part of the config surface, because expense is the module's defining trade-off: scope the mutation command to the DIFF (only mutate changed code, for example cargo-mutants `--in-diff`), and give it a time budget. A budget timeout is recorded as a partial result (mutants tested so far, with the rest reported as untested), not a hard failure, so the gate stays affordable and never blocks a step on an unbounded run. This is the module's answer to "expensive and slow": never mutate the whole tree, always the changed scope, always time-bounded.

What the tool scaffolds for mutation: a module guidance partial (`modules/mutation.md`) injected via the same module slot, describing the gate placement (once, post-convergence, diff-scoped, budgeted), the finding-routing to the test artifact, and the per-risk gating; guidance to add the `kind = "mutation"` entry; and nothing else (no role, no config file, no runner). Off -> byte-identical core (Principle 2).

### 2.4 Opt-in and risk-scaling

Opt-in: `--module mutation` (auto-enables `test-driven` and `checks` via transitive `requires`). Because it is the most expensive layer, it is gated MORE selectively than test-driven: only steps the plan marks as suite-critical (correctness-, security-, or money-sensitive; the same risk axis the convergence rule uses) run the mutation gate, and even then only over the changed diff within a time budget. Optional-on-optional, gated to when suite quality genuinely matters, exactly as the intake frames it. The human enables the module and sets the threshold at kickoff; the planner marks which steps warrant it.

### 2.5 Q-31 recommendation and reasoning

Build `mutation` as a content module requiring `test-driven`, adding a once-per-step, post-convergence, diff-scoped, time-budgeted mutation gate that reuses the checks-reviewer and triager, declares its tool as a `kind = "mutation"` checks.toml entry, and routes surviving-mutant findings back to the test artifact. No new role, no new config file, no new machinery (Principle 1, Principle 2). It is the top of the verification hierarchy and applies the tool's core ethos to the tests themselves (Principle 5, Principle 6), while its expense is contained by diff-scoping, time budgets, and the tightest risk gating of the three modules.

## 3. How they compose, with checks and with each other

Everything shares ONE substrate, which is the anti-duplication answer:

- One config file: `.agents/checks.toml`, kinds lint/format/test/mutation. Owned by the checks module; test-driven and mutation only add guidance to populate a `kind = "test"` and a `kind = "mutation"` entry. No second config, no drift (Principle 1).
- One read-only role: the checks-reviewer. It runs lint/format in the checks module, test (green) in the checks module's work-review round, test (red profile) for the test-driven test artifact, and mutation at the mutation gate. Same role, different (kinds, when, expected-outcome) parameters passed at spawn (Principle 2, minimal roles).
- One writer role: the implementer, authoring stubs, tests, and code alike. No `test-author`, no `mutation-fixer` (Principle 2).
- One findings-and-triage pipeline: the existing findings files and the separate triager adjudicate deterministic checks alongside LLM findings, unchanged.
- One AGENTS.md injection slot: the `{{instrument}}`-style `{{modules}}` variable, first established by the checks module, contributed to by each of checks, test-driven, and mutation, empty when none selected (Principle 2, byte-identical core).

The layering, stated as dependencies (each layer verifies the one below):

- checks: deterministic lint/format/test gate over code output. Standalone.
- test-driven: requires checks. Adds test-first ordering, the two-artifact / two-gate-profile model, the red gate, the interface stub, the frozen-tests tripwire, and the planner test-spec. Reuses checks' runner, checks-reviewer, and injection slot.
- mutation: requires test-driven. Adds the post-convergence mutation gate, routing findings to the test artifact. Reuses the checks-reviewer, triager, and injection slot.

The modules differ ONLY in (a) which kinds they add or activate, (b) which phase/gate/profile runs them, and (c) the discipline guidance they inject. That is the whole of the composition and the whole of why nothing is duplicated.

## 4. Sequencing

Confirm the plan's order: checks (`optional-modules` increment 2) -> test-driven (Q-30) -> mutation (Q-31). The ordering is not merely convenient, it is FORCED by dependency:

- test-driven requires checks for three concrete things: the deterministic test runner and checks.toml schema (`kind = "test"`, `compile`, `paths`), the checks-reviewer role it runs under two profiles, and the `{{modules}}` AGENTS.md injection slot that the checks module is the first to need. Building test-driven before checks would mean building all three inside test-driven and then tearing them out when checks lands, the opposite of Principle 1.
- mutation requires test-driven for the test artifact its findings route to and the two-artifact loop its fixes re-enter, and it requires checks for the runner and the checks-reviewer it reuses.

So the schedule stands: after the workflow self-improvement cluster and after checks (increment 2) lands, build test-driven, then mutation. I challenge nothing in the ordering. The one thing to state as a hard dependency for the plan: the `{{modules}}` guidance-injection slot must be a deliverable of the CHECKS increment (it is implied by Q-25's "workflow guidance wiring it in" but should be named explicitly), because test-driven and mutation both rely on it and it is the one piece of shared machinery beyond the checks-reviewer and checks.toml. If the checks increment somehow lands its guidance by a different mechanism, that becomes a small prerequisite step at the head of the test-driven increment.

## 5. Rejected alternatives

- test-driven as a self-contained module carrying its own test runner, its own config, and its own test-reviewer. Rejected: it duplicates the checks pipeline (a second config drifts from checks.toml, a second reviewer duplicates the checks-reviewer), violating Principle 1 and the single-source-of-truth aim. The discipline-only boundary avoids all of it.
- A new `test-author` writer role and a new `mutation-reviewer` read-only role. Rejected: the test-artifact-versus-code-artifact and the mutation-gate distinctions are phase-and-profile distinctions, not role distinctions. Reusing the implementer and the checks-reviewer keeps the role set minimal (Principle 2) and avoids parallel machinery (Principle 1).
- Encoding the red/green expected outcome IN checks.toml (a per-check `expect = "fail"` field, or a duplicate "red" check entry). Rejected: it couples the pure command declaration to workflow semantics and would need per-phase overrides, drifting the config. The profile is a phase parameter the orchestrator passes to the checks-reviewer; checks.toml stays a harness-agnostic list of commands (Principle 1).
- A purely LLM-judged red gate ("a reviewer confirms the tests fail"). Rejected: it is exactly the untrusted-LLM claim the module exists to eliminate (Principle 5, Principle 6). The deterministic compile-exit-0-plus-run-exit-nonzero signal does the mechanical part; the LLM judges only genuine judgment (non-vacuity, coverage of the plan's cases).
- Injecting module guidance by having each module edit AGENTS.md in place, or by dropping self-announcing files under `.agents/modules/` that a generic core sentence tells the orchestrator to read. Rejected in favour of the `{{instrument}}`-style `{{modules}}` render slot: the render-empty-when-off mechanism already exists and is already byte-identity-tested in `src/main.rs`, so reusing it invents nothing (Principle 1) and provably preserves the Principle 2 invariant, whereas in-place edits risk idempotency violations (Principle 4) and the self-announcing-file route needs both a core sentence and a discovery step for less coherence.
- Running mutation inside every code-review round. Rejected: mutation runs the suite once per mutant, so looping it multiplies an already-heavy cost. A single post-convergence, diff-scoped, time-budgeted gate gives the same signal affordably.
- Erroring when `--module test-driven` is passed without `--module checks`, rather than auto-enabling checks. A defensible alternative (more explicit about what is enabled), but rejected as the primary recommendation: auto-enabling the transitive `requires` closure makes the illegal state unrepresentable (Principle 5) and single-sources the dependency on the module metadata, and printing the resolved set keeps it from being invisible. The error variant remains an acceptable fallback if the human prefers explicit opt-in for every module.

## What not to build (the YAGNI boundary)

- No coverage-percentage gate. Line/branch coverage is a weaker, gameable proxy for what mutation measures directly; the test-spec-plus-red-gate establishes coverage of the PLAN's cases, and mutation establishes the tests actually kill defects. A coverage threshold would add a third, less meaningful number.
- No test SCAFFOLDING or generation by the tool. The tool scaffolds the discipline and the config surface, not the tests; tests come from the planner's spec and the implementer, so the tool stays a scaffolder (the same line the isolation module draws).
- No whole-tree mutation, ever. Always diff-scoped and budgeted; a full-tree mutation run is not a mode the module offers.
- No per-check red/green flag in checks.toml, no second "red" copy of the test check. The profile is a phase parameter, full stop.
- No new selection UI. `--module` (increment 1) plus transitive `requires` covers opt-in; the TUI module pane stays deferred as it already is.
