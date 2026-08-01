# Work review, round 1, `workflow-enforcement-tier-inc1`, reviewer: adversarial correctness

Commit under review: `f491c4e` (`fix: anchor the metrics log and the ledger to the plan source`), against base `69c0525`. Worktree `.claude/worktrees/wr-inc1-a`, branch `wr/inc1-a`.

Lens: attack the derivation. Everything below was RUN, not read off the diff. Every fixture log carries a DISTINCT record count, so the count printed by `status`/`next`/`validate` identifies the file actually read rather than being asserted from the path.

## Summary verdict

The derivation is SOUND on every layout I could construct. 51 attacks, 3 findings, all of them documentation or coverage; ZERO mechanism defects. The one wrong-answer case I reproduced (the `..` escape) is the disclosed one, and I rule it NOT a finding against the implementation: it is inherent to a decision the plan settles, it is not a regression, the obvious in-increment fix is unsound, and inc2's guard closes it. What IS a finding is that the code says the opposite, in the doc comment of the very function under review.

Severity counts: 0 critical, 0 high, 1 medium, 2 low.

Because the increment is `risky` and needs two consecutive clean rounds, this round is NOT clean: three findings stand. All three are cheap prose or test edits; none touches the mechanism.

Claims I re-established rather than trusted:

- 9 new tests, 7 red pre-change and 2 passing as pins. VERIFIED by building `69c0525` in a separate clone outside any repository, copying in the new test file, and running it: `test result: FAILED. 2 passed; 7 failed`. The 2 that pass are `the_correct_case_prints_the_same_relative_paths_it_always_did` and `a_bare_filename_from_inside_docs_plans_stays_a_silent_miss`, which is what the file claims for them.
- 395 tests passing. VERIFIED: `cargo test` with `TMPDIR=/tmp/wr-a-scratch` sums to `total passed: 395 | non-ok suites: 0`, exit 0.
- Acceptance check 9, byte-identity on the correct case. VERIFIED by running the PRE-change and POST-change binaries from the worktree root on `validate --source docs/plans/agent-scaffold.plan.toml --workflow` and diffing: stdout IDENTICAL, stderr IDENTICAL, both exit 0.
- `cargo clippy --all-targets -- -D warnings` clean; `render docs/plans/agent-scaffold.plan.toml --check` prints `up to date`.
- Scope: the diff adds no `canonicalize` (0 occurrences added), no `problems.push` (0 added), no new `exit(` (0 added), and no `.git` access. It does not do part of inc2's job, and `project_root_of_source(&Path)` is exactly the signature inc2 needs to call on a canonicalised source, so inc2's mechanism is enabled rather than foreclosed.

## Findings

| id | severity | file | one-line |
| --- | --- | --- | --- |
| `W1A-1` | medium | `src/main.rs:1168-1170` | The doc comment claims a `..` component still finds "the real `docs/plans` above it"; measured false for a `..` that escapes a `docs/plans`, which is the exact case that produces a self-concealing false green. |
| `W1A-2` | low | `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:162` | The same claim in the plan sidecar, which is the authority inc2's implementer will read; it presents an unmeasured case as measured. |
| `W1A-3` | low | `tests/metrics_and_ledger_anchor_to_the_plan_source.rs:19-22` | The module doc claims four pins including "the two no-anchor cases"; the file contains ONE no-anchor run, and the ledger's no-anchor default, which inc1 specifies, is pinned by nothing in the suite. |

## `W1A-1` (medium): the doc comment blesses the `..` case that produces a false green

`src/main.rs:1165-1173`, in `project_root_of_source`'s doc comment:

```
/// LEXICAL is a deliberate choice, not an omission. The derived path keeps the spelling
/// the caller typed, so a relative `--source` yields a relative log path and the printed
/// output on a correct run is byte-identical to what it was before anchoring; a
/// canonicalising rule would turn every printed path absolute and machine-specific. It
/// also means a `..` component is skipped rather than followed (`Path::file_name` is
/// `None` for it) and the real `docs/plans` above it still matches.
```

The final clause is false as written. The walk skips a `..`, correct; what it then matches is whatever `docs/plans` is lexically above the `..`, which is not "the real `docs/plans`" when the `..` climbs out through one.

Measured. Fixture: `trap/other/p.plan.toml` is a plan whose single step `borrowed-step` is `complete`; its own log at `trap/other/docs/metrics/workflow.jsonl` has 14 records and NO round for that slug; an unrelated project's log at `trap/docs/metrics/workflow.jsonl` has 13 records including a converged round for it. Both runs are made from a third directory and name THE SAME FILE.

Correct spelling:

```
$ agent-scaffold validate --source $F/trap/other/p.plan.toml --workflow
/tmp/.../trap/other/p.plan.toml vs /tmp/.../trap/other/docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records and no covering waiver; ...
exit=1
```

The `..` spelling of the same file:

```
$ agent-scaffold validate --source $F/trap/docs/plans/../../other/p.plan.toml --workflow
/tmp/.../trap/docs/metrics/workflow.jsonl: 13 records, valid
/tmp/.../trap/docs/plans/../../other/p.plan.toml: 1 steps, 0 questions, valid
/tmp/.../trap/docs/plans/../../other/p.plan.toml vs /tmp/.../trap/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

`next` fabricates on the same input. With the step moved to `in-progress`, the correct spelling gives `metrics: 14 records`, `state: awaiting-first-review`, `next: spawn a reviewer for the first review round`; the `..` spelling gives `metrics: 13 records`, `state: converged`, `streak: 1/1`, `rounds: 1/5`, `next: mark the step complete, re-render, and commit`, at exit 0. That is verbatim the output the step file names at line 186 as "the specific output the fix must make unreachable".

WHY THIS IS A FINDING AGAINST THE COMMENT AND NOT AGAINST THE MECHANISM. I rule the behaviour acceptable for inc1, on four grounds I checked rather than accepted:

1. It is inherent to a settled decision. The step file at line 158 fixes the rule as "purely LEXICAL ... No filesystem access and no canonicalisation". A lexical rule that does not normalise cannot distinguish these two spellings; that is what lexical means.
2. The obvious in-increment fix is unsound, as the implementer argues. Lexical `..` normalisation is wrong across symlinks (`a/b/../c` is not `a/c` when `b` is a symlink), so "just collapse the `..` first" would trade a narrow wrong answer for a broader one.
3. It is not a regression. MEASURED: against the pre-change binary, BOTH spellings print `docs/metrics/workflow.jsonl: 3 records, valid` and `workflow invariants hold` at exit 0, reading the CWD's log. Post-change one of the two is fixed and the other reads a different wrong log. No input got worse.
4. inc2 closes it. The guard's root comes from the source's canonicalised location, which for the trap spelling is `trap/other`, while the resolved log canonicalises to `trap/docs/metrics/workflow.jsonl`, which is not under it, so the predicate fires. The implementer's argument (c) checks out.

The comment is the defect because it tells a later reader the opposite: that `..` is handled and the lexical rule finds the right directory anyway. A reader who believes it has a reason to think the inc2 guard is redundant for this case, and the case is precisely the self-concealing kind that the risk classification exists for.

MINIMAL CORRECTION: keep the mechanical half, qualify the guarantee. For example, replace "and the real `docs/plans` above it still matches" with a statement that a `..` which stays below the project's own `docs/plans` still resolves to that project, while a `..` that climbs OUT through a `docs/plans` matches that directory instead of the file's real location, which inc2's canonical guard rejects. No code change is owed.

## `W1A-2` (low): the plan sidecar carries the same over-broad claim

`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:162`:

```
The `docs/plans` convention resolves correctly in every spelling constructed: absolute, relative, `./`-prefixed, and with a `..` inside the path (that last works because `Path::file_name` returns `None` for a `..` component, so the walk skips past it and still finds the real `docs/plans` above).
```

RULING ON THE QUESTION ASKED: YES, the claim is too broad as written. It generalises from the spellings explorer A actually constructed to all spellings containing a `..`. The parenthetical states the mechanism accurately and then draws a conclusion the mechanism does not support: `Path::file_name` returning `None` for `..` is exactly WHY the walk can match a `docs/plans` the file does not live under. The sentence's own framing ("in every spelling constructed") is the tell; it is a report of a case matrix, and the escaping `..` was not in it.

The wider sentence this sits in is a claim about what was MEASURED, so an unmeasured case presented inside it inherits a weight it was never given. That matters for inc2, whose implementer reads this file as the authority on what the derivation already handles.

MINIMAL CORRECTION, one clause and one added sentence, no restructuring:

- Narrow the parenthetical to the case measured: a `..` that stays inside the project below its `docs/plans` (for example `docs/plans/sub/../p.plan.toml`), because the walk skips the `..` and reaches that project's own `docs/plans` above it.
- Add that a `..` which climbs OUT through a `docs/plans` matches THAT directory and derives the wrong project's root, so the same file spelled two ways gives two answers; note that this is a consequence of the lexical rule rather than a defect in it, that it is not a regression (both spellings were wrong before), and that inc2's canonical guard is what closes it.

Nothing else in the file needs to move. The judgement recorded at the end of the same paragraph (nearest-wins) is already correctly labelled as a judgement; this case should get the same treatment rather than staying inside the measured list.

## `W1A-3` (low): the test module overstates its own pin coverage, and the ledger's no-anchor default is unpinned

`tests/metrics_and_ledger_anchor_to_the_plan_source.rs:19-22`:

```
//! Four of the tests are pins rather than red-then-green cases, marked as such on each:
//! the correct-case no-regression check, the two no-anchor cases, and the accepted-cost
//! bare-filename miss. They pass identically before and after the change, which is the
//! property they exist to hold.
```

There is ONE no-anchor case in the file, not two. Counted mechanically: runs in the file that pass neither `--source` nor `--plan`:

```
$ grep -n 'run(&[a-z_]*, &\[' tests/metrics_and_ledger_anchor_to_the_plan_source.rs | grep -v -- '--source' | grep -v -- '--plan'
434:	let (code, stdout, stderr) = run(&home, &["validate"]);
```

One line. The fourth pin the module doc is counting is the from-its-own-root run inside `a_source_with_no_docs_plans_ancestor_falls_back_to_its_own_directory`, which is the no-CONVENTION case, not a no-ANCHOR case; that test passes `--source` on both of its runs. The two are different rules with different fallbacks (`project_root_of_source`'s final `parent.to_path_buf()` versus `resolve_metrics_path`'s `None` arm), so conflating them in the enumeration hides which one is pinned.

The coverage consequence is real, not just labelling. `default_ledger_path`'s no-anchor arm, which inc1 specifies explicitly ("with NEITHER, the ledger keeps today's `docs/plans/<task>.ledger.md`", step file line 274), is asserted by nothing:

```
$ grep -rn 'task.ledger.md' tests/ src/
(no output)
$ grep -c -- '"--resume"' tests/metrics_and_ledger_anchor_to_the_plan_source.rs
1
```

The single `--resume` run passes `--source`. No other test file drives `status` or `next` through the binary at all (`tests/` holds `audit_command.rs`, `checks_missing_tmpdir.rs`, `checks_staged_hook_env.rs`, `scaffold_precommit_hook.rs`, and the two `validate_*` files). So an edit to `default_ledger_path`'s `map_or_else` default arm fails nothing.

I verified the behaviour itself is CORRECT (attack L5 below: `status --resume` with no anchor prints `no ledger at docs/plans/task.ledger.md`, identical pre and post). This is a missing pin on correct behaviour plus a doc comment that says the pin is there.

MINIMAL CORRECTION: either add one assertion to `plain_validate_and_a_sourceless_run_keep_their_behaviour` covering `status --resume` with no anchor (the historical `docs/plans/<task>.ledger.md`), which makes the module doc's "two no-anchor cases" true as written, or fix the count to one and say which case it is. The first is better: it costs three lines and closes the gap the sentence claims is already closed.

## Attack enumeration

Every attack below was run against the post-change RELEASE binary, and the whole A-to-G battery was ALSO run against a pre-change binary built from `69c0525` in a clone outside any repository, so "not a regression" is measured rather than asserted. Unless stated, runs are made from `home`, a project whose own log has 3 RECORDS: any output reporting 3 records means the process working directory won.

Fixture record counts: home 3, away 11, flat 12, trap 13, trap/other 14 (then rebuilt to 14 with no `borrowed-step` evidence), nested outer 15, nested inner 16, deep root 17, deep leaf 18, docsonly root 19, docsonly inner 20, notplans root 21, notplans inner 22, plansnotdocs root 23, plansnotdocs inner 24, symplans 25, symsrc real 26, symsrc outside 27, caseplans root 28, caseplans inner 29, non-UTF8 project 33.

### A. Path spellings (14 attacks, 1 wrong answer, that one disclosed)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| A1 | absolute `--source $F/away/docs/plans/p.plan.toml` | 11 | 11 | PASS (pre-change: 3) |
| A2 | relative `../away/docs/plans/p.plan.toml` | 11 | 11 | PASS (pre-change: 3) |
| A3 | `./docs/plans/p.plan.toml` from `away` | 11 | 11 | PASS |
| A4 | bare relative `docs/plans/p.plan.toml` from `away` | 11 | 11 | PASS |
| A5 | doubled separators `$F/away//docs//plans//p.plan.toml` | 11 | 11 | PASS |
| A6 | interior `./` segments `$F/./away/./docs/plans/p.plan.toml` | 11 | 11 | PASS |
| A7 | `..` BELOW `docs/plans`: `$F/away/docs/plans/sub/../p.plan.toml` | 11 | 11 | PASS (this is the spelling the sidecar's claim actually covers) |
| A8 | `..` ABOVE `docs/plans`: `$F/away/sub/../docs/plans/p.plan.toml` | 11 | 11 | PASS |
| A9 | `..` ESCAPING a `docs/plans`: `$F/trap/docs/plans/../../other/p.plan.toml` | 14 | 13 | WRONG FILE, the disclosed case, ruled under `W1A-1` |
| A10 | the same file spelled directly `$F/trap/other/p.plan.toml` | 14 | 14 | PASS (the pair A9/A10 is the demonstration) |
| A11 | bare filename from inside `docs/plans` | no log found | no log found | PASS, accepted cost (i) pinned |
| A12 | bare filename at a conventionless root (`flat`) | 12 | 12 | PASS |
| A13 | `./p.plan.toml` from inside `docs/plans` | no log found | no log found | PASS, same accepted cost |
| A14 | `../plans/p.plan.toml` run from `docs/metrics` | no log found | no log found | PASS, see the residual note below |

### B. Layout edge cases (7 attacks, 0 wrong answers)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| B1 | no `docs/plans` ancestor anywhere (plan at a bare root) | 12, via the fallback | 12 | PASS (`Q-55-noconvention`) |
| B2 | `plans/` NOT under `docs/` | 24, fallback to the source's own dir | 24 | PASS: no match, so no root derivation |
| B3 | `docs/notplans/` | 22, fallback | 22 | PASS |
| B4 | nested `docs/plans` inside another `docs/plans` | 16, nearest-wins inner | 16 | PASS, and correctly labelled a judgement in code and test |
| B5 | source directly in `docs/`, not `docs/plans/` | 20, fallback | 20 | PASS |
| B6 | source four levels below `docs/plans` (`a/b/c/`) | 17, the project root | 17 | PASS: the walk climbs past `c`, `b`, `a` and stops at the first `docs/plans` |
| B7 | case-variant `docs/Plans/` | 29, fallback | 29 | PASS: match is case-sensitive, correct on this platform |

### C. Symlinks (3 attacks, 0 wrong answers for inc1's remit)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| C1 | `<root>/docs/plans` is a SYMLINK to `<root>/elsewhere` | 25, the spelled root's own log | 25 | PASS. This is the layout that becomes accepted cost (ii) once inc2's canonical guard lands; inc1 reads it correctly, which is exactly the lexical/canonical disagreement the plan describes. |
| C2 | the same project via the REAL path `<root>/elsewhere/p.plan.toml` | fallback, log not found | no log found | PASS per the lexical rule (pre-change: 3, also wrong). Not a regression. |
| C3 | a SYMLINK to a project's plan, placed in another project | 27, the symlink's neighbours | 27 | Expected: this is explorer A's second false pass, assigned to inc2 by acceptance check 12. NOT an inc1 finding. |

### D. Explicit `--metrics` must be honoured verbatim (4 attacks, 0 wrong answers)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| D1 | explicit absolute foreign log | 13, the named file | 13 | PASS, identical pre and post |
| D2 | explicit RELATIVE `docs/metrics/workflow.jsonl` with a foreign `--source` | 3, CWD-relative as typed | 3 | PASS. This is the false pass that survives inc1 by design (step file line 176) and is check 11's red for inc2. |
| D3 | explicit with a `..` component | 12, the named file | 12 | PASS |
| D4 | explicit with NO `--source` at all | 11, the named file | 11 | PASS |

### E. The Markdown `--plan` substrate (3 attacks, 1 wrong answer, the same disclosed one)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| E1 | `--plan $F/away/docs/plans/p.md` only | 11 | 11 | PASS: one rule covers both substrates, as the plan requires |
| E2 | `--source` (flat) AND `--plan` (away) together | 12, source wins | 12 | PASS: matches `next::derive_task`'s source-then-plan order at `src/next.rs:997-999` |
| E3 | the `..` trap spelled on `--plan` | 14-equivalent | 13 | Same defect on the other substrate, same ruling. Confirms it is a property of the derivation, not of the TOML path. |

### F. The no-anchor case must be unchanged (3 attacks, 0 wrong answers)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| F1 | `status` with neither flag | 3, CWD-relative | 3 | PASS, identical pre and post |
| F2 | `next` with neither flag | 3 | 3 | PASS, identical pre and post |
| F3 | `validate` with neither flag | 3 | 3 | PASS, identical pre and post, and pinned by the suite |

### G. Missing, unreadable, and wrong-type inputs (7 attacks, 0 findings)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| G1 | `--source` names a file that does not exist | anchor still derives from the spelling; exit 0 | 11, exit 0 | PASS. `status` is best-effort; the plan half is empty and the metrics half is the derived project's. Better than pre-change (3). |
| G2 | `--source` is a DIRECTORY | hard error | `IsADirectory`, exit 1 | PASS, identical pre and post |
| G3 | `--source` is `chmod 000` | hard error | exit 1 | PASS, identical pre and post |
| G4 | the ANCHORED log path is a DIRECTORY | IO error propagates | exit 1 (pre-change from this input: exit 0) | NOT A FINDING, verified pre-existing: running the PRE-change binary from a directory whose own `docs/metrics/workflow.jsonl` is a directory also gives `IsADirectory`, exit 1, on both `status` and `next`. The `?` propagation is unchanged; only which path can reach it moved. No new refusal mechanism is introduced. |
| G5 | `--source ""` | rejected | clap usage error, exit 2 | PASS. No empty path reaches the derivation from the CLI, so the `sidecar-ref-empty-string` family does not apply here. |
| G6 | `--source .` | hard error | exit 1 | PASS |
| G7 | `--source ..` | hard error | exit 1 | PASS |

### H. Further edge cases (6 attacks, 0 wrong answers)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| H1 | a NON-UTF8 directory component above `docs/plans` | 33 | 33 | PASS. `to_str()` returning `None` for the invalid component cannot make `docs`/`plans` mismatch, and the derived root keeps the invalid bytes. |
| H2 | `--source` is a BROKEN symlink | anchor from the spelling, exit 0 | 11, `plan: not provided`, exit 0 | PASS |
| H3 | `--metrics ""` | rejected | clap usage error | PASS |
| H4 | many `..` that land back in the SAME project (`away/docs/plans/../../../away/docs/plans/p.plan.toml`) | 11 | 11 | PASS: nearest-wins picks the last `docs/plans`, which is the right one |
| H5 | absolute source, run from `/` | 11 | 11 | PASS: the derivation is independent of the CWD when the source is absolute |
| H6 | `--plan` given a `.plan.toml` filename (task derivation) | AWAY ledger | AWAY | PASS |

### L. Ledger resolution (14 attacks, 0 wrong answers)

All run from `home`, whose ledger says `HOME resume state.`; a leak is identifiable by content.

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| L1 | `status --resume --source <away>` | AWAY | AWAY | PASS (pre-change: HOME, the leak) |
| L2 | `next --source <away>` | AWAY | AWAY | PASS (pre-change: HOME) |
| L3 | explicit `--ledger-fragment` at HOME's ledger | HOME, verbatim | HOME | PASS: explicit wins, identical pre and post |
| L4 | the `..` TRAP spelling | TRAPOTHER, the right ledger | TRAPOTHER | PASS, and NOTABLE: the ledger rule is IMMUNE to the trap that catches the metrics rule, because it joins the spelling and lets the OS resolve the `..` rather than walking it. This is direct evidence for the plan's claim (line 136) that the ledger rule has fewer cases that can go wrong. |
| L5 | no anchor at all | `docs/plans/task.ledger.md` | `no ledger at docs/plans/task.ledger.md` | PASS, identical pre and post. This is the case `W1A-3` says nothing pins. |
| L6 | `--plan` anchor | AWAY | AWAY | PASS (pre-change: HOME) |
| L7 | conventionless root (`flat`) | FLAT | FLAT | PASS (pre-change: `no ledger at docs/plans/myplan.ledger.md`) |
| L8 | bare filename from inside `docs/plans` | AWAY | AWAY | PASS. This is explorer A's `docs/plans/docs/plans/...` case, now correct: the ledger, unlike the metrics log, works from inside `docs/plans`. |
| L9 | symlinked source in another project | the symlink's neighbour | SYMSRCOUTSIDE | Expected under the lexical rule; see the residual note below. |
| L10 | symlinked `docs/plans` directory | SYMPLANS | SYMPLANS | PASS |
| L11 | `next` with no anchor | no block | no block | PASS |
| L12 | `next --ledger-fragment` explicit | HOME, verbatim | HOME | PASS |
| L13 | nonexistent source | the note names the ledger beside the source | `no ledger at <away>/docs/plans/nope.ledger.md` | PASS |
| L14 | `--source` and `--plan` both given | source wins, FLAT | FLAT | PASS |

### T. The disclosed trap, worked as a false pass (4 attacks)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| T1 | `validate --workflow`, correct spelling, log without the slug's evidence | exit 1, W3 red | exit 1, W3 red naming the right log | PASS |
| T2 | the SAME file, `..` spelling | exit 1 | `workflow invariants hold`, exit 0 | FALSE PASS, `W1A-1`'s evidence |
| T3 | `next`, correct spelling, step `in-progress` | `awaiting-first-review` | `metrics: 14 records`, `state: awaiting-first-review`, `next: spawn a reviewer...` | PASS |
| T4 | `next`, `..` spelling | as T3 | `metrics: 13 records`, `state: converged`, `streak: 1/1`, `next: mark the step complete, re-render, and commit` | FABRICATION, `W1A-1`'s evidence |
| T5 | both spellings against the PRE-change binary | both wrong | both `workflow invariants hold` at exit 0 from the CWD's 3-record log | Establishes NOT A REGRESSION as a measurement |

### J. Machine surfaces (4 attacks, 0 findings)

| id | input | expected | got | verdict |
| --- | --- | --- | --- | --- |
| J1 | `status --json --source <away>` | `"records": 11` | `"records": 11`, exit 0 | PASS: the JSON surface anchors identically to the human one |
| J2 | `status --json` on the trap spelling | 14 | 13 | Same defect, same ruling; recorded so it is known the machine surface carries it too |
| J3 | `next --json --source <away>` | metrics 11, resume block AWAY | `"records": 11`, `"resume_state": "...AWAY resume state."` | PASS |
| J4 | `next --json` with no anchor | 3 | 3 | PASS. `no_active_loop_reason` is still `#[serde(skip)]` and absent from the JSON, which is correct for inc1: `Q-55-jsonreason` is inc2's. |

### Help, README, CHANGELOG, and scope checks (negatives)

- The clap `[default: docs/metrics/workflow.jsonl]` marker is GONE from all three commands: `validate --help`, `status --help` and `next --help` each contain 0 occurrences of `default:`. The documentation-impact item at step-file line 342 is met.
- The three `--metrics` help strings, the two `--ledger-fragment` help strings and `StatusArgs::resume`'s help all describe the implemented rule accurately, including the no-anchor case. I checked each claim against a run; none is falsified.
- The README paragraph added at `README.md:226` states the rule, names the `--source`-then-`--plan` order, states that explicit values are verbatim, states the no-anchor fallback, states that the rule never consults `.git`, and names accepted cost (i) explicitly. Every one of those is true of the build. No finding.
- The CHANGELOG entry names the behaviour change, the four consequences, the unchanged correct case and the verbatim-explicit rule. Its claim that "a run made from the plan's own project root ... still prints the relative paths it always did" is the byte-identity property I verified. No finding.
- "The rule never consults `.git`": verified by inspection (the function makes no filesystem or process call at all) and by every fixture run above, all of which are outside any git repository.
- Only `ValidateArgs`, `StatusArgs` and `NextArgs` carry a `metrics` field (`src/main.rs:431`, `:457`, `:481`); `AuditArgs` has none, so no fourth call site was missed.
- `run_status` calls `run_resume` before any metrics resolution, so `--resume` still returns before serialisation; nothing in the diff gives `status --resume` a JSON surface it must not have.

### Residuals I noted and am NOT filing as findings

Recorded so the orchestrator has them, not as defects against this increment.

- A14 and its family: any RELATIVE spelling whose text does not literally contain `docs/plans` as adjacent components misses the project's real log (`cd docs/metrics && --source ../plans/p.plan.toml` looks for `../plans/docs/metrics/workflow.jsonl`). This is accepted cost (i) with a different spelling; the step file states that cost only in its bare-filename form, so its actual scope is a little wider than the text. Not a regression (the pre-change build missed all of these too), and inc2's containment guard structurally cannot catch it, as the step file already says of the bare-filename case. It becomes a hard failure after inc3, which the step file argues is the right outcome.
- L9: the DEFAULT ledger beside a SYMLINKED source resolves beside the symlink, so a foreign project's `## RESUME STATE` can still be echoed. Inc2 as specified covers only an EXPLICIT `--ledger-fragment` outside the root (step file line 183 says the default ledger case is "already closed by the anchoring in inc1"), so this residual survives both increments. It is strictly better than pre-change (which leaked the CWD's block for every foreign source, symlink or not) and it needs a deliberately symlinked layout to reach. Worth a line in inc2's review rather than a change here.
- `default_report_path` (`src/main.rs:1249`) still builds `docs/plans/<task>.code-value-report.md` relative to the CWD, so `audit --source <foreign plan>` writes its report into the CWD's tree. Deliberately outside inc1's scope (the step file's inc1 list names only `--metrics` and `default_ledger_path`), and it is an OUTPUT path rather than a read, so it produces no wrong answer. Recorded only so a later reader does not mistake it for an oversight in this diff.

### What I did not attack

- The containment predicate, the refusal, and the serialised reasons: inc2, explicitly out of scope.
- Windows path semantics: not a target platform for this run.
- Concurrent modification of the log between resolution and read: unchanged by this diff.
