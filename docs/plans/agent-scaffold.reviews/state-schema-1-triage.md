# Triage: state-schema increment 1 (subcommand restructure)

Triager: independent of implementer and orchestrator. Diff range: `78571a0..e32dc5a`. Artifact classification: LOW-risk (one consecutive clean round to converge). Judged against `AGENTS.md` Principles 1-7 (and 20 for doc self-containment).

## Confirming the opus CLEAN verdict (correctness / behaviour-preservation)

Sound. Spot-checked `git diff 78571a0..e32dc5a -- src/main.rs`:

- The change is a mechanical `cli.<field>` -> `args.<field>` rename plus the struct split (`Cli` now holds only `#[command(subcommand)] command: Command`; the 11 flags move verbatim into `ScaffoldArgs`; `Command::Scaffold(ScaffoldArgs)` is the single verb).
- `main` now only dispatches `Command::Scaffold(args) => run_scaffold(args)`; `run_scaffold` is a faithful rename of the old `main` body. The only non-rename hunk is a rustfmt reflow of the `build_assets(...)` call (multi-arg wrap, no semantic change).
- `use_selector` takes `&ScaffoldArgs`; its open condition is unchanged. `arg_required_else_help = true` is added to the top-level command. All defaults, `conflicts_with`, `value_enum`, `value_name`, field order, and doc comments are preserved.
- Opus's empirical checks (byte-identical `scaffold-self`, 50/50 tests, clean clippy, preserved error-exit paths, bare invocation prints subcommand list and exits 2) support the CLEAN verdict.

No new finding from the correctness lens. Opus CLEAN upheld.

## Finding adjudicated

### F-1 (sonnet MEDIUM): top-level `about` advertises not-yet-existing `validate` / `status`

Location: `src/main.rs:256`. `about = "Scaffold the agent workflow into a project, and validate or project its state."`

Verdict: VALID. Severity: LOW (corrected down from the reviewer's medium).

Reasoning:

- The finding is real and evidenced, not a line-length / wrapping complaint. This increment introduces only the `scaffold` verb, so bare `agent-scaffold` and `agent-scaffold --help` describe "validate or project its state" while `agent-scaffold validate` / `agent-scaffold status` produce clap's "unrecognized subcommand" error. That is a genuine consistency gap between the advertised surface and the actual surface, and it touches Principle 20 (docs should describe what the reader can actually do). Note the diff regressed here: the old `about` described only scaffolding; the new forward-looking `about` was written in anticipation of the later verbs.
- Severity is low, not medium, because the impact is bounded and self-healing. The gap is transient across the step: increment 2 adds `validate` and increment 3 adds `status`, after which the `about` becomes accurate; the version is an unpublished `0.0.1` with no release planned between increments; and a user who tries the promised verb hits a fail-fast clap error (Principle 12) rather than any silent or damaging behaviour. There is no correctness, data, or safety impact, so "misleading intermediate `--help`" sits at low.
- The producer's defence (the `about` was written forward-looking by the orchestrator's spec, deliberately anticipating the verbs) explains the intent but does not make each committed increment honest about its own capability. Keeping every committed increment's `--help` truthful is cheap and is the cleaner state; the forward-looking wording is the kind of silent scope creep Principle 8 cautions against, dressed as documentation.

Recommended fix (trivial, applyable off-cycle):

- Scope the `about` to current capability now: `about = "Scaffold the agent workflow into a project."`
- Expand it when the later verbs land: add "validate" to the `about` in increment 2 and "or project its state" (status) in increment 3, so the string tracks actual capability at each step.
- This is a one-line string edit with no logic or test impact; it can be folded into the implementer's next touch off-cycle rather than requiring its own dedicated round. (Acceptable alternative, if the orchestrator prefers: accept the residual risk explicitly in the ledger given it self-heals by increment 3; but the reword is cheap enough that fixing is preferred to accepting.)

## Round outcome

Round 1 is NOT clean: one valid finding (F-1, low). The correctness lens (opus) is clean and upheld; the consistency lens raised one low-severity, valid, trivially-fixable inconsistency.

Because F-1 is valid, the consecutive-clean streak is 0. The implementer applies the trivial off-cycle reword (or the orchestrator records an explicit residual-risk acceptance), after which a single confirming clean round converges this LOW-risk artifact (one clean round required). No high/critical dismissal here, so the backstop re-check is not triggered.
