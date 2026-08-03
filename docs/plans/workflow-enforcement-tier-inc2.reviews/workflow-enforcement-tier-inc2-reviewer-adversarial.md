# Review findings: `workflow-enforcement-tier-inc2`, ADVERSARIAL CONSTRUCTION lens

LENS. Adversarial construction. Nothing below was concluded by reading the diff. Every claim was produced by building a project layout on disk, running the built binary against it, and recording the streams and the exit code. Two findings were then reduced to a minimal, self-contained reproduction that builds its own fixture from scratch.

ARTIFACT. `git diff main..HEAD` in the worktree `.claude/worktrees/rev-adversarial`, at HEAD `1543325` on branch `impl/wet-inc2`, five files (`src/main.rs`, `src/next.rs`, `tests/unsafe_pairings_are_refused_and_omitted.rs`, `README.md`, `CHANGELOG.md`). Binary built with `cargo build` in that worktree; every invocation below is `target/debug/agent-scaffold`.

SUITE STATE WHEN THE FINDINGS WERE TAKEN. `cargo test` is fully green (413 tests across 9 binaries, 0 failed), including all 13 of the increment's own new tests. Both findings below reproduce against that green suite, which is the situation the specification predicted at line 303 ("passing the increment's own tests is explicitly NOT sufficient evidence here").

WHAT I ATTACKED. Both directions the brief names.

- FALSE NEGATIVES: `..` components in every position (with existing, missing, dangling-symlink and symlink-to-directory intermediates); symlinks on the plan side, on the log side, on the project root, and on an in-project `docs/metrics`; relative, absolute, `./`-prefixed and `..`-detoured spellings of `--source`, `--plan` and `--metrics`; explicit `--metrics` and explicit `--ledger-fragment`; TOML-primary versus Markdown-primary sources; divergent `--source`/`--plan` pairings including the typo'd-source variant; nested `docs/plans`; a conventionless plan at a project root; plans in `docs/plans` subdirectories; and each of the four surfaces (`validate --workflow`, `status`, `status --resume`, `next`) on the same inputs so their answers could be read against each other.
- FALSE POSITIVES: twelve legitimate layouts, listed under ATTACKS THAT FAILED.

NOT RAISED, per the brief: the in-root bound, the four accepted costs, project identity, prose wrapping, and the specification's own wording.

## ADV-1: `next` echoes another project's `## RESUME STATE` block where `status --resume` refuses it, on identical inputs

SEVERITY: high.

CLAIM. When a surface reads no plan but an anchor exists (the reachable case is a Markdown-primary `--source` with no `--plan`), `status --resume` supplies itself a root from the anchors and refuses an out-of-root ledger, while `next` supplies itself nothing, reads the same ledger, and echoes another project's private resume block verbatim at exit 0, reporting `"resume_state_absent_reason": null` on the machine surface so a consumer cannot tell it is unvouched.

REPRODUCTION. Self-contained; builds its own two-project fixture, no scaffold required. Save as `repro.sh` and run `bash repro.sh <path-to-agent-scaffold> <a scratch dir outside any repo>`.

```sh
set -eu
BIN="$1"; R="$2"
rm -rf "$R"; mkdir -p "$R/alpha/docs/plans" "$R/beta/docs/plans"

# Project ALPHA: a MARKDOWN-primary <task>.plan.toml. No --plan is given, so no plan is READ.
cat > "$R/alpha/docs/plans/p.plan.toml" <<'TOML'
[meta]
title = "alpha"
primary = "markdown"
TOML

# Project BETA: an unrelated project with a private resume block.
cat > "$R/beta/docs/plans/b.ledger.md" <<'LED'
# beta ledger

## RESUME STATE

BETA-PRIVATE: branch feat/secret, worktree /home/beta/wt, in-flight review of step X.

## END
LED

echo "=== A: status --resume ==="
"$BIN" status --resume --source "$R/alpha/docs/plans/p.plan.toml" \
       --ledger-fragment "$R/beta/docs/plans/b.ledger.md"; echo "exit=$?"
echo "=== B: next, identical inputs ==="
"$BIN" next --source "$R/alpha/docs/plans/p.plan.toml" \
       --ledger-fragment "$R/beta/docs/plans/b.ledger.md"; echo "exit=$?"
echo "=== C: next --json, identical inputs ==="
"$BIN" next --json --source "$R/alpha/docs/plans/p.plan.toml" \
       --ledger-fragment "$R/beta/docs/plans/b.ledger.md"; echo "exit=$?"
```

OBSERVED (paths abbreviated to `$R`; everything else is verbatim).

```
=== A: status --resume ===
the ledger $R/beta/docs/plans/b.ledger.md is not under the plan's project root $R/alpha; nothing to resume
exit=0

=== B: next, identical inputs ===
task: p
source: no plan source
metrics: no log found

no active review loop (no plan steps found)

RESUME STATE (verbatim from the ledger):
## RESUME STATE

BETA-PRIVATE: branch feat/secret, worktree /home/beta/wt, in-flight review of step X.
exit=0

=== C: next --json, identical inputs ===
{
  "task": "p",
  "source": "no plan source",
  "metrics": null,
  "metrics_absent_reason": "log-absent",
  "active_loop": null,
  "resume_state": "## RESUME STATE\n\nBETA-PRIVATE: branch feat/secret, worktree /home/beta/wt, in-flight review of step X.",
  "resume_state_absent_reason": null,
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

A SECOND SPELLING OF THE SAME CASE, reproduced independently on the scaffolded fixtures before the minimal repro was reduced: the same divergence appears with a Markdown-primary `--source` plus a `--plan` that does not exist, since a non-canonicalisable `--plan` also yields no checked root.

```
$ agent-scaffold status --resume --source $ADV/MA/docs/plans/p.plan.toml \
    --plan $ADV/MB/docs/plans/NOSUCH.md --ledger-fragment $ADV/Aip/docs/plans/TEMPLATE.ledger.md
the ledger .../Aip/docs/plans/TEMPLATE.ledger.md is not under the plan's project root .../MA; nothing to resume
exit: 0
```

while `next` with the byte-identical argument list takes the ledger.

MECHANISM. `run_resume` roots on the ANCHORS through `resume_roots` (`src/main.rs:1421`, called at `src/main.rs:1451`), which is exactly what `Q-55-resumepairing` decided for the surface that reads no plan. `run_next` roots only on the CHECKED PLAN (`src/main.rs:1520`, `checked_plan_root`), and `checked_plan_root` returns `None` the moment the checked plan is absent (`src/main.rs:1313`, the `?` on `if toml_primary { source } else { plan }`). With a Markdown-primary `--source` and no `--plan`, `toml_primary` is false and `args.plan` is `None`, so the root is `None`, the filter at `src/main.rs:1546-1549` never fires, and the ledger is read at `src/main.rs:1552-1556`. `run_next` never calls `resume_roots`, although the function it needs already exists in the same file.

WHY IT MATTERS. This is defect C's third case, the one the specification calls "not a wrong boundary at all but CONTENT INJECTION into an instruction that the receiving agent has been told is authoritative and to read first" (line 127), surviving on the agent-facing surface after the increment whose stated purpose includes closing it. The asymmetry is the sharp part: the same predicate, the same two paths, the same decided rule, two different answers, and the permissive one is the surface an agent consumes. On `--json` the leak is worse than a bare `null` would have been, because `"resume_state_absent_reason": null` positively asserts the block is a genuine one for this plan; the vocabulary added by `Q-55-jsonreason` exists precisely so a consumer can tell that apart, and here it reports the wrong thing rather than nothing.

It also falsifies the documentation shipped in the same increment. `README.md:236` states without qualification: "Every one of these commands checks that the log (and, for the ledger readers, the ledger) it is about to read lives under the project root of the plan it is about to read". `next` is a ledger reader and does not, in this configuration.

WHY THE SUITE DID NOT CATCH IT. `tests/unsafe_pairings_are_refused_and_omitted.rs:627-632` does exercise an explicit `--ledger-fragment` outside the root on `next`, but only with a TOML-PRIMARY `--source` (`build_away` writes `plan_toml(status)`), which always yields a checked root, so the guard always fires there. No test in the file drives `next` with a Markdown-primary source and no `--plan`, and `plan_toml_markdown_primary()` is used only in the divergent-pairing tests, which always supply a `--plan` as well.

SECONDARY OBSERVATION, NOT A SEPARATE FINDING. The same missing root also passes an unpairable LOG through on that configuration: `next --source $R/alpha/docs/plans/p.plan.toml --metrics <another project's log>` prints `metrics: 262 records` with `"metrics_absent_reason": null`, and `status` does the same. I do not raise this as its own finding because `status` behaves identically, so there is no surface-to-surface contradiction to point at, and because no loop can be projected without steps, so no instruction is fabricated. It is recorded because the same fix (give `run_next` a root from the anchors when it has no checked plan) closes both halves, and because the metrics half of it is a wrong record count attributed to `task: p` at exit 0.

## ADV-2: `next` hands an agent the REJECTED ledger path as the active loop's `ledger:` slot, in the same output that says the ledger is not this plan's

SEVERITY: medium.

CLAIM. When the round log is pairable but the ledger is not, `next` correctly withholds the `## RESUME STATE` echo and prints the rejection note, and in the same breath emits a full `ACTIVE LOOP` whose `context.ledger` names the rejected ledger, in another project, as the ledger for this loop, on both the human and the JSON surface, at exit 0.

REPRODUCTION. Self-contained.

```sh
set -eu
BIN="$1"; R="$2"
rm -rf "$R"; mkdir -p "$R/alpha/docs/plans" "$R/alpha/docs/metrics" "$R/beta/docs/plans"
cat > "$R/alpha/docs/plans/p.plan.toml" <<'TOML'
[meta]
title = "alpha"
primary = "toml"
[[step]]
slug = "core"
title = "Core"
status = "in-progress"
order = 1
TOML
: > "$R/alpha/docs/metrics/workflow.jsonl"
printf '# beta ledger\n\n## RESUME STATE\n\nBETA-PRIVATE block.\n' > "$R/beta/docs/plans/b.ledger.md"
"$BIN" next --source "$R/alpha/docs/plans/p.plan.toml" \
       --ledger-fragment "$R/beta/docs/plans/b.ledger.md"; echo "exit=$?"
"$BIN" next --json --source "$R/alpha/docs/plans/p.plan.toml" \
       --ledger-fragment "$R/beta/docs/plans/b.ledger.md" | grep -E '"ledger"|resume_state'
```

OBSERVED (reminder text elided, marked; nothing else changed).

```
task: p
source: $R/alpha/docs/plans/p.plan.toml
metrics: 0 records

ACTIVE LOOP
  core  in progress -> record-round
  state: awaiting-first-review
  streak: 0/?
  rounds: 0/5
  isolation: unknown
  next: spawn a reviewer for the first review round
  role: reviewer
  prompt: .agents/prompts/reviewer.md
  context:
    isolation_tier: unknown
    ledger: $R/beta/docs/plans/b.ledger.md
    review_findings: docs/plans/p.reviews/core-reviewer-<disambiguator>.md
    triage_findings: docs/plans/p.reviews/core-triage.md
  reminders:
    [three reminders, elided]
  summary: first review round on step `core`: independent reviewer, cite file and line.

the ledger $R/beta/docs/plans/b.ledger.md is not under the plan's project root $R/alpha; nothing to resume
exit=0
```

and on the machine surface:

```
        "ledger": "$R/beta/docs/plans/b.ledger.md",
  "resume_state": null,
  "resume_state_absent_reason": "ledger-not-this-project",
```

REACHABLE WITHOUT `--ledger-fragment`. The same output arises on a divergent pairing when the log is named explicitly under the checked plan's root, so the metrics half is safe and only the DEFAULT ledger is rejected. Reproduced on the scaffolded fixtures:

```
$ agent-scaffold next --source $ADV/MA/docs/plans/p.plan.toml \
    --plan $ADV/MBip/docs/plans/p.md --metrics $ADV/MBip/docs/metrics/workflow.jsonl
...
  context:
    isolation_tier: unknown
    ledger: $ADV/MA/docs/plans/p.ledger.md
...
the ledger $ADV/MA/docs/plans/p.ledger.md is not under the plan's project root $ADV/MBip; nothing to resume
exit: 0
```

Here the loop is projected from MBip's steps and MBip's log, and the ledger slot names MA's ledger.

MECHANISM. `run_next` computes `resume_state_absent_note` and suppresses `resume_state` (`src/main.rs:1546-1559`), but `ledger_path` is passed into `NextInputs::ledger_path` unchanged, and `build_context` unconditionally writes it into the instruction's context slots (`src/next.rs:1001`). The suppression covers the ledger's CONTENT and not the ledger's ADDRESS.

WHY IT MATTERS. The `ledger` slot is not decoration: `src/next.rs:987-994` documents it as one of the two always-present context slots, and the ledger is where the orchestrator appends round records and moves the resume anchor. So the increment's own output tells an agent, at exit 0, both that a ledger is not this plan's and that it is the ledger for this loop. The specification's own framing of why the projections matter (line 127, "`next`'s false instruction is consumed by an AGENT that acts on it") applies directly: this instruction points a write at a file in a different project. It is a weaker case than ADV-1 because it is an address rather than borrowed content, and because the contradicting note is printed in the same output, so a careful human reader would catch it. It is not weaker on the JSON surface, where the note is not serialised at all (`metrics_absent_note` and `resume_state_absent_note` are both `#[serde(skip)]`, `src/next.rs`), leaving `context.ledger` as an unqualified instruction beside a `resume_state_absent_reason` a consumer must think to correlate.

SCOPE NOTE, stated so the triager can weigh it. The specification requires of an unsafe ledger only that "the `RESUME STATE` echo is omitted" (line 183) and says nothing about the `ledger:` context slot; it separately records at line 389 that `next`'s `review_findings`/`triage_findings` report paths are deliberately NOT anchored. So this is not a literal violation of a written requirement. I raise it because the increment's stated correctness property is a negative about what unsafe pairings may produce, and because the two halves of one output contradicting each other is a defect on its own terms whichever way the requirement is read.

## ATTACKS THAT FAILED

These are recorded so the next reader knows what ground is covered. Each was run; none produced a wrong answer.

### False negatives that did not work

- `..` ESCAPE THROUGH A MISSING INTERMEDIATE, the one structural hole in `resolve_for_containment` (`src/main.rs:1326`). A literal `..` does survive into the containment remainder and the predicate does then pass a path that lexically climbs out of the root: `--metrics <root>/docs/plans/nope/../../../../../../../../../../<abs path to another project's log>` produced NO refusal. It could not be turned into a wrong answer, because the same path is unopenable: `[ -r ]` on it is false, `metrics_path.exists()` is false, and `validate --workflow` reported `no metrics log at <path>; nothing to validate` at exit 0 rather than reading anything. The doc comment's argument at `src/main.rs:1323-1325` holds as measured.
- THE SAME ESCAPE WITH A DANGLING SYMLINK as the intermediate (`ln -s /definitely/not/here dangle`), on the theory that `canonicalize` and `open` might disagree. They do not: identical result, unreadable, `no metrics log at <path>`, exit 0.
- THE SAME ESCAPE WITH A RESOLVABLE SYMLINK-TO-DIRECTORY as the intermediate, which makes the path genuinely readable (`[ -r ]` true). Here `canonicalize` succeeds on the whole path, the remainder never goes literal, and the guard fires: exit 1 with the refusal naming the log and the root. This is the case that closes the class, since a readable path is exactly the one where the resolution is exact.
- PLAIN `..` ESCAPE with all components existing (spec check 13): refused, exit 1.
- `..` THAT STAYS INSIDE THE ROOT (`<root>/docs/plans/../metrics/workflow.jsonl`): allowed, the check ran and reported. Correct.
- SYMLINKED PLAN PLACED OUTSIDE ITS PROJECT with a full log beside the symlink (spec check 12): refused, exit 1, and the refusal correctly names the REAL project root of the symlink target rather than the symlink's directory.
- DIVERGENT `--source`/`--plan` PAIRING (spec check 13b) on all four surfaces: `validate --workflow` exit 1 naming B's plan, A's log and B's root; `next` withholds the whole loop AND the resume echo; `status` omits the metrics half; `status --resume` omits the block. All exit codes as specified.
- THE TYPO'D `--source` VARIANT of that pairing: still refused, exit 1, root taken from the `--plan` that was read.
- NESTED `docs/plans` with the inner plan pointed at the OUTER project's log by explicit `--metrics`: refused, exit 1 naming the inner root. The nearest-wins default also resolves to the inner log (`metrics: 0 records`, not the outer log's 262).
- EXPLICIT `--metrics` NAMING A FOREIGN LOG on all four surfaces (spec checks 11, 14, 14b, 14c): validator refuses at exit 1; `status` and `next` omit at exit 0; neither projection ever exited non-zero under any input I tried.
- THE 14d TRAP: with the fixture step at `in-progress` and a foreign log, `next` printed no `state:`, `streak:`, `rounds:`, `next:`, `role:`, `prompt:` or `summary:` line, and specifically did NOT fall through to `awaiting-first-review` / "spawn a reviewer for the first review round". The unsafe-is-not-absent distinction is genuinely implemented, not approximated.
- THE PRECEDENCE RULE (14f fourth run): an explicit `--metrics` outside the root naming a file that does NOT exist reports `log-not-this-project`, not `log-absent`, on both `next --json` and `status --json`. The same holds for a `--ledger-fragment` outside the root and missing: `ledger-not-this-project`, not `ledger-absent`.
- THE VOCABULARY SEPARATION (14f): `log-absent` with a derived `active_loop`, `log-not-this-project` with `metrics-not-this-project`, and `no-plan-steps` all produced distinct JSON. `no-resume-section` and `ledger-absent` also separate.
- THE CORRELATION RULE'S BOUNDARY: with all steps terminal AND an unpairable log, `no_active_loop_reason` is `all-steps-terminal` (the step cause), while `metrics_absent_reason` stays `log-not-this-project`. I checked the claim the retype rests on, that `select_active_loop` returns `None` only for no-steps-or-all-terminal, by reading the phase partition rather than trusting the comment: `StepPhase` has seven variants, `is_pending` covers `NotStarted`/`Next`, `is_terminal` covers `Complete`/`Skipped`/`Optional`/`Deferred`, and `InProgress` is the seventh, so `select_active_loop` (`src/next.rs:711-736`) and `steps_leave_no_loop` (`src/next.rs`) are exactly equivalent and the collapsed third reason really is unreachable. No input produced "all steps complete" for a plan with non-terminal steps.
- `validate` WITHOUT `--workflow` plus a foreign `--metrics`: exit 0, no refusal (spec check 14). Correct scoping.
- ARGUMENT ABUSE: `--metrics ""` and `--ledger-fragment ""` are rejected by clap at exit 2 before any resolution runs, so the empty-string route into the predicate is closed. `--metrics <a directory>` and `--source <a directory>` fail with `Os { code: 21, kind: IsADirectory }` at exit 1; this is pre-existing behaviour of the read, not caused by this increment, and produces no wrong answer.

### False positives that did not occur

Twelve legitimate layouts, all outside the four accepted costs, all still correct. The first nine are `validate --source ... --workflow` and each printed `workflow invariants hold` at exit 0 against the repository's own 262-record log.

1. From the repository root with a relative `--source`. Printed paths stayed relative, as spec check 9 requires.
2. From another directory entirely with an absolute `--source`.
3. From a subdirectory (`src/`) with a `../`-relative `--source`.
4. With a `./`-prefixed `--source`.
5. With an in-root `..` detour in the `--source` (`docs/plans/../plans/...`).
6. With an explicit relative `--metrics` naming the project's own log, from the root.
7. With an explicit absolute `--metrics` naming the project's own log.
8. With an explicit `--metrics` containing an in-root `..` (`docs/plans/../metrics/workflow.jsonl`).
9. With `--source` and `--plan` both naming the same project's two substrates, source TOML-primary (spec check 13b third run).
10. `status`, `next --json` and `status --resume` run from another directory against the repository's absolute `--source`: full record count, full active loop, full resume block, exit 0.
11. A SYMLINK TO THE WHOLE PROJECT ROOT, with both the plan and the log spelled through it: NOT refused, `workflow invariants hold`, exit 0. Worth stating because it looks like accepted cost (ii) and is not: the symlink resolves both sides to the same real root, so the divergence that cost describes never arises.
12. AN IN-PROJECT SYMLINKED `docs/metrics` (pointing at `<root>/logs` in the same project): NOT refused, `metrics: 262 records`, exit 0. Same reason.

Also confirmed correct and NOT refused: a Markdown-only project driven by `--plan` alone against its own log; a conventionless plan at a project root with no `docs/plans`, read both from that root and from elsewhere (spec check 8), resolving through the fallback to that root's own log in both spellings.

### Accepted costs, pinned as expected rather than raised

Run and confirmed to behave exactly as the specification records, so a later reader knows they were exercised and deliberately not raised: cost (i), the bare filename from inside `docs/plans` (stderr miss note, exit 0, no refusal); cost (ii), both the symlinked-`docs/plans` layout (`validate --workflow` exit 1 with the refusal naming the real root, `status` and `next` omitting the metrics half at exit 0) and its log-side twin; cost (iii), the same-project `--plan` outside any `docs/plans` with a Markdown-primary `--source` inside one (exit 1); cost (iv), `status --resume` on that pair (note, no block, exit 0).
