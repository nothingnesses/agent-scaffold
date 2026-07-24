# code-value-audit (Q-52): the concrete multi-signal "earning its keep" report (Explorer 2, multi-signal design lens)

Advisory design notes for the multi-signal audit report Q-52 asks for, grounded in this repo's actual `checks`, `next`, `metrics`, `validate`, and the `review`/acceptance workflow. Scope: the per-candidate report schema, how each of the five signals is computed and what it costs, the deletion-experiment design and how it maps onto the reserved `mutation` kind inside `checks`'s isolated-worktree machinery, the workflow integration point, and an explicit minimal-by-default build order. This is one lens (the concrete design); a prior-art survey and a skeptic lens are separate documents. All claims below are re-derived from `src/checks.rs`, `src/main.rs`, `src/metrics.rs`, `src/next.rs`, and `AGENTS.md`.

## Thesis (falsifiable, restated)

Code earns its keep iff removing it causes an OBSERVABLE loss: a test fails, the build breaks, a dependent module stops compiling, a documented requirement / public-API contract is violated, or a production behaviour changes. Usefulness is relational, not intrinsic; the audit makes it objective by making it falsifiable, the direct analogue of mutation testing (mutation asks "if I change this, does a test catch it?"; the audit asks "if I REMOVE this, does anything observably notice?"). No single signal decides a candidate: static reachability is blind to reflection and public API, coverage is necessary-not-sufficient, and the deletion experiment is the only DIRECT test of the thesis but is the most expensive. The report combines them and hands a human the evidence; it never deletes.

## What a "candidate" is in THIS repo

This crate is a binary (`agent-scaffold`), not a published library, so it has no external semver surface today; every `pub` item is crate-internal. A candidate is therefore a code unit inside the crate:

- A free function or an `impl` method (for example `checks::run_command`, `src/checks.rs:684`).
- A `struct`, `enum`, or a single field (for example the `Check::budget` / `Check::threshold` fields, `src/checks.rs:131-142`, parsed for the schema but not yet read).
- A whole module, or a `[[check]]` kind branch.
- A dependency in `Cargo.toml` (unused-dep is a first-class candidate class).

The CLI subcommand surface (`Command`, `src/main.rs:361-375`) and the cross-crate-consumed `pub` items are the crate's DE FACTO contract and are treated as the "public API" for the exclusion rule below, even though nothing is published yet. If a library crate is later split out, the same `semver_public` gate applies to its real `pub` surface.

## Report schema (per candidate + per-signal evidence)

The report is a KEPT advisory deliverable, a sibling of the review-entry-mode report at `docs/plans/<task>.review-report.md` (`AGENTS.md:69`). Proposed path: `docs/plans/<task>.code-value-report.md`, synthesised from a machine-readable intermediate (JSON) so the evidence is regenerable and the Markdown is a projection of it (Structured data first, project for humans). One record per candidate:

- `id`: stable path, `module::path::item` (for a dep, the crate name).
- `kind`: one of `fn`, `method`, `struct`, `enum`, `field`, `module`, `dep`.
- `span`: `file:line-line` (the evidence anchor, the same `file:line` form review reports use, `AGENTS.md:69`).
- `visibility`: `private`, `pub_cross_module`, or `pub_api` (the CLI/contract surface above).
- `semver_public`: boolean; `true` forces the verdict to `excluded` and routes to a deprecation cycle, never deletion (the gate below).
- `signals`: the five sub-objects, each `{ value, evidence, cost_tier }`:
  - `reachable`: `{ value: reachable | unreachable | unknown, evidence: <lint id / caller list / self-declared marker>, cost_tier }`.
  - `exercised`: `{ value: covered_pct (0-100) | none, evidence: <coverage region ids>, cost_tier }`.
  - `deletion_safe`: `{ value: survives_all | caught | not_run, evidence: <surviving-mutant id or build/test failure>, cost_tier }` (the STRONG signal; `survives_all` means "not earning its keep").
  - `traceable`: `{ value: traced | untraced, evidence: <step slug / Success Criterion / test name / API contract>, cost_tier }`.
  - `cost`: `{ churn: <commits touching span>, complexity: <branch/line proxy>, hotspot_score, evidence: <git range>, cost_tier }`.
- `verdict`: DERIVED, not stored as truth: `earning_keep` | `review_candidate` | `excluded`. A candidate is `review_candidate` only when the direct signal agrees with the cheap ones: `deletion_safe == survives_all` AND (`reachable == unreachable` OR `exercised == none`), and `semver_public == false`.
- `recommendation`: advisory prose (delete, inline, add a test, or keep-and-document-why).
- `human_decision`: empty until a human triages it (the report is advisory; the human owns each call).

Verdicts are recomputed from the signals every run (nothing here is a source of truth), matching how `status`/`next` are best-effort projections of durable files (`src/main.rs:368-370`).

## How each signal is computed, and its cost

Cost tiers below: T0 = free (already produced by a normal build/test), T1 = one extra bounded tool run, T2 = a per-candidate isolated build/test, T3 = a per-candidate mutation run (the most expensive).

### 1. reachable / depended-on? (static). Cost: T0-T1.

- rustc already emits `dead_code` and `unused_*` warnings on every `cargo build`; harvest them from `cargo build --message-format=json`. This is free (T0) and authoritative for statically-unreachable private items.
- Self-declared markers are direct evidence: `#[allow(dead_code)]` / `#[expect(dead_code)]` annotations are the author admitting an item is not statically reachable yet. This repo has live examples: `Check::budget` and `Check::threshold` carry `#[allow(dead_code, reason = "parsed for the schema; used by the later mutation module")]` (`src/checks.rs:135`, `src/checks.rs:140`). The audit reads such an annotation as `reachable = unknown` WITH the author's stated reason as evidence, not as a candidate: this is Chesterton's Fence encoded in the source.
- Unused dependencies: `cargo-machete` (no nightly needed) over `Cargo.toml`; each unused crate is a `dep` candidate. Cost T1.
- Known blind spots (why this signal is necessary-not-sufficient): reflection, FFI, dynamic dispatch through trait objects, and the CLI dispatch table (`src/main.rs:566-575`, which reaches every subcommand handler). The audit records the blind spot rather than trusting `unreachable` alone; that is why the deletion experiment exists.

### 2. exercised? (test coverage). Cost: T1.

- `cargo-llvm-cov` region coverage over `cargo test`, projected per candidate span. `exercised = none` for a private item is a necessary condition for "not earning its keep" but never sufficient (a well-covered helper can still be redundant; an uncovered item can still be load-bearing).
- Production coverage / "dark code" sampling is N/A for this repo: `agent-scaffold` is a short-lived dev CLI with no long-running production process to sample. This is a deliberate scope cut (see YAGNI below), not an oversight; the field stays in the schema for a future library or service consumer.

### 3. deletion-safe? (the DELETION experiment). Cost: T2-T3. The strong signal.

This is the direct test of the thesis and reuses `checks`'s existing isolated-worktree machinery. Two granularities:

- Function/method granularity: run a mutation pass that replaces the candidate's body with a default return (body-nulling), which is exactly the "pseudo-tested method" experiment (Niedermayr et al.): a mutant that SURVIVES the whole test suite means no test pins the body, so the code is effectively untested / weak-value. This maps directly onto `cargo-mutants`, whose default operator replaces function bodies. `cargo-mutants` is already the intended tool for the reserved mutation kind (a test fixture writes `command = "cargo mutants"`, `src/checks.rs:1250`).
- Item granularity (for `struct`/`enum`/`field`/`module` that a body-replacement operator does not target): actually remove or `#[cfg(any())]`-gate the item in the isolated worktree, then run `cargo build` and `cargo test`. If it still builds and every test passes, the deletion caused NO observable loss (`deletion_safe = survives_all`); a compile error or a test failure is the observable loss that earns its keep, captured as evidence.

Both granularities run in a throwaway worktree so the candidate mutation touches only discardable state, never the live tree; see the mapping section for the exact anchors.

### 4. traceable? (maps to a requirement / test / contract). Cost: T0-T1.

- Does a test name reference the item (from the coverage/test index of signal 2)?
- Does a Roadmap step slug or a Success Criterion name it? Cross-reference the plan's structured skeleton (`src/plan.rs`, the Roadmap and Success Criteria) the way `validate --workflow` already joins rounds to steps by slug (`src/metrics.rs:614+`).
- Is it on the CLI/contract surface (`visibility == pub_api`)? A public-contract item is a LEGIT keeper even if privately unreachable, because the contract is the requirement it traces to. `untraced` is a suspicion multiplier, not a verdict by itself.

### 5. cost > value? (churn x complexity hotspot). Cost: T1. The burden side.

- Churn: number of commits touching the candidate's span, from `git log --follow --numstat` over the file, attributed to the line range. This is the Tornhill/CodeScene behavioural-code-analysis hotspot metric.
- Complexity: a cheap proxy (branch count or non-blank lines in the span); a full cyclomatic tool is deferred.
- `hotspot_score = churn * complexity`. This does not decide keep-or-cut on its own; it RANKS the `review_candidate` set so a human triages the highest-burden weak-value code first. A high-hotspot item that is well-traced and deletion-caught is kept but flagged for refactor, not deletion.

## The deletion experiment mapped onto `mutation` / `checks` (file:line)

The `checks` module already owns the isolated-worktree machinery the deletion experiment needs; the audit reuses it rather than building a second isolation path (Minimal by default; Prefer the cleaner long-term architecture over the smallest diff).

- The check kinds are a closed enum `Kind { Lint, Format, Test, Mutation }` (`src/checks.rs:86-96`). `Mutation` exists as schema but is currently RESERVED and skipped: `runnable_for` returns `Runnable::Skip("mutation checks run in the later mutation module")` (`src/checks.rs:671-672`), and its doc says it belongs to a later module (`src/checks.rs:94-95`). The deletion experiment is that later module's job.
- The `Check` schema already carries the two knobs the experiment needs to bound its cost: `budget` (a wall-clock cap) and `threshold` (max surviving mutants), both parsed today but unused (`src/checks.rs:131-142`). The audit's mutation pass reads `budget` to cap runtime and `threshold` to set the surviving-mutant alarm, so no new config surface is invented.
- The isolation itself: `checks::run` (`src/checks.rs:734`) resolves the repo top level, prunes any orphaned runner worktree (`src/checks.rs:783`, `prune_orphan_worktrees` at `src/checks.rs:407-461`), captures the working-tree (or `HEAD`) state as a commit (`isolation_commit`, `src/checks.rs:493`), and creates a detached throwaway worktree with `git worktree add --detach` (`src/checks.rs:793-800`). Each command runs there via `sh -c` with stdin nulled (`run_command`, `src/checks.rs:684-724`). A `WorktreeGuard` `Drop` removes the worktree on every return (`src/checks.rs:315-341`), and a startup prune reclaims a worktree orphaned by a hard kill. The deletion experiment writes its body-replacement or item-removal into THIS discardable worktree, so a mutated build never touches the live tree; this is precisely the isolation guarantee the module documents (`src/checks.rs:15-42`).
- `cargo-mutants` performs the body-replacement operator, so the function-granularity deletion experiment IS a `Kind::Mutation` check running `cargo mutants` in the isolated worktree; the audit consumes its surviving-mutant list as the `deletion_safe` evidence. The item-granularity path (remove-then-build/test) is a thin additional runner in the same worktree for kinds `cargo-mutants` does not target.

Scoping is the cost control: the mutation/deletion pass runs ONLY over the shortlist that the cheap static + coverage signals already flagged, never over the whole crate, so the T3 cost is paid on a handful of candidates, not thousands.

## Workflow integration point (command / entry mode / cadence)

The audit is periodic and advisory, exactly the shape of the acceptance and review passes. Three integration seams, in order of how the pieces already fit:

- Cadence and trigger: run at the acceptance/UAT gate (`AGENTS.md:33`, phase 5, which already runs reviewers-then-triager and a documentation-currency check when no pending steps remain), or on a manual cadence a human invokes, matching Q-52's "run periodically ... at the acceptance/UAT step." The audit is NOT a convergence-blocking gate; like acceptance it is a single advisory pass whose findings route into triage.
- Command surface: add an `agent-scaffold audit` subcommand alongside `Scaffold / Validate / Status / Next / Checks / Render` (`src/main.rs:361-375`). It reuses `checks`'s worktree machinery for the deletion experiment (as `Checks` does, `src/main.rs:371-372`) and, like `next` (`src/main.rs:369-370`), is read-only with respect to the product: it writes ONLY its own report, never edits code. It offers `--json` (the machine intermediate) and a human Markdown projection, mirroring `next`'s `--json` (`src/main.rs:485-487`) and the `NextProjection`/`ActiveLoop` projection pattern (`src/next.rs:110-169`, `src/next.rs:564`).
- Feeding the loop: the report's `review_candidate` entries are the input to the existing review -> implement path. A human reads the kept report, decides each candidate (delete, inline, add a test, or keep-and-document), and turns accepted deletions into a `kickoff` task, exactly the handoff the review entry mode already produces (`AGENTS.md:47`, `AGENTS.md:69`). The audit itself never implements a deletion.
- Instrumentation: when `--instrument` is on (`src/main.rs:413-417`), emit ONE summary record to `docs/metrics/workflow.jsonl` so audit runs are calibratable, the same way a review-entry-mode run appends one `round` record with `phase: "review"` (`AGENTS.md:69`, the `Phase` enum at `src/metrics.rs:62-73`). A new `type: "audit"` record (candidate count, verdict histogram, deletion-experiment cost) is validated by `validate_log` (`src/metrics.rs:989`) and counted by `count_records` (`src/metrics.rs:610`). This record is DEFERRED (see below); the kept Markdown report is the always-on deliverable.

## What to build first vs defer (minimal-by-default ordering)

Build in cost order, each tier a shippable advisory report on its own, so the cheap high-signal slice lands before the expensive machinery (Minimal by default; Ground decisions in evidence):

- Tier 0 (build first): a report from the ZERO-extra-cost static signals only. Harvest rustc `dead_code`/`unused_*` warnings from `cargo build --message-format=json`, read `#[allow(dead_code)]`/`#[expect(dead_code)]` markers as author-declared reasons (`src/checks.rs:135`, `src/checks.rs:140`), and run `cargo-machete` for unused deps. Emit `docs/plans/<task>.code-value-report.md` with the `reachable` signal populated and the other four marked `not_run`. No new isolation, no deletion experiment, no subcommand yet (a script or a thin `audit` command). This is the 80/20 slice.
- Tier 1: add `exercised` (cargo-llvm-cov) and the cheap `traceable` (test-name and plan-slug cross-reference). Still no isolation.
- Tier 2: add the deletion experiment as a `Kind::Mutation` pass (`cargo-mutants`) plus the item-granularity remove-then-build/test runner, scoped to the Tier-0/Tier-1 shortlist, bounded by the existing `budget`/`threshold` fields (`src/checks.rs:131-142`), reusing `checks::run`'s worktree machinery (`src/checks.rs:734`). This turns the reserved `Mutation` kind (`src/checks.rs:94-95`, `src/checks.rs:671-672`) into a live runner. This is the strong signal and the first materially expensive tier.
- Tier 3: add the `cost` hotspot (churn x complexity) to RANK the shortlist, and formalise the `audit` subcommand with `--json` + acceptance-gate integration and the `--instrument` `type: "audit"` record.

## What NOT to build (YAGNI boundary) and the principle gates

- NEVER auto-delete. The report is advisory; a human decides each candidate (Chesterton's Fence, Advisory over autonomous). The tool has no write authority over the product, matching `next`'s read-only stance (`src/main.rs:369-370`).
- EXCLUDE the public-API/contract surface from deletion candidacy. When `semver_public == true` (or `visibility == pub_api` for the CLI contract), the verdict is forced to `excluded` and any change routes through a deprecation cycle, never a raw deletion. A published library's surface would use `cargo-semver-checks`, but only if and when a lib crate ships; do not build a semver engine now (Minimal by default).
- No production-coverage / dark-code sampling: there is no long-running production process for this dev CLI to sample. The field stays in the schema for a future consumer but no collector is built.
- No bespoke call-graph engine: rustc's `dead_code` lint and rust-analyzer already give reachability; a hand-rolled call graph is redundant (Make illegal states unrepresentable is served by the compiler, not a second analyzer).
- No second isolation path: the deletion experiment reuses `checks`'s worktree machinery (`src/checks.rs:734`, `src/checks.rs:315-341`) rather than inventing its own, so the audit inherits the same orphan-prune and cleanup guarantees (Prefer the cleaner long-term architecture over the smallest diff; Reproducible).
- No convergence loop for the audit: it is a single advisory pass like acceptance/review (`AGENTS.md:33`, `AGENTS.md:47`), not a consecutive-clean loop; treating it as a blocking gate would straitjacket the workflow the way Q-51 warns a mis-scoped driver would.

Principle gates, by name (the plan's Project Principles): Minimal by default (tiered cost-ordered build, cheap signals first, no engine we do not need); Ground decisions in evidence (every candidate carries per-signal `file:line` evidence and the falsifiable deletion result, not a heuristic score); Structured data first, project for humans (a JSON intermediate projected to the Markdown report); Prefer the cleaner long-term architecture over the smallest diff and Reproducible (reuse `checks`'s isolated worktree rather than a parallel path); Make illegal states unrepresentable (the `verdict` is derived from typed signals, never a stored free-text claim). The exclusion-and-advisory stance is the Chesterton's Fence commitment Q-52 sets: the audit shows the loss, the human owns the cut.
