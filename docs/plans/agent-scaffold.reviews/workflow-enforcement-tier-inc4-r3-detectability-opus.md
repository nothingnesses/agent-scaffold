# `workflow-enforcement-tier-inc4`, round 3, detectability lens

Reviewer C. Worktree `.claude/worktrees/rev-inc4-r3-c`, branch `review/wet-inc4-r3-c`, at `93ee357`. Every fixture and every saved mutation patch lives under `<scratchpad>/rev-inc4-r3-c/` only. No chmod was used, so nothing is owed a restore. Nothing outside that subdirectory was created, moved or deleted.

This lens does not look for another false sentence. It asks the inverse question: IF THIS ARTIFACT WERE WRONG, WOULD ANY MECHANICAL GATE SAY SO. The question is applied to the twenty in-scope valid findings rounds 1 and 2 already produced, and then measured by mutation rather than argued.

## Summary

THE TALLY OVER THE TWENTY IN-SCOPE VALID FINDINGS: **CATCHABLE 0, NOT CATCHABLE 20.**

Not one of the twenty would have been caught by `cargo test` (including the `agents-md-drift-guard` and `prompt-drift-guard` comparisons), by `cargo clippy --all-targets -- -D warnings`, by `agent-scaffold render --check` in either its warning or its `--strict` form, or by `agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow` and its W1 to W5 invariants.

SEVEN MUTATIONS RUN, one at a time, each followed by the full gate set. SIX SURVIVED COMPLETELY SILENT. One was caught, and it is the one class that is NOT represented among the twenty. TWO POSITIVE CONTROLS confirm the harness can detect something, so the six null results are measurements rather than a broken rig.

FINDINGS: 0 critical, 0 high, 4 medium, 1 low.

| id | severity | one line | disposition I think right |
| --- | --- | --- | --- |
| `R3C-1` | medium | The gate set catches 0 of 20. The project owns no detector for its highest-yield defect class. | NEW BACKLOG STEP, not a fix to this increment |
| `R3C-2` | medium | A waiver `note`'s per-round breakdown is checkable against round records in the same log the same command already opens, and nothing checks it. | NEW BACKLOG STEP (this is the buildable mechanism) |
| `R3C-3` | medium | Decision receipts are write-only for 29 of 51 distinct `q_id`s, and no check compares `chosen` against the plan. A human decision can be silently reversed. | NEW BACKLOG STEP |
| `R3C-4` | medium | Acceptance check 21 is not executable as written for 51 of the 73 citations in its own file, and its declared scope excludes the sites three of the twenty live at. | FIX IN THIS INCREMENT (check 21 is this increment's own authored text) |
| `R3C-5` | low | The project's one hard documentation gate was run by none of the eight inc4 reviewer and triager passes. | PROCESS NOTE |

ON THE STEP'S OWN THESIS, STATED PLAINLY BECAUSE I WAS ASKED TO RULE ON IT EITHER WAY. The 0-of-20 result does NOT falsify this step's thesis, and I will not claim it does. The step's backstop promise, as `pack/AGENTS.md:93` actually words it, is that `validate --workflow` is "the backstop that the required reviewed rounds happened before a step is marked complete". That promise is about ROUND-LOG EVIDENCE, and it holds: positive control `PC1` below shows W3 firing at exit 1 on exactly that condition. The backstop was never scoped to the truth of documentation prose, so a documentation-currency pass producing twenty prose defects no deterministic check saw is not the thesis failing. What the result DOES show is narrower and still worth routing: across this step, the single most productive defect class by a wide margin is a false claim in plan prose, and the project has no mechanism of any kind aimed at it.

## The gate set, and proof the harness works

Gate set as run, all from the worktree root with the binary built at `93ee357`:

```
cargo build --all-targets
cargo test
cargo clippy --all-targets -- -D warnings
agent-scaffold render docs/plans/agent-scaffold.plan.toml --check
agent-scaffold render docs/plans/agent-scaffold.plan.toml --check --strict   # the form .agents/checks.toml declares
agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
```

I added the `--strict` form after finding that the plain form WARNS AT EXIT 0. `README.md:204` states that is deliberate ("it warns locally (so a forgotten re-render never blocks an in-flight step) and, with `--strict`, fails hard"), and `.agents/checks.toml:18` declares the strict form as the project's render gate. Including it is what makes the `M5` result below honest rather than an artefact of reading exit codes only.

BASELINE AT `93ee357`, all six green:

```
cargo test                          exit 0
cargo clippy --all-targets -D warn  exit 0
render --check                      exit 0   docs/plans/agent-scaffold.plan.toml: up to date
render --check --strict             exit 0
validate --source ... --workflow    exit 0   workflow invariants hold
```

POSITIVE CONTROL `PC1`. Flip `test-tmpdir-repo-assumption` from `not-started` to `complete` in `docs/plans/agent-scaffold.plan.toml`, re-render, run the gates:

```
cargo test                          exit 0
cargo clippy --all-targets -D warn  exit 0
render --check                      exit 0
render --check --strict             exit 0
validate --source ... --workflow    exit 1
  docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step
  `test-tmpdir-repo-assumption` is `complete` but has no round records and no covering
  waiver; log its review rounds, or record a `type:"waiver"` for it if it predates
  logging or its review was skipped
VERDICT: CAUGHT
```

W3 fires. The rig detects what it is built to detect, so every "not caught" below is a measurement.

POSITIVE CONTROL `PC2`, and it did NOT behave as expected, which is the seed of `R3C-3`. Delete the entire `type:"decision"` receipt for `Q-55-w1figure` (log line 289) and re-run:

```
$ grep -v '"q_id":"Q-55-w1figure"' docs/metrics/workflow.jsonl > tmp && mv tmp docs/metrics/workflow.jsonl
receipt deleted, lines now: 292
$ agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 292 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
VALIDATE_EXIT=0
```

W4 does not fire, because `Q-55-w1figure` is not a registered `[[question]]`. See `R3C-3`. The log was restored from a byte copy taken before the edit and `git status --short` is clean.

## (A) The twenty findings, classified

I classify against the gate set only. The question is not whether a defect is important; it is whether any of the six commands changes its exit code or its output because of it.

ALL TWENTY SHARE ONE STRUCTURAL PROPERTY that decides the answer before the individual analysis: every one of them is a false or stale statement in PROSE, held in a plan source (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, two sibling step sidecars, and the `Q-55` `ask` and `-w1` waiver `note` in `docs/plans/agent-scaffold.plan.toml`), and every one of them was committed in a state where the source and the generated view AGREE. That combination puts them outside all six gates by construction:

- `cargo test` and `cargo clippy` compile and exercise Rust. Nineteen of the twenty touch no Rust at all. The twentieth, `R2B-5`, concerns two COMMENT corrections in `tests/unsafe_pairings_are_refused_and_omitted.rs`; comment text is not asserted by any test and does not affect compilation.
- The `agents-md-drift-guard` and `prompt-drift-guard` comparisons in `cargo test` compare root `AGENTS.md`, `.agents/AGENTS.reference.md` and `.agents/prompts/` against a fresh render OF THE PACK. None of the twenty is in `pack/` or in a deployed copy. The increment's own documentation-impact list says so in as many words at sidecar `:388` ("NOT `README.md`, NOT `pack/AGENTS.md` and NOT the deployed `.agents/` copies").
- `render --check`, in either form, compares the committed `<task>.md` against a fresh render of the TOML plus sidecars. It is a CONSISTENCY-OF-COPY check. It has no opinion on whether the copy is true. All twenty were rendered consistently.
- `validate --workflow` reads the plan's STRUCTURED fields (step slug, status, waiver identity, reason, evidence tier, question status) and the JSONL round, escalation, decision and intake records. It reads no prose body. Confirmed by reading `src/workflow.rs:544-621`: `w5_problems` inspects `waiver.step`, `waiver.unit`, `waiver.increment`, `waiver.evidence`, `waiver.evidence_tier` and `waiver.reason`, and NEVER `waiver.note`.

Per finding, with the reason it escapes:

| id | defect class | why no gate sees it |
| --- | --- | --- |
| `R1A-1` | incomplete enumeration of `{pid}-{nanos}` sites, sidecar `:55` of a sibling file | prose; no gate enumerates anything |
| `R1A-2` + `R1C-1` | inverted "appears exactly ONCE" claim about `#[serde(skip)]`, sidecar `:206` | prose claim about a grep result; nothing greps prose against source |
| `R1A-3` + `R1C-2` | stale present tense, sidecar `:195` | prose tense |
| `R1A-4` | two quoted fragments that resolve nowhere in the tree, sidecar `:201`, `:202` | no gate runs a quotation as a search |
| `R1A-6` | quoted command line omits the argument producing the quoted output, check 16 | no gate executes a quoted command |
| `R1B-1` | wrong figure, 20 against a true 13, sidecar `:308` and three twin sites | see `M2`: measured, uncaught |
| `R1B-2` | self-contradiction, check 21 calls its own procedure mechanical | prose |
| `R1C-3` | present-tense claim that `status --json` has no serialisation test, sidecar `:304` | prose claim about the suite; the suite does not read it |
| `R1C-4` | three present-tense claims inc2/inc3 falsified, one inverted, sidecar `:255`, `:257`, `:259` | see `M3`: measured, uncaught |
| `R1C-5` | the generated view contradicts itself, `agent-scaffold.md:168` against `:1614` | the view FAITHFULLY renders a source that disagrees with itself, so `render --check` is green; round 1's triage recorded this at its own line 480 |
| `R1C-6` | a citation re-pointed across a scope boundary, making check 21b's "AND ONLY THOSE" untrue of its own commit | prose; no gate reads an acceptance check |
| `R2A-1` + `R2C-1` | `README.md:228` citation resolves to the wrong line, plan TOML `:1732` | see `M1`: measured, uncaught |
| `R2A-2` | authored acceptance clause false of the tree, sidecar `:346` | prose |
| `R2A-3` + `R2C-2` | surviving present-tense tail, plan TOML `:1728` | prose tense |
| `R2A-4` | conclusion stranded in present tense, sidecar `:195` | prose tense |
| `R2B-1` | asserts the absence of a guard that is present, sidecar `:157` | prose claim about code |
| `R2B-2` | false "ONLY when" exhaustiveness, sidecar `:204` | prose |
| `R2B-3` | true premise carrying a false inference, sidecar `:206` | prose |
| `R2B-5` | wrong count, "the increment's one source change", sidecar `:282`, `:385` | prose; the two real test-comment edits compile identically |
| `R2C-3` | stale "today", sidecar `:304` | prose tense |

CATCHABLE: 0. NOT CATCHABLE: 20.

A ZERO-MUTATION CORROBORATION, because it needs no fixture at all. `checks-runner-worktree-name-collision.md:14` cites `src/checks.rs:78` for `RUNNER_PREFIX` and `src/checks.rs:791-792` for the worktree naming. At `93ee357`:

```
$ sed -n '78p' src/checks.rs
                        PathBuf,
$ sed -n '791,792p' src/checks.rs
        if !listed.status.success() {
                return Err(RunError::WorktreeSetup(format!(
```

Neither range holds its named subject. About eleven such citations sit in that file right now, deliberately, by the `Q-55-check21b` decision. All six gates are green over them. Stale `file:line` citations are therefore not merely undetected in principle; a population of them is in the committed tree at this moment with every gate reporting success.

## (B) The mutations, measured

Method. One mutation at a time, applied with an exact-string edit. Where the mutation is in a plan SOURCE it is followed by `agent-scaffold render docs/plans/agent-scaffold.plan.toml`, so that the committed view and the source agree and the mutation reproduces the state the twenty findings were actually committed in. Then the six gates. Then revert, with the exact diff saved to `<scratchpad>/rev-inc4-r3-c/patches/<id>.patch` first, so no measurement is unrecoverable.

### `M1`: a `file:line` citation re-pointed at a wrong line

`docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md:14`, `src/main.rs:2289-2305` becomes `src/main.rs:1105-1121`. The original resolves (line 2289 is `#[test]`, 2290 is `fn init_plan_defaults_to_git_and_skips_inside_a_repo()`); the replacement does not.

Run first WITHOUT re-rendering, then WITH. Both are informative.

```
WITHOUT re-render:
  cargo test                          exit 0
  cargo clippy --all-targets -D warn  exit 0
  render --check                      exit 0
      warning: docs/plans/agent-scaffold.md differs from a fresh render (a hand-edit,
      or a stale render after a source edit) (first difference at line 1814: ...)
  validate --source ... --workflow    exit 0
WITH re-render (the realistic committed state):
  cargo build --all-targets           exit 0
  cargo test                          exit 0
  cargo clippy --all-targets -D warn  exit 0
  render --check                      exit 0   docs/plans/agent-scaffold.plan.toml: up to date
  render --check --strict             exit 0
  validate --source ... --workflow    exit 0   workflow invariants hold
  VERDICT: ALL GREEN AND SILENT - NOT CAUGHT
```

The un-rendered run is why the exit-code-only reading is not safe: `render --check` NOTICED the source edit and still exited 0. Once re-rendered, nothing notices anything. This is `R2A-1`'s class exactly.

### `M2`: a waiver note's per-round figure reverted, which is also a silent reversal of a human decision

`docs/plans/agent-scaffold.plan.toml:1330`, the `workflow-enforcement-tier-w1` waiver `note`. "Three work-review rounds, 13 valid findings (3, 4, 6)" becomes "Three work-review rounds, 20 valid findings". Re-rendered.

This is `R1B-1`'s class. It is ALSO the exact state the human decision `Q-55-w1figure` (receipt at log line 289) rejected: its `options` are `["Correct to 13 with the per-round breakdown", "Leave it at 20, record as a residual", "Delete the count from the note"]` and its `chosen` is the first. `M2` restores the second while leaving the receipt in place saying the first was chosen.

```
  cargo build --all-targets           exit 0
  cargo test                          exit 0
  cargo clippy --all-targets -D warn  exit 0
  render --check                      exit 0   docs/plans/agent-scaffold.plan.toml: up to date
  render --check --strict             exit 0
  validate --source ... --workflow    exit 0   workflow invariants hold
  VERDICT: ALL GREEN AND SILENT - NOT CAUGHT
```

THE AGGRAVATING FACT, and the reason this is `R3C-2` rather than just another null. `validate --workflow` OPENS BOTH SIDES OF THIS CONTRADICTION IN THE SAME RUN. It reads the waiver out of `docs/plans/agent-scaffold.plan.toml` and the round records out of `docs/metrics/workflow.jsonl`, and joins them for W3 and W5. The refuting data is in the file it already parsed:

```
$ for L in 246 247 248; do sed -n "${L}p" docs/metrics/workflow.jsonl | grep -o '"valid_findings":[0-9]*'; done
"valid_findings":3
"valid_findings":4
"valid_findings":6
```

Those are the only `type:"round"` records for `workflow-enforcement-tier-inc1`. 3 + 4 + 6 = 13. The tool holds the note claiming 20 and the records summing to 13 in memory at the same moment and reports `workflow invariants hold`.

### `M3`: re-introduce a present-tense claim an increment falsified

Sidecar `:255`, "(i) A BARE FILENAME RUN FROM INSIDE `docs/plans` WAS A SILENT MISS" becomes "REMAINS A SILENT MISS". Re-rendered. This restores the exact sentence `R1C-4`(a) was raised on, and it is a claim the shipped binary refutes: on that input the tool exits 1 with a hard failure.

```
  cargo build --all-targets           exit 0
  cargo test                          exit 0
  cargo clippy --all-targets -D warn  exit 0
  render --check                      exit 0   docs/plans/agent-scaffold.plan.toml: up to date
  render --check --strict             exit 0
  validate --source ... --workflow    exit 0   workflow invariants hold
  VERDICT: ALL GREEN AND SILENT - NOT CAUGHT
```

### `M4`: an acceptance check's expected exit code inverted

Acceptance check 15, sidecar `:338`. "exits NON-ZERO and THE REPORTED PROBLEM names" becomes "exits 0 and THE REPORTED PROBLEM names". Re-rendered.

Check 15 is the acceptance criterion for defect A, and the non-zero exit IS inc3's entire behavioural change. After this mutation the check asserts the PRE-inc3 behaviour, so a future acceptance pass running the check as written would ratify a regression as correct.

```
  cargo build --all-targets           exit 0
  cargo test                          exit 0
  cargo clippy --all-targets -D warn  exit 0
  render --check                      exit 0   docs/plans/agent-scaffold.plan.toml: up to date
  render --check --strict             exit 0
  validate --source ... --workflow    exit 0   workflow invariants hold
  VERDICT: ALL GREEN AND SILENT - NOT CAUGHT
```

### `M5`: the generated view made to contradict its own source, by hand-edit. THE ONE THAT IS CAUGHT

`docs/plans/agent-scaffold.md:1614`, "RETYPED from `Option<String>` to a closed enum and NO LONGER `#[serde(skip)]`" becomes "... and STILL `#[serde(skip)]`". NOT re-rendered, because a hand-edit of the generated file is the whole point.

```
  cargo test                          exit 0
  cargo clippy --all-targets -D warn  exit 0
  render --check                      exit 0
      warning: docs/plans/agent-scaffold.md differs from a fresh render (a hand-edit,
      or a stale render after a source edit) (first difference at line 1614: ...)
  render --check --strict             exit 1
      error: docs/plans/agent-scaffold.md differs from a fresh render ...
  validate --source ... --workflow    exit 0
  VERDICT: CAUGHT, but only by the --strict form
```

TWO THINGS FOLLOW, and the second is why this does not rescue the tally.

First, the project DOES own a working hard gate for this class, and `.agents/checks.toml:18` declares it. Confirmed end to end: with the binary on `PATH`, `agent-scaffold checks` reports `pass render-check (lint)` and `checks: 1 passed, 0 failed, 0 skipped` at exit 0; without it on `PATH` it fails loudly at exit 1 rather than silently skipping, which is correct behaviour and not a defect.

Second, THIS CLASS IS NOT AMONG THE TWENTY. Not one of rounds 1 and 2's findings was a hand-edit of the generated view. `R1C-5`, the one finding that IS about the generated view contradicting itself, is the opposite case: a FAITHFUL render of a source that disagrees with itself, which `render --check` passes by design and which round 1's triage measured and recorded at its own line 480. So the project's single documentation gate guards consistency between two copies, and every one of the twenty was a truth defect in a pair of copies that were perfectly consistent.

### `M6`: a human decision silently reversed by deletion

Sidecar `:387`, the bullet "- `tests/unsafe_pairings_are_refused_and_omitted.rs`, the two comment corrections `Q-55-twinsites` authorised; no acceptance check states them." is DELETED. Re-rendered.

That bullet exists because of decision `Q-55-impactlist` (receipt at log line 293), whose `options` are `["Add the missing bullet", "Delete the exclusions bullet", "Accept as a residual"]` and whose `chosen` is "Add the missing bullet". The mutation restores the state the human was asked about and did not pick, and leaves the receipt asserting the opposite.

```
  cargo build --all-targets           exit 0
  cargo test                          exit 0
  cargo clippy --all-targets -D warn  exit 0
  render --check                      exit 0   docs/plans/agent-scaffold.plan.toml: up to date
  render --check --strict             exit 0
  validate --source ... --workflow    exit 0   workflow invariants hold
  VERDICT: ALL GREEN AND SILENT - NOT CAUGHT
```

### `M7`: a quotation of the tool's own output falsified

Sidecar `:96`, the quoted W3 message. "no covering waiver ... record a `type:\"waiver\"` for it" becomes "no covering exemption ... record a `type:\"exemption\"` for it". Re-rendered.

This quotation is not decorative: acceptance check 17 says "Expect exit 1 and the W3 message quoted above". `PC1` above prints the real message, so the mutation is refutable by running the binary once.

```
  cargo build --all-targets           exit 0
  cargo test                          exit 0
  cargo clippy --all-targets -D warn  exit 0
  render --check                      exit 0   docs/plans/agent-scaffold.plan.toml: up to date
  render --check --strict             exit 0
  validate --source ... --workflow    exit 0   workflow invariants hold
  VERDICT: ALL GREEN AND SILENT - NOT CAUGHT
```

### Mutation results, collected

| id | class | mirrors finding | caught |
| --- | --- | --- | --- |
| `M1` | wrong `file:line` citation, re-rendered | `R2A-1`, `R1C-6` | NO |
| `M2` | wrong waiver figure; also a reversed human decision | `R1B-1` | NO |
| `M3` | present-tense claim an increment falsified | `R1C-4`, `R2C-3` | NO |
| `M4` | acceptance check's expected exit code inverted | (none of the twenty; a worse form of `R1A-6`) | NO |
| `M5` | generated view hand-edited to contradict its source | (none of the twenty) | YES, `--strict` only |
| `M6` | human decision reversed by deleting what it ordered added | (none of the twenty; the `R3C-3` class) | NO |
| `M7` | quotation of the tool's own output falsified | `R1A-4`, `R1B-2` | NO |
| `PC1` | step `complete` with no rounds and no waiver | positive control | YES, W3 exit 1 |
| `PC2` | a `type:"decision"` receipt deleted outright | positive control, FAILED to fire | NO |

SIX OF SEVEN MUTATIONS SURVIVED SILENT. The one that was caught is in a class none of the twenty belongs to.

## (C) Checks 21, 21b, 22 and 23 judged as detectors

The brief asks whether running them as written would have caught the twenty. Verdicts differ sharply between them, so they are judged separately.

### Check 21: NOT EXECUTABLE AS WRITTEN, and it cannot fail on most of what it names. This is `R3C-4`.

Check 21 (sidecar `:345`) says: "Open each `file:line` citation in this file at the cited range and show the named subject is there; run each quoted fragment of source, test, `README.md` or `pack/AGENTS.md` text as a literal search against the file it is attributed to."

DEFECT 1, THE PROCEDURE IS UNDEFINED FOR MOST OF ITS OWN SUBJECTS. Measured over `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`:

```
$ grep -o '`[a-zA-Z0-9_./-]*\.\(rs\|md\|toml\|jsonl\):[0-9]\+\(-[0-9]\+\)\?`' <file> | wc -l
22
$ grep -o '`[a-zA-Z0-9_./-]*\.\(rs\|md\|toml\):[A-Za-z_][A-Za-z0-9_:]*`' <file> | wc -l
51
```

51 of the 73 citations are SYMBOLIC (`src/main.rs:Projection`, `src/main.rs:run_validate`, `src/next.rs:NextProjection::active_loop`, and so on). They carry no line number, so "open each `file:line` citation at the cited range" has no cited range to open for seventy percent of the file. The check is silent on what to do with them, and a symbolic citation cannot go stale by line movement, which is presumably why, but the check claims coverage it does not define.

DEFECT 2, THE SCOPE EXCLUDES WHERE THE FINDINGS WERE. Check 21 is scoped to "THIS FILE". Three of the twenty do not live in this file: `R2A-1`/`R2C-1` at `docs/plans/agent-scaffold.plan.toml:1732`, `R2A-3`/`R2C-2` at `:1728`, and `R1C-5`'s twin cluster in the `Q-55` `ask` at `:1713-1736`. Those are plan sources this increment edited, and check 21 does not reach them by its own terms.

DEFECT 3, THE COVERAGE OVER THE REMAINING SEVENTEEN IS THIN. Run as written, check 21 covers a citation resolving and a quotation matching. Squarely within that: `R1A-4` (two quoted fragments that resolve nowhere; a literal search returns exit 1 on both) and `R1A-3`/`R1C-2` (a symbolic citation whose named subject does not carry the attributed attribute). Arguably within it if the searcher counts hits rather than merely finding one: `R1A-2`/`R1C-1` and `R2B-3`, both of which turn on how many times `#[serde(skip)]` occurs. That is two squarely and two marginally, out of twenty. THE OTHER SIXTEEN ARE NOT CITATIONS OR QUOTATIONS AT ALL: they are wrong counts, stale tenses, false exhaustiveness claims, self-contradictions and incomplete enumerations, and no reading of check 21's procedure reaches them.

THE DECIDING EVIDENCE. Check 21 was authored by this increment to verify this increment, and the increment shipped claiming its acceptance checks were met. Twenty valid findings then arrived across two rounds, two of them squarely inside check 21's stated procedure. Either the check was not actually run, or it was run and missed its own two. A check in that position is worse than no check, because it converts an unexamined surface into one a reader believes was examined. `R1B-2` already found the adjacent defect (check 21 asserting "The check is mechanical rather than a reading") and it was fixed; the structural problem in the same sentence was not.

### Check 21b: EXECUTABLE, NARROW, AND ITS OWN HISTORY IS THE WARNING

Check 21b names three files and restricts itself to their `src/main.rs` and `tests/` citations. That IS executable, and it is honest about excluding the `src/checks.rs` population. It would have caught nothing among the twenty, because two of the twenty (`R1C-6`, `R2A-2`) are findings ABOUT check 21b's own exclusion clause rather than about citations it covers.

Its history is worth recording under this lens. Ledger `:555` records orchestrator defect (18): the orchestrator narrowed check 21b on a triager's factual premise that a symbol had ceased to exist, when it had not, and the ledger's own standing cure is "AN ORCHESTRATOR MUST NOT RULE ON A TRIAGER'S FACTUAL PREMISE WITHOUT CHECKING IT, and the check here was one `grep` for a symbol name". A one-grep verification would have prevented a `low` finding being answered with a `medium` one, and no mechanism performs it.

### Check 22: A GENUINE MECHANICAL DETECTOR. The only one of the four.

Check 22 says "MEASURE IT RATHER THAN READ IT" and states a command. Run as written:

```
$ agent-scaffold status --json --source docs/plans/agent-scaffold.plan.toml
{
  "plan": {
    "steps": [ { "slug": "core-assets", "status": "complete" }, ... ] }
exit 0
```

A populated `"plan"` object with no `--plan` given, as the check asserts. Its other clause also holds:

```
$ grep -n -B2 'plan: Option<PlanProjection>' src/main.rs
570-/// The plan projection, present when a TOML-primary `--source` or a readable `--plan` supplies one. It carries no
571-/// reason field: there is exactly one cause, so a reason there would inform nobody.
572: plan: Option<PlanProjection>,
$ grep -rn 'present only when a readable' src/
(no output, exit 1)
```

Check 22 is executable, falsifiable, and passes. It is the model the other three should be held to. It detects exactly one claim, which is the right scope for what it was written to cover.

### Check 23: CORRECTLY WRITTEN, and its output clause is load-bearing

Check 23 requires that `render --check` "reports up to date" and that `validate --workflow` "exits 0 with `workflow invariants hold`". Both hold at `93ee357`, measured in the baseline above.

The output clause matters more than it looks, and round 1's triage already identified why: because `render --check` warns at exit 0, a round reading only exit codes passes a stale render, while check 23's stated expected OUTPUT fails it. Check 23 is therefore correctly specified. It would still have caught none of the twenty, because all twenty were consistently rendered.

### Verdict on (C)

Of the four checks this increment wrote to verify itself, ONE (22) is a real mechanical detector, ONE (23) is correctly specified and green, ONE (21b) is executable but narrow, and ONE (21) is the load-bearing check for the increment's entire purpose and is not executable as written over most of its own subject matter. Running all four as written would have caught AT MOST two of the twenty, and only if check 21's searcher counted occurrences rather than confirming existence.

## (D) What a mechanism would look like

Three separable mechanisms, in descending order of buildability. All are described, not built; building them is out of scope for this increment.

### Mechanism 1, `W6`: join a waiver note's stated figures against the round records. THE CHEAPEST AND THE MOST DIRECTLY EVIDENCED.

WHAT IT READS: `[[step.waiver]]` entries in the plan TOML, and `type:"round"` records in the JSONL log. Both are already parsed by `validate --workflow` in the same run.

WHAT IT COMPARES: the waiver `note`'s per-round breakdown against the `valid_findings` of the round records that join to the waived unit. The breakdown has a stable shape this project already uses at three sites: `<total> valid findings (<r1>, <r2>, ...)`, seen at `-w1` ("13 valid findings (3, 4, 6)"), `-w2` ("24 valid findings (9, 5, 6, 4)") and `-w3` ("14 valid findings (6, 4, 2, 0, 2)"). One regex over the note extracts the total and the sequence.

WHAT IT EXITS NON-ZERO ON: the parenthesised sequence not equal, element by element and in round order, to the `valid_findings` of the joined `type:"round"` records; or the stated total not equal to their sum. A note carrying no recognisable breakdown is NOT a failure, so the check is opt-in by writing one, which keeps it from flagging the notes that do not use the convention.

WHY IT IS WORTH BUILDING. `R1B-1` is precisely this defect, it survived three rounds, and the commit that fixed it (`80f3dc2`) says why in its own message: "A breakdown checks itself against the log; a bare total does not, which is how this figure stayed wrong through three rounds." That commit identified the mechanism and then implemented it as a human convention rather than as code. `M2` measures what the convention costs when nothing enforces it: zero gates move. The data is already in memory; this is a comparison, not a new source.

### Mechanism 2: make decision receipts joinable, then compare `chosen` against the plan

WHAT IT READS: `type:"decision"` records, and the `[[question]]` entries in the plan TOML.

WHAT IT COMPARES, IN TWO STAGES. Stage one, dangling-receipt detection: every `type:"decision"` record's `q_id` must name a registered `[[question]]` id, or a registered id plus a declared sub-id suffix. Today 29 of 51 distinct receipt `q_id`s name nothing in the plan (measurement in `R3C-3`), which is why `PC2` did not fire. Stage two, and only once stage one holds: require the deciding question's prose to contain the `chosen` option string verbatim, so a plan edited away from a recorded human choice stops matching its own receipt.

WHAT IT EXITS NON-ZERO ON: stage one, a receipt whose `q_id` resolves to no question; stage two, a decided question whose recorded `chosen` string does not appear in the question body.

HONEST LIMITATION, STATED BECAUSE IT DECIDES HOW MUCH THIS BUYS. Stage two is a string-presence check, not a semantic one. It would have caught `M2` only if the `-w1` note quoted its chosen option, which it does not. It WOULD catch the class where a plan is edited to contradict a receipt whose option strings are quoted in the plan, and it makes the reversal visible in a diff rather than invisible. Stage one is unambiguous and cheap and should not wait for stage two.

### Mechanism 3: a citation and quotation resolver, run as a check. THE ONE THAT ADDRESSES THE BULK, AND THE LEAST CERTAIN.

WHAT IT READS: the plan sources (`<task>.plan.toml` and the sidecars under `<task>.steps/` and `<task>.questions/`), and the repository files those sources cite.

WHAT IT COMPARES, in two independent halves that should ship separately because their false-positive profiles differ completely.

HALF A, QUOTATIONS, which is nearly free and nearly false-positive-free. Extract every backticked span longer than some threshold that is attributed to a named file, and run it as a literal search against that file. Report a quotation with no match. This is EXACTLY what check 21 already instructs a human to do, so it automates an existing decided procedure rather than inventing a policy. It would catch `R1A-4` and `M7` outright. Skip any span containing an ellipsis, which is `R1B-2`'s finding restated as an implementation note.

HALF B, CITATIONS, which needs a decision this review should not make. For an explicit `file:line` citation the check can verify the file exists and the range is in bounds, which catches only the crudest staleness. Verifying that "the named subject is there" requires knowing what the named subject IS, and for `file:Identifier` citations (51 of 73 in the main artifact) the tractable form is the reverse of what check 21 says: look the identifier up in the file and report when it is ABSENT, ignoring line numbers entirely. That is a real check and it is cheap. Verifying that a line-numbered range still holds a named subject is not tractable without either parsing Rust or requiring citations to carry their subject, and I would not put that in a plan step without a decision on which.

WHAT IT EXITS NON-ZERO ON: half A, a quotation attributed to a file with no literal match in it; half B, a `file:Identifier` citation whose identifier does not appear in the named file.

SCOPE WARNING WORTH CARRYING INTO THE STEP. Run against this repository today, half B would immediately go red on the roughly eleven `src/checks.rs` citations in `checks-runner-worktree-name-collision.md` that `Q-55-check21b` DELIBERATELY left stale. So the mechanism needs a declared suppression, per file or per citation, before it can be turned on. That is a design question, not a blocker, but it is the reason this mechanism is third rather than first.

## The findings

### `R3C-1` (medium): no mechanical gate in this repository can detect the defect class that produced every one of this increment's twenty findings

EVIDENCE: the classification table in (A), the seven mutations in (B), and the two positive controls. CATCHABLE 0, NOT CATCHABLE 20. Six of seven mutations, drawn from the classes rounds 1 and 2 actually found, left all six gates green and silent; `PC1` proves the rig detects W3 violations at exit 1, so the null results are measurements.

WHY MEDIUM AND NOT HIGHER. Nothing behavioural is wrong. Nothing reaches a user of the tool. The class IS being caught, reliably and at volume, by adversarial human and agent review, which is the mechanism that found all twenty. The cost is that catching them costs review rounds, and this step has now spent eleven reviewer passes and three triages on one documentation increment. WHY NOT LOWER: this is the project's own most productive defect class by a wide margin, and the ledger's recorded transferable lesson from inc3 is that the highest-yield question available is whether the suite would notice if the artifact were wrong. Asked of this artifact, the answer is no, on every class.

WHY IT DOES NOT FALSIFY THE STEP'S THESIS, stated plainly because I was asked to rule either way. The backstop `pack/AGENTS.md:93` promises is about the required reviewed rounds having happened before a step is marked complete, and `PC1` shows that promise being kept at exit 1. The step never claimed a deterministic check on the truth of prose. Reporting this as "the step's own thesis is refuted" would be the more dramatic claim and it is not supported.

DISPOSITION: A NEW BACKLOG STEP, not a fix to this increment. Nothing here is a defect the inc4 implementer introduced or could have prevented within its scope, and the remedy is new mechanism, which `Q-55-currencyscope` closed this increment against.

### `R3C-2` (medium): a waiver note's per-round figures are checkable against records the same command already reads, and nothing checks them

EVIDENCE: `M2` above, green on all six gates. `src/workflow.rs:544-621` (`w5_problems`) reads `waiver.step`, `waiver.unit`, `waiver.increment`, `waiver.evidence`, `waiver.evidence_tier` and `waiver.reason`, and never `waiver.note`. The refuting records are lines 246 to 248 of the log the same invocation parses, carrying `valid_findings` 3, 4 and 6.

WHY MEDIUM. A waiver is how a step is exempted from the round-count requirement, and its `note` is the human-readable justification a later reader uses to judge whether the exemption was earned. `R1B-1` shows the quantitative content of that justification can be wrong by 54 percent and survive three review rounds. Not higher: the note is not load-bearing for any exit code, and the structured fields that ARE load-bearing (`reason`, `evidence_tier`, the escalation join) are all checked by W5 today.

DISPOSITION: NEW BACKLOG STEP. Mechanism 1 in (D) is the concrete form.

### `R3C-3` (medium): decision receipts are write-only for 29 of 51 distinct `q_id`s, and no check compares a recorded human choice against the plan

EVIDENCE, in three parts.

Part one, `PC2`: deleting the `Q-55-w1figure` receipt outright leaves `validate --workflow` at exit 0. W4 does not fire because `Q-55-w1figure` is not a registered `[[question]]`.

Part two, the measurement:

```
distinct q_ids in decision receipts: 51
registered [[question]] ids in the plan: 69
receipts whose q_id is NOT a registered question: 29
```

The 29 include every sub-question decision this step recorded: `Q-55-currencyscope`, `Q-55-twinsites`, `Q-55-receiptcurrency`, `Q-55-w1figure`, `Q-55-check21b`, `Q-55-spectime`, `Q-55-impactlist`, `Q-55-existsgate`, `Q-55-jsonreason`, `Q-55-refusalscope` and the rest. W4 requires a receipt for every decided registered question past `[meta].w4_baseline = "Q-44"`; it has nothing to say about an id it cannot resolve.

Part three, `M2` and `M6`: with the receipt present and untouched, the plan can be edited to the option the human REJECTED, in both directions (restoring rejected text, and deleting ordered text), with all six gates green.

WHY MEDIUM. This is the closest analogue in this artifact to inc3's recorded transferable lesson, where a decided human choice could be silently reversed with the suite unable to detect it. Here the analogue holds on the documentation substrate: a decision receipt is presented by `pack/AGENTS.md:145` as "auditable evidence the human-input contract was met", and for 29 of 51 ids it is an unjoined record that no check reads and whose deletion is invisible. Not higher: the receipts ARE written, they ARE in an append-only log, and a human auditing by hand can reconstruct everything. The defect is that nothing mechanical does.

DISPOSITION: NEW BACKLOG STEP. Mechanism 2 in (D), stage one first. Stage one is unambiguous and cheap. I flag that stage one turning red on 29 existing receipts means the step needs to decide whether sub-ids get registered as questions or get a declared sub-id convention, and that is a human decision, not an implementer's.

### `R3C-4` (medium): acceptance check 21 is not executable as written over most of its own subject, and its scope excludes three of the twenty

EVIDENCE: the three defects set out in (C). The measurement, over `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`: 22 citations carry an explicit `file:line` or `file:line-line`; 51 are symbolic `file:Identifier` with no line number at all. Check 21's procedure ("open each `file:line` citation at the cited range") has no cited range for 51 of 73. Its scope ("THIS FILE") excludes `docs/plans/agent-scaffold.plan.toml:1732`, `:1728` and `:1713-1736`, where `R2A-1`/`R2C-1`, `R2A-3`/`R2C-2` and `R1C-5`'s twins live, all of which this increment edited.

WHY MEDIUM. Check 21 is the acceptance criterion for the increment's entire stated purpose. As written it is unfalsifiable over seventy percent of the citations it claims to cover and silent about plan sources the increment changed, which is the "a check that cannot fail is worse than no check" condition. Not higher: no behaviour is wrong, and the surface the check does not reach was in practice covered by reviewers.

DISPOSITION: FIX IN THIS INCREMENT, and it is the one finding here that belongs to inc4 rather than to the project. Check 21 is text this increment authored. The minimal true form is a narrowing to what the check can actually do: say that line-numbered citations are opened at their range, that symbolic citations are verified by the identifier existing in the named file, and either extend the scope to the plan TOML sites the increment edited or state that they are covered elsewhere. I note the standing risk recorded as orchestrator defect (18) at ledger `:555`: the last narrowing of an acceptance check in this increment was authored on an unchecked premise and produced a `medium` finding. Any narrowing here should be verified against the file before it is written, and the deletion-class form should be preferred to an authored one.

### `R3C-5` (low): the project's one hard documentation gate was run by none of the eight inc4 reviewer and triager passes

EVIDENCE:

```
$ grep -c -- '--strict' docs/plans/agent-scaffold.reviews/workflow-enforcement-tier-inc4-*.md
workflow-enforcement-tier-inc4-r1-authored-sonnet.md:0
workflow-enforcement-tier-inc4-r1-citations-opus.md:0
workflow-enforcement-tier-inc4-r1-completeness-opus.md:0
workflow-enforcement-tier-inc4-r1-triage.md:0
workflow-enforcement-tier-inc4-r2-coldread-opus.md:0
workflow-enforcement-tier-inc4-r2-rendered-sonnet.md:0
workflow-enforcement-tier-inc4-r2-residue-opus.md:0
workflow-enforcement-tier-inc4-r2-triage.md:0
```

Neither triage's "Mechanical gates" section runs `render --check --strict` or `agent-scaffold checks`, and no reviewer file mentions either. Both triages ran the warning form, whose exit code is 0 whether or not the view is stale (`M1` un-rendered, and `M5`, both measured above).

WHY LOW, AND WHY IT IS NOT A RE-RAISE. Round 1's triage recorded the underlying property in dismissing `R1B-3`, and I am not re-raising `R1B-3`, whose claim (that the gates exercise nothing inc4 changed) was correctly dismissed. The new observation is narrower: the correctly-specified form exists, is declared in `.agents/checks.toml:18`, works end to end, and was run by nobody. In practice no harm resulted, because both triages pasted the "up to date" OUTPUT text rather than relying on the exit code alone, which is exactly the safeguard check 23 was written to provide. That is why this is `low` and not higher.

DISPOSITION: PROCESS NOTE for the orchestrator's gate transcript convention, not a fix to the increment.

## What this review varied, and what it held fixed

VARIED: mutation site (step sidecar, sibling step sidecar, plan TOML waiver note, plan TOML question ask, generated view); mutation class (citation line, figure, tense, exit code, quotation, deletion of ordered content, hand-edit of the generated file); render state (mutated-and-re-rendered against mutated-and-not-re-rendered); gate form (`render --check` against `render --check --strict`, plus `agent-scaffold checks` with and without the binary on `PATH`); receipt state (present, deleted); step status (`not-started`, `complete`).

HELD FIXED, so a defect here survives this review. One platform (Linux, local filesystem), one build profile (debug), one binary (built at `93ee357`), uid 1000 only; I ran nothing under `unshare -Ur`. I ran no concurrency and no TOCTOU case. I rebuilt no historical binary, so every "before incN this printed ..." clause in the artifact is unverified by me. I did NOT re-derive the citation sweep or the completeness sweep, and I raised no new false-sentence finding of my own: that was the other two reviewers' lens this round, and a false sentence neither of them found survives this review too. My classification of the twenty rests on the two triage files' adjudicated descriptions plus my own reading of the gate implementations; I re-measured the classes by mutation rather than re-litigating each finding individually.

RESIDUALS AND SETTLED FINDINGS CHECKED BEFORE WRITING. I checked all five findings against inc2's four recorded residuals (the in-root bound; the single-anchor `..` case; `ADV-2`'s rejected-ledger context slot; the inc2-era `R2A-2`'s off-convention `--source` surface), against inc3's four (`R3A-1`'s inert remedy clause; `R4A-1`'s reader-level discrimination; the plain-`validate` mode-000 file versus unsearchable directory inconsistency; the containment TOCTOU), and against the five settled dismissals (`R1A-5`, `R1A-7`, `R1A-8`, `R1B-3`, `R2A-5`). NONE OF THE FIVE IS A RE-RAISE. `R3C-5` is adjacent to `R1B-3` and its relationship to that dismissal is argued explicitly in its own entry.

## Mutation cleanliness

Every mutation was reverted. Each diff was saved before reverting, to `<scratchpad>/rev-inc4-r3-c/patches/`, so no measurement is unrecoverable: `M1.patch`, `M2.patch`, `M3.patch`, `M4.patch`, `M5.patch`, `M6.patch`, `M7.patch`, `PC1.patch`, and `workflow.jsonl.orig` (the byte copy `PC2` was restored from).

```
$ git -C /home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-inc4-r3-c status --short
$ git -C /home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-inc4-r3-c rev-parse HEAD
93ee35706364a7367a7f80f395cc03e9ab8633a8
```

`status --short` prints nothing: the tree carries no mutation. The only file this review adds is this findings file. No chmod was used at any point, so none is owed a restore. Nothing outside `<scratchpad>/rev-inc4-r3-c/` was created, moved or deleted.
