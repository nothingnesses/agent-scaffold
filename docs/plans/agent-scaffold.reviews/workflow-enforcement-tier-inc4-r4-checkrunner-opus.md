# `workflow-enforcement-tier` inc4, round 4, check-runner lens

LENS: run the step's own acceptance-check list, all 33 entries, each from its own stated preconditions, and report PASS / FAIL / UNRUNNABLE plus whether the check is FALSIFIABLE. Nobody had run the whole list before this round.

HEADLINE: 33 checks, 33 run, 33 PASS, 0 FAIL, 0 UNRUNNABLE. Every RED half the list specifies was also run, against binaries built from the true pre-increment commits rather than from a merge parent. TWO checks are UNFALSIFIABLE in one clause each, and both are recorded below as findings at `low`. NO behavioural defect was found in any of the 33.

## Environment, so every command below is re-runnable verbatim

```sh
W=/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/rev-inc4-r4-a   # the tree under review, 7ab5d48
AS=$W/target/debug/agent-scaffold                                                   # the binary built from it
S=<scratchpad>/rev-inc4-r4-a                                                        # every fixture lives here
export TMPDIR=$S                                                                    # outside any git repository, per the check-list preamble
```

THE THREE HISTORICAL BINARIES, built for the RED halves. The parent of each increment's first code commit is the true baseline; a merge parent is not:

| binary | commit | is | built from |
| --- | --- | --- | --- |
| `$S/as-pre1` | `1dac3dc` | pre-inc1 | parent of `609ddcf` "fix: anchor the metrics log and the ledger to the plan source" |
| `$S/as-pre2` | `285a6a3` | after inc1, before inc2 | parent of `8beb1c2` "feat: refuse and omit on a round log or ledger the plan cannot vouch for" |
| `$S/as-pre3` | `5684b5f` | after inc2, before inc3 | parent of `6b1c847` "fix: fail validate --workflow when the round log is missing" |

A TRAP WORTH RECORDING FOR THE NEXT RUNNER, because it produced a confidently wrong binary here before it was caught. Extracting three revisions with `git archive` and building them into ONE shared `CARGO_TARGET_DIR` silently reused a stale fingerprint: `as-pre2` and `as-pre3` came out BYTE-IDENTICAL (`md5sum` equal), so every "before inc3" measurement would have been an "after inc2" measurement. The fix is one target directory per revision plus `find . -name '*.rs' -exec touch {} +` before the build. Verified afterwards: the three binaries have three distinct `md5sum`s, and `validate --help` shows three distinct `--metrics` help strings (the `[default:]` present only on `as-pre1`).

THE FIXTURES, all under `$S`:

- `fix`: the plain scaffold of check 2 (`example-step`, `not-started`, no `docs/metrics/`).
- `borrowed`, `borrowed_nolog`: `fix` with the single step's slug set to `triager-runs-only-on-findings` at `complete`, with and without an empty log of its own.
- `inprog`: the same at `in-progress`, which is what checks 14b and 14d require.
- `collide`: `inprog` with the plan renamed `agent-scaffold.plan.toml`, for check 7.
- `nocon`: a plan at a root with NO `docs/plans`, its own 3-record log, for check 8.
- `sym`: a symlink to `borrowed_nolog`'s plan with a full 296-record log beside the SYMLINK, for check 12.
- `A`, `B`, `B2`: the divergent-pairing pair of check 13b. `A` is MARKDOWN-primary with a real 296-record log and a `## RESUME STATE` block; `B` is a Markdown plan carrying the borrowed slug at `complete` with its Step Detail heading renamed; `B2` is `B` at `in progress`.
- `probe`: a real readable log for check 16's two `try_exists` `Err` spellings.
- `L1`, `L2`: check 19's two symlink layouts. `L3`: check 19b's `notes/p.md` layout.

This repository's own log carries 296 records today, not the 235 the step file records historically. The step file's 235 is a dated measurement and is written as one, so the difference is not a defect.

## The per-check table

RED = the pre-change half the check specifies, where it specifies one. FALS = is there a state of the tree in which this check fails.

| # | Verdict | RED half | FALS | One-line evidence |
| --- | --- | --- | --- | --- |
| 1 | PASS | n/a | PARTLY. See `R4A-1` | `cargo build` ok; `cargo test` exit 0 (all suites green); `cargo clippy --all-targets -- -D warnings` exit 0; `render --check` prints `up to date` |
| 2 | PASS | n/a | Yes | scaffold prints `(30 changed, 0 left untouched)`; `ls $S/fix/docs` prints only `plans` |
| 3 | PASS | RUN at `as-pre1` | Yes | at its stated "AFTER INC1" precondition (`as-pre2`) it is exactly as written: fixture's own missing-log note, exit 0. At `as-pre1` it read agent-scaffold's own 296-record log and printed `workflow invariants hold` at exit 0 |
| 4 | PASS | RUN at `as-pre1` | Yes | no green at HEAD; with a log of its own the correct RED (W3 naming `triager-runs-only-on-findings`), exit 1. RED: `workflow invariants hold`, exit 0 |
| 5 | PASS | RUN at `as-pre1` | Yes | HEAD prints `metrics: no log found` and `state: awaiting-first-review`; RED printed `metrics: 296 records`, `state: converged`, `next: mark the step complete, re-render, and commit` |
| 6 | PASS | RUN at `as-pre1` | Yes | HEAD: `metrics: no log found` and `metrics: 0 records` on the fixture with its own log; RED: `metrics: 296 records` |
| 7 | PASS | RUN at `as-pre1` | Yes | HEAD prints `no ledger at $S/collide/docs/plans/agent-scaffold.ledger.md; nothing to resume`, and `next` echoes zero `RESUME STATE` lines; RED printed this repository's whole block |
| 8 | PASS | n/a | Yes | the 3-record count identifies the file: `3 records, valid` both from that root and from the agent-scaffold root |
| 9 | PASS | RUN at `as-pre1` | Yes | `diff` of the three stdout lines against the pre-fix binary is EMPTY, exit 0 both sides |
| 10 | PASS | n/a | Yes | plain `validate --source` and bare `validate` both exit 0 with `no metrics log at docs/metrics/workflow.jsonl; nothing to validate` on stderr |
| 11 | PASS | RUN at `as-pre2` | Yes | HEAD exits 1 with the refusal naming source, log and root; RED printed `workflow invariants hold` at exit 0 |
| 12 | PASS | RUN at `as-pre2` | Yes | HEAD exits 1; RED joined the symlink's 296-record neighbour and printed `workflow invariants hold` at exit 0 |
| 13 | PASS | RUN at `as-pre2` | Yes | escaping `..` exits 1 with the refusal; `..` staying inside exits 1 with the CORRECT W3 result; RED greened the escape |
| 13b | PASS | RUN at `as-pre2` | Yes | all three runs. Refusal names B's plan, A's log, B's root; typo'd `--source` also exits 1; same-project TOML-primary pair exits 0. RED at the ANCHOR-ROOTED build printed `workflow invariants hold` at exit 0 |
| 14 | PASS | n/a | Yes | without `--workflow`, exit 0; `status` and `next` exit 0 on all six unsafe input shapes tried |
| 14b | PASS | RUN at `as-pre2` | Yes | no `state:`/`streak:`/`rounds:`/`next:`/`role:`/`prompt:`/`summary:`, no record count, reason names log and root, exit 0. RED printed the full converged block |
| 14c | PASS | RUN at `as-pre2` | Yes | all three runs: plan half intact with a reason in place of the count; explicit `--ledger-fragment` outside the root noted and omitted; DEFAULT ledger under the divergent pairing omitted. RED printed A's block verbatim |
| 14d | PASS | n/a | Yes | on the `in-progress` fixture the output is NOT the zero-rounds projection; check 5's genuinely-absent case DOES print `awaiting-first-review`, so the two are distinguished |
| 14e | PASS | RUN at `as-pre2` | Yes | `next --json`: `"active_loop": null` with `"no_active_loop_reason": "metrics-not-this-project"`, `"metrics": null` with `"metrics_absent_reason": "log-not-this-project"`; `status --json` the same reason. RED: neither field exists |
| 14f | PASS | n/a | Yes | four runs, four outcomes: `log-absent` WITH a derived `active_loop`; `log-not-this-project` + `metrics-not-this-project`; `no-plan-steps`; precedence run gives `log-not-this-project`, not `log-absent` |
| 14g | PASS | RUN at `as-pre2` | Yes | `ledger-absent`, `no-resume-section`, `ledger-not-this-project` (present and missing), plus the DEFAULT-ledger run under the divergent pairing. RED emitted `mark the step complete` and A's ledger path |
| 14h | PASS | RUN via `git diff` | Yes | correct run serialises all three new reasons as `null`; the `GOLDEN_JSON` diff across inc2 is EXACTLY the three added fields |
| 15 | PASS | RUN at `as-pre3` | Yes | exit 1, problem names the resolved log and says the check could not run. RED: `skipping the workflow check`, exit 0 |
| 16 | PASS | n/a (pins) | Yes | both `Err` spellings reproduce the recorded divergence exactly, and the mode-000-file residual reproduces exactly |
| 17 | PASS | n/a | Yes | empty log inside the borrowed-slug fixture: exit 1 with the quoted W3 message |
| 18 | PASS | RUN at `as-pre2` | Yes | after inc1 alone: miss note, exit 0. At HEAD: hard failure naming the path it looked for. The suite test the check asks for exists |
| 19 | PASS | RUN at `as-pre2` | Yes | BOTH layouts, BOTH surfaces: `validate` exits 1 with the refusal, `status` and `next` omit the metrics half at exit 0. RED read the log and greened |
| 19b | PASS | RUN at `as-pre2` | Yes | exit 1 at HEAD, `workflow invariants hold` at exit 0 before inc2; projections omit; `status --resume` omits in BOTH `primary` spellings |
| 20 | PASS | RUN via `git show` | Yes on the grep; the predict-clause is a judgement | the non-instrumented `AGENTS.md` now carries the qualifier and the refusal outcome; both drift guards pass. RED: the pre-inc3 sentence at `pack/AGENTS.md:93` was unconditional |
| 21 | PASS | n/a | PARTLY. See `R4A-2` | 13 distinct line citations opened and holding; 23 distinct identifier citations present; 69 quotations swept, and every one attributed to a tree file resolves at the revision its tense names |
| 21b | PASS | n/a | Yes | all four in-scope citations (`src/main.rs` x1 in each of two files, three `tests/` files) opened and holding their named subject |
| 22 | PASS | RUN via `git show` | Yes | measured: `status --json --source <toml-primary>` with no `--plan` serialises a populated `"plan"`; the doc comment now says "a TOML-primary `--source` or a readable `--plan`". RED: the pre-inc4 comment said "present only when a readable `--plan` was given" |
| 23 | PASS | n/a | Yes | `render --check` prints `up to date`; `validate --source docs/plans/agent-scaffold.plan.toml --workflow` exits 0 with `workflow invariants hold` |

TALLY: 33 checks. 33 run. 33 pass. 0 fail. 0 unrunnable. 2 carry an unfalsifiable clause (checks 1 and 21). 21 checks specify a pre-change half and all 21 were run.

## Findings

TWO, both `low`, both about a check's ability to fail rather than about the tree. NOTHING at `medium`, `high` or `critical` was found. I state that explicitly rather than manufacturing a finding to fill the range: 33 checks were run from their own preconditions with 21 pre-change comparisons, and the shipped behaviour matched the specification in every one.

### `R4A-1` (`low`): check 1's render clause cannot fail, because `render --check` without `--strict` exits 0 on a divergent tree

The acceptance-check preamble is explicit that the list is settled by exit codes: "Every claim below is a command with an expected exit code, so a round is settled by running it rather than by reading the diff". Check 1's fourth clause is "Plan render pinned: `cargo run -- render docs/plans/agent-scaffold.plan.toml --check`". Run under that preamble, that clause passes on ANY tree, including one whose rendered view has been hand-edited.

Demonstrated by mutation, in a COPY of `docs/plans` under the scratchpad so the tree under review was not touched:

```sh
$ cp -r $W/docs/plans $S/renderprobe/docs/plans && cd $S/renderprobe
$ $AS render docs/plans/agent-scaffold.plan.toml --check
docs/plans/agent-scaffold.plan.toml: up to date
exit=0

$ printf '\nMUTATION-PROBE-LINE\n' >> docs/plans/agent-scaffold.md

$ $AS render docs/plans/agent-scaffold.plan.toml --check
warning: docs/plans/agent-scaffold.md differs from a fresh render (a hand-edit, or a stale render after a source edit) (the committed file has 2042 line(s); a fresh render has 2040); re-render with `agent-scaffold render docs/plans/agent-scaffold.plan.toml`
exit=0

$ $AS render docs/plans/agent-scaffold.plan.toml --check --strict
error: docs/plans/agent-scaffold.md differs from a fresh render (a hand-edit, or a stale render after a source edit) (the committed file has 2042 line(s); a fresh render has 2040)
exit=1
```

THE ASYMMETRY INSIDE THIS FILE IS WHAT MAKES THIS CONCRETE RATHER THAN A MATTER OF TASTE. Check 23 names the SAME command and asserts its OUTPUT ("reports up to date"), so check 23 fails on the mutated tree when it is run as written. Check 1 names the command alone under an exit-code preamble, so it does not. Two clauses of one list, one falsifiable and one not, on identical machinery.

MINIMAL REMEDY: add `--strict` to check 1's render clause, or assert its output text the way check 23 does. `.agents/checks.toml:18` already declares the strict form as the project's own render gate, so the remedy copies an in-tree precedent rather than inventing one.

RELATION TO SETTLED GROUND, STATED PLAINLY SO THE TRIAGER CAN RULE FAST. Round 3's `R3C-5` recorded that the inc4 review passes ran the warning form rather than the strict one, was confirmed `low`, and was ruled OUT OF SCOPE with its remedy placed in the orchestrator's transcript convention, OUTSIDE the plan. This finding is about the check's own TEXT rather than about the reviewers' transcripts, and the new evidence is the mutation above, which the `R3C-5` triage did not run: that triage's stated safeguard was that the triages "pasted the `up to date` OUTPUT text rather than reading exit codes alone", which is a property of those readers and not of check 1. If the triager reads the round-3 ruling as covering the check text too, dismiss this as settled; I raise it because my brief requires reporting any check that would pass regardless, and this is one.

### `R4A-2` (`low`): check 21's quotation half cannot fail on a past-tense quotation

Check 21 says: "run each quoted fragment of source, test, `README.md` or `pack/AGENTS.md` text as a literal search against the file it is attributed to ... A quotation with no match in the tree is either RE-TENSED, so the sentence describes the pre-increment state it was written about, or DELETED where the sentence carries nothing else".

The procedure therefore has exactly two outcomes for a quotation that does not match the current tree: it is past-tense, in which case it is accepted, or it is present-tense, in which case it must be re-tensed or deleted. NOTHING in the procedure asks whether a past-tense quotation ever matched ANY revision. A fabricated or drifted historical quotation, which is precisely what a currency pass can author when it re-tenses a sentence it did not verify, passes check 21 as written by being written in the past tense.

This is not hypothetical for this increment. The step file's quotation surface is DOMINATED by historical quotations: of 69 distinct quoted fragments of 25 characters or more, 50 do not match the current tree, and the majority of those 50 are past-tense quotations of pre-increment source, of pre-increment doc comments, or of the superseded first planner pass. Check 21 reaches none of them.

I RAN THE MISSING HALF, so this finding is a gap in the check rather than a live defect. Every historical quotation I could attribute to a tree file resolves at the pre-increment revision whose state its tense names:

```
HIT   pre-inc2 285a6a3   src/next.rs   Why there is no active loop, for the human renderer. Not serialised (the JSON contract is exactly the fields above)
HIT   pre-inc2 285a6a3   src/next.rs   Every derived part is optional so a missing plan or log yields a partial projection rather than a failure (mirrors `status`'s `Projection`)
HIT   pre-inc2 285a6a3   src/main.rs   Every part is optional so a missing plan or metrics file yields a partial projection rather than a failure
HIT   pre-inc2 285a6a3   src/next.rs   or `None` when the ledger is absent or carries no such section
HIT   pre-inc2 285a6a3   src/next.rs   all steps complete, every pending step blocked, or no plan source
HIT   pre-inc2 285a6a3   src/main.rs   A missing ledger or absent section prints a note and exits 0
HIT   pre-inc3 5684b5f   src/main.rs   An absent file (the metrics log, or a `--plan` path) is not a validation failure
HIT   pre-inc3 5684b5f   src/main.rs   a missing file prints a note to stderr and is skipped rather than hard-failing (the same treatment for both, so the behaviour is consistent)
HIT   pre-inc3 5684b5f   tests/validate_workflow_toml_source_needs_no_plan.rs   with a source present but metrics missing the tool still soft-skips
HIT   pre-inc3 5684b5f   src/main.rs   --workflow has a plan source but the metrics log is missing; skipping the workflow check
HIT   pre-inc1 1dac3dc   src/main.rs   PathBuf::from(format!("docs/plans/{task}.ledger.md"))
```

and the `pack/AGENTS.md:93` sentence, the step file's most load-bearing historical quotation, resolves at pre-inc3 AT THE CITED LINE:

```sh
$ git show 5684b5f:pack/AGENTS.md | grep -c "the deterministic \`validate --workflow\` check, once built, is the backstop that the required reviewed rounds happened before a step is marked complete"
1
$ git show 5684b5f:pack/AGENTS.md | grep -n "once built, is the backstop" | cut -d: -f1
93
```

MINIMAL REMEDY, AND IT IS A DELETION-CLASS ONE RATHER THAN AUTHORED PROSE, which this project's calibration prefers: change "either RE-TENSED, so the sentence describes the pre-increment state it was written about" to require that the re-tensed quotation resolve at the revision its tense names. That is one clause, it names an existing and cheap procedure (`git show <pre-increment sha>:<path>`), and this round demonstrates it is runnable in one pass.

SCOPE NOTE, RAISED RATHER THAN ASSUMED. Check 21 was authored by inc4 to verify inc4, so it is in the increment's own range, and round 3 already amended it twice (`R3C-4`'s symbolic-citation clause and its plan-source scope). This is a THIRD clause on the same check, which is exactly the shape round 3's triage flagged as the place a fix pass manufactures the next round's finding. If the triager judges the added clause more expensive than the gap, recording the gap in the ledger rather than editing check 21 again is a defensible close, and I say so here so the decision is available rather than forced.

## What I did NOT find, stated explicitly

- NO check failed. NO check was unrunnable.
- NO behavioural defect. The refusal, the omissions, the reason vocabulary, the precedence rule, the correlation rule, the exit codes and the four accepted costs all behave exactly as this file specifies, on every case the list names and on several it does not.
- NO stale citation. All 13 distinct line-numbered citations in the step file are in bounds and hold their named subject; all 23 distinct identifier citations resolve; the plan-source regions check 21 was widened to cover (the `Q-55` record at `docs/plans/agent-scaffold.plan.toml:1714` and the three `workflow-enforcement-tier-w*` waiver notes at `:1324`, `:1333`, `:1342`) carry only citations that resolve.
- NO unclosed finding from an earlier round. The two round-3 remedies to check 21 are present in the text and I ran both: the identifier procedure resolves 23 of 23, and the plan-source scope resolves.
- NO recorded residual re-raised. The in-root bound, the widened single-anchor `..` bound, `ADV-2`, `R2A-2`, `R3A-1`, `R4A-1` (the inc3-era id) and the plain-`validate` mode-000 inconsistency are all left where they were recorded. The last of those I measured, because check 16 pins it, and it reproduces exactly as pinned.

## Evidence appendix: the runs that carry the most weight

### Check 13b, the case that separates the two rootings (`Q-55-endproperty`)

The RED half is the one worth pasting, because it was run against `as-pre2`, which IS the anchor-rooted build the check says it must defeat:

```sh
$ $S/as-pre2 validate --source $S/A/docs/plans/p.plan.toml --plan $S/B/docs/plans/p.md --workflow
$S/A/docs/metrics/workflow.jsonl: 296 records, valid
$S/A/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
$S/B/docs/plans/p.md: 1 steps, 0 open-questions items, valid
$S/B/docs/plans/p.md vs $S/A/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0

$ $AS validate --source $S/A/docs/plans/p.plan.toml --plan $S/B/docs/plans/p.md --workflow
--workflow would join $S/B/docs/plans/p.md against $S/A/docs/metrics/workflow.jsonl, which is not under the plan's project root $S/B; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair
exit=1

$ $AS validate --source $S/A/docs/plans/NOSUCH.plan.toml --plan $S/B/docs/plans/p.md --workflow
no source plan at $S/A/docs/plans/NOSUCH.plan.toml; nothing to validate
--workflow would join $S/B/docs/plans/p.md against $S/A/docs/metrics/workflow.jsonl, which is not under the plan's project root $S/B; ...
exit=1

$ cd $W && $AS validate --source docs/plans/agent-scaffold.plan.toml --plan docs/plans/agent-scaffold.md --workflow
docs/metrics/workflow.jsonl: 296 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.md: generated projection of a TOML-primary source; skipping the Markdown plan validator
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

BUILDING FIXTURE B TAKES ONE STEP THE CHECK NAMES AND THE NEXT RUNNER WILL MISS. Rendering the Markdown plan from a borrowed-slug source leaves the Step Detail heading reading `### \`example-step\``, so `validate` reports two cross-reference problems and BOTH halves exit 1, which hides the red. The check does say "with its Step Detail heading renamed to match"; renaming it in the rendered `p.md` is what makes the RED visible.

### Check 14g's fourth run, the default-ledger half

```sh
$ cd $W && $AS next --source $S/A/docs/plans/p.plan.toml --plan $S/B2/docs/plans/p.md
task: p
source: $S/B2/docs/plans/p.md
metrics: unavailable, the round log $S/A/docs/metrics/workflow.jsonl is not under the plan's project root $S/B2, so its records cannot be paired with this plan

no active review loop (the round log $S/A/docs/metrics/workflow.jsonl is not under the plan's project root $S/B2, so its records cannot be paired with this plan)

the ledger $S/A/docs/plans/p.ledger.md is not under the plan's project root $S/B2; nothing to resume
exit=0

$ $AS next --json --source $S/A/docs/plans/p.plan.toml --plan $S/B2/docs/plans/p.md
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  "active_loop": null,
  "resume_state": null,
  "resume_state_absent_reason": "ledger-not-this-project",
  "no_active_loop_reason": "metrics-not-this-project"

$ $S/as-pre2 next --source $S/A/docs/plans/p.plan.toml --plan $S/B2/docs/plans/p.md
metrics: 296 records
ACTIVE LOOP
  triager-runs-only-on-findings / triager-runs-only-on-findings-inc1  in progress -> mark-step-complete
  state: converged
  streak: 1/1
  rounds: 2/5
  next: mark the step complete, re-render, and commit
  context:
    ledger: $S/A/docs/plans/p.ledger.md
exit=0
```

### Check 16, both `Err` spellings and the pinned residual

```sh
$ cd $S/probe && chmod 600 docs/metrics
$ $AS validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
exit=0
$ $AS validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl --workflow
--workflow requested but the round log at docs/metrics/workflow.jsonl could not be checked (Permission denied (os error 13)): the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log
exit=1
$ chmod 755 docs/metrics

$ $AS validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl/
no metrics log at docs/metrics/workflow.jsonl/; nothing to validate
exit=0
$ $AS validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl/ --workflow
--workflow requested but the round log at docs/metrics/workflow.jsonl/ could not be checked (Not a directory (os error 20)): the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log
exit=1

$ chmod 000 docs/metrics/workflow.jsonl
$ $AS validate --source docs/plans/TEMPLATE.plan.toml --metrics docs/metrics/workflow.jsonl
Error: Os { code: 13, kind: PermissionDenied, message: "Permission denied" }
exit=1
$ chmod 644 docs/metrics/workflow.jsonl
```

THE UID DIMENSION THE CHECK CLAIMS WAS ALSO VARIED, since check 16 says the trailing-slash spelling holds "at every uid including root". Under `unshare -Ur` (uid 0) the trailing-slash run still exits 1 with `Not a directory (os error 20)`, while the mode-600 directory becomes readable to root and the run greens at exit 0, which is exactly why the check scopes the "every uid" claim to the trailing slash alone. Every fixture mode was restored (`drwxr-xr-x` on `docs/metrics`, `-rw-r--r--` on the log).

### Check 19, both layouts, both surfaces

```sh
# layout 1: <root>/docs/plans is a symlink to <root>/elsewhere
$ cd $S/L1 && $AS validate --source docs/plans/TEMPLATE.plan.toml --workflow
--workflow would join docs/plans/TEMPLATE.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root $S/L1/elsewhere; ...
exit=1
$ $AS status --source docs/plans/TEMPLATE.plan.toml
plan: 1 steps (1 not started); 0 open-questions items
metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's project root $S/L1/elsewhere, so its records cannot be paired with this plan
exit=0
$ $S/as-pre2 validate --source docs/plans/TEMPLATE.plan.toml --workflow
docs/metrics/workflow.jsonl: 3 records, valid
docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0

# layout 2: <root>/docs/metrics is a symlink out of the root
$ cd $S/L2 && $AS validate --source docs/plans/TEMPLATE.plan.toml --workflow
--workflow would join docs/plans/TEMPLATE.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root $S/L2; ...
exit=1
```

Both layouts reproduce accepted cost (ii) in both of its manifestations, the loud refusal and the quiet omission, exactly as the check pins them.

### Check 21b, the four in-scope citations

| citation | subject named by the sidecar | found at the cited range |
| --- | --- | --- |
| `test-tmpdir-repo-assumption.md` -> `src/main.rs:2279-2287` | "a second `fn scratch(name)`" building `agent-scaffold-poc-{pid}-{name}` | yes, `fn scratch` spans 2279 to 2287 |
| `test-tmpdir-repo-assumption.md` -> `src/main.rs:2289-2305` | `tests::init_plan_defaults_to_git_and_skips_inside_a_repo` | yes, the test spans 2289 to 2305 |
| `test-tmpdir-repo-assumption.md` -> `src/main.rs:2878-2889` | `tests::install_precommit_hook_skips_a_non_repo` asserting "not a git repository" | yes, the test spans 2878 to 2889 and carries that string |
| `checks-runner-worktree-name-collision.md` -> `src/main.rs:2280-2285`, `tests/audit_command.rs:20`, `tests/checks_staged_hook_env.rs:50`, `tests/scaffold_precommit_hook.rs:14` | scratch helpers that "discriminate by a per-test literal name rather than by the clock" | yes: `agent-scaffold-poc-`, `agent-scaffold-audit-`, `agent-scaffold-hookenv-`, `agent-scaffold-e2ehook-`, four distinct literal prefixes |
| `instrument-magic-filename.md` -> `src/main.rs:257-258` | `source.read("instrument.md").unwrap_or_default()` for the `{{instrument}}` slot | yes, at 257 to 258 |

The `src/checks.rs` citations in `checks-runner-worktree-name-collision.md` were NOT examined, per the brief and per check 21b's own "DRIFT WITH ANOTHER CAUSE IS OUT OF SCOPE".

### Three derivation claims the checks do not pin, probed anyway

The step file at `:155` makes three claims about `..` handling that no numbered check exercises on the SOURCE side. All three hold:

```sh
$ cd $S/borrowed
$ $AS validate --source docs/plans/../plans/TEMPLATE.plan.toml --workflow
docs/plans/../plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records ...
exit=1                      # a `..` that stays below the project's own docs/plans reaches that project's log

$ $AS validate --source docs/plans/../../other/p.plan.toml --workflow
--workflow would join docs/plans/../../other/p.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root $S/borrowed/other; ...
exit=1                      # a `..` that climbs OUT through a docs/plans is rejected by the canonical guard

$ $AS validate --source other/p.plan.toml --workflow
--workflow requested but no round log at other/docs/metrics/workflow.jsonl: ...
exit=1                      # the same file, spelled directly, resolves through the fallback
```

## Dimensions this lens held fixed, so its clean result is read at its true width

One platform (Linux), one build profile (`dev`), uid 1000 with a single uid-0 cell under `unshare -Ur` for check 16, no concurrency, and no `--instrument` scaffold. Fixtures were built by the HEAD binary and then run against the historical binaries, which is correct for every check here (each check names the fixture as an input, not as output under test) but would not be correct for a check about `scaffold` output itself; check 20 is the one such check, and its red half was taken from `git show 5684b5f:pack/AGENTS.md` rather than from a rebuilt fixture, which is the stronger evidence anyway because it reads the source of the render rather than one render of it.
