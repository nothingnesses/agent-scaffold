# `workflow-enforcement-tier-inc3` work review, ROUND 5, ADVERSARIAL LENS

Reviewed in worktree `.claude/worktrees/rev-inc3-r5-adversarial` on branch `review/inc3-r5-adversarial` at `3f19bd1`, the tip of the branch under review. This is a fifth-round pass by a fresh model, after four prior rounds (6, 4, 2, 0 valid findings) and a round-4 triage that upheld zero. The artifact has not changed since round 4's triaged commit (`30df7f4`): `git diff 30df7f4..3f19bd1 --stat` touches only `docs/metrics/workflow.jsonl` and two ledger/review docs, nothing in `src/` or `tests/`.

## Method

TOOLCHAIN, confirmed before any build-dependent claim, no `2>/dev/null` on the export:

```
$ cd <worktree> && direnv allow && eval "$(direnv export bash)" && which cargo && cargo --version
/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo
cargo 1.98.0-nightly (a335d47ff 2026-06-26)
```

TWO BINARIES, both built by me from source:

| Name | Commit | Location |
| --- | --- | --- |
| NEW | `3f19bd1` | this worktree's `target/debug/agent-scaffold` |
| PRE | `9eeca42` | `<scratch>/r5adv/build/pre/target/debug/agent-scaffold`, exported with `git archive` and built independently under its own `direnv allow` |

`<scratch>` abbreviates the session scratchpad directory; all fixtures live under `<scratch>/r5adv/`, a directory of my own naming. `TMPDIR` pointed at `<scratch>/r5adv/tmpdir`, outside any git repository, for `cargo test`.

GATES MEASURED BY ME on the tree as reviewed: `cargo test` 422 passed / 0 failed across nine binaries (summed: 378+5+1+1+9+3+20+1+4); `cargo clippy --all-targets -- -D warnings` exit 0; `render --check` on `docs/plans/agent-scaffold.plan.toml` reports `up to date`. No source edit was made at any point; `git status --porcelain` is empty on this worktree, confirmed again at the end, and the main repository was not touched. No `nix fmt`, no `just scaffold-self`.

I read all four prior triage files in full before starting, plus the ledger's frozen-history block for this increment and the four standing residuals, before constructing anything.

## What I attacked, and what came of each

### 1. An explicit `--metrics` naming a different, well-formed project's log inside the same root, or the resolved default spelled differently

Built a minimal well-formed instrumented TOML fixture (`<scratch>/r5adv/fx1`, using the exact `[[step]]`/`title`/`order` schema and round-record shape from `tests/validate_workflow_toml_source_needs_no_plan.rs`, since my first attempt with a hand-rolled `[[steps]]` schema failed to parse and would have wasted the round on a fixture bug rather than a product one) and confirmed a green baseline:

```
$ cd <scratch>/r5adv/fx1 && NEW validate --source docs/plans/p.plan.toml --workflow
docs/metrics/workflow.jsonl: 1 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

Then spelled the same explicit `--metrics` with redundant path components (`./docs/metrics/../metrics/workflow.jsonl`): resolves to the identical file, prints the caller's own spelling verbatim (lexical, by design), same green result, `exit=0`. No difference from the canonical spelling beyond the echoed string, which is the documented and previously-verified lexical/canonical split.

The "different well-formed project inside the same root" half of this direction is the recorded IN-ROOT BOUND residual: `is_outside_root` tests only that the artifact is a descendant of the checked plan's canonical root, not that it is the SAME sub-project, and `validate --workflow`'s containment guard uses the identical shared predicate the ledger already records this against (`src/main.rs`'s own comment: "The predicate is not re-implemented and not widened: each root is still tested by `is_outside_root`, exactly as `run_resume` tests its own"). This is the standing residual the brief instructs me not to re-raise, and I did not construct it as a new finding; I confirmed only that it is the same mechanism, not a new one specific to this arm.

### 2. Repeated or contradictory flags

```
$ NEW validate --source docs/plans/p.plan.toml --workflow --metrics A --metrics B
error: the argument '--metrics <METRICS>' cannot be used multiple times
exit=2

$ NEW validate --source docs/plans/p.plan.toml --workflow --metrics ""
error: a value is required for '--metrics <METRICS>' but none was supplied
exit=2
```

The empty-string result looked promising at first (an argv element that is genuinely `""`, confirmed with an explicit `for` loop printing `arg6=[]`), but it reproduces byte-for-byte on PRE, and on an entirely unrelated flag (`--plan ""`) and an unrelated subcommand (`status --metrics ""`), so it is clap 4.6.1's own generic value-parsing behaviour, present everywhere in the binary, not something this increment's diff touches or could have introduced.

A whitespace-only `--metrics " "` DOES pass clap (it is not empty), resolves to a literal one-character path that does not exist, and correctly reaches the new arm: `no round log at  : the workflow check could not run...` (path displayed as a lone space). Correct, if cosmetically odd, and not misleading about what was passed.

`--workflow` has no `--json` counterpart on `validate` (only `status` and `next` have `--json`), so that specific combination in the brief's hint does not exist to test.

I also checked `--workflow --json` is simply not a valid flag pair (no such flag on `ValidateArgs`), confirmed by reading the struct rather than by running it.

### 3. The problem-reporting path: streams and what a naive `stdout` reader would conclude

Read the full body of `run_validate` and confirmed by grep (`grep -n "println!" src/main.rs`) that inside this function exactly one `println!` exists (`src/main.rs:1083`), gated by `if problems.is_empty()`, printing only the collected `summaries`. Every other emission inside the function is `eprintln!`. This means: whenever `problems` is non-empty for ANY reason (a malformed metrics record, a broken plan region, a containment refusal, the new missing-round-log problem, anything), `summaries` is unconditionally discarded and STDOUT IS EMPTY. A script doing `agent-scaffold validate --workflow ... ; if [ $? -eq 0 ]` or piping only stdout has nothing to misread; the exit code and stderr are the only signal, and they agree.

Verified this holds even when a foreign, well-formed log sits at the metrics path but the run is independently refused for containment: the metrics-validate step (which runs unconditionally, before `--workflow` is even consulted) still computes a `"N records, valid"` summary string for that foreign log, but it goes into the same `summaries` vector that gets discarded once the containment problem is pushed, so nothing about the foreign log's validity reaches stdout either.

Also enumerated all 8 combinations of `(toml_primary, plan_contents, metrics_contents)` presence against the four-arm match by hand: the three combinations where a check actually runs all pass through `report_workflow`, which unconditionally pushes either a summary (`workflow invariants hold`) or one problem per violation, so there is no path through the match that leaves both `summaries` and `problems` untouched (a silently empty, falsely-successful run). This is exhaustive over the match's own arms, not a sample.

Nothing found on this axis.

### 4. The boundary between this increment's problem and the `(None, None, _)` arm's problem

Before constructing anything I checked round 3's own sweep here: `...-r3-reviewer-adversarial.md` ran 22 `--workflow` inputs against NEW, BASE and PRE specifically probing precedence between the containment refusal, the `(None, None, _)` arm, the new `_` arm, and a malformed `--workflow-spec`, and the round-3 triage separately confirmed "PRECEDENCE, FOUR WAYS" and "PRECEDENCE AGAINST THE `(None, None, _)` ARM IS CORRECT" by direct measurement, including the exact case I intended to build (a malformed `--source` with no `--plan`, which reports the source's own parse-error problem alongside, not instead of, `no plan source resolved`). I re-derived the same case by reading rather than re-running it wholesale (the tuple-combination enumeration in section 3 above is the same territory from the code side), and did not find a ninth combination the two round-3 sweeps missed. I do not have new evidence against either round-3 ruling, so I am not re-litigating it, per the brief's own instruction to only reopen a settled item with new evidence.

### 5. My own reading: the doc/comment surfaces this diff also changed

Read the full diff of `AGENTS.md`, `.agents/AGENTS.reference.md`, `pack/AGENTS.md` (the drift-guarded triplet, identical as expected), `README.md`, and `CHANGELOG.md`. Checked each added or changed sentence against the code:

- The backstop sentence ("when instrumentation is on, the deterministic `validate --workflow` check is the backstop... and on a project with no round log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero...") is true of the code (`--instrument` only toggles whether `pack/instrument.md` is rendered into the guidance text, per `src/main.rs:257-258`; it creates no `docs/metrics/workflow.jsonl` at scaffold time either way, confirmed by reading `build_assets`/`init` rather than assumed). Round 4 already spot-checked this exact sentence against a freshly-instrumented and a freshly-non-instrumented fixture (check 20) and I did not find a case that contradicts it.
- The README's two changed paragraphs and the CHANGELOG's new "Changed" bullet all describe the same rule consistently with each other and with the code: plain `validate` unaffected, `--workflow` fails on a missing log naming the resolved path, accepted cost (i) now fails loudly instead of silently. No overclaim found (each uses "cannot see" / "cannot run" language that covers both the `Ok(false)` and `Err` sub-cases without asserting which one fired, which is accurate either way).
- The one CHANGELOG line that DELETES text ("It requires `--plan` and reuses the same metrics log..." to "It reuses the same metrics log...") reflects a relaxation from a prior increment (Inc 6) and is a correction of stale text, not an introduction of a new falsehood; I checked this is not itself new scope creep by confirming `--workflow` no longer `requires = "plan"` in the current `ValidateArgs` (it does not: only `workflow_spec` carries a `requires`).

Nothing found on this axis either.

## What I did not find

Four rounds, roughly the shape of a full sweep each, produced 6, 4, 2, and 0 valid findings across ordinary paths, precedence between all four match arms, every filesystem errno class reachable at the log path (ENOENT, EACCES, ENOTDIR, ELOOP, ENAMETOOLONG, a directory, a mode-000 ancestor, a dangling symlink), 35+ input byte-identity sweeps against plain `validate`, `status`, `next`, `status --resume`, `render --check` and `audit`, a containment TOCTOU (pre-existing, routed), and a from-scratch re-derivation of the arm's comment claims (round 4). My own pass, aimed specifically at the five directions the brief named as unexplored, found nothing new: the CLI-parsing edges are generic clap behaviour reproducing identically on PRE, the stream separation is airtight by construction (one gated `println!` in the whole function), the arm-boundary precedence was already measured exhaustively in round 3 and I could not find a ninth case, and the documentation surfaces this diff touches are internally consistent and consistent with the code.

## Coverage gaps, stated plainly

- I did not run a fresh adversarial sweep of the filesystem-errno classes (ENOTDIR/ELOOP/ENAMETOOLONG/EACCES) against the `--workflow` arm myself; I read and cross-checked round 2 and round 3's own transcripts of exactly this instead of re-building four symlink-loop and permission-denied fixtures, because the brief asks not to re-confirm what construction already established and those transcripts show real command output, not just claims. If those transcripts were themselves fabricated, that would not be caught by my pass.
- I did not attempt a new TOCTOU construction beyond reading the round-4 triage's FIFO/symlink-swap reproduction and its attribution to pre-existing, untouched code; I did not build a new interleaving of my own between the `try_exists` probe and the later containment check specifically for THIS arm's message text (as opposed to the exit-0 false-green class round 4 already attributed to pre-existing code).
- I did not test non-UTF-8 byte sequences in a `--metrics` path (arbitrary `OsStr` bytes on Linux). This is a `Path::display()` lossy-conversion question that predates this increment's arm (the old skip-note path also calls `.display()`), so a bug there would not be this increment's, but I did not verify it either way.
- I did not re-run the exhaustive multi-problem-composition and ordering checks from round 1 (which explicitly cover the case that turned out to be R2A-4, the doubled "nothing to validate" / hard-problem stderr pair); I read that it is ACCEPTED AS A RESIDUAL and did not re-measure it myself, since re-raising it would be relitigation without new evidence.
- I did not audit the non-`--workflow` structured-data surfaces (`audit`, `checks`, `pack`, `init`) at all; the diff does not touch them and four prior rounds' sweeps already crossed them for regressions.

## Tally

| Severity | Valid | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 0 | |
| low | 0 | |

ZERO FINDINGS. I found nothing that reproduces as a defect, nothing that contradicts a prior round's measurement, and no new evidence against any settled verdict.

## Relitigation and constraints check

Nothing above raises or reopens the four standing residuals (the in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger context slot, the off-convention `--source` surface); accepted costs (i) to (iv), used only as fixtures and controls; round 1's `ADV-4` or `SC-3`; round 2's `R2A-4`, `R2B-2` or `R2B-3`; round 3's `R3A-1` or `R3A-3`; round 4's `R4A-1`; the queued plain-`validate` inconsistency; the check-16 vacuous pass; or `Q-55-existsgate`'s declined `try_exists()?` gate change. No line-length, prose-wrapping, or comment-raggedness observation appears anywhere in this file.

FIXTURE HYGIENE: all fixtures under `<scratch>/r5adv/`, a directory of my own naming; `find <scratch>/r5adv -perm 000 -o -perm 600` at close returns only cargo's own internal `.lock` build files under `build/pre/target/`, not a fixture I built, and nothing was chmodded to a restrictive mode by me at any point. `TMPDIR` pointed outside any git repository for `cargo test`. No source edit was made; `git status --porcelain` is empty on this worktree at both the start and the end of the session, and the main repository was not touched.
