# Q-52 exploration: skeptic lens (false positives, semver, false confidence, auditor cost)

Advisory note. Read-only explorer. This is one lens (adversarial / skeptic) among several; it argues
the case AGAINST building the code-value audit as proposed and is deliberately one-sided so the
human's decision is not. The orchestrator's Q-52 description (plan.toml, "exploring" block,
2026-07-19) is the target: a multi-signal advisory tool combining static reachability + coverage +
a deletion experiment (remove candidate, see if any test or canary notices, mutant-survives =>
weak value) + traceability + churn-complexity hotspots, producing a human-triaged candidates
report. The adversarial question is: where does this tool produce outcomes that are WORSE than not
having it?

Judged throughout against the plan's numbered principles by name (plan.toml `[[principle]]` 1-8):
P1 Prefer-the-cleaner-long-term-architecture, P2 Minimal-by-default, P6 Ground-decisions-in-evidence,
P8 Structured-data-first.

## 1. False-positive taxonomy

A false positive here means: the tool flags code as "not earning its keep" when the code is
genuinely necessary. Flagging is not the same as deleting, but an advisory report is only useful if
its signal-to-noise ratio is high enough that a human reviewer can triage it in reasonable time. A
tool with a 50% false-positive rate is not an advisory tool; it is a coin flip with documentation.

### 1a. Dynamic dispatch and trait objects

In Rust, `dyn Trait` vtable dispatch is invisible to static call-graph analysis. A function `fn foo`
that is only reachable as a concrete impl of a trait method behind a `Box<dyn Trait>` looks
unreachable from the static reachability perspective: `foo` has no direct caller in the graph. The
deletion experiment does not resolve this either: if the test suite creates the `Box<dyn Trait>`
with a concrete type and calls through the trait object, the method is exercised, so the test suite
fails after deletion and the signal fires correctly. But if the test suite mocks or stubs the trait
object without using the concrete impl (common in unit tests, where a `MockThing` replaces the real
`impl`), the deletion experiment passes and the real impl is flagged as worthless. The function is
reachable in production via dynamic dispatch; the test suite's mock simply never touched the
concrete path.

The static reachability filter makes this structurally worse: it filters OUT the candidate before
the deletion experiment even runs, because the function looks unreachable. A function that is
unreachable to the static graph AND survived by the deletion experiment is the highest-confidence
false positive the tool can produce.

### 1b. FFI and `#[no_mangle]` functions

A Rust function declared `#[no_mangle] pub extern "C" fn ...` is called from C, C++, assembly, or
another language's FFI. No Rust test calls it by name: the test suite calls Rust, not the linking
language. The deletion experiment passes silently. The function is essential; the tool says it is
worthless.

The static reachability graph does not cross the FFI boundary. `cargo-udeps` and similar tools have
the same limitation; they document it explicitly. The difference here is that `cargo-udeps` flags
dependencies, not items within a crate, and the scope of false positives from a stale dependency is
bounded. Flagging a load-bearing `extern "C"` function as deletable and presenting it to a human
auditor under time pressure is not bounded.

### 1c. Public API surface consumed only downstream

The most structurally irreducible false positive category for any published Rust crate. Every `pub
fn`, `pub struct`, `pub trait`, and `pub const` that is part of the crate's published surface is a
contract with downstream consumers. Local tests cannot cover downstream use: they run inside the
crate, with access only to the same source tree the tool analyzes.

For this specific project, `agent-scaffold` is published to crates.io (`cargo install agent-scaffold`
works; the plan's status narrative says so). Any `pub` item in `src/` that is part of the library
surface is a potential API dependency. A deletion experiment that removes a `pub fn` and sees all
LOCAL tests pass has proven exactly nothing about downstream breakage.

Coverage tools compound this: they measure lines exercised by the LOCAL test suite. A `pub fn`
with 0% local coverage but 100% downstream caller saturation looks like a prime candidate to the
coverage signal. The multi-signal combination (low reachability + low coverage + deletion-safe)
INCREASES confidence in the false positive rather than reducing it.

### 1d. `#[cfg(...)]` gated and platform-specific code

Rust's conditional compilation is not observable by a deletion experiment unless the experiment
runs under all relevant cfg combinations. Code gated on `#[cfg(target_os = "windows")]` never
compiles on a Linux CI runner. The deletion experiment passes trivially because the code was never
compiled. Reporting "this code was not missed when deleted" when the code never compiled is a
tautology, not evidence.

Feature flags (`#[cfg(feature = "some-feature")]`) compound this: a test suite run without
`--all-features` leaves all feature-gated code unchecked. A deletion experiment run without
`--all-features` silently passes on feature-gated code. The tool would need to enumerate all
feature/cfg combinations and run the deletion experiment for each, which multiplies the already
large cost of the deletion experiment by the number of distinct cfg combinations (potentially
exponential in the number of feature flags).

The plan's checks module (`src/checks.rs`) uses `#[allow(dead_code, reason = "parsed for the schema;
used by the later mutation module")]` on `budget` and `threshold` fields in `Check`, explicitly
because the non-test build reports them dead while the test build would report the `#[expect]`
unfulfilled. This is a real and current example: the same code has different reachability depending
on the cfg. A deletion experiment on the non-test build would flag those fields. A deletion
experiment on the test build would not. Neither result is reliable evidence of the field's value.

### 1e. Test fixtures, examples, and `mod tests`

Test code does not earn its keep by being exercised by other tests; it earns its keep by exercising
production code. A deletion experiment cannot distinguish "this helper is only called from tests,
making it low-value" from "this helper IS test infrastructure and deleting it breaks the tests that
exercise production code." In the second case, the deletion experiment catches the breakage
(the tests fail), so the signal fires correctly. But the first case, a test helper called from
integration tests that is also responsible for setting up the fixture state that makes the
integration tests pass, will fire the signal correctly too: the integration tests fail. So the
deletion experiment actually handles test code better than static reachability.

The false positive here is subtler: items in `examples/` that are example programs for users, not
tests. They are not run by `cargo test`. Deleting them and running `cargo test` produces no signal.
But example programs are part of the published documentation contract for a crate. Flagging them as
worthless because `cargo test` did not notice is wrong.

### 1f. Error paths and defensive code not exercised by the test suite

This is the highest-cost false positive category in terms of production risk. Test suites
systematically under-exercise error paths: disk-full behavior, out-of-memory handlers, network
timeouts, permission-denied branches, parse failures on malformed input the test suite does not
generate. These paths are Chesterton's fence par excellence: the person who wrote them knew a real
failure mode, the test suite author did not write a test for it (or the failure mode cannot be
easily injected in tests), and the deletion experiment passes.

Consider `RunError::GitUnavailable` in `src/checks.rs` (lines 256-258): it is produced when the
`git` binary is not found. The test suite (lines 854-1211) calls `git_ok` which asserts success;
no test simulates a missing git binary. A deletion experiment that removes the `GitUnavailable`
variant and its match arm in `fmt::Display` would likely fail to compile (the match is exhaustive),
so the signal fires correctly at the structural level. But a deletion experiment that removes only
the user-facing error MESSAGE for `GitUnavailable` (the string in `Display`) and leaves the
variant present would probably not be detected by the test suite. The function that produces a good
error message is not exercised; the deletion experiment marks it deletable. The user who runs the
tool without git installed gets a bad error message.

This is the general pattern: the VALUE of the defensive code is that it handles a case that tests
do not exercise. The deletion experiment's power to detect lost value is exactly zero for these
cases, because the test suite was not written to exercise them.

### 1g. Readability and future-proofing code

Named constants, type aliases, well-named helper functions that reduce duplication for human
readers. These have no distinct runtime behavior from an inlined literal. `const MAX_RETRIES: u32 =
3;` is functionally identical to `3u32` at every call site. A deletion experiment (replace the
constant with its value, or delete the helper and inline) passes. The human maintainability cost is
invisible.

This category also includes extension points: a pluggable interface designed to accommodate a
future requirement. A function that exists to make the architecture extensible has no callers yet.
Deleting it and running the tests passes. Building it back in a year, after the requirement
materializes, costs more than keeping it. This is the direct definition of Chesterton's fence and
is unobservable to any test-based signal.

### 1h. Rust-specific: trait implementations for standard library traits

`impl From<X> for Y`, `impl Display for X`, `impl Error for X`, `impl Iterator for X`, `impl
Clone for X` (when not derived). These are called via the `?` operator, format macros, trait
bounds in generic code, and the standard library machinery. Call-graph analysis may not resolve the
connection: the compiler calls `From::from` through a desugaring, and `Display::fmt` through
format string machinery. The static reachability graph may not trace through these.

More specifically: if `impl From<io::Error> for RunError` (present in `src/checks.rs`, line 309)
is never called explicitly in tests (it is called via `?` in the checked functions), the static
graph may miss it. If the tests exercise code that uses `?`, the deletion experiment catches the
break. But if the tests only call the high-level `run` function, which internally uses `?`, and
the static graph does not follow `?` through the `From` impl, the item appears unreachable. The
deletion experiment is the stronger signal here and should catch it, but only if the test exercises
the error path that triggers the conversion, which returns to the error-path coverage problem.

### 1i. Rust-specific: `proc_macro` crates and `build.rs`

A `proc_macro` crate's entire value is at compile time: it generates code that the compiler
substitutes. There is no runtime call to the macro function. A deletion experiment at runtime would
not detect its absence; the compile would fail. But the deletion experiment as described (delete
code, run tests) includes compilation; so if deleting the proc macro causes a compile failure, the
signal fires correctly. This is a category where the deletion experiment works correctly, but it is
worth flagging because the tool's coverage component would misreport it: a proc macro has 0% RUNTIME
test coverage by definition. A naive coverage-based filter would flag it as a deletion candidate
before the deletion experiment even runs.

Similarly, `build.rs` runs at compile time. In this project specifically, `build.rs` is present
and its value is visible in the test suite only insofar as the compile succeeds; the script's
internal logic is not covered by runtime test coverage metrics.

## 2. The public-API / semver problem

The deletion experiment is structurally and completely blind to downstream consumers. This is not
a tooling gap that a better implementation could close: it is a fundamental property of the
experiment's design. The experiment asks "does anything in THIS repository observably notice when
I remove this code?" Downstream consumers are not in this repository.

For a published Rust crate, the correct tool for this concern is `cargo-semver-checks`, which
compares a new version of the crate against a previously published version and reports API
breakage. It operates on the published interface, not on local test coverage. It is a separate,
well-scoped tool with its own limitations (it catches signature changes and removals but not
behavioral changes) and is not a substitute for the deletion experiment; rather, it catches the
category the deletion experiment is categorically blind to.

The implication is not "add cargo-semver-checks to the pipeline" but "the deletion experiment's
output must ALWAYS be pre-filtered to exclude every public item before a human sees it." If this
filter is not applied, the highest-confidence false positives (public items with no local tests)
will be at the top of the advisory report, because they have both low local coverage and a
deletion-experiment pass. A human auditor who follows the report will delete public API items,
break downstream consumers, and learn not to trust the tool.

The coverage signal has the same blindness. A `pub fn` that is called only from downstream
consumers shows 0% coverage in the local test suite. Combining 0% coverage with a deletion
experiment pass and a low reachability score produces the highest-possible "not earning its keep"
composite score for exactly the class of items that are most certainly earning their keep for
downstream consumers.

For this specific project (`agent-scaffold` on crates.io), this means: any `pub` item in any
module exposed through the crate root must be excluded from the advisory report before it reaches
a human. If the tool does not do this exclusion automatically, every advisory report for every
published crate is actively harmful.

## 3. Chesterton's fence and the false-confidence cost asymmetry

The fence argument: do not remove a fence until you understand why it was built. The deletion
experiment answers "the test suite does not notice when I remove it." The test suite is not a
complete oracle of why code exists. A "safe to delete" verdict from the tool is therefore not
evidence of safety; it is evidence only that the test suite does not notice, which is a much
weaker claim.

Cost asymmetry: the cost of a false negative (real dead code is not flagged) is that the dead code
stays in the repository. It accumulates some maintenance burden: it must be compiled, read during
future refactors, and occasionally updated when surrounding code changes. This is a real but
diffuse and recoverable cost. The cost of a false positive acted upon (real useful code is deleted)
is: a regression (production failure), a security hole, an API break, or a data loss event. These
are acute and may be hard or impossible to recover from (especially if the deletion reaches
production before the git history is consulted).

The asymmetry strongly favors erring toward false negatives. A tool that flags genuinely dead code
at 70% accuracy and genuinely live code at 30% false positive rate is not a useful audit tool; it
is noise with a formal-looking presentation. The operative question is what false-positive rate the
tool actually achieves across the categories above. Given that the highest-volume false-positive
categories (public API, cfg-gated code, error paths, FFI) are structural rather than incidental,
the tool's false-positive rate in real Rust codebases is probably high enough to make the
advisory report expensive to triage.

False confidence is the subtler risk. An audit that runs and produces a "no candidates found"
result is taken as evidence that the codebase is clean. But the tool's systematic blind spots (all
the categories above) mean "no candidates found" means "no candidates passed all the filters I
know how to check." It is not evidence of absence of dead code. A team that relies on periodic
audits to discipline their codebase will be less vigilant about the categories the audit misses.
The audit becomes a compliance ritual that substitutes for the judgment it was meant to support.

The false-confidence cost is asymmetric in time: the false confidence accumulates slowly and
invisibly, while the cost of acting on a false positive is visible and acute. The auditor who
says "I audited last quarter; we are clean" is harder to argue with than the evidence that the
audit missed the FFI boundary.

## 4. The auditor's own cost

### 4a. The deletion experiment's runtime cost

The deletion experiment is O(candidates x (compile_time + test_time)). For a typical mid-sized
Rust crate: compile time 30-120 seconds, test time 10-60 seconds. For 100 candidates, that is
50-180 minutes of CI time per audit run. Incremental compilation helps when only one item is
removed, but:

- Removing a `pub` item from a widely-used module may invalidate a large portion of the build graph.
- Each deletion must run in an isolated worktree (the checks module already does this, `src/checks.rs`
  lines 315-342), so there is no cross-candidate compilation sharing.
- Candidate identification (static reachability, coverage ingestion, traceability scan) adds
  additional upfront cost before the deletion loop even starts.

The mutation testing literature documents this cost and treats it as a primary limitation: mutation
testing is typically not run on every commit and is scoped to small, high-value modules. The
deletion experiment is structurally equivalent to a class of mutation operators (body-nulling,
item removal) and inherits this cost. The Q-52 description explicitly references the mutation
module already planned for this project. If the mutation module is itself deferred (labeled
`Kind::Mutation` as "reserved for the later mutation module," `src/checks.rs` line 95), building
the deletion experiment as a separate facility before the mutation module is piloted introduces a
duplicate mechanism before the first one ships.

### 4b. The tool's own maintenance cost

The audit tool is code that can rot. The false-positive exclusion lists (public API items, cfg
variants, FFI boundaries) must be kept current as the codebase evolves. A stale exclusion list
means the tool's false-positive rate silently increases until a human notices that the advisory
report is full of known-good items. By P2 (Minimal-by-default), a tool that requires ongoing
configuration and exclusion-list maintenance to remain useful imposes a continuous tax on the team
that uses it.

The integration dependencies also rot: the tool must integrate with coverage reporting
(potentially `llvm-cov` or `grcov`), static call-graph analysis (potentially a custom tool or
`cargo-call-stack`), and the planned mutation module. Each dependency brings its own versioning
and API evolution. Keeping the integration current across Rust edition bumps, LLVM version changes,
and cargo tool updates is non-trivial maintenance.

### 4c. Ceremony and alarm fatigue

Once a periodic audit is introduced into the workflow, it becomes an expected artifact. Teams learn
to run it and file the output, regardless of whether the output is useful. If the false-positive
rate is high, the human triager learns to dismiss most findings. The audit becomes ceremony: a
checkmark in the process rather than a genuine quality signal. Alarm fatigue makes the tool
WORSE than nothing: the team is less attentive to the real dead-code cases because the audit is
supposed to catch them.

This project's own workflow is particularly vulnerable to this failure mode. The review entry mode
and the triager role are designed to process findings. An advisory report from the code-value audit
feeds into the triage loop. If the report has high false-positive noise, the triager wastes rounds
on non-findings. The triage loop is a scarce resource (it has a review streak limit, an escalation
path, and a convergence requirement). Feeding it with noisy audit output degrades the loop for
legitimate findings.

### 4d. When NOT building is correct (minimal-by-default gate)

By P2 (Minimal-by-default) and P6 (Ground-decisions-in-evidence), the question is not "is the
audit tool a good idea in principle?" but "does the observed evidence of dead-code cost, in this
codebase, at this stage, justify building a heavy multi-signal audit tool?"

Concrete counter-evidence: Rust's compiler already emits `dead_code` warnings for unreachable
items, and `clippy` extends this with additional dead-code lints. These are zero-cost (they run as
part of compilation), have no false positives in the FFI and cfg-gated categories (the compiler
knows cfg semantics), and have very low false-positive rates for the other categories. The plan
already requires `cargo clippy --all-targets -- -D warnings` before each commit (plan.toml status
narrative). If the compiler and clippy are already enforcing dead-code discipline, the incremental
value of a multi-signal audit tool on top of that is: coverage of cases the compiler misses (error
paths, readability code, public API) minus the false-positive noise the tool introduces.

Given the false-positive categories analyzed above, the incremental true-positive rate of the
deletion experiment over what `clippy -D warnings` already catches is not obviously large, while
the incremental false-positive rate is structurally guaranteed to be high (public API, cfg-gated,
FFI, error paths). This is the minimal-by-default argument against building the tool: the tool
that is already present (the compiler + clippy) has better precision in the common cases, and the
proposed tool's incremental value is in the rare cases that require expensive per-candidate
compilation cycles.

## 5. Guardrails that would make it safe, and the parts NOT worth building

### Required guardrails (the deletion experiment is dangerous without these)

- Public-API filter: every `pub` item that is part of the crate's published API surface must be
  excluded from deletion candidates BEFORE any human sees the advisory report. Not opt-in; mandatory.
  For a crate with no binary (a library crate), this is the entire public interface. For a binary
  crate with a library interface, it is the public items of the library. Without this filter, the
  highest-confidence findings in the report are the highest-confidence false positives.
- cfg combination coverage requirement: the tool must enumerate and test all `#[cfg(...)]`
  combinations relevant to the candidate, or it must exclude cfg-gated code entirely. A deletion
  experiment run on a single cfg combination produces unreliable results for cfg-gated items.
  Given the cost multiplication (N candidates x M cfg combinations), this requirement may make the
  experiment impractical.
- FFI exclusion: every item with `#[no_mangle]` or `pub extern "C"` must be excluded entirely.
  No test harness can reliably simulate a C caller.
- Error-path disclosure: the advisory report must explicitly flag that error paths and defensive
  code are structurally under-covered by the deletion experiment, and that a passing deletion
  experiment for an item that handles error cases is lower-confidence than for an item that
  handles the happy path.
- Explicit "not evidence of absence" caveat: the report must communicate that a "no candidates
  found" result does not mean the codebase has no dead code; it means no candidates passed the
  tool's incomplete filters.

### Parts NOT worth building

- The deletion experiment as a general-purpose code-value engine: the compiler and clippy already
  cover the common case at zero marginal cost. The deletion experiment's incremental value is
  concentrated in the cases where the test suite exercises the code but the code adds no
  distinguishable behavior, which is a narrow and expensive-to-find category. Defer until there
  is a measured count of cases where clippy passed and the deletion experiment would have caught
  real dead code.
- A coverage-to-deletion-candidate pipeline: coverage is necessary but not sufficient for value.
  Using low coverage as a deletion candidate filter systematically pre-selects for the
  false-positive categories (public API, cfg-gated, error paths). A coverage filter combined with
  the deletion experiment is more harmful than the deletion experiment alone.
- A churn-complexity-to-deletion-candidate combinator: churn x complexity measures maintenance
  cost; the deletion experiment measures behavioral necessity. These are orthogonal signals. A
  high-churn low-complexity function (e.g., a constants file that changes often) scores high on
  maintenance cost and zero on the deletion experiment. Combining them without a validated model
  of their interaction produces a composite score that is hard to interpret and easy to
  misuse.
- Any automatic deletion, commit generation, or PR creation: the advisory output must terminate
  at human review. If the tool can delete or stage deletions, the false-confidence problem
  compounds: a human who approves a tool-generated PR is in a weaker epistemic position than one
  who reviewed the deletion themselves.
- A combined multi-signal audit before the mutation module is piloted: the deletion experiment is
  structurally equivalent to a class of mutation operators. The mutation module is already
  planned and its `Kind::Mutation` variant is reserved in the checks schema (`src/checks.rs`
  line 95). Building the deletion experiment as a separate facility before the mutation module
  exists creates two overlapping mechanisms. The correct sequencing is: pilot the mutation module
  on a small, well-scoped module; measure its false-positive rate in practice; then extend the
  coverage if the false-positive rate is acceptable.

### The one part worth building first

The narrowest useful subset: a report of items that are `private` (not `pub`), have zero static
callees in the same crate (dead by the compiler's own judgment, i.e., items that would already be
flagged by `#[warn(dead_code)]` if the compiler's dead-code analysis were run without
suppression), and have been stable (no churn) for a configurable period. This does not require
a deletion experiment, does not require running the test suite, and is O(1) in compilation cost
because it reuses the compiler's own dead-code analysis. It avoids ALL the false-positive
categories above: public items are excluded by the `private` filter, cfg-gated items are excluded
because the compiler knows their cfg context, FFI items are excluded because they are `pub extern`,
and error paths are partially excluded because they are typically called (the borrow checker and
match exhaustiveness enforce this). The output of this subset is a list that `clippy -D warnings`
would also produce in a stricter form. If this subset produces no actionable findings, the more
expensive deletion experiment is unlikely to produce more. If it does produce findings, they can
be reviewed and acted on without the tool.
