# `workflow-enforcement-tier-inc4` round 5: the source-side claim surface

Reviewer lens: every doc comment, `///` item comment, inline `//` comment, clap `--help` string and user-visible message string in `src/` and `tests/` that describes behaviour this step's four increments changed. This lens has never been run in this loop; the twelve prior reviewer passes were all pointed at the plan documents, which is why `R4B-1` survived four rounds.

Worktree `.claude/worktrees/rev-inc4-r5-a`, branch `review/wet-inc4-r5-a`, at `cf9ff9c`. Binary built `--release` from that tree. Suite re-run at that tree: `cargo test --release` exits 0.

## Result

FOUR findings: `R5A-1` (medium), `R5A-2` (medium), `R5A-3` (medium), `R5A-4` (low).

No `high` and no `critical`. Nothing the tool DOES is wrong. Every finding is a false statement in the source about behaviour the step introduced, and every one is reproducible by running the shipped binary.

THE HEADLINE IS THAT `R4B-1` IS NOT CLOSED. Round 4 established that both containment reasons fire where the surface reads no plan, and the fix at `9c9aa00` deleted the plan attribution from the two ENUM DOC COMMENTS. The two USER-VISIBLE STRINGS that state the same falsehood to the operator were not touched, and the round 4 triage recorded "no user-visible string is wrong" as a fact while weighing severity. That premise is false and I refute it below by running the binary on the round 4 fixture shape.

## Denominator

The main contribution of a lens that has never been run is its denominator, so here it is, with the verification level stated per group rather than averaged.

| Region | Claims examined | Confirmed true | False |
| --- | --- | --- | --- |
| `src/main.rs` clap `--help` strings (`ValidateArgs`, `StatusArgs`, `NextArgs`) | 26 | 24 | 2 |
| `src/main.rs` struct and field doc comments (`Projection`, `PlanProjection`, `MetricsProjection`) | 5 | 4 | 1 |
| `src/main.rs` function doc comments and inline comments (`run_validate`, `run_status`, `run_resume`, `run_next`, `toml_source`, `note_missing_anchors`, `resolve_metrics_path`, `project_root_of_source`, `canonical_project_root`, `checked_plan_root`, `containment_roots`, `resolve_for_containment`, `is_outside_root`, `unpairable_log_note`, `unpairable_ledger_note`, `default_ledger_path`, `resume_roots`) | 43 | 39 | 4 |
| `src/next.rs` module doc, the three reason enums, `NextProjection` and its fields, `project`, `select_active_loop`, `steps_leave_no_loop`, `no_loop_reason`, `derive_task`, `extract_resume_state`, `render_human`, `no_loop_text`, `StepPhase` | 29 | 27 | 2 |
| `src/plan/source.rs` (`is_toml_primary`, `parse_toml`, `step_views`) and `src/workflow.rs` (`check_workflow`) | 5 | 4 | 1 |
| `tests/` doc comments in the three files that describe acceptance checks | 48 | 48 | 0 |
| TOTAL | 156 | 146 | 10 |

The 10 false claims are 10 SITES across the 4 findings: 2 for `R5A-1`, 2 for `R5A-2`, 5 for `R5A-3`, 1 for `R5A-4`.

VERIFICATION LEVEL, stated because it differs by group. Of the 108 `src/` claims, 47 were checked by RUNNING the binary against a purpose-built fixture and reading the output (every claim about which file is read, which exit code is produced, which reason token is serialised, and which sentence is printed). The remaining 61 were checked by reading the cited code path, and are claims about internal structure that no invocation can distinguish (for example "`Some` exactly when X is `None`" biconditionals, which I checked by confirming that every branch that sets one sets the other). The 48 `tests/` claims were checked by reading each doc comment against its own test body and by re-running the suite green; one of them (`tests/validate_workflow_toml_source_needs_no_plan.rs`, the mode 111 negative control, which the file DESCRIBES but does not RUN) I reproduced directly, and it holds.

## What I did NOT reach

- `src/checks.rs`, `src/audit.rs`, `src/manifest.rs`, `src/tui.rs`, `src/agents_md_drift.rs`, `src/pack.rs`, `src/plan/render.rs`, `src/recommendation_rule.rs`, `src/isolation_policy.rs`, `src/findings_naming.rs`, `src/workflow_spec.rs`. None of the four increments changed behaviour these describe. I grepped all of them for the step's subject vocabulary (`metrics`, `ledger`, `--workflow`, `containment`, `not this project`, `unaffected`) and opened every hit; the only hits outside my remit were `src/checks.rs:490` (about a worktree add, unrelated) and `src/workflow.rs:151-152`, which I examined and confirmed true as scoped.
- `tests/audit_command.rs`, `tests/checks_missing_tmpdir.rs`, `tests/checks_staged_hook_env.rs`, `tests/scaffold_precommit_hook.rs`, `tests/validate_toml_primary_skips_markdown_plan.rs`. Same reason; I ran the suite over all of them but did not inventory their doc comments.
- The `#[cfg(test)]` module doc comments inside `src/next.rs`, `src/workflow.rs` and `src/metrics.rs` BELOW the golden-output section. I read `src/next.rs`'s reason-vocabulary tests (`the_absent_causes_serialise_distinguishably`, `a_terminal_plan_reports_the_step_cause_not_the_log_cause`, `a_blocked_pending_step_still_yields_a_loop_so_it_is_not_a_no_loop_reason`, `an_unpairable_ledger_prints_its_note_in_place_of_the_block`) and counted their doc comments in the `src/next.rs` row. I did not inventory `src/workflow.rs`'s or `src/metrics.rs`'s ~1400 lines of unit-test comments.
- `README.md`, `pack/AGENTS.md`, `CHANGELOG.md` and the deployed `.agents/` copies. Outside `src/` and `tests/`, and already covered by other lenses.
- CLAIM KINDS I did not reach: historical RED claims in test doc comments ("RED against the round 3 tip: ..."). Verifying those needs a binary built from each named ancestor commit, which round 3's historical-truth lens ran and reported clean. I took its result rather than repeating it, and I record that my numbers therefore do not re-establish it.

## Declined regions, not raised

I raised nothing about `run_validate`'s "`--plan` is still clap-required" claims, `src/next.rs:162`'s "Every derived part is optional", or `src/next.rs:181-183`'s `active_loop` `None` disjunct. I opened all three to confirm they are the sites my brief excludes and that my findings are elsewhere: `R5A-1` is at `src/main.rs:1509` and `:1522`, `R5A-2` at `src/next.rs:103` and `:136`, `R5A-3` at five sites in `src/main.rs` and `src/plan/source.rs`, and `R5A-4` at `src/main.rs:571`. None is a declined line.

I raised nothing on the four inc2 residuals, the four inc3 residuals, `F-5`, or the settled dismissals `R1A-5`, `R1A-7`, `R1A-8`, `R1B-3`, `R2A-5`. `R5A-4` sits NEXT TO `R1A-7`'s subject and I say below exactly why it is a different claim and what is new.

---

## `R5A-1` (medium): `R4B-1` IS NOT CLOSED. The two containment notes still tell the OPERATOR the root came from a plan, on the same fixture shape the round 4 finding was established with

### The claim

`src/main.rs:1504-1513`:

```
fn unpairable_log_note(
    log: &Path,
    root: &Path,
) -> String {
    format!(
        "the round log {} is not under the plan's project root {}, so its records cannot be paired with this plan",
```

`src/main.rs:1517-1526`:

```
fn unpairable_ledger_note(
    ledger: &Path,
    root: &Path,
) -> String {
    format!(
        "the ledger {} is not under the plan's project root {}; nothing to resume",
```

(the source's leading hard tabs are shown as spaces here so this file stays ASCII-printable; nothing else is altered)

Both say THE PLAN'S PROJECT ROOT. That is the exact clause `Q-55-reasondefs` deleted from the sidecar at `:217` and `:229`, and that `9c9aa00` then deleted from the two enum doc comments at `src/next.rs:105` and `:140` one round later.

### Measured false

Fixture at `<scratch>/rev-inc4-r5-a`: `projA/docs/plans/p.plan.toml` declares `[meta].primary = "markdown"` with one step; `foreign/` is a separate tree with its own round log and its own ledger. NO `--plan` is passed, so the surface reads NO PLAN and the root is supplied by the anchor.

```
$ agent-scaffold status --source <S>/projA/docs/plans/p.plan.toml \
    --metrics <S>/foreign/docs/metrics/workflow.jsonl
plan: not provided
metrics: unavailable, the round log <S>/foreign/docs/metrics/workflow.jsonl is not under the plan's project root <S>/projA, so its records cannot be paired with this plan
exit=0
```

`plan: not provided` is the tool's own report that it read no plan, printed two lines above a sentence attributing the root to that plan. Same shape on `next`, which reports it in three places at once:

```
$ agent-scaffold next --source <S>/projA/docs/plans/p.plan.toml \
    --metrics <S>/foreign/docs/metrics/workflow.jsonl \
    --ledger-fragment <S>/foreign/p.ledger.md
task: p
source: no plan source
metrics: unavailable, the round log <S>/foreign/docs/metrics/workflow.jsonl is not under the plan's project root <S>/projA, so its records cannot be paired with this plan

no active review loop (no plan steps found)

the ledger <S>/foreign/p.ledger.md is not under the plan's project root <S>/projA; nothing to resume
exit=0
```

`source: no plan source` and `no active review loop (no plan steps found)` are the tool saying there is no plan, in the same six lines as two sentences about "the plan's project root".

THE LEDGER HALF IS WORSE THAN THE LOG HALF, because `status --resume` READS NO PLAN AT ALL, EVER, BY DESIGN. `run_resume` (`src/main.rs:1640-1665`) never calls `toml_source` and never opens a plan; it derives its roots from `resume_roots`, which is the anchor policy `Q-55-resumepairing` decided for exactly the surface that reads no plan. So this sentence attributes the root to a plan on a surface whose whole definition is that it did not read one:

```
$ agent-scaffold status --resume --source <S>/projA/docs/plans/p.plan.toml \
    --ledger-fragment <S>/foreign/p.ledger.md
the ledger <S>/foreign/p.ledger.md is not under the plan's project root <S>/projA; nothing to resume
exit=0
```

### Not a re-raise, and the new evidence against a settled premise

`R4B-1` is closed AS RECORDED: its two named sites, `src/next.rs:105` and `:140`, carry the corrected text at this tree. I checked both.

WHAT IS NEW is that the round 4 triage's severity reasoning contains a factual premise I can refute. `workflow-enforcement-tier-inc4-r4-triage.md:(1)`, under "SEVERITY: `medium` CONFIRMED, and I weighed `low` before confirming it", reads verbatim:

> FOR `low`: no behaviour is wrong, no user-visible string is wrong, and four words of a qualifier are a small thing.

The three commands above are user-visible strings and they are wrong. The triager weighed `low` against a premise that a single `grep` refutes:

```
$ grep -n "plan's project root" src/main.rs
1509:            "the round log {} is not under the plan's project root {}, so its records cannot be paired with this plan",
1522:            "the ledger {} is not under the plan's project root {}; nothing to resume",
```

I am not asking for `R4B-1`'s verdict to be revisited; it was ruled valid and fixed. I am reporting that the DECISION `Q-55-reasondefs` reached two sites of what is at least six, and that the two it has never reached are the two an operator actually sees. The round 4 triage itself framed the trap as "asks what REMAINS rather than what the decision REACHED"; the same trap sits one layer out.

### Provenance and scope

`git log --oneline -S "is not under the plan's project root" -- src/main.rs` returns exactly `8beb1c2` ("feat: refuse and omit on a round log or ledger the plan cannot vouch for"), which is THIS STEP'S OWN INC2. The claim was authored inside the step and falsified inside the step, by inc2's own fix-round commit `269d075` ("supply a root to the surfaces that read no plan, and pin six unguarded clauses"), which introduced `containment_roots` and the anchor-supplied root. That is `R4B-1`'s provenance shape exactly, and `Q-55-twinsites` settled the reading ("a stale claim THE INCREMENT'S OWN CHANGE BROKE is in scope regardless of authorship").

### Severity: `medium`, and I weighed `low` and `high`

AGAINST `high`: nobody acts wrongly on it. The withholding itself is correct, the reason token on `--json` is correct, and the paths named in the sentence are both accurate. An operator who reads the sentence and goes looking for a plan at that root finds one only by luck, but the remedy they need (correct the `--source`, or pass a `--metrics` under that root) is the same either way.

AGAINST `low`: this is what the OPERATOR reads, not what a maintainer reads. `R4B-1` was confirmed `medium` on the ground that these are the only definitions of two serialised contract tokens; the string an operator is shown is at least as load-bearing as the comment beside the type, and on `status --resume` there is no `--json` surface at all, so the sentence is the ONLY output that surface produces. A settled `medium` verdict resting on "no user-visible string is wrong" is an aggravation rather than a mitigation.

### Minimal remedy: DELETION, four words, at two sites

The same four words the sidecar fix and `9c9aa00` deleted. `src/main.rs:1509` becomes "the round log {} is not under the project root {}, so its records cannot be paired with this plan". `src/main.rs:1522` becomes "the ledger {} is not under the project root {}; nothing to resume". NOTHING IS AUTHORED.

SIXTEEN TEST ASSERTIONS PIN THE CURRENT SPELLING AND MUST BE UPDATED IN THE SAME CHANGE, so the deletion is not silently reverted. `grep -n "is not under the plan's project root" tests/unsafe_pairings_are_refused_and_omitted.rs` returns `:222`, `:376`, `:457`, `:501`, `:525`, `:555`, `:602`, `:612`, `:707`, `:749`, `:1080`, `:1237`, `:1357`, `:1520`, `:1564`, `:1634`. Note that `:222`, `:376`, `:555` and some others assert against the `--workflow` problem string at `src/main.rs:1000`, which must NOT change; whoever applies this must split the two populations rather than doing one blind substitution. `src/main.rs:1000`'s `--workflow` problem string carries the same words and is CORRECT there and must NOT be changed: on `validate`, `checked_plan_root` canonicalises a plan that exists, so the root always does come from a plan that was read (confirmed: a Markdown-primary `--source` with no `--plan` never reaches that string, it hits the `(None, None, _)` arm instead).

SECONDARY SITES OF THE SAME PLAN ATTRIBUTION, offered so the fix is written once rather than as separate findings, all doc comments rather than strings: `src/main.rs:573` ("present only when the metrics log exists AND is this plan's"), `src/next.rs:177` ("is the plan's own"), `src/next.rs:185` ("is not this plan's"). I did not open `src/next.rs:162` or `:181-183`, which my brief excludes.

---

## `R5A-2` (medium): `log-absent` and `ledger-absent` are DEFINED as "no file at the resolved path", and both fire for a file that IS on disk with real content behind it

### The claim

`src/next.rs:103-104`:

```
    /// No file at the resolved metrics path.
    LogAbsent,
```

`src/next.rs:136-137`:

```
    /// No file at the resolved ledger path.
    LedgerAbsent,
```

These are the only definitions of two serialised contract tokens, in the file that defines the type, and both are AFFIRMATIVE assertions about the filesystem.

### Measured false

Fixture at `<scratch>/rev-inc4-r5-a/mode111`: a conventional `docs/plans` + `docs/metrics` layout with a TOML-primary plan, a REAL round log holding one valid `round` record, and a REAL ledger holding a `## RESUME STATE` block. The technique is the one the repository already owns: mode 600 on the containing directory, so it stays READABLE and stops being SEARCHABLE and `fs::metadata` on the child fails with EACCES while the child is listable by name.

THE LOG HALF, at uid 1000:

```
$ chmod 600 docs/metrics
$ agent-scaffold status --json --source docs/plans/p.plan.toml
{
  "plan": { "steps": [ { "slug": "only-step", "status": "not started" } ], "open_questions": [] },
  "metrics": null,
  "metrics_absent_reason": "log-absent"
}
exit=0

$ agent-scaffold next --json --source docs/plans/p.plan.toml
  "metrics": null,
  "metrics_absent_reason": "log-absent",
exit=0

$ agent-scaffold status --source docs/plans/p.plan.toml
metrics: no log found

$ chmod 755 docs/metrics
$ agent-scaffold status --json --source docs/plans/p.plan.toml
  "metrics": { "records": 1 },
  "metrics_absent_reason": null
```

The control on the last two lines is the whole point: nothing changed but the mode, and the file the tool called absent counts one record.

THE LEDGER HALF, same fixture, mode 600 on `docs/plans` instead:

```
$ chmod 600 docs/plans
$ agent-scaffold next --json --source <D>/docs/plans/p.plan.toml
  "resume_state": null,
  "resume_state_absent_reason": "ledger-absent",

$ agent-scaffold status --resume --source <D>/docs/plans/p.plan.toml
note: --source <D>/docs/plans/p.plan.toml could not be checked: Permission denied (os error 13)
no ledger at <D>/docs/plans/p.ledger.md; nothing to resume
exit=0

$ chmod 755 docs/plans
$ agent-scaffold next --json --source <D>/docs/plans/p.plan.toml
  "resume_state": "## RESUME STATE\n\nprojP resume state, really here.",
  "resume_state_absent_reason": null,
```

THE `status --resume` RUN IS THE ONE TO READ TWICE. In ONE invocation the tool prints, on stderr, that a file in that directory COULD NOT BE CHECKED and names the errno, and then prints, on stdout, that a second file in the SAME directory is NOT THERE. Two files, one unreadable directory, two contradictory diagnoses, from two probes in the same process: `note_missing_anchors` uses `try_exists` with the three-way split (`src/main.rs:1159-1165`), and `run_resume` uses `ledger_path.exists()` (`src/main.rs:1655`).

### This is the exact question `Q-55-existsgate` decided INSIDE THIS STEP, and it was decided the other way

The decision is recorded in the source at `src/main.rs:1056-1066`:

> TWO CLAIMS, NOT ONE, and only the first is safe to make. `Ok(true)` is `metadata().is_ok()`, so the `None` that lands here answers false for a log that is not there AND for one whose directory cannot be traversed. The SAME probe splits them: `Ok` asserts absence and prescribes recording rounds, `Err` says that the question could not be answered and names the error [...] because a real log may sit behind that error and sending its operator to record rounds that are already recorded is the falsehood `Q-55-emptyroot` decided against.

And the same argument is made a third time, for anchors, at `tests/unsafe_pairings_are_refused_and_omitted.rs:937-943`:

> AN ANCHOR THE TOOL CANNOT ASK ABOUT IS NOT AN ANCHOR THAT IS MISSING. `Path::exists` answers `false` both for a path that is not there and for one whose metadata cannot be read [...] a loud line that states a falsehood about the filesystem is worse than a quiet one: it sends the operator to fix a path that is already correct.

So the tree contains the argument twice, the fix twice, and the technique twice, and the two reason tokens this step INTRODUCED assert the thing both of them call a falsehood.

`src/main.rs:1064-1066` scopes the inc3 fix as "ARM-SCOPED BY `Q-55-existsgate`: the gate above keeps that predicate, so plain `validate` is untouched and only the surface that asked for the check gains the distinction." That sentence is TRUE as written and I do not raise it, but it names ONE untouched surface where there are THREE: plain `validate`, `status` and `next` all keep the collapsed predicate, and only the latter two SERIALISE the collapse as a token whose definition asserts absence.

### Not a residual already recorded

The recorded inc3 residual is "the plain-`validate` mode-000-file-versus-unsearchable-directory inconsistency". That is a different surface (plain `validate`'s stderr note at `src/main.rs:867`) and a different artifact (a note, not a serialised contract token). I am not raising that note. `R4A-1`'s residual is reader-level discrimination between a non-instrumented project and a mis-anchored run, which is a third thing again.

NEVER RAISED IN THIS LOOP. `grep -rn "LogAbsent\|log-absent\|LedgerAbsent\|ledger-absent" docs/plans/agent-scaffold.reviews/` returns 15 hits across twelve findings and four triage files. Every one either quotes the token in a JSON sample, lists it in a vocabulary inventory (round 4's cross-artifact facts 26 and 28 record "AGREE on the tokens", which is an agreement between SITES and not a check of the definition against the tree), or asserts the precedence rule (unsafe wins over absent). None examines whether the definition is true.

### Why the suite does not catch it

`src/next.rs:1905-1955` (`the_absent_causes_serialise_distinguishably`, acceptance check 14f) HAND-BUILDS `metrics_absent_reason: Some(MetricsAbsentReason::LogAbsent)` as an INPUT to `project`. It cannot detect the mapping from filesystem state to token, because `project` never performs that mapping (the caller does, in `run_status` and `run_next`). That is honest design, not a defect, and it is why check 14f passes while the definition is false.

### Provenance and scope

`git log --oneline -S "No file at the resolved" -- src/next.rs` returns exactly `8beb1c2`, inc2's feature commit. Both definitions were authored inside this step, for tokens this step introduced. They were BORN false rather than falsified later, which is a real difference from `R5A-1` and I state it plainly: the step's own inc3 is what established, by human decision, that this conflation is a falsehood the project refuses to print, and inc3 applied that decision to one of the three surfaces that make it.

### Severity: `medium`, and I weighed `low`

FOR `low`: the population is narrow (a directory the caller cannot traverse), no behaviour is wrong (withholding is right either way), and the exit code is unaffected.

WHAT CARRIES `medium` is the ground `R4B-1` and `R3B-1` were both confirmed on. These two comments are the ONLY definitions of two serialised contract tokens anywhere in the source. `Q-55-jsonreason` exists precisely so a machine consumer can tell the causes apart, and a consumer author reading `src/next.rs:103` concludes that `log-absent` means the file is not there, when the file may be there with round evidence in it. An agent reading `next --json` and told the log is absent is told the project is not instrumented, when it is instrumented and merely unreadable from here. That is the same false-green family the whole step exists to remove, one register quieter.

### Minimal remedy: DELETION-CLASS at the two definitions, and the behaviour question is SEPARATE

DOC REMEDY, which is all this finding asks for. `src/next.rs:103` becomes "The resolved metrics path did not answer as a file." `src/next.rs:136` becomes "The resolved ledger path did not answer as a file." Each drops an assertion the code cannot make and asserts only what the probe returned. Twin sites in the sidecar at `:216` and `:227` carry the same two sentences verbatim and must be corrected in the same change, or `R4B-1`'s pattern repeats a third time. `src/main.rs:461`'s `StatusArgs::resume` help ("Exits 0 with a note when the ledger is absent, ...") is the same claim in a `--help` string and is the fourth site.

I am NOT prescribing a source change. Splitting the probe on `status`, `next` and `status --resume` the way `Q-55-existsgate` split it on `validate --workflow` would mint new tokens on a documented JSON contract, which `Q-55-jsonreason` explicitly reserved to a human decision ("DO NOT widen or rename them without a new decision"). That is a decision to put, not a fix to apply, and it plausibly belongs beside the recorded plain-`validate` residual in the validation-constraints step.

---

## `R5A-3` (medium): FIVE sites, two of them `--help` text, promise that a Markdown-primary `--source` leaves a project unaffected. This step made it withhold that project's own log

### The claim, at five sites

`src/main.rs:452` (`StatusArgs::source`, clap `--help`):

> Path to a `<task>.plan.toml` structured source. When it declares `[meta].primary = "toml"`, the plan projection is read from it instead of --plan (else --plan is used, so a Markdown-primary or absent source is unaffected).

`src/main.rs:476` (`NextArgs::source`, clap `--help`): the same parenthetical, verbatim.

`src/main.rs:1096-1098` (`toml_source` doc):

> A `None` path, a missing file, or a source whose `[meta].primary` is `markdown` all yield `None`, so the caller falls back to the Markdown `--plan` and the live repo (no TOML source, or a Markdown-primary one) is unaffected.

`src/main.rs:1199-1201` (`run_status` inline):

> The Inc 4 gate: when a `--source` is a `<task>.plan.toml` declaring `[meta].primary = "toml"`, project the plan from it; otherwise fall back to the Markdown `--plan`, so a Markdown-primary or absent source is unaffected.

`src/plan/source.rs:406-411` (`is_toml_primary` doc):

> [...] when it is `markdown` (the default) the Markdown + JSONL path is used unchanged, so a repo with no TOML source, or one still declaring `markdown`, is unaffected.

Every one names TWO populations and asserts they behave alike: a repo with no TOML source, and a repo whose source declares `markdown`.

### Measured false, and the falsification needs no historical binary

The two populations the sentence groups now diverge on IDENTICAL inputs. Same foreign `--metrics`, same working directory, one variable: whether a Markdown-primary `--source` is supplied.

```
$ agent-scaffold status --json --metrics <S>/foreign/docs/metrics/workflow.jsonl
{
  "plan": null,
  "metrics": { "records": 1 },
  "metrics_absent_reason": null
}

$ agent-scaffold status --json --source <S>/projA/docs/plans/p.plan.toml \
    --metrics <S>/foreign/docs/metrics/workflow.jsonl
{
  "plan": null,
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project"
}
```

`"plan": null` in both, because the Markdown-primary source is not read as a plan, which is what the sentence is about. But the log is COUNTED in the first and WITHHELD in the second. Same on `next`:

```
$ agent-scaffold next --json --metrics <S>/foreign/docs/metrics/workflow.jsonl
  "task": "task",
  "source": "no plan source",
  "metrics": { "records": 1

$ agent-scaffold next --json --source <S>/projA/docs/plans/p.plan.toml \
    --metrics <S>/foreign/docs/metrics/workflow.jsonl
  "task": "p",
  "source": "no plan source",
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
```

A Markdown-primary `--source` is not neutral. It supplies a CONTAINMENT ROOT, through `containment_roots` falling back to `resume_roots` when `checked_plan_root` is `None` (`src/main.rs:1436-1437`), and that root withholds artifacts an otherwise identical invocation reads. It also changes the derived task slug and therefore the default ledger path.

### What falsified it, and when

`git log --oneline -S "so a Markdown-primary or absent source is unaffected" -- src/main.rs` returns `e05e71f` and `e30bba8`; the `src/plan/source.rs` site returns `e30bba8`; the `toml_source` site returns `8017a2c`. All three commits PREDATE this step, so all five sites were true when written: before inc2, a Markdown-primary `--source` supplied nothing to any predicate.

WHAT MADE THEM FALSE IS THIS STEP'S OWN INC2, and specifically `269d075` ("supply a root to the surfaces that read no plan, and pin six unguarded clauses"). Its own message states the change: the surfaces that read no plan previously "fell through with NO root", and it made the anchors supply one. Inc2's feature commit `8beb1c2` alone did NOT falsify them, because `checked_plan_root(false, source, plan)` returns `None` with no `--plan` and both filters went vacuous. `269d075` is the same commit that falsified `R4B-1`'s two comments, so this is `R4B-1`'s falsifier reaching two more files and three more sites.

The `containment_roots` doc comment at `src/main.rs:1409-1417` DESCRIBES the new behaviour correctly and names the case by name ("`status` and `next` reach that same configuration with a Markdown-primary `--source` and no `--plan`"). So the tree contains an accurate description of the change and five sites that still promise the opposite, and the accurate one is the one nobody reads first: the two `--help` strings are what an operator sees.

### Severity: `medium`, and I weighed `high` and `low`

AGAINST `high`: the divergent behaviour is the DESIGNED and CORRECT one, it is the fail-safe direction, and it reports its own reason on both surfaces. Nobody is led into a wrong action; they are led into surprise.

AGAINST `low`: two of the five sites are clap `--help` text, which is the tool's primary self-description and is not a maintainer-only artifact. The promise is specifically that supplying this flag COSTS NOTHING, and the actual cost is that a run stops counting the log it counted a moment ago. An operator who adds `--source` to a `status` invocation on a Markdown-primary project and finds `metrics: unavailable` has been told by `--help` that this cannot happen. Five sites of one false universal, in three files, is also the twin-site pattern `Q-55-twinsites` records this task being bitten by repeatedly.

### Minimal remedy: DELETION at all five, no authoring

Each site's remedy is to delete the trailing "so ... is unaffected" clause. Every sentence carries its real content before that clause (which substrate is read), and the clause is an AFFIRMATIVE EXHAUSTIVENESS CLAIM ABOUT DERIVED OUTPUT, which is the class the human authorised deleting rather than narrowing on 2026-08-02 (ledger `:957`, four data points `W1A-3`, `W2B-4`, `W2B-3`, `W3A-2`), and which commit `8060898` already applied to a `src/main.rs` comment inside inc4 itself. A deleted claim cannot be falsified at an edge.

`src/workflow.rs:151-152` carries a sibling sentence ("A repo with no `<task>.plan.toml`, or one whose `[meta].primary` is `markdown`, uses this path and is byte-for-byte unaffected") and I confirmed it is TRUE and must NOT be changed: it is scoped to `check_workflow`'s own code path, which the step did not touch. I checked it rather than sweeping it in by keyword.

---

## `R5A-4` (low): inc4 corrected `Projection.plan`'s first sentence and left the second one asserting there is "exactly one cause", which the correction itself made incongruous

### The claim

`src/main.rs:569-572`, with the inc4 edit shown:

```
struct Projection {
-   /// The plan projection, present only when a readable `--plan` was given. It carries no
+   /// The plan projection, present when a TOML-primary `--source` or a readable `--plan` supplies one. It carries no
    /// reason field: there is exactly one cause, so a reason there would inform nobody.
    plan: Option<PlanProjection>,
```

`c6c848d` ("docs: make the step's own claims current and specify inc4") rewrote line 570 and left line 571 untouched. The result names TWO suppliers and then, in the next clause, asserts ONE cause.

### Measured: five distinct inputs, one bare `null`, three different stderr diagnoses

```
$ agent-scaffold status --json                                          # (a) nothing supplied
  "plan": null                                                          # no stderr note

$ agent-scaffold status --json --source <S>/projA/.../p.plan.toml       # (b) Markdown-primary source
  "plan": null                                                          # no stderr note

$ agent-scaffold status --json --plan <S>/typo.md                       # (c) --plan not there
note: --plan <S>/typo.md does not exist
  "plan": null

$ agent-scaffold status --json --source <S>/typo.plan.toml              # (d) --source not there
note: --source <S>/typo.plan.toml does not exist
  "plan": null

$ agent-scaffold status --json --source <S>/bad.plan.toml               # (e) --source does not parse
note: --source <S>/bad.plan.toml did not parse as a `<task>.plan.toml`; projecting from --plan
  "plan": null
```

The TOOL ITSELF emits three different sentences for these, so the tool distinguishes causes that the comment says number one. And `note_missing_anchors`'s own doc comment, in the same file at `src/main.rs:1136-1138`, says the conflation of two of them is worth a dedicated stderr line:

> `source: no plan source` prints identically for "no plan was asked for" and "the plan you named is not there", so without this the only visible consequence of a typo is a containment verdict reached against a name with nothing behind it.

Case (b) is the one inc4's own edit created. Before the edit the field's stated supplier was `--plan` alone, so "no readable `--plan`" was a coherent single cause. After it, a `--source` can be SUPPLIED, PRESENT, READABLE and VALID and the field is still `null`, because it declares `[meta].primary = "markdown"`. That route did not exist in the sentence the comment was written for.

### Why this is not `R1A-7`, which was dismissed

`R1A-7` was raised in round 1 and DISMISSED. Its subject was the ENCLOSING STRUCT doc comment at `src/main.rs:561-567` and its "a missing plan" clause, and the triage dismissed it on three grounds, one of which was that `:561-567` "is a different comment, is not on the closed list".

MY SUBJECT IS THE OTHER COMMENT, and the round 1 triage's own words put it on the closed list:

> `Q-55-currencyscope` names `Projection.plan`'s doc comment, which is the FIELD comment at `src/main.rs:570-571`. The pass corrected it and acceptance check 22 measures the correction [...]

So `:570-571` is the site a human decision named for this increment, the triage recorded that "the pass corrected it", and the correction reached the first of its two sentences. The round 1 reviewer examined `:570` and wrote "The corrected field comment at `:570` is TRUE and complete on the positive direction" without opening `:571`. The round 3 detectability lens QUOTED BOTH LINES VERBATIM while checking acceptance check 22 and read past the second. Check 22 itself only asks that the field comment "no longer says the plan half is 'present only when a readable `--plan` was given'", which `:570` satisfies, so the check passes over `:571` by construction.

The round 1 triage's third dismissal ground is the one that cuts my way rather than against it:

> this step's own human-authorised round-3 sweep ruled that affirmative exhaustiveness claims about derived output are DELETED rather than narrowed, on four data points inside this step.

"there is exactly one cause" IS an affirmative exhaustiveness claim about derived output. The sweep that class authorised did not reach it.

### The counter-argument, put and answered

The sidecar's own specification at `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (the "WHAT WAS CONSIDERED AND DELIBERATELY NOT ADDED" paragraph) reads: "that field has exactly ONE cause (no readable plan source was given)". Under that coarse framing, all five routes above collapse to "no readable plan source", and the comment is defensible.

I do not think that survives inc4's own edit, and this is why I raise it rather than dropping it. Route (b) supplies a plan source that IS readable and IS valid; what disqualifies it is its declared `primary`, not its readability. The corrected `:570` says so in the same breath ("a TOML-primary `--source` ... supplies one"). But I record the counter-argument because it is a live one and it is why this is `low` and not `medium`.

### Severity: `low`, and I weighed `medium`

FOR `medium`: it sits at the one source site a human decision (`Q-55-plandoccurrency`) named for this increment; the increment's OWN edit is what made it incongruous; and it is the increment's declared failure mode ("a pass that re-tenses a false claim can write a NEW false claim in its place ... a reviewer must check what was written and not only what was removed"), which the plan records as the FIRST of the two factors that make inc4 `risky`.

WHAT HOLDS IT AT `low`: the sentence is a design rationale for something that was deliberately NOT built, not a definition of a shipped contract token. No consumer reads it, no operator sees it, and the coarse reading above is available to a careful reader. The worst outcome is that a future maintainer weighing a reason field on `plan` is told the question is settled when it is not.

### Minimal remedy: DELETION, one clause

`src/main.rs:571` becomes "It carries no reason field." The design rationale that justifies it already lives in the sidecar, where a human decided it and where it can be qualified without touching the source. Alternatively delete the second sentence entirely; the field's meaning is complete after `:570`.

---

## Negative results, recorded because a lens that finds nothing in a region should say so

- ALL 26 CLAP `--help` CLAIMS ABOUT PATH RESOLUTION AND EXIT CODES HOLD except the two in `R5A-3`. I ran the `--workflow` help string's five behavioural claims (TOML-primary needs no `--plan`; the Markdown path still needs `--plan`; neither is an error; no log at the resolved path is an error; a path the check cannot answer for is an error) and all five are exactly as documented, including the errno-naming variant. The `--metrics` default-resolution sentence is correct on all four of its branches.
- THE SEVEN "SOME EXACTLY WHEN" BICONDITIONALS ALL HOLD (`Projection::metrics_absent_reason`, `NextProjection::metrics_absent_reason`, `::resume_state_absent_reason`, `::no_active_loop_reason`, `::metrics_absent_note`, `::resume_state_absent_note`, `NextInputs::metrics_absent_reason`). Each is set in the same branch as its partner in `run_status` and `run_next`, and `project` sets `no_active_loop_reason` from `active_loop` directly.
- `NoActiveLoopReason`'s "TWO STEP-DERIVED ANSWERS, NOT THREE" claim HOLDS, and I checked the structural reason rather than the test: `StepPhase`'s seven variants partition exactly into `InProgress`, `is_pending()` (`NotStarted`, `Next`) and `is_terminal()` (`Complete`, `Skipped`, `Optional`, `Deferred`), so `select_active_loop` returns `None` if and only if `steps_leave_no_loop` is true.
- `NoActiveLoopReason::MetricsNotThisProject`'s definition, which DOES still attribute the root to a plan ("the round log resolved for this plan is not the plan's own"), is TRUE and I deliberately do not fold it into `R5A-1`. That variant is reachable only when `!steps_leave_no_loop(steps)`, which requires non-empty non-terminal steps, which requires a plan to have been read, which makes `checked_plan_root` the root supplier. Its `human_text()` string at `src/next.rs:157` is correct for the same reason.
- `tests/validate_workflow_toml_source_needs_no_plan.rs`'s mode 111 claim, which the file states but does not run, HOLDS. `chmod 111` on the log's directory: `docs/metrics/workflow.jsonl: 1 records, valid` and `workflow invariants hold` at exit 0. Search permission on the ancestor is the discriminator, exactly as the comment says.
- `unpairable_log_note`'s claim to be "used by `status` and by `next` (in both the metrics line and the no-loop reason)" HOLDS; `unpairable_ledger_note`'s claim to be "printed verbatim by `status --resume` and echoed by `next`" HOLDS. I reproduced all four call sites.
- `src/workflow.rs:151-152`'s "byte-for-byte unaffected" claim HOLDS as scoped to `check_workflow`'s own path.
- NO FIX-INDUCED REGRESSION IN THE SOURCE. `cargo test --release` is green at `cf9ff9c`, including all four tests in `tests/validate_workflow_toml_source_needs_no_plan.rs` and all twenty in `tests/unsafe_pairings_are_refused_and_omitted.rs`.

## Fixture safety

Everything was built under `<scratchpad>/rev-inc4-r5-a/`, in the subdirectory named for this review. Nothing was written to bare `/tmp` and nothing outside that subdirectory was deleted. Every `chmod` was restored: `ls -ld` after each block confirms `drwxr-xr-x` on `mode111/docs/metrics` and `mode111/docs/plans`. No file in the worktree was modified except this findings file.
