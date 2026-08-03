# `workflow-enforcement-tier-inc2`, work review round 1, ISOLATED TRIAGE

ARTIFACT. `git diff main..HEAD` at commit `1543325` on `impl/wet-inc2`: `src/main.rs`, `src/next.rs`, `tests/unsafe_pairings_are_refused_and_omitted.rs`, `README.md`, `CHANGELOG.md`. Triaged in the worktree `.claude/worktrees/triage-inc2-r1`.

SPECIFICATION. `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`, read in full (390 lines).

THE THREE SOURCE FILES, read on main:

- `docs/plans/workflow-enforcement-tier-inc2.reviews/workflow-enforcement-tier-inc2-reviewer-adversarial.md` (ADV-1 high, ADV-2 medium).
- `docs/plans/workflow-enforcement-tier-inc2.reviews/workflow-enforcement-tier-inc2-reviewer-conformance.md` (no findings, one required ruling).
- `docs/plans/workflow-enforcement-tier-inc2.reviews/workflow-enforcement-tier-inc2-reviewer-evidence.md` (EVI-1 high, EVI-2 to EVI-5 medium, EVI-6 low).

BASELINE I ESTABLISHED FIRST-HAND, before any verdict below, with `TMPDIR` pointed outside every repository as the specification requires at line 311:

```
cargo build                                -> Finished dev profile
cargo test                                 -> 413 passed, 0 failed, across 9 binaries
cargo clippy --all-targets -- -D warnings  -> clean
cargo run -- render docs/plans/agent-scaffold.plan.toml --check -> up to date
```

METHOD. Every verdict below was produced by building a fixture on disk and running the built binary (`target/debug/agent-scaffold`) against it, or by applying the named mutation, running the FULL suite, observing the behavioural difference, and reverting. Nothing was concluded from reading a reviewer's file. Seven mutations were applied and reverted one at a time; the working tree was confirmed clean after each.

---

## 1. THE RULING ON THE TWO `next` LEAK REPRODUCTIONS

**They are TWO DEFECTS, not one. ADV-1 is a live, in-scope hole in the predicate's APPLICATION. The conformance lens's construction is genuinely the IN-ROOT BOUND and was correctly not filed. The conformance lens did NOT mis-attribute a live defect to an out-of-scope bucket; it never built the configuration ADV-1 found, and one sentence of its ruling is false as a result.**

### 1.1 The discriminating control I built

The two constructions differ in exactly one structural fact, and I isolated it by running the SAME files twice, once nested and once disjoint.

Fixture: an OUTER project with a Markdown Roadmap carrying `borrowed-step` at `in-progress`, its own empty log, and its own ledger. An INNER project with a Markdown-primary `p.plan.toml`, a log holding one converged round for `borrowed-step`, and a ledger whose `## RESUME STATE` block says `INNER resume state, must not leak.` Command in both cases: `next --source <inner>/docs/plans/p.plan.toml --plan <outer>/docs/plans/p.md`, so the CHECKED plan is outer's Markdown and the anchor is inner's Markdown-primary source.

CASE 1, inner NESTED at `$R/n/outer/inner`:

```
metrics: 1 records
  state: ready-to-plan
  streak: 0/?
  rounds: 0/5
  next: spawn a planner to draft the step plan
RESUME STATE (verbatim from the ledger):
## RESUME STATE
INNER resume state, must not leak.
exit=0
```

CASE 2, BYTE-IDENTICAL files, inner moved out to the sibling `$R/d/sibling`:

```
metrics: unavailable, the round log $R/d/sibling/docs/metrics/workflow.jsonl is not under the plan's project root $R/d/outer, so its records cannot be paired with this plan
no active review loop (the round log $R/d/sibling/docs/metrics/workflow.jsonl is not under the plan's project root $R/d/outer, so its records cannot be paired with this plan)
the ledger $R/d/sibling/docs/plans/p.ledger.md is not under the plan's project root $R/d/outer; nothing to resume
exit=0
```

The only variable is whether `inner` sits inside `outer`. Nested leaks both artifacts; disjoint refuses both. That isolates the conformance lens's cause to NESTING and nothing else.

### 1.2 Question 1: one defect or two? TWO

I reproduced ADV-1's construction independently, from ADV's own self-contained script, against my own build:

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
  "resume_state": "## RESUME STATE\n\nBETA-PRIVATE: branch feat/secret, ...",
  "resume_state_absent_reason": null,
```

`alpha` and `beta` are TOP-LEVEL SIBLINGS. Neither is inside the other. There is no nesting anywhere in this construction, so nesting cannot be its cause, and the in-root bound cannot be its explanation.

I also reproduced the ADV-1 shape a second way, on my own fixture with an explicit `--ledger-fragment` naming a disjoint sibling's ledger, and confirmed the same-input contradiction:

```
=== next (no --plan; no plan is READ) ===
metrics: 0 records
no active review loop (no plan steps found)
RESUME STATE (verbatim from the ledger):
## RESUME STATE
OUTER resume state.
exit=0
--- status --resume, SAME anchors, SAME --ledger-fragment ---
the ledger $R/d/outer/docs/plans/p.ledger.md is not under the plan's project root $R/d/sibling; nothing to resume
exit=0
```

And ADV's second spelling, a Markdown-primary `--source` beside a `--plan` that does not exist, reproduces identically: `status --resume` refuses, `next` with the byte-identical argument list prints `BETA-PRIVATE block.`

So the two constructions have DISJOINT causes. ADV-1's cause is that `next` has NO ROOT AT ALL and the predicate never fires, at any distance. The conformance lens's cause is that `next` HAS a root and the foreign artifacts lie inside it. They are two defects, and only one of them is in scope.

### 1.3 Question 2: is the conformance lens's reproduction genuinely the in-root bound? YES

Three independent grounds, and I hold this against the instruction to be rigorous in both directions.

FIRST, the control at 1.1. The same files, the same command, refused when disjoint and passed when nested. Nesting alone accounts for it.

SECOND, the specification states this case in terms, twice, and neither statement is a stretch. Line 183, on `next`'s ledger: "The predicate rooted on the checked plan catches that WHERE THE `--source` LIES OUTSIDE THE CHECKED PLAN'S ROOT; an anchor-rooted one cannot, for the same reason it cannot catch the metrics case, and where it lies INSIDE, neither rooting catches it (the IN-ROOT BOUND below)." Line 229, on the `ledger-not-this-project` variant: "a `--source` in a different project reaches this only when that project is not NESTED inside the root." The conformance lens's construction is a `--source` in a different project that IS nested inside the root. The specification predicted it and excluded it before the code was written.

THIRD, the bound's own definition at line 267 names this exact shape: "a log copied to this plan's own `docs/metrics/`, and equally a NESTED project's own log and ledger at their own conventional paths". Inner's log and ledger sit at inner's own conventional paths. That is the second member verbatim.

The conformance lens's decision not to file was CORRECT, and its evidence is worth keeping: no earlier round had shown that the bound reaches the single-root mechanism and the two-anchor mechanism through the same layout.

ONE SENTENCE OF ITS RULING IS FALSE, and I correct it here because the orchestrator will otherwise carry it forward. The lens wrote that "the one place the two arities disagree is the already-recorded, explicitly out-of-scope IN-ROOT BOUND". It is not the one place. They also disagree on the no-plan-read case, which is reachable between two DISJOINT projects, and that is ADV-1. This is an omission of construction, not a mis-attribution: the lens attributed the case it built to the right bucket and never built the other case. Its filing of zero findings is nonetheless a MISS on ADV-1.

### 1.4 Question 3: is ADV-1's cause correctly diagnosed, and is the no-plan-read path outside the in-root bound? YES to both

THE CAUSE IS DIAGNOSED CORRECTLY, verbatim as ADV states it. `checked_plan_root` (`src/main.rs:1308`) opens with `let checked = if toml_primary { source.as_ref() } else { plan.as_ref() }?;` at `src/main.rs:1313`. With a Markdown-primary `--source`, `toml_primary` is false; with no `--plan` (or a `--plan` that does not canonicalise), the `?` short-circuits and the root is `None`. `run_next` then computes both guards as `checked_root.as_ref().filter(...)` (`src/main.rs:1526-1529` for the log, `:1546-1549` for the ledger), so a `None` root makes both filters vacuous and the ledger is read at `:1550-1559`. `run_resume` (`src/main.rs:1443`) instead calls `resume_roots` (`src/main.rs:1421`) at `:1451`, which roots on the ANCHORS, and refuses. `run_next` never calls `resume_roots`, although the function is 90 lines above it in the same file.

IT IS OUTSIDE THE IN-ROOT BOUND, and the distinction is structural rather than a matter of degree. The in-root bound presupposes a derived root and asks whether an artifact lies inside its subtree; it is a bound on the predicate's REACH. ADV-1 is a case where there is NO root, so the predicate does not run at all, for any artifact, at any distance, including a project on the other side of the filesystem. That is a hole in the predicate's APPLICATION. My CASE 3 run above proves it empirically: two disjoint siblings, no containment relationship of any kind, and the leak still happens.

THE REACHABILITY BOUND, stated honestly because it is the one thing that could argue the severity down. With no plan read, both DEFAULTS anchor on the `--source` (`resolve_metrics_path` and `default_ledger_path` are both source-first), so the default log and the default ledger are always inside the source's own root and are safe by construction. Reaching the leak therefore needs an explicit `--metrics` or an explicit `--ledger-fragment`. That does NOT soften it, because the explicit-flag population is precisely what inc2 exists for: specification line 175 records that "Anchoring changes where the DEFAULT resolves; it does nothing to an EXPLICIT `--metrics` naming a foreign log", and that without `Q-55-refusalscope` "`next --source <foreign plan> --metrics <this repository's log>` would keep emitting `state: converged`". The explicit flag is the increment's own reason to exist, and check 14c pins an explicit `--ledger-fragment` on `status --resume` for the same reason.

### 1.5 The standing residual: does ADV-1 undermine the "one shared primitive at two arities" ruling?

PARTLY, and the amendment matters.

WHAT SURVIVES. `is_outside_root` (`src/main.rs:1351`) and `canonical_project_root` (`src/main.rs:1289`) are single implementations called by all four surfaces. Nothing is re-implemented per surface. One source of truth holds at the function level, and the conformance lens's core ruling is right: this is one primitive at two arities, not two rules.

WHAT DOES NOT SURVIVE. The ruling's implicit claim that the ARITY is the whole of the difference. There are in fact TWO ROOT-SUPPLY POLICIES: `checked_plan_root` (root from the plan the surface reads) for `validate --workflow`, `status` and `next`, and `resume_roots` (roots from the anchors) for `status --resume`. The specification decided the second policy for "the surface that reads NO PLAN" (`Q-55-resumepairing`, line 182). `next` and `status` ALSO read no plan in the ADV-1 configuration, and neither switches policy; they simply fall through with no root. So the residual's correct ruling is: ONE PRIMITIVE, TWO ROOT-SUPPLY POLICIES, AND THE SECOND POLICY IS INCOMPLETELY APPLIED. ADV-1 is the consequence of the gap, not evidence that the primitive is duplicated.

That distinction changes the prescribed fix: it is not "unify the two predicates", it is "give `run_next` and `run_status` the root-supply policy that already exists for the no-plan case".

---

## 2. VERDICT TABLE

| Raw id | Verdict | My severity | Reviewer severity | Dedup group | One-line prescription |
| --- | --- | --- | --- | --- | --- |
| ADV-1 | VALID | high | high | G-ROOT | `run_next` (and `run_status`) must fall back to `resume_roots` when `checked_plan_root` is `None`; authored logic, roughly six lines, reusing an existing function. |
| ADV-2 | VALID | low (DOWN from medium) | medium | G-SLOT | Route as a DECISION, not a silent fix: every available remedy changes behaviour the specification does not cover. |
| EVI-1 | VALID | medium (DOWN from high) | high | G-COV-PREDICATE | One test with the log's LEAF symlinked out of the root; the shipped code is correct, the clause is unpinned. |
| EVI-2 | VALID | medium | medium | G-COV-PREDICATE | One assertion that the refusal is the ONLY problem reported. |
| EVI-3 | VALID | medium | medium | G-COV-VOCAB | Two assertions on `status --json`: its `log-absent` value, and its precedence rule. |
| EVI-4 | VALID | medium | medium | G-COV-VOCAB | One invocation with a `--ledger-fragment` both outside the root and missing. |
| EVI-5 | VALID | medium | medium | G-COV-NOTE | One assertion that `next`'s human stdout carries the unpairable-ledger note. |
| EVI-6 | VALID | low | low | G-COV-NOTE | One assertion on the third remedy in `a_divergent_source_and_plan_pairing_is_refused`. |
| TRI-1 | NEW (triager) | low | n/a | G-DOC | One-line correction to a doc comment shipped by this increment whose justification is false on a reachable path. |

INVALID: NONE. Every raw finding replayed for me.

NO `high` OR `critical` WAS DISMISSED. EVI-1 was RE-SEVERITISED from `high` to `medium` and remains VALID; see section 4.3 for the precise ground, prominently stated.

VALID COUNT AFTER DEDUP: 9. By severity: `critical` 0, `high` 1, `medium` 5, `low` 3.

DEDUPLICATION NOTES. No two raw findings collapse into one. ADV's own "SECONDARY OBSERVATION" (the same missing root passes an unpairable LOG through on `next` and on `status`) is FOLDED INTO ADV-1 rather than filed separately, and I extend ADV-1's prescription to cover `run_status` because I reproduced the `status` half. G-COV-PREDICATE, G-COV-VOCAB and G-COV-NOTE are grouped because each pair shares a cause (an unpinned load-bearing clause, an unpinned vocabulary rule on the surface the specification itself flags as unguarded, an unpinned message), but each member needs its own assertion in its own test, so none of them merge.

---

## 3. THE ONE `high`

### ADV-1: `next` echoes another project's `## RESUME STATE` where `status --resume` refuses it, on identical inputs

VERDICT: VALID. SEVERITY: `high` (UPHELD).

WHAT I RAN. ADV's self-contained script verbatim, against my own `cargo build` of `1543325`, plus two constructions of my own (section 1.2, the disjoint-sibling explicit-fragment case and the ADV second spelling with a nonexistent `--plan`). I also grepped the new test file to check ADV's coverage claim.

WHAT I OBSERVED. Reproduced exactly, in all three spellings, at exit 0, with the leaked block printed verbatim under the heading `RESUME STATE (verbatim from the ledger):`. On `--json`, `"resume_state_absent_reason": null` while `"resume_state"` carries beta's private block.

REASONING.

The defect is not that a wrong path was resolved. It is that on identical anchors and an identical `--ledger-fragment`, the tool gives two different answers, and the permissive one is the surface an agent consumes. Specification line 127 is explicit about what that costs: `status --resume` leaking a block "is not a wrong boundary at all but CONTENT INJECTION into an instruction that the receiving agent has been told is authoritative and to read first", and line 183 requires `next` to omit the echo "with the same note naming the rejected ledger path in its place that `status --resume` prints". `next` prints the block instead of the note.

Three aggravating facts, each of which I confirmed myself.

- THE MACHINE SURFACE REPORTS THE OPPOSITE OF THE TRUTH. `"resume_state_absent_reason": null` does not merely omit an explanation, it positively asserts that the block is a genuine one for this plan. The vocabulary that `Q-55-jsonreason` added exists so a consumer can tell that apart, and here it says the wrong thing rather than nothing.
- IT FALSIFIES DOCUMENTATION SHIPPED IN THE SAME COMMIT. `README.md:236` states without qualification: "Every one of these commands checks that the log (and, for the ledger readers, the ledger) it is about to read lives under the project root of the plan it is about to read". I read that line first-hand in the worktree. `next` is a ledger reader and does not, in this configuration.
- THE SUITE CANNOT SEE IT, and I verified why rather than taking ADV's word. `plan_toml_markdown_primary()` is called at exactly three places in `tests/unsafe_pairings_are_refused_and_omitted.rs` (`:221`, `:490`, `:653`), and every one of them also supplies a `--plan`, which always yields a checked root. `grep -n '"--plan"'` over the file confirms no test drives `next` or `status` with a Markdown-primary source and no `--plan`. There is no configuration in the suite where `checked_plan_root` returns `None`.

WHAT I CONSIDERED AND REJECTED AS A GROUND FOR RULING IT INVALID. Specification line 157 says "Where NO plan is read there is no root, so the predicate does not fire and every surface behaves as it does today". Read alone, that appears to bless `next`'s behaviour. It does not, for two reasons. That sentence is written about `validate --workflow`, and its own continuation says so: "on `validate --workflow` that case is the match's own `(None, None, _)` arm, already a hard problem for its own reason". And it was superseded for the no-plan case by `Q-55-resumepairing` (line 182), a LATER human decision (2026-08-02) whose text is about "THIS SURFACE READS NO PLAN, so the rule SUPPLIES a root rather than being re-implemented per surface". `next` in this configuration reads no plan. The decided rule covers it on its own terms; the implementer applied it only to the surface the specification named by title. This is a specification gap, but it is not one the code could dodge: the code produces a self-contradiction the specification nowhere sanctions, and one of the two answers is definitely wrong.

PRESCRIBED FIX. `src/main.rs:1520` (`run_next`) and `src/main.rs:1149` (`run_status`). AUTHORED LOGIC, roughly six lines, reusing a function that already exists rather than adding one:

- compute the roots as `checked_plan_root(...).map_or_else(|| resume_roots(&args.source, &args.plan), |root| vec![root])`, so a surface that reads a plan keeps the `Q-55-endproperty` rooting unchanged and a surface that reads none gets the `Q-55-resumepairing` rooting `status --resume` already has;
- test each artifact with `roots.iter().find(|root| is_outside_root(artifact, root))`, exactly as `run_resume` does at `src/main.rs:1451-1453`, so the primitive is still called and nothing is re-implemented;
- apply it to BOTH artifacts on `next` (log and ledger) and to the log on `status`, since I reproduced the metrics half on both commands.

Two tests are owed with it: `next` with a Markdown-primary `--source`, no `--plan`, and an out-of-root `--ledger-fragment`, asserting the note and `"resume_state_absent_reason": "ledger-not-this-project"`; and the same shape with an out-of-root `--metrics`, asserting `"metrics_absent_reason": "log-not-this-project"` on `next --json` and `status --json`.

DO NOT extend `resume_roots` to the plan-reading case. Rooting `next` on the anchors whenever a plan IS read would reverse `Q-55-endproperty`, which was decided by the human on 2026-08-02 and is what check 13b separates. The fallback must fire only when `checked_plan_root` is `None`.

WHAT IS NOT IN SCOPE OF THIS FIX, stated so the implementer does not overreach. It does NOT close the conformance lens's nested case. After this fix, my CASE 1 still leaks, and correctly so: that is the in-root bound and it is recorded, not closed.

---

## 4. THE REMAINING FINDINGS

### 4.1 ADV-2: `next` hands an agent the REJECTED ledger path as the loop's `ledger:` slot

VERDICT: VALID. SEVERITY: `low`, RE-SEVERITISED DOWN from `medium`.

WHAT I RAN. ADV's self-contained script, plus the variant ADV describes without an explicit `--ledger-fragment`, which I built myself: a Markdown-primary `--source` in project MA carrying a ledger, a `--plan` in project MB, and an explicit `--metrics` naming MB's own log so the metrics half is SAFE and only the DEFAULT ledger is rejected.

WHAT I OBSERVED. Both reproduce. On the explicit-fragment run, the human output carries `ledger: $R/beta/docs/plans/b.ledger.md` inside the `ACTIVE LOOP` context and, eleven lines later, `the ledger $R/beta/docs/plans/b.ledger.md is not under the plan's project root $R/alpha; nothing to resume`. On the no-flag variant:

```
    ledger: $R/MA/docs/plans/p.ledger.md
the ledger $R/MA/docs/plans/p.ledger.md is not under the plan's project root $R/MB; nothing to resume
exit=0
```

On `--json` the qualifying note is absent entirely, since both note fields are `#[serde(skip)]` (`src/next.rs:198`, `:202`), leaving `context.ledger` unqualified beside a `resume_state_absent_reason` the consumer must think to correlate.

REASONING, AND WHY I MOVED IT DOWN. The finding is real: one output says two contradictory things about one path, and the machine surface drops the half that contradicts. But three things hold it at `low`.

- IT VIOLATES NO WRITTEN REQUIREMENT, and ADV says so honestly in its own scope note. Specification line 183 requires of an unsafe ledger only that "the `RESUME STATE` echo is omitted". The `ledger:` context slot is not mentioned anywhere in the specification.
- THE ADDRESS IS ACCURATE. `next` reports the path `default_ledger_path` genuinely resolved by the decided source-first rule. This is a wrong DESTINATION under a divergent pairing, not borrowed CONTENT, and the human surface prints the contradiction in the same output.
- THE NEAREST PRECEDENT POINTS THE OTHER WAY. Specification line 389 records that `next`'s `review_findings` and `triage_findings` slots "are built from the task name alone and stay relative to the process working directory", so `next --source <a foreign plan>` already "emits one instruction whose `ledger:` is anchored into that project while its `review_findings:` is not", measured at work review on inc1 and explicitly not fixed by any increment here. An unanchored context slot is a known, recorded, unowned residual of this exact family.

PRESCRIBED ACTION: ROUTE AS A DECISION, do not let the implementer choose. Every remedy changes behaviour no decision covers, and the options are genuinely different in kind: omit the `ledger` slot when the ledger is unpairable (which makes an always-present slot sometimes-absent, an `ActiveLoop` contract change); name what the CHECKED plan would resolve to instead of what the anchor did (consistent with `Q-55-endproperty`'s own logic, a one-line change at `src/main.rs:1542-1545`, but it invents a ledger path nobody asked for); or accept and record it beside the line 389 residual. `src/next.rs:1001` (`build_context`) is where the slot is written and `src/main.rs:1542-1545` is where the path is chosen.

### 4.2 EVI-2: nothing pins that the refusal REPLACES the check rather than accompanying it

VERDICT: VALID. SEVERITY: `medium` (UPHELD).

WHAT I RAN. I applied the mutation myself at `src/main.rs:995`, replacing `} else {` with `}` followed by a bare block so the four-arm match runs beside the refusal, and ran the full suite.

WHAT I OBSERVED. SUITE GREEN under the mutation: 413 passed, 0 failed, all 9 binaries ok, all 13 integration tests pass. Fixture: `home`'s log holds a round for `other-step` only, `away` claims `borrowed-step` at `complete`. Mutated build:

```
--workflow would join $R/away/docs/plans/p.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root $R/away; ...
$R/away/docs/plans/p.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `borrowed-step` is `complete` but has no round records and no covering waiver; ...
exit=1
```

Reverted, rebuilt, same command on HEAD:

```
--workflow would join $R/away/docs/plans/p.plan.toml against docs/metrics/workflow.jsonl, which is not under the plan's project root $R/away; ...
exit=1
```

REASONING. The exit code is identical, which is exactly why no test notices. The mutated build says in one breath that it cannot attribute the log to this plan and then reports a W3 verdict on that same pairing, which is the negative-direction assertion the refusal's own comment forbids at `src/main.rs:984-989` and which specification line 104 forbids ("it must say so and exit non-zero rather than proceed"). This is the strongest of the six evidence findings because the requirement is written into the code as a comment and nothing checks it.

PRESCRIBED FIX. `tests/unsafe_pairings_are_refused_and_omitted.rs:165` (`an_explicit_metrics_outside_the_plans_root_is_refused`). AUTHORED TEST LOGIC, small: give the foreign log a record set that does NOT satisfy the borrowed slug, and assert `!stderr.contains("has no round records")`, or count the problem lines exactly. No product change.

### 4.3 EVI-1: the symlinked log LEAF is unguarded

VERDICT: VALID. SEVERITY: `medium`, RE-SEVERITISED DOWN from `high`. **STATED PROMINENTLY, since a `high` moved: this is a DOWNGRADE, not a dismissal. The finding stands and a fix is owed.**

WHAT I RAN. I built the fixture myself: `home/docs/metrics/workflow.jsonl` holds one converged round for `borrowed-step`; `away/docs/plans/p.plan.toml` claims `borrowed-step` at `complete` with no evidence of its own; `away/docs/metrics/workflow.jsonl` is a SYMLINK to `home`'s log. I then applied the mutation at `src/main.rs:1332` (`.ancestors()` to `.ancestors().skip(1)`) and ran the full suite.

WHAT I OBSERVED. HEAD, correct:

```
--workflow would join $R/away/docs/plans/p.plan.toml against $R/away/docs/metrics/workflow.jsonl, which is not under the plan's project root $R/away; ...
exit=1
```

MUTATED, suite fully GREEN (413 passed, 0 failed, all 9 binaries ok), and the false pass is re-opened:

```
$R/away/docs/metrics/workflow.jsonl: 1 records, valid
$R/away/docs/plans/p.plan.toml: 1 steps, 0 questions, valid
$R/away/docs/plans/p.plan.toml vs $R/away/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

with `next` reporting `metrics: 1 records` and `status --json` reporting `"metrics_absent_reason": null`.

REASONING, AND MY GROUND FOR THE DOWNGRADE. Everything EVI-1 claims is true and I reproduced all of it. The distinction from `high` is that THE SHIPPED CODE IS CORRECT: HEAD refuses this layout. The finding is a coverage gap on a load-bearing clause, not a live defect, and its harm is entirely prospective (a later refactor of `resolve_for_containment` re-opens the increment's end property with a green suite). EVI-1's own framing agrees: its `WHAT WOULD CLOSE IT` is one test, not a code change. I reserve `high` for a defect a user can hit today, which is the standard I applied to ADV-1, and applying two different standards within one round would make the severities incomparable.

I also confirm EVI-1's rebuttal of the obvious dismissal. This is NOT accepted cost (ii). Cost (ii) is a FALSE POSITIVE (a legitimate layout refused); this is a FALSE NEGATIVE and is the increment's stated end property at specification line 104. And both existing symlink tests symlink a DIRECTORY: I read `accepted_cost_two_the_symlinked_layouts_are_pinned` at `tests/unsafe_pairings_are_refused_and_omitted.rs:707` and confirmed layout 1 symlinks `docs/plans` and layout 2 symlinks `docs/metrics`, so neither reaches the leaf clause.

PRESCRIBED FIX. `tests/unsafe_pairings_are_refused_and_omitted.rs`, a new test beside the two symlink ones. AUTHORED TEST LOGIC, small; the file already has a `symlink` helper at `:848`. The log's LEAF symlinked out of the plan's root, asserting the refusal on `validate --workflow` and the omission on `status` and `next`. The clause it pins is `src/main.rs:1332`, documented at `src/main.rs:1319-1321`.

### 4.4 EVI-3: `status`'s reason vocabulary is only half pinned

VERDICT: VALID. SEVERITY: `medium` (UPHELD).

WHAT I RAN. Both mutations, separately, each with a full suite run and a revert.

MUTATION B (the `log-absent` value at `src/main.rs:1167` changed to `LogNotThisProject`). SUITE GREEN, 413 passed. Observed on a project whose own log is genuinely absent:

```
  "metrics_absent_reason": "log-not-this-project"    <- mutated
  "metrics_absent_reason": "log-absent"              <- HEAD
```

MUTATION A (the precedence rule at `src/main.rs:1154-1157` replaced by an existence test). SUITE GREEN, 413 passed. Observed with an out-of-root `--metrics` naming a file that does not exist:

```
status --json : "metrics_absent_reason": "log-absent"             <- mutated
next   --json : "metrics_absent_reason": "log-not-this-project"   <- same input, unmutated path
```

REASONING. Acceptance check 14f's whole content is that the vocabulary SEPARATES the causes, and specification line 208 names `status --json` as the half with no golden and no serialisation test. Under mutation B a `status --json` consumer cannot distinguish check 14f's case (a) from case (b) at all, which is the "the defect has moved rather than closed" condition the check names in its own words. Under mutation A the two commands disagree on the same input, which is the precedence rule (line 231) silently gone. The `next` half of both is already pinned at `tests/unsafe_pairings_are_refused_and_omitted.rs:561-592`, so this is an asymmetry rather than a uniform gap, which is what makes it cheap to close.

PRESCRIBED FIX. `tests/unsafe_pairings_are_refused_and_omitted.rs:549-558` and `:820-844`. TWO ADDED ASSERTIONS on runs that already exist, no new fixture: `status --json` on a project whose own log is missing must show `"metrics_absent_reason": "log-absent"`, and `status --json` with an out-of-root `--metrics` naming a nonexistent file must show `"log-not-this-project"`. No product change; `src/main.rs:1154-1168` is already correct.

### 4.5 EVI-4: `status --resume`'s precedence rule is unguarded

VERDICT: VALID. SEVERITY: `medium` (UPHELD).

WHAT I RAN. I moved the `if !ledger_path.exists()` block ABOVE the `resume_roots(...).find(...)` block at `src/main.rs:1449-1461` and ran the full suite.

WHAT I OBSERVED. SUITE GREEN, 413 passed. With a `--ledger-fragment` that is both outside the root and nonexistent:

```
no ledger at /tmp/nope-nowhere.ledger.md; nothing to resume                                      <- mutated, exit 0
the ledger /tmp/nope-nowhere.ledger.md is not under the plan's project root $R/proj; nothing ...  <- HEAD, exit 0
```

REASONING. The code carries an explicit comment at `src/main.rs:1449-1450` stating the rule it is implementing ("so an unsafe ledger is never reported as a missing one (the precedence rule: unsafe is not absent)"), and nothing checks it. The equivalent case on `next` IS pinned (`the_resume_reasons_separate_and_cover_the_default_ledger`, the "Outside the root AND missing" run, which I read at `:635-646`), so a reader would reasonably assume the rule is covered everywhere. I considered downgrading this to `low` on the ground that `status --resume` has no machine surface and the block is omitted either way; I did not, because the regression it admits actively misleads (a user is told "no ledger" for a ledger that exists in another project), and the precedence rule is a named specification rule at line 231 rather than an implementation detail.

PRESCRIBED FIX. `tests/unsafe_pairings_are_refused_and_omitted.rs:437` (`status_omits_only_the_unpairable_part`). ONE ADDED INVOCATION with a `--ledger-fragment` both outside the root and nonexistent, asserting the containment note rather than `no ledger at`. No product change; `src/main.rs:1449-1461` is already correct.

### 4.6 EVI-5: `next`'s human note for an unpairable LEDGER is not pinned end to end

VERDICT: VALID. SEVERITY: `medium` (UPHELD).

WHAT I RAN. I forced `resume_state_absent_note` to `None` in `run_next` at `src/main.rs:1546-1550` while leaving the reason computed and reported, then ran the full suite.

WHAT I OBSERVED. SUITE GREEN, 413 passed. Tail of `next --source <alpha plan> --ledger-fragment <beta ledger>`:

```
MUTATED, last line of output:
  summary: first review round on step `core`: independent reviewer, cite file and line.

HEAD, last line of output:
the ledger $R/beta/docs/plans/b.ledger.md is not under the plan's project root $R/alpha; nothing to resume
```

The JSON was unchanged under the mutation (`"resume_state": null`, `"resume_state_absent_reason": "ledger-not-this-project"`), which is why the machine-surface tests do not catch it.

REASONING. Specification line 183 requires the echo to be omitted "with the same note naming the rejected ledger path in its place that `status --resume` prints", and `Q-55-refusalscope` is an OMIT plus SAY WHY decision, not an omit decision. The renderer half is pinned by a unit test that supplies the note itself; the caller half is pinned by nothing, so the whole SAY WHY half can vanish from the agent-facing human surface with a green suite. The metrics equivalent IS pinned end to end, which makes this an asymmetry between two artifacts of one decision rather than a uniform gap.

PRESCRIBED FIX. `tests/unsafe_pairings_are_refused_and_omitted.rs:600` (`the_resume_reasons_separate_and_cover_the_default_ledger`). ONE ADDED ASSERTION, on the HUMAN (non-`--json`) run, that stdout contains `is not under the plan's project root`, on both the explicit-fragment run and the default-ledger divergent-pairing run. No product change; the caller at `src/main.rs:1546-1550` and the arm at `src/next.rs:1175-1178` are already correct.

### 4.7 EVI-6: the refusal message's third remedy is unguarded

VERDICT: VALID. SEVERITY: `low` (UPHELD).

WHAT I RAN. I shortened the message at `src/main.rs:991` to drop "or correct the `--source` and `--plan` pair" and ran the full suite.

WHAT I OBSERVED. SUITE GREEN, 413 passed, 0 failed, including `a_divergent_source_and_plan_pairing_is_refused`, which is the only test where that remedy is the RELEVANT one.

REASONING. Specification line 157 adds the third member for a specific measured reason ("neither of A's two remedies names that cause"), so it is a required element rather than wording taste. It is `low` because losing it degrades a message rather than a verdict: the refusal still fires, the exit code is unchanged, and the user still learns the pairing was rejected.

PRESCRIBED FIX. `tests/unsafe_pairings_are_refused_and_omitted.rs:215` (`a_divergent_source_and_plan_pairing_is_refused`). ONE-LINE TEST ADDITION: `assert!(stderr.contains("correct the `--source` and `--plan` pair"))`. No product change.

---

## 5. RULINGS ON THE IMPLEMENTER'S EIGHT JUDGEMENT CALLS

The eight are recorded at `docs/plans/agent-scaffold.ledger.md:397`. I rule on all eight; the three the brief names are argued at length and the rest briefly, in proportion to what my reproductions bear on.

**(a) Two `#[serde(skip)]` note fields carried through `NextInputs` to get the assembled human note to `render_human`. SOUND. Something small IS owed, and it is TRI-1 below.**

The constraint is real and I checked it in the code rather than accepting the framing. `render_human` is a pure function of `NextProjection`, and `NextProjection` carries no path fields, while specification line 212 fixes that "THE ENUM IS THE MACHINE VALUE ONLY: the paths a human message names are not carried on it, so the CALLER assembles that message from the paths it already holds". So the assembled note must either travel on the projection or `render_human`'s signature must change. Skipped fields are the smaller change, they follow the precedent already in the same struct, and the wire is guarded: the evidence lens's M15 (adding the note to the wire) is CAUGHT by `golden_json`, which is a strict byte compare I confirmed is unrelaxed. The rejected alternative (a data-carrying enum) really would have contradicted line 212. Nothing is owed on the mechanism.

TWO OBSERVATIONS THAT DO NOT CHANGE THE RULING. `#[serde(skip)]` now appears TWICE in `src/` (`src/next.rs:198`, `:202`) where the specification's own sweep at line 206 recorded it appearing "exactly ONCE in the whole of `src/`". That sentence is a description of the pre-change tree, not a constraint, so this is currency rather than a defect, and the specification is settled. And the doc comment on one of the two new fields makes a claim that is false on a reachable path, which I file as TRI-1.

**(c) Reading check 19's "a SYMLINK to a sibling directory" as a sibling of the ROOT. SOUND, and I verified it by running both readings rather than arguing them.**

The reading applies to check 19's SECOND layout, the LOG side. I built both.

Log side, target INSIDE the root (`<root>/docs/metrics -> <root>/logs`):

```
$R/c1/docs/metrics/workflow.jsonl: 1 records, valid
$R/c1/docs/plans/p.plan.toml vs $R/c1/docs/metrics/workflow.jsonl: workflow invariants hold
exit=0
```

Log side, target OUTSIDE the root (`<root>/docs/metrics -> <sibling of root>`):

```
--workflow would join $R/c2/docs/plans/p.plan.toml against $R/c2/docs/metrics/workflow.jsonl, which is not under the plan's project root $R/c2; ...
exit=1
```

So the implementer's stated ground is CORRECT for the layout it was applied to: on the log side the symlink target IS the contained artifact, and an in-root target does not breach containment at all, so only a sibling of the ROOT can pin cost (ii) there. I confirmed the test does exactly this: `accepted_cost_two_the_symlinked_layouts_are_pinned` builds `two/docs/metrics -> root/two-metrics`, a sibling of the root.

AND THE OTHER HALF IS ALSO RIGHT, which is what makes the pair correct rather than lucky. Layout 1 (the PLAN side) uses `one/docs/plans -> one/elsewhere`, a target INSIDE the root, which is the specification's own worked example at line 257. I ran it and it is refused:

```
--workflow would join $R/c3/docs/plans/p.plan.toml against $R/c3/docs/metrics/workflow.jsonl, which is not under the plan's project root $R/c3/elsewhere; ...
exit=1
```

It breaches because the ROOT DERIVATION moves, not because the artifact escapes, which is a different mechanism from the log side. The implementer chose the right layout for each side. Nothing owed.

**(e) Refusing INSTEAD OF running the four-arm match rather than alongside it. SOUND, and EVI-2 is the evidence that settles it rather than an argument against it.**

The specification's "Before the four-arm match" (line 157) admits the weaker reading, alongside. The END PROPERTY does not: line 104 requires that "where the tool cannot establish that the two belong together, it must say so and exit non-zero rather than PROCEED". My EVI-2 reproduction shows what the alongside reading actually emits: the refusal, and then a W3 verdict on the very pairing just declared unvouchable, at the same exit code. That is the assertion "in either direction" the refusal's own comment at `src/main.rs:984-989` forbids. The implementer's reading is the only one consistent with the end property. NOTHING IS OWED ON THE BEHAVIOUR; what is owed is EVI-2's assertion, so the choice is pinned rather than resting on a comment.

**(b) The exact wording of the three notes. SOUND.** The wording is single-sourced in `unpairable_log_note` (`src/main.rs:1362`) and `unpairable_ledger_note` (`src/main.rs:1375`), so `status`, `next` and `status --resume` cannot drift on how they explain one verdict, which is the One-source-of-truth answer to a question the specification shaped but did not fix. I confirmed the identical string appears on all three surfaces in my runs. EVI-5 and EVI-6 show the wording is UNDER-PINNED, and those are what is owed; the choice itself is right.

**(d) The `..` that survives `resolve_for_containment`'s literal re-append. SOUND, and I re-ran it rather than inheriting ADV's result.** With a missing intermediate the escape path survives into the remainder and is NOT refused, and it is inert because the same path is unopenable:

```
readable by test -r: NO
no metrics log at $R/d/docs/plans/nope/../../../../far/docs/metrics/workflow.jsonl; nothing to validate
exit=0
```

With all intermediates existing the walk resolves the whole path and the guard fires (`exit=1`). The doc comment's argument at `src/main.rs:1323-1325` holds as measured. I add one forward observation the lenses did not: after inc3 the inert branch becomes a hard failure rather than a skip, so it stays non-exploitable and does not become a workaround. Nothing owed.

**(f) All three shared enums in `src/next.rs` rather than a new module `status` would co-own. SOUND.** `run_status` already imports from `next` for `extract_resume_state` and `derive_task`, so `next::MetricsAbsentReason` follows an established direction rather than creating a new coupling, and minting a module for three enums is against Minimal by default. Nothing owed.

**(g) Leaving `no_active_loop_reason` declared last so the JSON order around a pre-existing field does not move. SOUND.** I confirmed the field order in `NextProjection` (`src/next.rs:168-203`) and that the `GOLDEN_JSON` diff is additive only, which is exactly what acceptance check 14h demands ("every pre-existing field keeps its name, position and value"). Nothing owed.

**(h) `GOLDEN_JSON` gaining `"resume_state_absent_reason": "ledger-absent"` rather than a `null`. SOUND, and in fact REQUIRED.** The field's contract is `Some` exactly when `resume_state` is `None` (specification line 225, and the doc comment at `src/next.rs:187`), and the golden fixture passes `resume_state: None`, so a `null` there would violate the invariant the field exists to carry. Check 14h's "serialises the new reasons as `null`" refers to a CORRECT RUN against a real project, which is pinned separately by `a_correct_run_serialises_the_new_reasons_as_null`. No conflict, nothing owed.

---

## 6. WHAT THE THREE LENSES MISSED

### TRI-1: the `metrics_absent_note` doc comment's justification is false on the default-`--metrics` path

SEVERITY: `low`. Found while ruling on judgement call (a).

THE CLAIM IN THE CODE, `src/next.rs:193-197`: "Not serialised: `--json` reports the token, and a machine consumer already holds the paths it passed in."

WHAT I RAN, on the divergent pairing with NO `--metrics` and NO `--ledger-fragment`, which is the case acceptance check 14g's fourth run exists for:

```
$ agent-scaffold next --json --source $R/MA/docs/plans/p.plan.toml --plan $R/MB/docs/plans/p.md
  "metrics": null,
  "metrics_absent_reason": "log-not-this-project",
  "resume_state": null,
  "resume_state_absent_reason": "ledger-not-this-project",
  "no_active_loop_reason": "metrics-not-this-project"
```

WHAT I OBSERVED. The consumer passed neither the resolved log path (`MA/docs/metrics/workflow.jsonl`, DERIVED by the tool) nor the derived root (`MB`), and neither appears anywhere on the machine surface. So it does not hold the paths, and the stated justification does not hold on the path that `Q-55-endproperty` exists for.

REASONING. The BEHAVIOUR is conformant and must not change: specification line 212 fixes that the enum carries no paths. The defect is in the JUSTIFICATION. The specification's own reasoning is that "the CALLER assembles that message from the paths it already holds", where the caller is `run_next`; the implementer transposed that onto the MACHINE CONSUMER, which is a different actor with different knowledge. This matters in this project's terms because the increment devotes a whole section to four doc comments falsified by the change, and a NEW false claim shipped in the same commit is the defect that section exists to prevent, in miniature.

PRESCRIBED FIX. `src/next.rs:196-197`. A ONE-LINE CHANGE, deletion-leaning: drop the false clause and keep the true one, for example "Not serialised: the enum beside it is the machine value and carries no paths (`Q-55-jsonreason`)." No behaviour change, no test change.

NOTHING ELSE. I attacked the four surfaces on divergent pairings, nested and disjoint layouts, symlinked leaves and directories on both sides, `..` escapes with existing, missing and readable intermediates, the precedence and correlation rules on both commands, and both `primary` spellings, and found no further defect the three lenses did not already have.

---

## 7. ROUND OUTCOME

**NEW VALID FINDINGS. This round is NOT CLEAN.**

VALID COUNT AFTER DEDUPLICATION: **9**.

- `critical`: 0.
- `high`: 1 (ADV-1).
- `medium`: 5 (EVI-1, EVI-2, EVI-3, EVI-4, EVI-5).
- `low`: 3 (ADV-2, EVI-6, TRI-1).

INVALID: NONE. Every raw finding replayed against my own build.

NO `high` OR `critical` WAS DISMISSED. EVI-1 was moved from `high` to `medium` and REMAINS VALID with a fix owed; my ground is at 4.3 and is that the shipped code is correct on the reported layout, so the harm is prospective rather than live, and I reserved `high` for a defect reachable today so the two severities in this round stay comparable.

ONE FINDING IS A PRODUCT DEFECT (ADV-1). ONE IS A DECISION TO ROUTE (ADV-2). SIX ARE TEST-ONLY (EVI-1 through EVI-6, closable with six assertions and one new test, no product change). ONE IS A ONE-LINE DOC DELETION (TRI-1). The fix pass is therefore mostly ADDITIVE TESTS plus one small piece of authored logic, which is the cheaper of the two shapes this project has calibration data on.
