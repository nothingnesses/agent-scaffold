# Build plan: `code-value-audit-static` (step 86, Q-52)

A design pass, written before any code, for the already-decided Tier-0 slice of the Q-52 code-value audit: a cheap, static-only advisory report of code that may not be earning its keep, projected from a machine-readable JSON intermediate to a kept `docs/plans/<task>.code-value-report.md`. This document validates the step's premise against the actual code (`file:line` for every claim), settles the open sub-decisions, and marks which ones need a human and which I recommend confidently enough to fold. It follows the human-input-contract-written-to-a-file shape: for each decision, the question, the design space, each option's trade-offs judged against the plan's Project Principles BY NAME, a recommendation with its reasoning, and an explicit YAGNI boundary. It is read-only with respect to the product; it is prose for a human and a reviewer, not a machine schema.

The eight Project Principles cited by name throughout (from `docs/plans/agent-scaffold.plan.toml` `[[principle]]`, lines 1631-1669): "Prefer the cleaner long-term architecture over the smallest diff", "Minimal by default", "Safe on existing projects", "Idempotent", "Make illegal states unrepresentable", "Ground decisions in evidence", "Reproducible", "Structured data first, project for humans".

## Bottom line up front

- The sidecar's honest-scope note (line 7) is not just modestly true, it is sharper than written on THIS repo: `cargo build --message-format=json` yields ZERO `dead_code`/`unused_*` diagnostics here, because the repo is clippy-clean at `-D warnings` and every latent unused item is explicitly `#[allow]`/`#[expect]`-suppressed at the source. An attribute-level `#[allow(dead_code)]` cannot be re-enabled by a command-line `-W dead_code` (the attribute wins), so the rustc harvest fundamentally cannot see through the existing suppressions. On any repo that gates on `-D warnings`, the rustc dead-code signal produces no candidates at all. I confront this head-on in section 1 and let it drive the scope cut.
- What genuinely earns its keep in this step: (a) the KEPT, structured, human-projected report artifact and JSON intermediate that clippy's ephemeral stderr does not leave behind, and its role as the measurement surface for step 87's evidence gate; (b) the `cargo-machete` unused-dependency signal, the ONE thing rustc and clippy do not do; (c) an inventory of every `#[allow/expect(dead_code)]` marker as an author-declared-reason ledger (the Chesterton's Fence encoding the step names explicitly). The rustc dead-code harvest itself earns its keep only on an adopting project that does NOT enforce `-D warnings`; on this self-hosted repo it is a zero-yield formality that is nonetheless nearly free to include.
- Recommended build: four small, all-low_risk increments (schema + projection + empty-report subcommand; the marker inventory; the rustc harvest; the machete harvest). Everything else in the multi-signal design (coverage, traceability, hotspot, the deletion experiment) is Tier 1+ or step 87 and stays out.
- Two genuine human decisions (section 10): how `cargo-machete` is obtained under Nix (the one dependency question, with a real P7-versus-P2 trade-off), and whether to ship the rustc dead-code harvest at all this step given its zero yield on `-D warnings` repos. Everything else I recommend confidently for the orchestrator to fold.

---

## 0. Grounding: what the code actually is (the evidence)

- Pure binary, confirmed. There is no `[lib]` in `Cargo.toml` (deps at `Cargo.toml:13-19`; no `[lib]` table) and no `src/lib.rs` (the crate root is `src/main.rs`, which declares the modules at `src/main.rs:12-25`). So there is NO downstream Rust API surface. The "public API" the skeptic doc worries about (`docs/plans/code-value-audit.explorations/Q-52-skeptic.md` sections 1c, 2) does not exist here; the only contract to exclude from candidacy is the subcommand / flag / output-format / emitted-pack surface, exactly as the sidecar states (line 5).
- The CLI surface a new report command must fit: `Command` is a closed clap-derive enum with six variants (`src/main.rs:362-375`): `Scaffold`, `Validate`, `Status`, `Next`, `Checks`, `Render`. Dispatch is a single `match` (`src/main.rs:568-575`). Each subcommand has a `<Verb>Args` struct with `#[derive(Args)]` and doc-comment help; the closest shapes for a new read-mostly command are `NextArgs` (`src/main.rs:468-488`, with `--source`/`--plan`/`--metrics`/`--ledger-fragment`/`--json`) and `RenderArgs` (`src/main.rs:514-524`).
- The structured-then-projected precedent this step calls for is already shipped twice. `render` reads a `<task>.plan.toml` skeleton plus sidecars and writes exactly one generated file `<task>.md` via `plan::write_rendered` (`run_render`, `src/main.rs:593-672`; the write at `src/main.rs:660`), strict (on any error it prints problems and exits 1, writing nothing). `next` builds a typed `NextProjection` and EITHER serialises it to pretty JSON (`src/main.rs:1203-1205`) OR projects it to human text via `next::render_human` (`src/main.rs:1207`); the JSON is the machine intermediate and the Markdown is a projection of the same typed value. That `--json`-or-human pattern (`NextArgs::json` at `src/main.rs:485-487`) is the exact template for this step.
- The deletion-experiment sibling's territory (step 87), which this plan must NOT encroach on: `checks::Kind` is a closed enum `{ Lint, Format, Test, Mutation }` (`src/checks.rs:86-96`); `Mutation` is reserved and skipped today (`runnable_for` returns `Runnable::Skip("`mutation` checks run in the later mutation module")`, `src/checks.rs:671-672`). The `Check` schema already carries the two cost knobs a future mutation pass needs, `budget` and `threshold`, parsed but unused (`src/checks.rs:131-142`), and they carry the live `#[allow(dead_code, reason = "parsed for the schema; used by the later mutation module")]` at `src/checks.rs:135` and `src/checks.rs:140`. The isolated-worktree machinery lives in `checks::run` with a `WorktreeGuard` Drop cleanup. NONE of this is touched by Tier-0: the static report never mutates, never removes, never builds in a worktree.
- Live author-declared-reason markers already in the tree (the Chesterton's Fence entries the report inventories):
  - `src/checks.rs:135`, `src/checks.rs:140`: `#[allow(dead_code, reason = "parsed for the schema; used by the later mutation module")]` on `Check::budget` / `Check::threshold`.
  - `src/manifest.rs:80`: `#[expect(dead_code, reason = "declared for the schema and TUI; not yet read by the loader")]` on a module `description` field.
  - `src/pack.rs:37`: a bare `#[allow(dead_code)]` (no `reason =`), with the rationale in an adjacent line comment (`src/pack.rs:35-36`) explaining it is `allow` not `expect` because the field is read in the test build.
  - `src/metrics.rs:51`: a bare `#[allow(dead_code)]` inside a macro, with the reason in the adjacent doc comment (`src/metrics.rs:48-50`).
  - `src/next.rs:218`: `#[cfg_attr(not(test), allow(dead_code))]` on `LoopState::Done`, a cfg-split test-only construction.
- Real `#[cfg(...)]`-gated code that a naive single-cfg pass would mishandle: `#[cfg(unix)]` / `#[cfg(not(unix))]` at `src/main.rs:132`, `src/main.rs:142`, `src/main.rs:2130`, `src/main.rs:2135`, and others. (Note in section 4 that rustc's static dead-code lint is NOT actually fooled by these, unlike the deletion experiment.)
- No FFI in the tree today: a grep for `no_mangle` and `extern "C"` over `src/` returns nothing. So the FFI exclusion is a forward-looking guard, not one that fires on this repo now.
- Dependency tooling: the flake devshell (`flake.nix:77-94`) provides the rust toolchain, `rust-analyzer`, `bacon`, `cargo-edit`, `lldb`, `just`, `git`, `gh`. `cargo-machete` is NOT in the devshell. Both `cargo-machete` and `cargo-mutants` ARE available in nixpkgs (verified: `nix eval nixpkgs#cargo-machete.pname` and `nix eval nixpkgs#cargo-mutants.pname` both resolve). `cargo-machete` needs no nightly (it is a source-grep heuristic; `docs/plans/code-value-audit.explorations/Q-52-prior-art-survey.md` cluster 1), unlike `cargo-udeps`.
- Metrics record types today are `"round"` and `"decision"` only (`src/metrics.rs`, e.g. the parse arms at `src/metrics.rs:447` and `src/metrics.rs:501`). There is no `"audit"` record type; adding one would be new work, which the multi-signal design explicitly defers (`docs/plans/code-value-audit.explorations/Q-52-multi-signal-design.md:96`).
- Documentation touchpoints: `README.md:126` ("Bare `agent-scaffold` prints the list of subcommands"), `README.md:208` ("Two read-only subcommands inspect the state ... they never write anything"), and the clap-generated `--help`. No test enumerates the `Command` set, so adding a variant breaks no byte-guard, but the hand-written README prose above goes stale (section 8).

---

## 1. The honest-scope reckoning (confronting the sidecar's sharpest cut)

The sidecar (line 7) says `cargo clippy --all-targets -- -D warnings` plus rustc `dead_code` "already cover most of this cheap case at zero marginal cost, so this step's incremental value over the existing guards is modest". Grounded in this repo, it is sharper than "modest" for the rustc signal specifically, and I will not paper over it.

Why the rustc harvest yields nothing here: rustc emits a `dead_code` diagnostic only for an item it compiled and found unreachable AND whose lint is not suppressed at or above that item's scope. This repo suppresses every such item at the source with `#[allow(dead_code)]` / `#[expect(dead_code)]` (the five sites in section 0). A command-line `-W dead_code` or `RUSTFLAGS="--force-warn dead_code"` cannot override an inner-attribute `#[allow]` (the nearer attribute wins; `--force-warn` is the only lint-cap that overrides `allow`, and using it would defeat the entire point, which is to READ the author's suppression as a declared reason, not to fight it). So on a repo that is clippy-clean at `-D warnings`, the Tier-0 rustc harvest returns an empty candidate list by construction. The same holds for any adopting project that enforces `-D warnings`.

What that means for value, stated plainly:

- The rustc `dead_code` harvest is NOT a new candidate generator over clippy on any `-D warnings` repo. Its candidates are a subset of what clippy already fails the build on. On this repo the subset is empty. Its only marginal value is on an adopting project that runs the audit but does NOT gate on `-D warnings`, where rustc warnings exist un-actioned; there, harvesting them into a kept report is genuinely additive over "warnings scroll past in the build log". It is also the natural baseline for step 87's gate measurement (what the static pass said was clean).
- The `cargo-machete` unused-dependency signal is the ONE part of this step that rustc and clippy do not do at all. Neither the compiler nor clippy flags an unused `[dependencies]` entry. This is the strongest standalone justification for the step. (Even this may find nothing on this repo, where all five deps are used, but it is real incremental coverage on a general project, and it is cheap.)
- The `#[allow/expect(dead_code)]` marker inventory is not a candidate generator at all; it is a transparency artifact. clippy shows nothing for these (they are suppressed). The value is a single kept ledger of every suppression with its author-declared reason, so the Chesterton's Fences are visible and auditable in one place rather than scattered across five files. This directly serves "Ground decisions in evidence" (each fence carries its stated reason) and is the encoded-in-source Chesterton's Fence the step names.
- The kept, structured report + JSON intermediate is the durable thing clippy does not leave behind, and it is the measurement surface step 87 needs. This is the part that most clearly serves "Structured data first, project for humans" and "Prefer the cleaner long-term architecture over the smallest diff" (one report reused by the later deletion-experiment tier).

Conclusion I carry into the sub-decisions: build the kept report + machete + marker-inventory unconditionally (they earn their keep), and treat the rustc dead-code harvest as the weakest earner, included because it is near-free and is the step-87 baseline, but explicitly offered to the human as a candidate to cut (section 10). This is the "recommend a SMALLER build than the sidecar's full signal list if the evidence supports it" the charter invites, applied precisely to the one signal the evidence undercuts.

---

## 2. Sub-decision 1: invocation / CLI surface

Question: is the Tier-0 report a new `agent-scaffold` subcommand, or a throwaway script? What is its name, inputs, output path, and flags? This matters because a new subcommand ADDS to the very CLI contract the audit excludes from candidacy, so it must be deliberate, not incidental.

Design space:

- Option A: a throwaway script (e.g. under a scratch dir, or a `just` recipe) that shells out to `cargo build --message-format=json` and `cargo-machete`, parses them, and writes the report. No change to the `Command` enum.
- Option B: a new `agent-scaffold audit` subcommand, a seventh `Command` variant with an `AuditArgs` struct, a `run_audit` dispatch arm, and the `--json`-or-Markdown output shape of `next`.
- Option C: fold the report into an existing surface (e.g. a flag on `checks` or `status`).

Trade-offs against the principles by name:

- "Structured data first, project for humans" and "Prefer the cleaner long-term architecture over the smallest diff": favor B strongly. The step's whole premise is a kept, regenerable JSON intermediate projected to committed Markdown, reused by the later deletion-experiment tier. A script (A) produces the same bytes once but leaves no maintained, tested, discoverable surface, and the Tier-2 sibling would then have to build the command anyway; B is the surface the later tier grows into rather than a throwaway the later tier discards. The multi-signal design floated "a script or a thin `audit` command" for Tier-0 (`docs/plans/code-value-audit.explorations/Q-52-multi-signal-design.md:102`), but it also names an `audit` subcommand as the destination (`:94`); B reaches the destination directly.
- "Minimal by default": favors A on pure diff size (no new CLI surface), and is the reason to be deliberate: B enlarges the contract the audit itself excludes from candidacy. The mitigation is that B is small (one enum variant, one args struct, one dispatch arm) and mirrors an existing shape exactly, so the added surface is one well-understood command, not an engine.
- "Make illegal states unrepresentable": mildly favors B; the args are a typed clap struct with a closed set of flags, versus a script's positional argv.
- Reject C: overloading `checks` (which runs configured commands in an isolated worktree) or `status` (a best-effort no-write projection) would blur two clean module boundaries; `checks` is step 87's isolation home and must stay uncluttered, and `status` promises never to write while this command writes a report.

Recommendation: Option B, a new `audit` subcommand. Reasoning: the durable value of this step (section 1) is precisely the kept, structured, projected, reusable artifact, and B is the only option that delivers it as a maintained surface the later tier extends. B's cost over A is a single small command mirroring `next`, so "Minimal by default" is not badly served. The one deliberate acknowledgement: adding `audit` to the CLI contract means the audit must exclude its own new subcommand-and-flag surface from candidacy, which the exclusion mechanism (section 4) does for free because rustc already sees `main`'s dispatch reach every handler.

Concrete shape I recommend folding (name, inputs, output, flags):

- Name: `audit`. (Short, verb-shaped like the others; unambiguous.)
- Inputs mirror `next`/`status` for `<task>` derivation and add a crate-root dir like `checks`:
  - `--source <path>` and/or `--plan <path>`: to derive `<task>` for the output path, the same `derive_task` logic `next` uses (`src/main.rs:1140`, `next::derive_task`). When neither is given, fall back to a default task name (mirror `next`'s "no plan source" handling, `src/main.rs:1164`).
  - `--dir <path>` (default `.`): the Rust crate root whose `Cargo.toml` and `src/` are audited, mirroring `ChecksArgs::dir` (`src/main.rs:529-531`). This is what `cargo build` and `cargo-machete` run against.
  - `--json`: emit the machine intermediate to stdout instead of writing the Markdown report, mirroring `NextArgs::json` (`src/main.rs:485-487`). (See below on whether `--json` writes or prints.)
  - `--out <path>` (optional override): default `docs/plans/<task>.code-value-report.md`, exactly the step's stated default.
- Output: by default, write the Markdown report to `docs/plans/<task>.code-value-report.md` (a generated artifact beside the plan, like `render` writes `<task>.md`; "Safe on existing projects" and "Idempotent" are served because the report is a generated projection whose overwrite is deterministic given the same inputs, not a user working file). Read-only with respect to the product: it never edits `src/`, `Cargo.toml`, the plan TOML, the sidecars, or the metrics log; it writes ONLY its own report (and, under `--json`, prints the intermediate to stdout and writes nothing, matching `next --json`).
- A design nuance to fold: keep the JSON intermediate as the in-memory typed value that is EITHER serialised to stdout (`--json`) OR projected to the Markdown file (default), exactly as `run_next` branches (`src/main.rs:1203-1208`). Do not persist a separate `.json` file on disk in Tier-0 (the Markdown is the kept deliverable; the JSON is regenerable and printed on demand). Persisting a committed `.json` sidecar is a possible Tier-2 need (step 87 may want to diff intermediates), deferred here (YAGNI, section 9).

---

## 3. Sub-decision 2: the structured JSON intermediate schema

Question: what is the typed source that projects to the report? Define one candidate record and make illegal states unrepresentable where reasonable.

The record must distinguish three fundamentally different row kinds that the step names: a dead-code candidate (a suspicion), an unused-dependency candidate (a suspicion), and an author-declared-reason entry (NOT a candidate; a fence with its reason). These carry different evidence fields, so a flat struct with a pile of `Option<...>` would admit illegal combinations (a dep row with a `file:line` symbol span, a dead-code row with a machete caveat). "Make illegal states unrepresentable" says encode the kind as a Rust enum whose variants carry only their own evidence.

Recommended schema (folded; a Serialize enum, projected to Markdown):

```
struct CodeValueReport {
    task: String,
    generated_from: SignalSet,   // which signals actually ran, for the caveat
    caveat: &'static str,        // the mandatory "not evidence of absence" text
    records: Vec<AuditRecord>,
}

enum AuditRecord {
    // A rustc dead_code / unused_* candidate (a suspicion, never a verdict).
    DeadCode {
        span: Span,              // file:line[:col], the evidence anchor
        symbol: String,          // the item name from the diagnostic
        lint: String,            // e.g. "dead_code", "unused_variables"
        source: Signal,          // Signal::RustcBuildJson
        exclusion: Option<Exclusion>,  // Some(...) => shown but not a candidate
    },
    // A cargo-machete unused-dependency candidate.
    UnusedDep {
        crate_name: String,
        manifest: Span,          // Cargo.toml:line of the dependency
        source: Signal,          // Signal::CargoMachete
        caveat: &'static str,    // machete's own imprecision note (macros/re-exports)
    },
    // An author-declared reason: a suppressed item, shown as a fence, NOT a candidate.
    DeclaredReason {
        span: Span,
        symbol: String,
        marker: Marker,          // Allow | Expect
        reason: Option<String>,  // the `reason = "..."` string, if present
    },
}

struct Span { file: PathBuf, line: u32 }         // col optional
enum Signal { RustcBuildJson, CargoMachete, SourceScan }
enum Marker { Allow, Expect }
enum Exclusion { CfgGated, Ffi, Suppressed, ContractSurface }
```

Notes on the design choices, by principle:

- "Make illegal states unrepresentable": the enum keeps kind-specific evidence in its own variant, so a dep row cannot carry a symbol span and a dead-code row cannot carry a machete caveat. `reason: Option<String>` on `DeclaredReason` is correct rather than avoidable, because the tree has bare `#[allow(dead_code)]` with no `reason =` (`src/pack.rs:37`, `src/metrics.rs:51`) alongside ones that do carry it; the absence is real data (an undeclared fence), so it is modelled as `None`, and the projection notes "no machine-readable reason (see adjacent comment)".
- Verdict is DERIVED, not stored. The multi-signal design's schema (`docs/plans/code-value-audit.explorations/Q-52-multi-signal-design.md:35-39`) stores signals and recomputes the verdict every run; Tier-0 has only one candidate-bearing signal per row, so the "verdict" is trivial (a `DeadCode`/`UnusedDep` with `exclusion == None` is a review-candidate; anything with `exclusion == Some` or any `DeclaredReason` is shown-not-candidate) and should be a projection-time function, not a stored field. This matches how `status`/`next` are best-effort projections of durable files, never a source of truth (`src/main.rs:368-370`).
- "Structured data first, project for humans": the Markdown groups records by kind (Candidates: dead code; Candidates: unused deps; Author-declared reasons / fences; Excluded, with reason), each with its `file:line` anchor (the same anchor form review reports use), and leads with the mandatory caveat (section 6).
- Do NOT add the four not-yet-computed signal sub-objects (`exercised`, `deletion_safe`, `traceable`, `cost`) from the multi-signal schema as `not_run` placeholders. That schema is the Tier 1+/step-87 destination; carrying dead fields now is speculative structure that "Minimal by default" and "Ground decisions in evidence" both argue against (there is no consumer for them in Tier-0, and step 87 can widen the enum when it lands). This is a YAGNI cut (section 9).

---

## 4. Sub-decision 3: signal harvesting (feasibility + exact invocation)

### 4a. rustc `dead_code` / `unused_*` via `cargo build --message-format=json`

Feasibility: high mechanically, low yield on this repo (section 1). `cargo build --message-format=json` (or `cargo check --message-format=json`, cheaper since it skips codegen) emits one JSON object per line; compiler diagnostics are `{"reason":"compiler-message","message":{...}}` objects whose `message.code.code` is the lint name (e.g. `"dead_code"`, `"unused_variables"`, `"unused_imports"`) and whose `message.spans[]` carry `file_name`, `line_start`, `column_start`. Parse: run the command, read stdout line by line, `serde_json::from_str` each into a minimal typed struct (reason tag + message.code + primary span), keep only lines where `reason == "compiler-message"` and the code is a `dead_code`/`unused_*` lint, and map each to a `DeadCode` record. `serde_json` is already a dependency (`Cargo.toml:18`), so no new dep for parsing.

Exact invocation: `cargo check --message-format=json --all-targets` run in `--dir` via `std::process::Command` (the crate already shells out with `std::process::Command`, e.g. `src/main.rs:1260`). Prefer `check` over `build` for speed; `--all-targets` so tests/examples are analysed too. Do NOT pass `--force-warn dead_code`: it would override the author's `#[allow]` suppressions, which is the opposite of reading them as declared reasons (section 1); the harvest takes the diagnostics rustc emits under the project's own lint config, nothing more.

The honest caveat to bake in: on a `-D warnings` repo this returns empty. That is correct behaviour, not a bug, and the report's caveat (section 6) must say so: "no unsuppressed dead-code diagnostics" is necessary-not-sufficient and only relative to the project's lint configuration.

### 4b. `#[allow(dead_code)]` / `#[expect(dead_code)]` markers as author-declared reasons

Feasibility: high. These must be read from SOURCE, not from compiler diagnostics, because that is the whole point: a suppressed item emits no diagnostic (4a cannot see it). A `#[expect(dead_code)]` whose expectation is FULFILLED (the item really is dead) is silent; one that is UNFULFILLED (the item is live) emits an `unfulfilled_lint_expectations` warning, which 4a would catch as a different signal, but we want the marker itself regardless of fulfilment, so a source scan is the right tool.

Detection: a line-oriented source scan over `src/**/*.rs` for attributes containing `dead_code` inside `allow(...)` / `expect(...)` (including `cfg_attr(..., allow(dead_code))`, e.g. `src/next.rs:218`), capturing the file, line, the `Allow`/`Expect` marker, and the `reason = "..."` string if present. Associate each with the item it annotates (the next non-attribute, non-comment line) for the `symbol` field; a full parse is not needed for Tier-0 and would pull in a syntax crate (`syn`) that "Minimal by default" does not justify for a marker inventory. A regex/line scan over the five known sites (section 0) is sufficient and robust enough; note in the code that this is a heuristic scan, not a parse, and that a marker split across lines or written unusually may be missed (acceptable for an advisory inventory).

Classification: these become `DeclaredReason` records, explicitly NOT candidates. This is the Chesterton's Fence the step names: the author has already declared why the item is not statically reachable yet, so the report shows the fence and its reason rather than proposing removal.

### 4c. `cargo-machete` for unused dependencies

Feasibility: high; this is the one genuinely additive signal. `cargo-machete` (`docs/plans/code-value-audit.explorations/Q-52-prior-art-survey.md` cluster 1) is a fast source-grep heuristic that does not need nightly. It self-documents false positives (a dep used only via a macro or a re-export can be flagged), so the report must carry machete's imprecision note per row (the `caveat` field on `UnusedDep`) and never auto-trust it.

Exact invocation under Nix: `cargo-machete --with-metadata` (the `--with-metadata` mode uses `cargo metadata` for more precise crate resolution) run against `--dir`. Machete's human output is a list; for stable parsing prefer capturing its output and matching the "<crate> -- <manifest path>" lines, or, if a machine format is available in the pinned version, use it (verify at build time; fall back to line parsing). The acquisition question (how machete is on PATH under Nix) is a genuine human decision, section 10 and the dependency note below.

Feasibility summary: 4a is free-but-empty-here (parse existing build output), 4b is a cheap source scan (no new dep), 4c is one extra bounded tool run whose acquisition is the one open dependency question.

---

## 5. Sub-decision 4: the exclusion mechanism

Question: how to keep the CLI/output/pack contract surface, `#[cfg(...)]`-gated code, and FFI (`#[no_mangle]`/`extern`) out of the candidate set BEFORE any candidate reaches a human. Programmatic detection versus a maintained denylist.

The key grounded insight that shrinks this sub-decision: the Tier-0 STATIC signals mostly self-exclude, because rustc's own dead-code analysis already respects the things the skeptic doc worried about (that doc's section 1 is aimed largely at the DELETION experiment of step 87, which compiles one cfg and infers absence; a static rustc pass does not make that error):

- Contract surface (subcommands, flag handlers, emitted pack): rustc sees `main`'s dispatch `match` reach every subcommand handler (`src/main.rs:568-575`), so these are LIVE to the compiler and never appear as `dead_code` diagnostics. No exclusion work needed for the rustc signal; the contract surface is excluded for free.
- `#[cfg(...)]`-gated code: rustc emits `dead_code` only for code it compiled. Code under a non-matching cfg (e.g. `#[cfg(not(unix))]` on Linux, `src/main.rs:142`) is not compiled, so it produces no diagnostic to exclude; code under the matching cfg is analysed normally. So the STATIC signal is not fooled by cfg the way the deletion experiment is. The only residual care: run the harvest under one cfg (the default target) and note in the caveat that cfg-gated code on other targets was not analysed (necessary-not-sufficient, relative to the analysed cfg set).
- FFI: none in the tree today (section 0). As a forward guard, exclude any item carrying `#[no_mangle]` or `extern "C"` via the same source scan that reads the markers (4b), classifying it `Exclusion::Ffi`. Cheap and programmatic.
- `cargo-machete`: excludes nothing by denylist; its own `--with-metadata` mode reduces false positives, and its residual imprecision is disclosed per-row (4c) rather than denied.

Design space for the residual exclusion: (A) programmatic only, riding the signals' own semantics plus a small source scan for suppressions/FFI; (B) a maintained denylist file the user curates.

Trade-offs:

- "Minimal by default": strongly favors A. A maintained denylist is exactly the ongoing tax the skeptic doc warns about (`docs/plans/code-value-audit.explorations/Q-52-skeptic.md` section 4b: a stale exclusion list silently raises the false-positive rate). For Tier-0, where the signals self-exclude, a denylist would be pure maintenance burden with almost nothing to list.
- "Make illegal states unrepresentable": favors A. Letting the compiler's own reachability and cfg semantics do the exclusion means the exclusion is derived from ground truth (what rustc compiled and reached), not from a hand-curated list that can drift out of agreement with the code.
- "Ground decisions in evidence": favors A. Each exclusion carries its own evidence (the `#[allow]` marker, the `#[cfg]` attribute, the `#[no_mangle]`), not a human's say-so in a list.
- A denylist (B) would only earn its keep if the signals produced structural false positives the semantics cannot exclude. On a pure binary with no downstream API, they do not.

Recommendation (fold): Option A, programmatic exclusion only. The contract surface is excluded by rustc's own reachability (no work), cfg by rustc's own compile scope (no work, plus a caveat line), suppressions and FFI by the same cheap source scan that produces the marker inventory (4b), reclassifying any harvested candidate that also carries a suppression/FFI attribute into an `Exclusion`. No maintained denylist in Tier-0. If a real adopting project later surfaces a structural false-positive class the semantics cannot exclude, a denylist can be added then with evidence; do not build it speculatively now.

---

## 6. Sub-decision 5: the mandatory caveat and advisory framing

This is settled by the step (line 5) and the prior-art survey (correction 3); I record it as a fold, not a fork, and pin exactly where it lives.

- Every report carries an explicit "not evidence of absence" caveat as a `&'static str` at the head of the Markdown and as the `caveat` field of the JSON intermediate. Wording to the effect: "This report is advisory. 'Nothing flagged' is necessary-not-sufficient and only relative to the named signal set (rustc dead-code under this project's lint configuration, source suppression markers, and cargo-machete's source-grep heuristic). Suppressed, cfg-gated (non-analysed targets), FFI, dynamically dispatched, and reflection-reached code is not covered. A passing audit is not proof the codebase has no dead code." This is single-sourced (one `const`), matching how the repo single-sources shared fragments (e.g. `ISOLATION_POLICY_FRAGMENT`), so the caveat cannot drift between the JSON and the Markdown.
- The report NEVER auto-deletes, never edits `src/` or `Cargo.toml`, never stages a commit, never opens a PR. It writes only its own report. A human reads it and decides each candidate; an accepted deletion becomes a kickoff task, the same handoff the review entry mode produces (`AGENTS.md:47`, `AGENTS.md:69`). This is the read-only-with-respect-to-the-product stance `next` takes (`src/main.rs:368-370`), extended to "writes only its own deliverable" like `render`.
- The `generated_from` / `SignalSet` field records which signals actually ran (e.g. machete absent if the tool was unavailable), so the caveat is accurate to the run rather than boilerplate; an absent signal widens the "not covered" disclosure rather than silently passing.

By principle: "Ground decisions in evidence" (the caveat states the oracle set explicitly, per prior-art correction 3), "Make illegal states unrepresentable" (advisory-only is enforced by the command having no write path to product files, not by a runtime guard), and "Safe on existing projects" (the tool cannot clobber a user working file because it only writes its own generated report).

---

## 7. Sub-decision 6: increment decomposition, risk classes, and the step-87 measurement surface

Recommended order (each increment a reviewable, independently shippable slice; all low_risk with reasoning):

- Increment 1: the schema + the Markdown projection + the `audit` subcommand emitting an EMPTY report with the mandatory caveat. Adds the `AuditRecord` enum and `CodeValueReport` (section 3), the `audit` `Command` variant + `AuditArgs` + `run_audit` dispatch (section 2), the single-sourced caveat const (section 6), and the `--json`-or-Markdown branch mirroring `run_next` (`src/main.rs:1203-1208`). No signal harvesting yet; the report is the caveat plus an empty record list. This lands the structured-first, project-for-humans skeleton ("Structured data first, project for humans", "Prefer the cleaner long-term architecture over the smallest diff") and the CLI surface first, so later increments only add record producers.
  - Risk class: low_risk. Advisory-only output, no product/plan/code/metrics mutation, writes only its own generated report, fully reversible, no security/safety/money/data sensitivity. Under the built-in spec (`src/workflow_spec.rs`), low_risk converges on 1 consecutive clean round.
- Increment 2: the marker inventory (4b). Adds the source scan for `#[allow/expect(dead_code)]` (and `cfg_attr(..., allow(dead_code))`) plus `#[no_mangle]`/`extern "C"`, producing `DeclaredReason` records and the FFI/suppressed `Exclusion` reclassification hook. Cheap, no new dep, high transparency value; exercises the five live sites (section 0) as test fixtures.
  - Risk class: low_risk (same reasoning; a read-only source scan).
- Increment 3: the rustc harvest (4a). Adds `cargo check --message-format=json --all-targets` in `--dir`, the diagnostic parser (reusing `serde_json`), and the mapping to `DeadCode` records with the exclusion pass reclassifying any harvested item that also carries a suppression (from Increment 2's scan). This is the primary measurement surface for step 87 (see below).
  - Risk class: low_risk (read-only parse of build output; the build runs in `--dir` and mutates nothing the tool writes).
- Increment 4: the machete harvest (4c). Adds `cargo-machete --with-metadata` invocation and output parsing to `UnusedDep` records with the per-row imprecision caveat. Gated on the dependency-acquisition decision (section 10); the invocation path is the only thing that changes between the acquisition options.
  - Risk class: low_risk on the product axis (advisory only, reversible). The one caveat that does NOT raise the class: it introduces an external tool dependency and (under the on-demand Nix option) a network/registry-eval dependency at run time; a missing tool degrades to an absent signal recorded in `generated_from`, never a failure that touches product files.

The measurement surface for step 87's evidence gate: Increments 1 and 3 together. Step 87 un-gates only once there is a measured count, from THIS report in practice, of cases where `clippy -D warnings` + rustc `dead_code` PASSED but a deletion experiment would have caught real dead code (`docs/plans/agent-scaffold.steps/code-value-audit-deletion-experiment.md:5`). The kept report from Increment 1 is where that count accumulates over time, and the rustc-harvest baseline from Increment 3 is the "static said clean" set the deletion experiment's future findings are measured against. Increment 2's fence inventory also feeds it (a fence whose reason later proves stale is a candidate the static pass deliberately did not flag). This is the honest incremental value the bare lints do not provide: not more candidates today, but a durable, structured baseline that makes step 87's gate measurable at all.

Ordering rationale: schema/skeleton first so every later increment is purely additive record production; the two zero-dependency signals (markers, rustc) before the one-dependency signal (machete), so the dependency question (section 10) blocks only the last increment and the first three can proceed while it is decided.

---

## 8. Sub-decision 7: documentation impact

Planned as work, not an afterthought (the planner's phase-2 duty, `AGENTS.md:30`):

- `README.md`: the line "Two read-only subcommands inspect the state a running workflow keeps (they never write anything)" (`README.md:208`) goes stale, because `audit` is a new read-mostly command that DOES write its report (so it neither fits "two" nor "never write anything"). Add an `audit` subsection near the validating/projecting section, describing the advisory report, the signals, the mandatory caveat, and the `--json` intermediate. The list-of-subcommands sentence (`README.md:126`) is fine as prose (it does not enumerate), and `--help` regenerates automatically from clap.
- CLI `--help`: the new `Command::Audit` doc-comment (the `///` above the variant) IS the CLI contract text and must be authored with the same care as the existing variants' doc-comments (`src/main.rs:363-374`); state clearly that it is advisory, writes only its own report, and never deletes.
- `AGENTS.md`: minimal for Tier-0. The multi-signal design ties the audit into the acceptance/UAT cadence (`docs/plans/code-value-audit.explorations/Q-52-multi-signal-design.md:93`), but wiring it into a workflow phase is deferred (YAGNI, section 9); so no AGENTS.md workflow-phase edit is required by this step. If the orchestrator chooses to mention the command's existence, that is a small optional addition, not a staleness the change forces.
- `CHANGELOG.md`: add an entry for the new `audit` command (the repo maintains one, `CHANGELOG.md`).
- Plan artifacts: the orchestrator flips the step-86 status and, if the shape here is accepted, the step sidecar's framing (e.g. if the rustc harvest is cut, section 10) is corrected to match; re-render with the render pipeline and `render --check`. No byte-guard test pins the `Command` set, so no drift-guard test breaks, but the marker-inventory tests should use the five live sites (section 0) as fixtures, and a fixture-based test of the JSON-to-Markdown projection follows the render/next golden-projection precedent.

---

## 9. YAGNI boundary (what NOT to build)

- Do NOT build any of the four Tier 1+ signals as populated or as `not_run` placeholder fields: no coverage (`exercised`), no traceability (`traceable`), no churn/complexity hotspot (`cost`), and above all no deletion experiment (`deletion_safe`). The deletion experiment is step 87, gated behind an evidence gate, and must not ship before the mutation module is piloted (`docs/plans/agent-scaffold.steps/code-value-audit-deletion-experiment.md:6-7`). "Minimal by default" and "Ground decisions in evidence" both forbid carrying dead schema now; step 87 widens the `AuditRecord` enum when it lands.
- Do NOT touch `checks`'s isolated-worktree machinery, the `Kind::Mutation` variant (`src/checks.rs:94-95`, `src/checks.rs:671-672`), or the `budget`/`threshold` fields (`src/checks.rs:131-142`). Tier-0 never builds, mutates, or removes anything in a worktree; it only reads and reports. This is the boundary between this step and step 87.
- Do NOT build a maintained denylist for exclusions (section 5). The static signals self-exclude; a denylist is the maintenance tax the skeptic doc warns of.
- Do NOT auto-delete, auto-inline, stage a commit, or open a PR. The report terminates at human review (section 6).
- Do NOT add a `type:"audit"` metrics record or `--instrument` hook this step. Instrumentation is explicitly deferred in the multi-signal design (`docs/plans/code-value-audit.explorations/Q-52-multi-signal-design.md:96`); the kept Markdown report is the always-on deliverable, and the metrics schema (`"round"`/`"decision"` only today) is not widened here.
- Do NOT use `cargo-udeps` (needs nightly; `cargo-machete` covers unused deps without it) or build a bespoke call-graph engine (rustc's dead-code analysis is the reachability oracle; a second analyzer is redundant, "Prefer the cleaner long-term architecture over the smallest diff").
- Do NOT persist a committed `.json` intermediate file in Tier-0; the JSON prints on `--json` and the Markdown is the kept artifact. A committed JSON sidecar is a possible step-87 need (diffing intermediates across runs), raised then with evidence.
- Do NOT wire the audit into a workflow phase (acceptance/UAT gate) in this step; that integration is Tier-3 in the design and is not part of the Tier-0 kept-report scope.
- Do NOT reach for a full-syntax parse (`syn`) for the marker scan; a line-oriented scan over the known attribute forms is sufficient for an advisory inventory ("Minimal by default").

---

## 10. Decisions to escalate to the human vs. confident folds

Two genuine human decisions (real trade-offs, reasonable people could differ):

1. How `cargo-machete` is obtained under Nix (the one dependency addition). Options: (A) add `cargo-machete` to the flake devshell (`flake.nix:78-94`), so it is pinned via `flake.lock` and always on PATH; (B) invoke it on demand via `nix shell nixpkgs#cargo-machete --command cargo-machete ...` from the tool, keeping the devshell lean; (C) drop machete entirely and ship Tier-0 with only the rustc harvest and the marker inventory. Trade-offs by name: "Reproducible" favors A (pinned to the flake's nixpkgs via the lock, identical on every machine; the on-demand `nix shell nixpkgs#...` in B resolves against the user's registry nixpkgs, NOT the flake's pinned input, so it is less reproducible and needs network/eval at run time). "Minimal by default" favors B or C (machete is a periodic-audit tool, not a per-build tool, so adding it to the devshell taxes every developer's shell for a rarely-run command; C avoids the dependency altogether). The honest-scope reckoning (section 1) is why C is even on the table: machete is the ONE additive signal, but it may also find nothing on a small, tidy crate. My recommendation: Option A (devshell), because machete is the single genuinely-incremental signal this step has over clippy/rustc, and "Reproducible" is a named plan principle that A satisfies and B measurably weakens (registry-versus-locked nixpkgs); the devshell cost is one small tool. If the human weighs "Minimal by default" higher for a tool run only at audit cadence, B is the acceptable lean fallback (with the reproducibility caveat noted in the report), and C is defensible only if the human judges unused-dep detection not worth any dependency at this stage, which would reduce the step to the rustc harvest plus the marker inventory. This is a genuine fork because it trades "Reproducible" against "Minimal by default" and reasonable people could weigh them differently.

2. Whether to ship the rustc dead-code harvest (Increment 3) at all this step. The evidence (section 1) is that on this `-D warnings`-clean repo it yields zero candidates and is not a candidate generator over clippy on any `-D warnings` project. Options: (A) ship it, because it is near-free (parse existing build output) and is the step-87 measurement baseline and does add value on adopting projects that do not gate on `-D warnings`; (B) cut it from Tier-0, shipping only machete + the marker inventory + the kept report, and add the rustc harvest when step 87 needs the baseline. Trade-offs: "Ground decisions in evidence" cuts both ways: the evidence says it produces nothing here (argues B), but the evidence also says step 87's gate needs a static baseline to measure against (argues A). "Minimal by default" mildly favors B (one fewer parser). My recommendation: Option A (ship it), because the marginal cost is genuinely low (it reuses `serde_json` and the build output the project already produces) and it is the concrete measurement surface the sibling step's evidence gate depends on; cutting it would leave step 87 with no baseline to count against. But I flag it as the weakest earner and the sidecar's framing (which lists it first among the signals) should be corrected to say the machete signal and the kept report are the load-bearing value, with the rustc harvest as the near-free step-87 baseline, not a primary candidate source. This is a real fork because the "modest incremental value" the sidecar admits is, for this one signal on this repo, actually "zero", and the human may prefer to defer it.

Confident recommendations the orchestrator can fold without a human decision (each argued above): the `audit` subcommand and its name/inputs/output/flags (section 2); the typed-enum JSON schema with a derived verdict and no dead placeholder fields (section 3); the exact harvest invocations (`cargo check --message-format=json --all-targets`, a source scan for markers/FFI, `cargo-machete --with-metadata`) (section 4); programmatic-only exclusion with no maintained denylist (section 5); the single-sourced mandatory caveat and the advisory, never-auto-delete, writes-only-its-own-report framing (section 6); the four-increment order, all low_risk with the stated reasoning, and Increments 1+3 as the step-87 measurement surface (section 7); and the documentation impact plan (section 8).

Overall scope recommendation: build a SMALLER thing than the sidecar's full framing implies. The kept report + JSON intermediate, the machete signal, and the marker inventory earn their keep unconditionally; the rustc dead-code harvest is included as a near-free step-87 baseline but is honestly zero-yield on this repo and is offered to the human as the cut candidate. Everything past Tier-0 (coverage, traceability, hotspot, the deletion experiment, instrumentation, workflow wiring) stays out, per the step's scope and the YAGNI boundary above.
