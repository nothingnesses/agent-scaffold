# `workflow-enforcement-tier-inc3` work review, round 1, DOCUMENTATION-TRUTH lens

Branch `review/inc3-r1-doctruth` off `cd257dd` (`main..HEAD` = `2356473`, `cd257dd`). Every claim below was checked by running a command; the commands and their output are quoted in full so the triager can re-run them.

Fixtures live under `/tmp/claude-1000/-home-jessea-Documents-projects-agent-scaffold/2fed83bd-4a13-402b-9e76-143356c0d130/scratchpad/doctruth/`, built with the worktree's own `target/debug/agent-scaffold`. `TMPDIR` was pointed at `.../scratchpad/doctruth/tmp` for `cargo test`.

Four findings: two `medium`, two `low`. No `high` or `critical`.

---

## `DOC-1` (medium): the enforcement boundary is written at the `--instrument` FLAG, but the tool tests for the LOG, and a freshly instrumented project is on the wrong side of the written boundary

### The text

`pack/AGENTS.md`, the "Worktree lifecycle and merge-back" paragraph, and its two deployed renders (`AGENTS.md`, `.agents/AGENTS.reference.md`), which are byte-identical here:

> Correctness against the plan is established by the review loop and the acceptance review, not by the merge; when instrumentation is on, the deterministic `validate --workflow` check, once built, is the backstop that the required reviewed rounds happened before a step is marked complete, **and a project scaffolded without `--instrument` has no round log for it to read, so on such a project that check exits non-zero reporting that it could not run rather than passing**.

`README.md`, the `validate` paragraph:

> That is the boundary between the two enforcement tiers, and **a project scaffolded without `--instrument` keeps no round log, so `--workflow` fails there**; plain `validate` without `--workflow` is unaffected and still notes an absent log on stderr at exit 0.

`CHANGELOG.md`, `## [Unreleased]` / `### Changed`, the new first bullet:

> **THE POPULATION THIS BREAKS is every project scaffolded without `--instrument`**, which keeps no round log at all: such a project has the guidance tier of the workflow and not the deterministic one, and `validate --workflow` now says so rather than passing. The scaffolded `AGENTS.md` carries the same boundary on the sentence that promises the backstop, so a reader of that sentence can predict the exit code.

### Why it is false or incomplete

All three draw the tier boundary at the `--instrument` FLAG. The code draws it at `metrics_path.exists()` (`src/main.rs:run_validate`, the `_` arm of the `--workflow` match). Those are different sets, because **`--instrument` creates no `docs/metrics/` directory and no round log**. It renders the `{{instrument}}` block into `AGENTS.md` and nothing else (`src/main.rs:build_assets`, `instrument_block`). So a project scaffolded WITH `--instrument` is in exactly the same state as one scaffolded without it until an orchestrator writes the first round record, and `validate --workflow` answers it identically.

The affirmative half of the pack sentence, "when instrumentation is on, the deterministic `validate --workflow` check ... is the backstop", plus the contrastive scoping of the second half ("a project scaffolded WITHOUT `--instrument` ... so on SUCH a project"), tells a reader that turning instrumentation on puts them on the working side. It does not. The first `validate --workflow` on a fresh `--instrument` project exits 1 with a message the guidance has just told them belongs to projects scaffolded without the flag, so the guidance leads an orchestrator to misdiagnose its own tier.

This is also the exact property acceptance check 20 asks for ("a reader of that sentence alone must be able to predict check 15's exit code") and that the CHANGELOG asserts in its own words ("so a reader of that sentence can predict the exit code"). The sentence supports that prediction only over the population check 15 exercises. Rule 2 of this lens: a negative claim is bounded by the dimensions the fixture varied. The fixture varied only the no-`--instrument` case, and the claim was transcribed as if the flag were the discriminator.

### Evidence

Both fixtures built from the branch binary, one without and one with `--instrument`:

```
$ BIN=.../rev-inc3-r1-doctruth/target/debug/agent-scaffold
$ $BIN scaffold --output-dir "$S/noinst" --write --force --principles default
Wrote to .../noinst (30 changed, 0 left untouched).
$ $BIN scaffold --output-dir "$S/inst2" --write --force --principles default --instrument
Wrote to .../inst2 (30 changed, 0 left untouched).

$ ls "$S/noinst/docs"
plans
$ ls "$S/inst2/docs"
plans
$ grep -c "Instrumentation (metrics logging)" "$S/inst2/AGENTS.md"
1
```

`--instrument` changed the guidance and created no `docs/metrics/`. Confirmed at the file level: `diff -rq` between the two trees reports exactly two differing files, `AGENTS.md` and `.agents/AGENTS.reference.md`, and no file present in one and absent from the other.

Acceptance check 15, run in each fixture:

```
$ (cd "$S/noinst" && $BIN validate --source docs/plans/TEMPLATE.plan.toml --workflow)
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
exit=1

$ (cd "$S/inst2" && $BIN validate --source docs/plans/TEMPLATE.plan.toml --workflow)
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
exit=1
```

Byte-identical output and the same exit code on both sides of the boundary the three sentences describe. The backstop sentence itself is also byte-identical in both fixtures' `AGENTS.md` (`grep -o "Correctness against the plan is established by the review loop[^.]*\.[^.]*\." <fixture>/AGENTS.md` returns the same string), so the pack does not adapt the qualifier to the rendered tier either.

Corroborating: nothing in `src/` ever creates the directory. `grep -rn "create_dir" src/` returns no site under a non-test path that targets `docs/metrics`, and `--instrument`'s own help is accurate about this where the prose is not: "render calibration-logging INSTRUCTIONS into the guidance so the workflow records metrics to `docs/metrics/workflow.jsonl`".

### Smallest remedy

Re-scope the second clause from the flag to the log, in each of the three artifacts. This is a substitution of equal or shorter length, not an addition, and it leaves the sidecar's "when instrumentation is on" qualifier (which is a true statement about which tier owns the backstop) untouched.

- `pack/AGENTS.md`: "and a project scaffolded without `--instrument` has no round log for it to read, so on such a project that check exits non-zero" -> "and on a project with no round log yet, which every project scaffolded without `--instrument` remains, that check exits non-zero". Regenerate the two deployed copies.
- `README.md`: "a project scaffolded without `--instrument` keeps no round log, so `--workflow` fails there" -> "`--workflow` fails on any project with no round log yet, which is every project scaffolded without `--instrument`".
- `CHANGELOG.md`: "THE POPULATION THIS BREAKS is every project scaffolded without `--instrument`, which keeps no round log at all" -> "THE POPULATION THIS BREAKS is every project with no round log at the resolved path, permanently for a project scaffolded without `--instrument` and until its first round record for one scaffolded with it".

If the triager prefers to change nothing in the CHANGELOG, the pack sentence is the one that must move: it is the one acceptance check 20 binds and the one an agent reads at runtime.

---

## `DOC-2` (medium): `once built` is retained in a sentence that now reports what the check does today, so the sentence contradicts itself and defers a check that exists

### The text

`pack/AGENTS.md`, the same "Worktree lifecycle and merge-back" paragraph, and its two deployed renders:

> the deterministic `validate --workflow` check, **once built**, is the backstop that the required reviewed rounds happened before a step is marked complete, and a project scaffolded without `--instrument` has no round log for it to read, so on such a project **that check exits non-zero reporting that it could not run** rather than passing.

### Why it is false

`validate --workflow` is built. The clause was already stale before this change; what this change does is put a present-tense report of that check's runtime behaviour into the same sentence as the clause that says it does not exist yet. A reader is told, in one sentence, both that the backstop is future work and what its exit code is today. Only one of those can be acted on, and the one that is false is the one that tells an orchestrator not to bother running it.

This matters more than a stale word normally would, because the sentence is the scaffolded guidance an orchestrator reads to decide whether the deterministic backstop is available to it. "Once built" says no. The step exists to make the answer yes.

The sidecar quotes the passage verbatim including `once built` under "THE PASSAGE", but its instruction is about what the qualifier must ADD; it does not assert that the quoted clause is currently true. This is lens rule 3 applied to a retention rather than a deletion: the sentence was edited, and the edit made a false neighbouring clause load-bearing.

### Evidence

The check runs, on this branch, with no `--plan` at all:

```
$ cd .../rev-inc3-r1-doctruth
$ ./target/debug/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 276 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

`workflow invariants hold` is the backstop reporting a pass. The check is not "once built"; it is built, and the same paragraph's new clause reports its behaviour on a second input.

Site count: `grep -rn "once built" pack/ README.md AGENTS.md .agents/` returns exactly three lines, `pack/AGENTS.md:93`, `AGENTS.md:93`, `.agents/AGENTS.reference.md:93`, which is the one pack source plus its two renders. There is no other site to keep consistent.

### Smallest remedy

Delete the two words. In `pack/AGENTS.md`: "the deterministic `validate --workflow` check, once built, is the backstop" -> "the deterministic `validate --workflow` check is the backstop". Regenerate the two deployed copies with `cargo run -- scaffold --output-dir . --write --force --principles default --instrument`.

This is a pure deletion, which is the class of fix the sidecar's own note prefers.

---

## `DOC-3` (low): the README still describes accepted cost (i) as a note, on an invocation that now exits 1

### The text

`README.md`, the anchoring paragraph, closing sentence (the passage beginning "The round log is resolved FROM THE PLAN"):

> One consequence to know about: a bare filename run from inside `docs/plans` (`cd docs/plans && agent-scaffold validate --source my-task.plan.toml --workflow`) has no parent directories to derive a root from, so it looks for `docs/metrics/workflow.jsonl` beneath `docs/plans` and **reports that it found no log**; run it from the project root instead.

### Why it is incomplete

The behaviour is correct and pinned; I am not asking for it to change. The sentence describing it is stale in one respect: the quoted invocation carries `--workflow`, so after this increment it does not merely "report that it found no log", it FAILS. "Reports that it found no log" was written against the inc1 answer (the stderr miss note at exit 0) and reads, in a paragraph whose other failure cases are all explicitly labelled "exit 1" or "exits non-zero", as the benign one.

The sidecar's own INC3 documentation-impact list does not name this site, and its acceptance check 18 states the correction that the README did not receive: "After inc1 alone: the stderr miss note and exit 0. After inc3: a HARD FAILURE naming the path it looked for." The test that pins the behaviour was renamed for exactly this reason (`a_bare_filename_from_inside_docs_plans_stays_a_silent_miss` -> `..._stays_a_miss_and_now_fails_loudly`) and the CHANGELOG entry calls the case out by name; only the README sentence about the same case was left at the old answer.

### Evidence

```
$ cd "$S/noinst/docs/plans"
$ .../agent-scaffold validate --source TEMPLATE.plan.toml --workflow
no metrics log at docs/metrics/workflow.jsonl; nothing to validate
--workflow requested but no round log at docs/metrics/workflow.jsonl: the workflow check could not run, so it cannot report that the invariants hold; pass a `--metrics` naming this project's log, or record the project's review rounds there
exit=1
```

### Smallest remedy

One word. "reports that it found no log" -> "fails, naming the log it looked for". Same length, no new sentence.

---

## `DOC-4` (low): the CHANGELOG's `Added` entry for `--workflow` says it requires `--plan`, in the same unreleased section as the new `Changed` entry that says it does not

### The text

`CHANGELOG.md`, `## [Unreleased]` / `### Added`, the `validate --workflow` bullet:

> `validate --workflow` cross-references a plan's Roadmap against the round log (`src/workflow.rs`) ... **It requires `--plan`** and reuses the same metrics log as the rest of `validate`.

### Why it is false

`--plan` has not been required since the Inc 6 clap relaxation. `ValidateArgs::plan` is a plain `Option<PathBuf>` with no `required`, and `ValidateArgs::workflow` carries no `requires = "plan"`. Both bullets sit in the same unreleased section and will ship in the same release notes, where the new `Changed` bullet ("whether the plan came from a TOML-primary `--source` or a Markdown `--plan`") and the new `--workflow` help ("A TOML-primary `--source` needs no `--plan`") contradict the older `Added` bullet directly.

I raise this at `low` and flag its provenance honestly: this sentence was falsified by an earlier increment, not by inc3. It is in scope for this lens only because inc3 edits the same section about the same flag, the entry has not shipped, and a release note that contradicts itself about a flag's required arguments is a false claim a user will act on.

### Evidence

```
$ cd .../rev-inc3-r1-doctruth
$ ./target/debug/agent-scaffold validate --source docs/plans/agent-scaffold.plan.toml --workflow
docs/metrics/workflow.jsonl: 276 records, valid
docs/plans/agent-scaffold.plan.toml: 95 steps, 69 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

No `--plan`, no usage error, the check ran. The suite pins the same property in `tests/validate_workflow_toml_source_needs_no_plan.rs::workflow_on_a_toml_source_runs_without_a_markdown_plan`, which passes on this branch.

### Smallest remedy

Delete three words: "It requires `--plan` and reuses" -> "It reuses".

---

## Claims I checked that turned out to be TRUE

Recorded so a later round does not re-check them, and so the negative results are bounded by what was actually run.

1. **Acceptance check 15** holds on the fixture it is written for. Non-instrumented fixture, `validate --source docs/plans/TEMPLATE.plan.toml --workflow`: exit 1, and the problem names the resolved log path (`docs/metrics/workflow.jsonl`) and says "the workflow check could not run". Verified above.
2. **Acceptance check 16** holds. Same fixture, `validate --source docs/plans/TEMPLATE.plan.toml` with no `--workflow`: exit 0, stderr `no metrics log at docs/metrics/workflow.jsonl; nothing to validate`, stdout `docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid`. Plain `validate` is untouched.
3. **Acceptance check 20's drift half.** `cargo test` with `TMPDIR` outside any repository: 421 passing, 0 failing, across all binaries. `agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render` and `::the_committed_role_prompts_match_a_fresh_render` both pass, so both deployed copies match a fresh render of the edited pack. I confirmed independently that the fixture `AGENTS.md` carries the new sentence.
4. **The CHANGELOG's cross-product claim** ("whether the path came from the anchored default or from an explicit `--metrics`, and whether the plan came from a TOML-primary `--source` or a Markdown `--plan`") is true on all FOUR cells, not just the three the new test asserts. The test covers TOML+default, Markdown+default and TOML+explicit; I ran the fourth, Markdown `--plan` with an explicit missing `--metrics` inside the root, and got the same problem naming `docs/metrics/absent.jsonl` at exit 1.
5. **The precedence between the containment refusal and the new problem** does not falsify the CHANGELOG's "reports a problem naming that path". With an explicit `--metrics` that is both missing AND outside the plan's root, the containment refusal answers instead of the new problem, and it still names that path and still exits 1.
6. **The CLI problem message's omission of `--instrument` was CORRECT.** The lens asked whether the flag creates `docs/metrics/`. It does not (evidence under `DOC-1`), so naming it as a remedy would have been a false remedy: re-scaffolding with `--instrument` leaves the exit code at 1. The implementer's stated reason for the omission is borne out. What the omission does expose is `DOC-1`, since the pack and README statements draw the boundary as if the flag were the fix.
7. **The `run_validate` doc comment is true clause by clause**, and the two clauses it deleted were both already false, so nothing true was removed. `With --workflow (which still requires --plan)` and `--plan stays clap-required for now (the relaxation for a TOML-only project is deferred)` are both falsified by `ValidateArgs::plan` being an unrequired `Option<PathBuf>` (see `DOC-4`'s evidence). The surviving text checks out: the retained "not a validation failure on its own ... prints a note to stderr and is skipped" is now correctly hedged by "on its own" and by the new exception paragraph; "Both of the check's inputs answer that way: no resolvable plan source, and no round log at the resolved metrics path" enumerates exactly the two arms that push a problem; and "if a needed file is absent the run reports it as a problem and exits non-zero" matches every run above.
8. **The `ValidateArgs::workflow` help addition** is true: "So is no round log at the resolved path at all: the check cannot run, and a check that did not run must not report success." The chain of "is an error, and so is ..., So is ..." reads correctly against the three problem-pushing paths.
9. **The README `validate` paragraph does now say what the sidecar asked for**: "A `--workflow` run that cannot see a round log is itself one of those failures rather than a skip ... it reports that and exits non-zero instead of reporting success for a project it never checked", and "plain `validate` without `--workflow` is unaffected and still notes an absent log on stderr at exit 0". Both verified by runs 1 and 2 above.
10. **The `status`/`next` README paragraph did not go stale.** It says the projections "never fail on a missing or malformed file" and "All three still EXIT 0", and explicitly contrasts itself with the validator's refusal. This increment adds a second validator-only non-zero exit, which the paragraph's framing already covers, and the projections' behaviour is unchanged.
11. **`pack/instrument.md` did not go stale**, as the sidecar predicted. Its only `validate` sentence, "The log can be checked against this schema with `agent-scaffold validate`, which exits non-zero and reports any malformed record", describes plain `validate` on a malformed record and is unaffected.
12. **The role prompts did not go stale.** `grep -rn "validate --workflow\|agent-scaffold validate" pack/prompts pack/user-prompts` returns nothing; the only pack sites are `pack/AGENTS.md:93` and `pack/instrument.md:5,9,13`, all checked above.
13. **The test module docs are true.** The rewritten header of `tests/validate_workflow_toml_source_needs_no_plan.rs` claims three directions and the file asserts four runs covering them; its stated RED (`--workflow has a plan source but the metrics log is missing; skipping the workflow check` at exit 0) matches the string the change deletes from `src/main.rs`. The corrected comment inside `workflow_with_no_plan_source_hard_errors_instead_of_skipping` ("a missing log is now its own hard error ... so a present log is what proves THIS hard error is about the plan source") is now the correct justification for the empty log that test writes, where the old text asserted the soft-skip this increment removes. The renamed accepted-cost test's doc matches `DOC-3`'s measured behaviour.

## Judged and NOT raised

- **The doubled stderr output.** A failing run prints `no metrics log at <path>; nothing to validate` and then the new problem about the same path. Both are true statements, they are in causal order, and the second is the one that carries the exit code. Suppressing the first would be a code change with no truth-value defect behind it, so under this lens's "a style preference is not a finding" rule I do not raise it.
- **The CLI problem message's second remedy**, "or record the project's review rounds there". A non-instrumented project's `AGENTS.md` omits the round-record schema by design, so a reader of that project cannot follow the instruction from their own docs; but the instruction is not false, the path is named, and every remedy I could write for it is more words on a message that is already long. Raising it would trade a true-but-terse message for a longer one, which is the fix-pass pattern this project has measured as manufacturing the next round's findings.
- **`docs/plans/agent-scaffold.md` and `docs/plans/agent-scaffold.plan.toml`** both carry present-tense descriptions of the pre-fix behaviour (`the skip IS announced, twice ... The operative defect is the EXIT CODE alone`). These are the `Q-55` question text and its recorded correction, a historical record of the defect as raised, not a claim about the shipped tool. Rewriting a decided question's provenance to match the fix would destroy the record.
- **Line length and wrapping** anywhere. Out of scope by instruction.
- The four accepted residuals and accepted costs (i) through (iv) were not raised. `DOC-3` concerns the README sentence DESCRIBING accepted cost (i), not the cost itself, which I confirm is pinned and behaving as check 18 specifies.
