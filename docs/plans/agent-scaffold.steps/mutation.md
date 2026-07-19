### `mutation`: Mutation-testing module (`Q-31`)

Not started; after `test-driven` (dependency-forced). Decided (`Q-31`, human, same design pass; design record in `docs/plans/test-modules.explorations/test-driven-and-mutation-{A,B}.md`). An OPTIONAL module layered ON TOP of `test-driven` that mutates the code and confirms the tests kill the mutants, verifying the tests are robust rather than vacuous (adversarial verification applied to the tests themselves). Conceptual hierarchy, each layer verifying the one below: checks module (runs lint/format/test) -> `test-driven` -> `mutation`.

Decided design:

- Dependency: HARD-DEPENDS on `test-driven` (auto-enabled via `requires`), which in turn requires checks.
- Config: a `kind = "mutation"` entry in `.agents/checks.toml` (with the optional `budget` / `threshold` fields the checks schema reserves).
- Cadence: fires ONCE per step AFTER green convergence (not every round), diff-scoped (mutates only the step's changed code), time-budgeted, and risk-scaled (risky steps only), reusing the existing convergence risk classification so cost is spent where suite quality matters.
- Review: reuses the checks-reviewer at a DISTINCT gate (no separate mutation-reviewer role); a SURVIVING mutant is a finding that routes back to the TEST artifact (the tests, not the code, are strengthened).
- Graceful degradation: when no mutation tool exists for the language, the module degrades to a RECORDED skip rather than a failure (tooling is language-specific and less universal than test runners, for example `cargo-mutants`, `mutmut`, Stryker).
