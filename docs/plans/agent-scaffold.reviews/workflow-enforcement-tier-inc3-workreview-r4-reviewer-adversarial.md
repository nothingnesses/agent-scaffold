# `workflow-enforcement-tier-inc3` work review, ROUND 4, REVIEWER: ADVERSARIAL CONSTRUCTION

Reviewed in worktree `.claude/worktrees/rev-inc3-r4-adversarial` on branch `review/inc3-r4-adversarial` at `aed035c`, the tip of the branch under review. Target: `git diff main...HEAD`, eight files, 294 insertions, 40 deletions.

All three prior triage files were read in full before any construction, and every ruling in them is treated as settled. Nothing below re-raises a settled item; the one item adjacent to a settled one says so and gives its own evidence.

## Method

TOOLCHAIN, confirmed before every build-dependent claim:

```
$ cd <worktree> && direnv allow && eval "$(direnv export bash)" && which cargo
/nix/store/76jaab43a2l7n7fiifxjngp68kk167vm-rust-mixed/bin/cargo
cargo 1.98.0-nightly (a335d47ff 2026-06-26) / rustc 1.98.0-nightly (f46ec5218 2026-06-30)
```

No `2>/dev/null` was used on the `direnv export` call, and no claim below rests on a build made outside that environment.

TWO BINARIES, both built by me from source:

| Name | Commit | Location |
| --- | --- | --- |
| NEW | `aed035c` | the review worktree's `target/debug/agent-scaffold` |
| PRE | `9eeca42` | `<scratch>/r4adv/build/pre/target/debug/agent-scaffold`, exported with `git archive` and built independently |

`<scratch>` abbreviates `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad`. Every fixture lives under `<scratch>/r4adv/`, a directory of my own naming. `TMPDIR` was `<scratch>/r4adv/tmpdir`, outside any git repository, for `cargo test`.

GATES MEASURED AT `aed035c` in the project toolchain: `cargo test` 422 passed / 0 failed across nine binaries; `cargo clippy --all-targets -- -D warnings` exit 0; `cargo run -- render docs/plans/agent-scaffold.plan.toml --check` reports `up to date`.

NO SOURCE EDIT was made at any point. `git status --short` is empty on this worktree and on the main repository. No `nix fmt` and no `just scaffold-self` was run.

THE FIXTURE RECIPE used throughout, so every command below is reproducible:

```sh
mkdir -p $D/docs/plans $D/docs/metrics
cat > $D/docs/plans/p.plan.toml <<'TOML'
[meta]
title = "TOML-only project"
primary = "toml"

[[step]]
slug = "only-step"
title = "The only step"
status = "not-started"
order = 1
TOML
cat > $D/docs/metrics/workflow.jsonl <<'JSONL'
{"type":"round","task":"only-step","step":"only-step","increment":"only-step","artifact":"a","phase":"work_review","changed_since_prev":true,"outcome":"clean","valid_findings":0,"severities":[],"consecutive_clean":1,"risk_class":"low_risk"}
JSONL
```

I did not re-run the attacks the brief lists as already established. I confirmed the gates and then spent the whole budget on the six surfaces the brief names plus a systematic exit-code sweep.

---

# Findings

ONE finding, `low`. It is a claim in a source comment authored by this increment, falsified by construction against the increment's own pinned accepted cost. It is not a request to change any behaviour.

## `R4A-1` `low`: the arm's comment claims the message distinguishes a non-instrumented project from a mis-anchored run, and on the mis-anchoring this project pins as accepted cost (i) the two runs produce byte-identical stdout, stderr and exit code

### The claim under test

`src/main.rs:1051-1052`, added by `e74589a` (this increment's build commit) and absent at `9eeca42`:

> The problem names the RESOLVED path, so a non-instrumented project and a mis-anchored run are distinguishable.

Provenance, established rather than assumed:

```
$ git log --oneline main..HEAD -S "non-instrumented project and a mis-anchored run are distinguishable" -- src/main.rs
e74589a fix: fail validate --workflow when the round log is missing
$ git show 9eeca42:src/main.rs | grep -c "mis-anchored"
0
```

### The construction

Two fixtures, one from each of the two populations the sentence names.

- `dA`, A NON-INSTRUMENTED PROJECT: the recipe above with `rm -rf docs/metrics`. No log anywhere, correct anchoring, run from the project root.
- `dB`, A FULLY INSTRUMENTED PROJECT, MIS-ANCHORED: the recipe above unchanged (a real one-record log at `docs/metrics/workflow.jsonl`), run with a bare `--source` filename from inside `docs/plans`. This is accepted cost (i) exactly as `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:468` pins it.

```
$ (cd $S/fix/dA && NEW validate --source docs/plans/p.plan.toml --workflow)
exit=1
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there

$ (cd $S/fix/dB/docs/plans && NEW validate --source p.plan.toml --workflow)
exit=1
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there

$ cmp A.err B.err && cmp A.out B.out
>>> STDOUT+STDERR BYTE-IDENTICAL
```

Compared with `cmp` on both streams and on the exit code, not by eye.

THE CONTROL, which is what makes `dB` the instrumented population rather than a second non-instrumented one. The same fixture, the same binary, run from its own root:

```
$ (cd $S/fix/dB && NEW validate --source docs/plans/p.plan.toml --workflow)
docs/metrics/workflow.jsonl: 1 records, valid
docs/plans/p.plan.toml: 1 steps, 0 questions, valid
docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

The log was there, readable, valid and joined for the whole run above.

### The pre-increment comparison

```
$ (cd $S/fix/dA && PRE validate --source docs/plans/p.plan.toml --workflow)
exit=0
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
docs/plans/p.plan.toml: 1 steps, 0 questions, valid

$ (cd $S/fix/dB/docs/plans && PRE validate --source p.plan.toml --workflow)
exit=0
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow has a plan source but the metrics log is missing; skipping the workflow check
p.plan.toml: 1 steps, 0 questions, valid
```

PRE's stderr is identical between the two populations too, so the CONFLATION is pre-existing and I do not claim otherwise. WHAT IS NEW IS THE CLAIM, and I state one honest sharpening and its own qualification. On PRE the two runs' total output was not byte-identical: the summary line spelled the `--source` path as the caller typed it (`docs/plans/p.plan.toml:` versus `p.plan.toml:`). On NEW even that is gone, because a non-empty `problems` list suppresses every summary (`src/main.rs:1081-1090`). THE QUALIFICATION: that difference is an echo of the argument the caller supplied, not a discriminator the tool computes, so it never distinguished the two populations in any useful sense. I record it only so the reader can see that after the change literally nothing in the output separates them.

### The same conflation is mechanised in this increment's own suite

This is the part I would ask the triager to weigh most, because it needs no fixture of mine. Two tests, both touched or added by this increment, assert the SAME stderr substring over the two populations the comment says are distinguishable:

- `tests/validate_workflow_toml_source_needs_no_plan.rs:212`, over a fixture with NO log anywhere: `stderr.contains("no round log at docs/metrics/workflow.jsonl") && stderr.contains("could not run")`.
- `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:471`, over `build_away(&root, "complete")`, which writes a REAL log at `away/docs/metrics/workflow.jsonl` (`:113`), mis-anchored by the bare filename: the identical assertion, character for character.

So the tree already pins that the two populations produce the same sentence, in the same increment whose comment says naming the path tells them apart.

### Why it is `low` and not higher

The exit code is right in both cases and identical, so no CI gate is misled and there is no false green anywhere in this finding. Nothing user-facing repeats the claim: I grepped `README.md`, `CHANGELOG.md`, `pack/`, `AGENTS.md` and `.agents/` for it and the only hits for "distinguish" are an unrelated `reviewers[].harness` CHANGELOG bullet and a `principles.toml` line about newtypes. The claim lives in one source comment, and its cost is what `R3B-1` and `R3A-2` were rated for in this same increment: a maintainer who trusts it concludes the message already carries the anchoring information and does not look further.

Not lower than `low`, for two reasons. The sentence is the JUSTIFICATION the arm offers for why one message serves both cases, so it is the sentence a later reader would rely on when deciding whether the message needs more. And this increment's own CHANGELOG, one paragraph away, spends a sentence on exactly the mis-anchoring that falsifies it: "a bare `--source` filename run from inside `docs/plans` has no parents to derive a root from, so it looks for a log beneath `docs/plans` and finds none, which was a note at exit 0 and is now this failure naming the path it looked for." The CHANGELOG's own words are "this failure", the same one. The two documents were authored in the same increment and disagree.

### THIS IS NOT A RE-RAISE, stated explicitly because the neighbourhood is crowded

- NOT accepted cost (i). I am not asking for the anchoring behaviour to change; cost (i) is pinned as expected behaviour and I use it only as a fixture. This is the shape round 1 ruled VALID as `T-5`: a sentence describing cost (i), not the cost.
- NOT `T-2`. `T-2` was the SHIPPED PACK sentence predicating the tier on the `--instrument` FLAG, and its remedy landed and is closed. This is a SOURCE COMMENT making a different claim (that the message discriminates), and it survived `T-2`'s fix untouched. Round 1's triage used the same measurement as SUPPORTING REASONING for raising `T-2`'s severity but raised no finding against this sentence, and no round 2 or round 3 artifact mentions it: `grep -rn "distinguishable\|distinguish" docs/plans/agent-scaffold.reviews/` returns three hits, none of them this sentence.
- NOT `R3B-1`. That was the word "only" further down the same comment block, and its fix landed at `37df2ab`.

### Right behaviour

A deletion, which is what this project prefers and what the last two fix passes have been. Delete the clause and keep the true half of the sentence:

> ... no enforcement at all. The problem names the RESOLVED path. This is the tier boundary and nothing wider: ...

"The problem names the RESOLVED path" is true and is the fact the arm actually relies on; the inference drawn from it is the part that is false. Ten words removed, zero authored, and no behaviour, message, test or exit code changes. If the human would rather keep an inference there, the true one is that the resolved path lets a reader see WHICH path was checked, which is what the CHANGELOG already says.

I did not measure this deletion against the suite, because no test asserts source-comment text; `cargo test` cannot be affected by it.

---

# Observations that are NOT findings, each with the reason

Four. None is counted, and each says why, so a fix pass does not act on them.

## 1. A containment TOCTOU: a symlink swapped mid-run yields `workflow invariants hold` over another project's log. REPRODUCED, and BYTE-IDENTICAL ON PRE, so it is not this increment's

This is the most consequential thing I constructed and it is pre-existing. I record it in full because it is a false green on the surface this increment is about, and because no prior round attacked the window between the log READ (`src/main.rs:847`) and the CONTAINMENT check (`:991`).

The window is widened arbitrarily by making `--source` a FIFO, which blocks the run at `fs::read_to_string(source_path)?` (`:881`), after the log has been read and before containment is computed.

```
FIXTURE
  proj/docs/plans/p.plan.toml  a FIFO (the plan body, with one `complete` step, is fed later)
  proj/docs/metrics/decoy.jsonl  EMPTY, in-root
  proj/docs/metrics/workflow.jsonl  a SYMLINK, initially -> ../../../foreign/foreign.jsonl
  foreign/foreign.jsonl  a converged round record for `only-step`, OUTSIDE the project root

RACE
  1. start `validate --source docs/plans/p.plan.toml --workflow` in proj
  2. sleep 1                     (the log has been read: the FOREIGN contents)
  3. ln -sfn proj/docs/metrics/decoy.jsonl proj/docs/metrics/workflow.jsonl
  4. feed the plan body into the FIFO

RESULT, NEW
  docs/metrics/workflow.jsonl: 1 records, valid
  docs/plans/p.plan.toml: 1 steps, 0 questions, valid
  docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
  exit=0

RESULT, PRE
  (byte-identical, exit=0)
```

TWO CONTROLS, both run statically on the same fixture with the same NEW binary:

```
symlink left at the in-root DECOY (what containment approved):
  Roadmap step `only-step` is `complete` but has no round records and no covering waiver ...   exit=1
symlink at the FOREIGN log, no race (what was actually read):
  --workflow would join docs/plans/p.plan.toml against docs/metrics/workflow.jsonl, which is
  not under the plan's project root ...                                                        exit=1
```

So both endpoints of the race refuse, and only the interleaving passes. The evidence that produced the green came from outside the root, which is what `README.md:236`'s promise ("resolving both through their real on-disk locations so a symlink cannot disguise one as the other") exists to prevent.

WHY IT IS NOT A FINDING AGAINST THIS INCREMENT. It is byte-identical on `9eeca42`, which predates the whole increment; the containment guard and the position of the log read are both unchanged context in `git diff main...HEAD`; and this diff's only product change is the gate rebinding and the `_` arm. It is the same disposition round 3 gave `R3A-3`: pre-existing, out of scope, ROUTE it. I would route it to the validation-constraints step beside `R2A-4`, `R3A-3` and the queued plain-`validate` inconsistency, since all four are about a gate or a guard answering from an observation other than the one that decides. I note without arguing it that the reachability is a concurrent writer or an adversary inside the repository, which is a weaker threat model than the rest of that queue.

## 2. Three "the check could not run" paths bypass the increment's machinery and print a bare io error. PRE-EXISTING and byte-identical, and the queued item already owns two of them

Under `--workflow`, when the probe answers `Ok(true)` and the READ then fails, `run_validate` propagates through `?` and `fn main() -> io::Result<()>` prints a Debug io error naming no path, no flag, and not the fact that the check did not run:

```
                                  NEW                                              PRE
log FILE at mode 000    Error: Os { code: 13, kind: PermissionDenied ... } exit=1   identical
a DIRECTORY at the path Error: Os { code: 21, kind: IsADirectory ... }     exit=1   identical
invalid UTF-8 in the log Error: Error { kind: InvalidData, message:
                          "stream did not contain valid UTF-8" }           exit=1   identical
```

All three exit non-zero, so the increment's contract ("a check that did not run must not report success") holds on every one. Byte-identical to PRE in all three. The first two are the pre-existing plain-`validate` inconsistency the brief names as QUEUED to validation-constraints; the third is the same family. NOT RAISED.

## 3. A FIFO at the resolved log path hangs the run forever, with and without `--workflow`. PRE-EXISTING and not gated on the flag

```
$ timeout 5 NEW validate --source docs/plans/p.plan.toml --workflow    -> exit 124 (hung)
$ timeout 5 PRE validate --source docs/plans/p.plan.toml --workflow    -> exit 124 (hung)
$ timeout 5 NEW validate --source docs/plans/p.plan.toml               -> exit 124 (hung)
```

`try_exists` answers `Ok(true)` on a FIFO and `fs::read_to_string` blocks at `open` until a writer appears. Identical on PRE and identical without `--workflow`, so it is neither new nor confined to this surface. NOT RAISED.

## 4. `--workflow-spec` accepts a required streak of `0`, and the scaffolded `.agents/workflow.toml` is not read unless named. PRE-EXISTING and DOCUMENTED

```
$ NEW validate --source ... --workflow --workflow-spec <spec with low_risk = 0, risky = 0>
  docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold   exit=0
  (over a `complete` step whose only round is a non-converged one)
```

And with the project's own `.agents/workflow.toml` edited to `risky = 3`, a run WITHOUT `--workflow-spec` enforces the built-in `2` (identical on NEW and PRE); passing `--workflow-spec .agents/workflow.toml` enforces `3`. NOT RAISED: `--workflow-spec` is untouched by this diff, its clap help states the default explicitly ("When omitted, the built-in default (today's constants) is used, so the check is unchanged"), the shipped asset is drift-guarded equal to `builtin()`, and a control-constant value edit being reviewable data is the stated design of `src/workflow_spec.rs`. I record it only because "a user following the scaffolded guidance end to end" is one of the surfaces I was asked to cover, and the pack documents no `--workflow-spec` anywhere (`grep -rn "workflow-spec" README.md pack/ AGENTS.md .agents/ CHANGELOG.md justfile` returns nothing).

---

# What I attacked that produced nothing

Everything here was run on both binaries with the output compared as one string. A negative result is listed because a clean round is only credible if the reader can see the surface covered.

## The parse path after a successful probe (brief item 2). Six hostile log bodies, all correct, all byte-identical to PRE

| Log body | NEW | PRE | Verdict |
| --- | --- | --- | --- |
| Invalid UTF-8 (`\xff\xfe` prefix) | `Error: ... stream did not contain valid UTF-8`, exit 1 | identical | no false green |
| A NUL byte inside a JSON string | `workflow.jsonl:1: invalid JSON: control character (U+0000 to U+001F) found while parsing a string at line 1 column 26`, exit 1 | identical | correct, names the line |
| A truncated final line (no newline, partial JSON) | `workflow.jsonl:2: invalid JSON: EOF while parsing a string`, exit 1 | identical | correct, names the line |
| A UTF-8 BOM before a valid record | `workflow.jsonl:1: invalid JSON: expected value at line 1 column 1`, exit 1 | identical | correct |
| An empty file | `0 records, valid` + `workflow invariants hold`, exit 0 | identical | this is round 1's `T-7`, ACCEPTED AS A RESIDUAL, not re-raised |
| Whitespace-only lines | `0 records, valid` + `workflow invariants hold`, exit 0 | identical | same class as `T-7` |

No malformed body produced a false green and none produced a message that misdescribes what happened. `validate_log` reports every bad line with a 1-based number, and a non-empty `problems` list suppresses every summary, so `workflow invariants hold` cannot be printed beside a reported problem.

## Semantically hostile but schema-valid logs. Three shapes, all caught

```
a hand-written record claiming consecutive_clean 5 after a `new_valid` outcome, step `complete`
  -> round log line 1: increment `only-step` records consecutive_clean 5 but its outcome
     sequence implies 0                                                              exit=1
a log holding only a `decision` record, step `complete`
  -> Roadmap step `only-step` is `complete` but has no round records and no covering
     waiver ...                                                                      exit=1
one self-declared `clean` low_risk round, step `complete`
  -> workflow invariants hold                                                        exit=0
```

The third is the designed minimum (one clean round converges a `low_risk` artifact) and is not a defect. The self-declared `risk_class` is the tool's documented input and W5 owns waiver evidence; nothing there belongs to this increment.

## Several problems at once, and the reporting order (brief item 3)

The three-problem maximum is constructible and all three are reported, in the order metrics, source, plan, workflow:

```
$ NEW validate --source docs/plans/md.plan.toml --plan plan.md --workflow
  (a Markdown-primary --source with a schema error, a --plan with a Roadmap status error,
   and no round log)
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
docs/plans/md.plan.toml: malformed `<task>.plan.toml`: TOML parse error at line 13 ...
plan.md: Roadmap step `only-step` has an unknown status `bogus`
--workflow requested but no round log at docs/metrics/workflow.jsonl: ...          exit=1
```

Nothing is dropped and the exit code is right. THE NEW PROBLEM CANNOT CO-OCCUR WITH A CONTAINMENT REFUSAL, which I established by construction rather than by reading: the guard is evaluated before the match (`:992`), so an out-of-root `--metrics` that is ALSO absent reports only the refusal, on NEW and PRE alike, byte-identical. That is the right precedence (the pairing has to be fixed before the log's presence matters) and it is unchanged by this diff. Nor can it co-occur with a metrics-record problem, since that requires the log to have been read.

A malformed `--workflow-spec` still short-circuits before the match (`std::process::exit(1)` at `:973`), so the spec error is reported and the round-log problem is not; identical on PRE. Round 1's triage already verified that precedence and I reproduced it.

## `--workflow-spec` and other flag combinations crossed with the new failure (brief item 4)

| Combination, with NO round log | NEW | PRE |
| --- | --- | --- |
| `--workflow-spec` naming a missing file | `Error: Os { code: 2, kind: NotFound ... }` exit 1 | identical |
| `--workflow-spec` malformed | `bad.toml: malformed workflow spec: TOML parse error ...` exit 1 | identical |
| `--workflow-spec` naming a directory | `Error: Os { code: 21, kind: IsADirectory ... }` exit 1 | identical |
| `--workflow-spec` valid | the new round-log problem, exit 1 | the old skip note, exit 0 |
| `--workflow-spec` without `--workflow` | clap usage error, exit 2 | identical |
| `--metrics ""` | clap: "a value is required for '--metrics <METRICS>'", exit 2 | identical |
| `--metrics .` and `--metrics docs/metrics/` (directories) | `Error: Os { code: 21 ... }` exit 1 | identical |
| `--metrics` at an in-root symlink to `/dev/null` | the containment refusal, exit 1 | identical |

The last is a positive result for the guard: `is_outside_root` resolves through the real on-disk location, so a symlink out of the root is refused even though the lexical path is inside it. That is the property the race in observation 1 defeats only by changing the target mid-run.

## Exit-code semantics (brief item 5). A systematic sweep, and no false green anywhere

Eleven `--workflow` inputs, NEW against PRE, exit code and stderr head compared:

| Fixture | NEW | PRE |
| --- | --- | --- |
| present valid log | 0 | 0 |
| log absent, `docs/metrics` present | 1 (new problem) | 0 (skip note) |
| `docs/metrics` absent entirely | 1 | 0 |
| dangling symlink at the log path | 1 (`Ok(false)` -> absent sentence) | 0 |
| symlink loop at the log path | 1 (`Err` -> could-not-be-checked) | 0 |
| a directory at the log path | 1 (bare io error) | 1 |
| log FILE at mode 000 | 1 (bare io error) | 1 |
| `docs/metrics` at mode 600 | 1 (`Err` -> could-not-be-checked) | 0 |
| empty log | 0 | 0 |
| ENOTDIR via `--metrics <file>/log.jsonl` | 1 | 0 |
| ENAMETOOLONG via a 300-character leaf | 1 | 0 |

EVERY DIFFERENCE IS A 0 TO 1 FLIP AND THERE IS NO 1 TO 0 ANYWHERE, which is the increment's whole intent. I found no input on which `--workflow` exits 0 while the check did not run, and I looked for one by enumerating the routes rather than by sampling: exit 0 under `--workflow` requires `metrics_contents` to be `Some`, which requires the probe to answer `Ok(true)` AND `fs::read_to_string` to succeed, after which the match reaches a `report_workflow` arm and the check has run.

ON THE DISTINCTION A CALLER CANNOT MAKE FROM THE EXIT CODE: "the check ran and found violations" and "the check could not run" are both exit 1, and the discriminator is the message. I am NOT raising that. The arm's comment states it as a deliberate choice ("Exit 1 either way, since the check did not run regardless"), the increment's stated contract is that success must not be reported rather than that the two must be separable by code, and no shipped sentence promises a caller can tell them apart from the status. I record it as measured and deliberate so a later round does not have to re-establish it.

## The scaffolded pack as shipped, walked end to end (brief item 6)

A fresh `scaffold --output-dir . --write --instrument` with the NEW binary, then the documented flow:

```
STEP 1  fresh instrumented scaffold, the README's own command
  NEW: --workflow requested but no round log at docs/metrics/workflow.jsonl ...   exit=1
  PRE: skip note + `TEMPLATE.plan.toml: 1 steps, 0 questions, valid`              exit=0

STEP 2  follow AGENTS.md's instrumentation block literally: create docs/metrics/ and append
        one `round` record built only from the fields that block documents
  NEW: 1 records, valid / 1 steps, 0 questions, valid / workflow invariants hold  exit=0

STEP 3  mark the template's example step `complete`, re-run
  NEW: workflow invariants hold                                                   exit=0
```

So the guidance's own record schema joins to the step and the check goes green at the point the guidance says it will. The scaffolded tree confirms `--instrument` still renders no `docs/metrics` and no log (`find` for `*metrics*` and `*.jsonl` returns nothing), which is what `T-2` was about, and the `AGENTS.md` sentence at `:93` now predicates on the log rather than the flag, so a reader on either population predicts step 1's exit 1 correctly. I found no point where the guidance leaves a follower unprepared for the new failure. Nothing scaffolded invokes `validate --workflow` (I re-confirmed round 3's negative over the SCAFFOLDED OUTPUT rather than over `pack/`: `grep -rn "validate" <scaffolded tree>` finds it only in prose).

## Concurrency beyond the two settled FIFO cells (brief item 1)

- The PROBE-TO-READ window (`:845` to `:847`) is unwidenable: nothing blocking sits between the two calls, and the window is byte-for-byte the same code position as PRE's. I did not construct a win there and I say so plainly.
- The READ-TO-CONTAINMENT window IS widenable and produces the false green in observation 1. Byte-identical on PRE.
- The arm's own re-stat is gone: it reads the captured `metrics_probe`, so round 2's `V-2` disagreement directions are closed by construction, which the round 3 triage already measured and I did not repeat.
- A log GROWING during the read is a non-event: `read_to_string` reads to EOF once and the record count is over what it read.

---

# Coverage gaps, stated plainly

1. **Linux only.** Every construction ran on this machine. Nothing about Windows path semantics, `try_exists` behaviour there, or the `#[cfg(unix)]` test's absence was examined.
2. **No root-namespace run.** Round 2's `V-1` was about the suite as namespace root and its fix is closed; I did not re-run `unshare -Ur cargo test`, so I add no independent evidence that the root-fragility stays closed. The brief lists the 422/0 as-root result as established.
3. **No large-input or resource-exhaustion testing.** I did not test a multi-gigabyte log, a log with millions of records, or a `/dev/zero` symlink at the log path. `fs::read_to_string` reads the whole file into memory with no cap, which is pre-existing, but I did not measure where it breaks or whether the failure is graceful.
4. **The race in observation 1 is demonstrated, not bounded.** I did not measure how narrow the window is without a FIFO, so I cannot say whether it is reachable by an ordinary concurrent process rather than a deliberate one.
5. **No fuzzing of the plan TOML or the record schema.** My hostile inputs were hand-constructed from reading `metrics.rs` and `workflow.rs`, so a shape neither of us thought of would be missed. In particular I did not attack `w4_problems` or `w5_problems` at all; my semantic attacks were on the round-log consistency check and W3, because those are what the tier boundary's green depends on.
6. **I did not re-derive the settled negatives.** `Ok(true)` unreachability in the arm, plain `validate`'s byte-identity with PRE across 35 inputs, and the byte-identity of `status`, `next`, `status --resume`, `render` and `audit` were taken from the brief and the prior triages rather than re-measured. If any of those is wrong, nothing in this file catches it.
7. **The `Ok` arm's two remedy clauses were checked for liveness but not exhaustively.** `pass a --metrics naming this project's log` and `record the project's review rounds there` are each live on at least one reachable input (a mis-anchored run and a genuinely non-instrumented project respectively), and I found no input where both are inert. I did not enumerate the input space to prove there is none; `R3A-1`'s residual is the `Err` arm's analogue and is settled.

---

# Relitigation and constraints check

I checked every item above against the settled list. The four standing residuals (the in-root bound, the single-anchor `..` case, the earlier increment's rejected-ledger context slot, the off-convention `--source` surface) appear nowhere. Accepted costs (i) to (iv) are treated as PINNED EXPECTED BEHAVIOUR throughout: cost (i) is used as `R4A-1`'s fixture and as a control, never as a defect, and `R4A-1`'s remedy changes no behaviour on it. Round 1's `T-7` (`ADV-4`, the empty log) is reproduced in my parse-path table as a control and explicitly not re-raised. Round 1's `SC-3`, round 2's `R2A-4`, `R2B-2` and `R2B-3`, and round 3's `R3A-1` and `R3A-3` are not raised or reopened. The queued pre-existing plain-`validate` inconsistency appears only in observation 2, routed rather than raised. `Q-55-existsgate`'s DECLINED `try_exists()?` gate change is not asked for anywhere; `R4A-1`'s remedy is a comment deletion and touches no gate.

`R4A-1` is adjacent to round 1's `T-2` and to accepted cost (i), and the finding says so in its own section with the evidence that it is neither.

No line-length, prose-wrapping or comment-raggedness observation appears anywhere in this file. The two deliberately ragged comment lines left by the round 3 fix are correct and are not mentioned as a defect.

FIXTURE HYGIENE: all fixtures under `<scratch>/r4adv`, a directory of my own naming; nothing outside it was written or deleted. Every directory chmodded to 600 or 000 and every file chmodded to 000 was restored; the closing `find <scratch>/r4adv -type d ! -perm -u+rwx`, `find <scratch>/r4adv -type f -perm 000` and `find <scratch>/r4adv -type p` all return nothing. `TMPDIR` pointed outside any git repository for `cargo test`. No source edit was made at any point, `git status --short` is empty on this worktree, and the main repository at `/home/jessea/Documents/projects/agent-scaffold` was not touched.

# Tally

| Severity | Count | Findings |
| --- | --- | --- |
| critical | 0 | |
| high | 0 | |
| medium | 0 | |
| low | 1 | `R4A-1` |

ONE finding, `low`. Its remedy is a ten-word deletion from a source comment that authors nothing and changes no behaviour, message, test or exit code.

NOT COUNTED: four observations, all measured byte-identical to `9eeca42` and therefore pre-existing, of which observation 1 (the containment TOCTOU) is the one I would route to the validation-constraints step.

THE PRODUCT ITSELF IS CORRECT ON EVERY INPUT I CONSTRUCTED. Across eleven filesystem arrangements, six hostile log bodies, three semantically hostile logs, eight flag combinations and a three-problem composition, `--workflow` never reported success for a check that did not run, never dropped a problem, and never got an exit code wrong. Every difference from the pre-increment build is a 0 to 1 flip on an input where the check could not run. The one finding is about what a comment ASSERTS, not about what the tool DOES.
