# `workflow-enforcement-tier-inc2`, work review ROUND 2, ISOLATED REVIEWER, FIX-VERIFICATION AND REGRESSION LENS

ARTIFACT. The fix pass alone is `git diff HEAD~1..HEAD` at commit `6bf5280` ("fix: supply a root to the surfaces that read no plan, and pin six unguarded clauses"); the full increment is `git diff main..HEAD`. Reviewed in the worktree `.claude/worktrees/r2-fixverify`.

SOURCES READ. `docs/plans/workflow-enforcement-tier-inc2.reviews/workflow-enforcement-tier-inc2-triage.md` (authoritative on each finding and its prescription), the three round 1 reviewer files, and `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`.

METHOD. Every verdict below is a measurement. The nine findings were verified by building the fixture or applying the named mutation against my own build, never by reading the fix pass's report. Twenty mutations were applied and reverted one at a time, each with a FULL suite run and a clean-tree check after. Two binaries were built for differential work, `bin-prefix` from `HEAD~1` and `bin-postfix` from `HEAD`, so every "unchanged" claim below is a byte diff of two real runs rather than an inference.

BASELINE I ESTABLISHED FIRST, with `TMPDIR` pointed outside every repository as the specification requires at line 311:

```
cargo build                                -> Finished dev profile
cargo test                                 -> 415 passed, 0 failed, across 9 binaries
cargo clippy --all-targets -- -D warnings  -> clean
cargo run -- render docs/plans/agent-scaffold.plan.toml --check -> up to date
cargo run -- validate --source docs/plans/agent-scaffold.plan.toml --workflow -> invariants hold, exit 0
```

The integration file `tests/unsafe_pairings_are_refused_and_omitted.rs` went from 13 tests to 15; the suite total went from 413 to 415.

---

## 1. THE NINE FINDINGS: VERIFICATION TABLE

**ALL NINE VERIFIED CLOSED. Nought not closed.** Every mutation named by the triage as SURVIVING now fails the suite, and each fails on the assertion the triage prescribed rather than incidentally.

| Id | Kind | Verdict | Evidence I produced |
| --- | --- | --- | --- |
| ADV-1 | product | VERIFIED CLOSED | Two-sibling fixture rebuilt from the adversarial lens's own script; `next` human and `--json` now refuse, `status --resume` byte-identical, and the LOG half closed on `next` and `status` too. Section 1.1. |
| EVI-1 | test-only | VERIFIED CLOSED | M19 (`.ancestors().skip(1)`, `src/main.rs:1356`) -> SUITE RED, 411 passed, 1 failed, `a_symlinked_log_leaf_outside_the_root_is_refused`. |
| EVI-2 | test-only | VERIFIED CLOSED | M26 (`else` removed at `src/main.rs:995`) -> SUITE RED, 411 passed, 1 failed, `an_explicit_metrics_outside_the_plans_root_is_refused`. |
| EVI-3 | test-only | VERIFIED CLOSED | BOTH mutations. M27 (`src/main.rs:1167`, `LogAbsent` -> `LogNotThisProject`) -> RED, 1 failed. M4 (the precedence rule at `src/main.rs:1154-1157` replaced by an existence test) -> RED, 1 failed. Both `the_machine_surface_separates_the_causes_on_both_commands`. |
| EVI-4 | test-only | VERIFIED CLOSED | M5 (`run_resume` tests existence BEFORE containment, `src/main.rs:1473-1485`) -> SUITE RED, 411 passed, 1 failed, `status_omits_only_the_unpairable_part`. |
| EVI-5 | test-only | VERIFIED CLOSED | M28 (`resume_state_absent_note: None` at `src/main.rs:1606`, reason still computed) -> SUITE RED, 410 passed, 2 failed, `the_resume_reasons_separate_and_cover_the_default_ledger` and `a_surface_that_reads_no_plan_is_supplied_a_root`. |
| EVI-6 | test-only | VERIFIED CLOSED | M24 (third remedy dropped from the message at `src/main.rs:991`) -> SUITE RED, 411 passed, 1 failed, `a_divergent_source_and_plan_pairing_is_refused`. |
| TRI-1 | doc | VERIFIED CLOSED | The false clause is gone from the whole of `src/`; the retained text at `src/next.rs:193-197` is true on every path I drove. Section 1.2. |
| ADV-2 | decided residual | NOT RE-RAISED; the fix pass's byte-identical claim VERIFIED | Both spellings, human and `--json`, diffed pre-fix against post-fix: BYTE-IDENTICAL. Section 1.3. |

A RED SUITE STOPS AT THE FAILING BINARY, so a mutation run reports 411 or 410 rather than 414: `cargo test` does not run the binaries after `unsafe_pairings_are_refused_and_omitted`. The failing-test names, not the totals, are the evidence.

### 1.1 ADV-1, verified closed on both surfaces, with `status --resume` unchanged

I rebuilt the two-sibling fixture myself (`$R/alpha` and `$R/beta` are TOP-LEVEL SIBLINGS; no nesting anywhere, so the in-root bound cannot be the explanation), and ran the same argument lists against `bin-prefix` and `bin-postfix`. The diff, paths abbreviated to `$R`:

```
 === B: next, identical inputs ===
-RESUME STATE (verbatim from the ledger):
-## RESUME STATE
-
-BETA-PRIVATE: branch feat/secret, worktree /home/beta/wt, in-flight review of step X.
+the ledger $R/beta/docs/plans/b.ledger.md is not under the plan's project root $R/alpha; nothing to resume
 exit=0
 === C: next --json, identical inputs ===
-  "resume_state": "## RESUME STATE\n\nBETA-PRIVATE: ...",
-  "resume_state_absent_reason": null,
+  "resume_state": null,
+  "resume_state_absent_reason": "ledger-not-this-project",
```

`status --resume` on the same inputs (run A) does not appear in the diff at all: its line is byte-identical before and after, which is the no-regression half of the fix.

THE SECONDARY OBSERVATION the triage folded into ADV-1 is closed with it, on both commands:

```
 === D: next --json with a foreign explicit --metrics, no --plan ===
-  "metrics": { "records": 1 },  "metrics_absent_reason": null,
+  "metrics": null,              "metrics_absent_reason": "log-not-this-project",
 === E: status --json, same ===  (identical change)
 === F: status human, same ===
-metrics: 1 records
+metrics: unavailable, the round log $R/beta/docs/metrics/workflow.jsonl is not under the plan's project root $R/alpha, ...
```

ADV's SECOND SPELLING (a Markdown-primary `--source` beside a `--plan` that does not exist) is closed identically, and I confirm the fix is itself guarded: reverting `containment_roots` to the pre-fix `map_or_else(Vec::new, ...)` turns the suite RED on `a_surface_that_reads_no_plan_is_supplied_a_root` alone.

### 1.2 TRI-1: deletion was sufficient

The remaining text at `src/next.rs:193-197` is "The human phrasing of an unpairable round log, naming the resolved log and the derived project root. Assembled by the CALLER, which holds those paths, because the reason enums above are the machine value and carry none; `Some` exactly when `metrics_absent_reason` is `LogNotThisProject`. Not serialised: `--json` reports the token."

I checked each clause against a run rather than against the fix pass's account. The caller does hold both paths (`metrics_path` and the root are in scope at `src/main.rs:1550-1553`); the enums carry no paths (specification line 212); `Some` tracks `LogNotThisProject` exactly, since one `Option` drives the other at `src/main.rs:1555`; and the note is genuinely absent from `--json` on every invocation I made. `grep -rn "already holds the paths"` over `src/` returns nothing, so the false clause is gone rather than moved.

DELETION WAS SUFFICIENT AND THE DEPARTURE FROM THE TRIAGE'S WORDING COSTS NOTHING. The triage's suggested replacement was to state the justification "the enum beside it is the machine value and carries no paths". That justification is already in the IMMEDIATELY PRECEDING clause of the same comment ("because the reason enums above are the machine value and carry none"), so the suggested wording would have said it twice. Nothing is owed.

### 1.3 ADV-2: not re-raised, and the byte-identical claim holds

I built both spellings the adversarial lens gives (the explicit `--ledger-fragment` one, and the no-flag divergent pairing where the log is named explicitly under the checked plan's root so only the DEFAULT ledger is rejected) and diffed `bin-prefix` against `bin-postfix` across the full human and `--json` output of all four runs. `diff -u` is EMPTY. The rejected ledger still fills the loop's `ledger:` slot, at `$R/beta/docs/plans/b.ledger.md` on the human surface and `"ledger": "$R/beta/docs/plans/b.ledger.md"` on the wire, exactly as before; the fix neither closed it nor disturbed it, which is what the decision `q_id:"Q-55-ledgerslot"` requires.

---

## 2. REGRESSION

### 2.1 The round 1 CAUGHT mutations are STILL CAUGHT

Thirteen of the evidence lens's twenty-two CAUGHT mutations were re-run against the post-fix tree, chosen to cover every surface the fix touched plus the central one. NONE was weakened. Where the round 1 file names the catching tests, the same tests still fail.

| Mutation | `file:line` (post-fix) | Round 2 result |
| --- | --- | --- |
| M2, `checked_plan_root` roots on the ANCHOR (`source.or(plan)`), ignoring `toml_primary`. THE CENTRAL ONE, the defect `Q-55-endproperty` exists to prevent. | `src/main.rs:1313` | STILL CAUGHT. RED, 3 failed: `a_divergent_source_and_plan_pairing_is_refused`, `the_resume_reasons_separate_and_cover_the_default_ledger`, `accepted_costs_three_and_four_are_pinned`. The same three as round 1. |
| M1, `is_outside_root` returns `false` unconditionally. | `src/main.rs:1379` | STILL CAUGHT, and MORE STRONGLY: 13 of the 15 integration tests fail, up from 11 of 13. |
| M29, `is_outside_root` returns `true` unconditionally (the false-positive direction). | `src/main.rs:1379` | STILL CAUGHT. 8 tests in the inc1 file `metrics_and_ledger_anchor_to_the_plan_source.rs`, the same 8 as round 1. |
| M6, the refusal removed on `validate` only. | `src/main.rs:981` | STILL CAUGHT. 7 failed. |
| M7, the metrics omission removed on `status` only. | `src/main.rs:1150-1153` | STILL CAUGHT. 7 failed, including `status_omits_only_the_unpairable_part`. |
| M8, the metrics omission removed on `next` only. | `src/main.rs:1550-1553` | STILL CAUGHT. 7 failed, including `next_withholds_the_whole_loop_on_an_unpairable_log`. |
| M9, `next` treats an unsafe log as ABSENT (the specification line 187 trap). | `src/main.rs:1559` | STILL CAUGHT. 4 failed. |
| M17, `canonical_project_root` made LEXICAL. | `src/main.rs:1290` | STILL CAUGHT. `a_symlinked_source_cannot_borrow_its_neighbours_log`, `accepted_cost_two_the_symlinked_layouts_are_pinned`. |
| M18, `resolve_for_containment` drops canonicalisation entirely. | `src/main.rs:1356-1362` | STILL CAUGHT. 3 failed, one of them the NEW leaf test. |
| M20, `Path::starts_with` replaced by a string-prefix comparison. | `src/main.rs:1379` | STILL CAUGHT. `accepted_cost_two_the_symlinked_layouts_are_pinned`. |
| M21, `resume_roots` drops the `--plan` anchor. | `src/main.rs:1449` | STILL CAUGHT. `resume_omits_the_default_ledger_under_a_divergent_pairing`, `accepted_costs_three_and_four_are_pinned`. The two-anchor intersection on `status --resume` is still guarded. |
| M22, `next`'s LEDGER containment check removed. | `src/main.rs:1571-1574` | STILL CAUGHT. 2 failed. |
| M23, `status --resume`'s ledger containment check removed. | `src/main.rs:1475-1481` | STILL CAUGHT. 4 failed. |

NO MUTATION THAT WAS CAUGHT IN ROUND 1 SURVIVES IN ROUND 2. The fix weakened no existing guard.

### 2.2 The root supply for `run_status` and `run_next`, every configuration

`containment_roots` changed the root supply for two surfaces, so I drove a full matrix rather than sampling: TWELVE anchor configurations crossed with THREE artifact sets (none, the project's own explicit `--metrics`/`--ledger-fragment`, a FOREIGN project's), across SIX surfaces (`status`, `status --json`, `status --resume`, `next`, `next --json`, `validate --workflow`). 216 invocations, 2442 lines of output, run twice, once per binary, and diffed.

The anchor configurations: TOML-primary `--source` alone; Markdown-primary `--source` alone; `--plan` alone; both agreeing TOML-primary; both agreeing Markdown-primary; both diverging with a Markdown-primary `--source`; both diverging with a TOML-primary `--source`; neither; a nonexistent `--source` beside a real `--plan` in the same project; the same with the nonexistent `--source` spelled into the OTHER project; a nonexistent `--source` alone; a Markdown-primary `--source` beside a `--plan` that does not exist.

**THE PRE-FIX / POST-FIX DIFF OVER THE WHOLE MATRIX IS EXACTLY TWO BLOCKS, BOTH IN THE DIRECTION OF REFUSAL.** Markdown-primary `--source` alone plus FOREIGN artifacts, and Markdown-primary `--source` plus a nonexistent `--plan` plus FOREIGN artifacts. Every other cell is byte-identical.

- NOTHING THAT WORKED BEFORE NOW REFUSES. The two newly-rooted configurations with the project's OWN explicit artifacts do not appear in the diff, which is the load-bearing negative: the artifacts are under the supplied root, so the new root changes nothing there.
- NOTHING THAT SHOULD OMIT NOW PASSES. Both changed cells moved from `metrics: 1 records` plus a verbatim foreign `## RESUME STATE` to `metrics: unavailable, <note>` plus the containment note, with `"metrics_absent_reason": "log-not-this-project"` and `"resume_state_absent_reason": "ledger-not-this-project"` on the wire.
- `validate --workflow` WAS CORRECTLY LEFT ALONE. It does not call `containment_roots`, and it does not need to: in every no-plan-read configuration it hard-fails first with `--workflow requested but no plan source resolved`, exit 1, which is the `(None, None, _)` arm specification line 157 names. Extending `containment_roots` to it would have been dead code.

### 2.3 The two-anchor case, and whether an empty root vector can mean "everything passes"

THE TWO-ANCHOR INTERSECTION IS STRUCTURALLY UNREACHABLE THROUGH `containment_roots`, and that is correct rather than a gap. `containment_roots` reaches `resume_roots` only when `checked_plan_root` is `None`. With `toml_primary` true the source was read, so it canonicalises and the root is `Some`; so `None` implies `toml_primary` is false AND the `--plan` is absent or does not canonicalise, and `resume_roots` filters that same `--plan` out. The vector `status` and `next` test against therefore has at most ONE element, always. `status --resume` is the only surface that ever intersects two, and M21 confirms that intersection is still guarded.

I MEASURED THE CONSEQUENCE rather than leaving it as an argument. Under a divergent pairing (`--source` Markdown-primary in project one, `--plan` in project two) with an explicit `--metrics` and `--ledger-fragment` both naming project TWO's own artifacts, `next` and `status` accept them (root is two, the checked plan's) while `status --resume` refuses (roots are one AND two). That divergence is byte-identical before and after the fix, and it is what the specification decides: line 183 roots `next` on the checked plan, line 182 roots `status --resume` on the anchors, and the round 1 triage explicitly forbade unifying them ("DO NOT extend `resume_roots` to the plan-reading case"). It is `Q-55-resumepairing` and `Q-55-endproperty` behaving as decided, not a defect, and not something the fix introduced.

AN EMPTY ROOT VECTOR CAN SILENTLY MEAN "EVERYTHING PASSES", AND I FOUND A REACHABLE CONFIGURATION WHERE IT DOES. That is FV-1 below. It is not caused by the fix (byte-identical before and after) but it is live in the increment, it is not the in-root bound, and I built the discriminating control to prove it.

### 2.4 No test's meaning changed

`git diff --numstat HEAD~1..HEAD -- tests/` is `214 0`: two hundred and fourteen insertions, ZERO deletions. No existing assertion was edited, relaxed or removed. The five additions inside existing tests are appended AFTER those tests' existing assertions, including the one that writes a new fixture file (`docs/metrics/other.jsonl` in `an_explicit_metrics_outside_the_plans_root_is_refused`), so no earlier assertion sees a changed fixture. The two new tests are wholly new functions. Coverage grew; nothing was reinterpreted.

I also read the new `a_surface_that_reads_no_plan_is_supplied_a_root` (`tests/unsafe_pairings_are_refused_and_omitted.rs:578`) to check it pins the finding rather than something adjacent. It does: `alpha` and `home` are top-level siblings so the in-root bound cannot supply the result, it asserts what `status --resume` answers FIRST and then that the other two surfaces agree with it rather than merely asserting the note, and it covers the log half on both commands. The `--ledger-fragment` it uses is relative and resolves against `home`, so the block it must not print is genuinely another project's.

### 2.5 The guards

`cargo clippy --all-targets -- -D warnings` clean. `render --check` up to date. `validate --source docs/plans/agent-scaffold.plan.toml --workflow` reports invariants hold at exit 0 against the repository's own 264-record log. Full suite 415 passed, 0 failed, on a clean tree before and after every mutation.

---

## 3. NEW FINDINGS

### FV-1: a `--source` that does not exist supplies NO root, so all three projection surfaces read and echo another project's ledger and log

SEVERITY: `medium`.

CLAIM. `containment_roots` supplies a root "from the anchors" where no plan is read, but `resume_roots` derives a root only through `canonical_project_root`, which returns `None` for a path that does not exist (`src/main.rs:1289-1291`). A `--source` naming a file that does not exist, with no `--plan`, therefore yields an EMPTY root vector, `find` over it is vacuous, and `status`, `status --resume` and `next` all read an explicit `--metrics` or `--ledger-fragment` in another project with nothing to reject it, at exit 0, reporting `"metrics_absent_reason": null` and `"resume_state_absent_reason": null`.

`file:line`. `src/main.rs:1332-1339` (`containment_roots`), `src/main.rs:1445-1454` (`resume_roots`), `src/main.rs:1289-1291` (`canonical_project_root`); consumed at `src/main.rs:1149`, `:1550`, `:1571` and `:1475`.

THE DISCRIMINATING CONTROL, built because the brief requires one before anything is attributed to the in-root bound. The same fixture, the same commands, run TWICE: once with `alpha` and `beta` as top-level SIBLINGS, once with `beta` moved INSIDE `alpha`'s root. Each run also carries a control invocation that differs from the leaking one in ONE character of the `--source` path.

```
===== DISJOINT SIBLINGS (no nesting anywhere)
--- next --source $R/disjoint/alpha/docs/plans/TYPO.plan.toml \
         --ledger-fragment $R/disjoint/beta/docs/plans/b.ledger.md
task: TYPO
source: no plan source
metrics: no log found

no active review loop (no plan steps found)

RESUME STATE (verbatim from the ledger):
## RESUME STATE

BETA-PRIVATE: branch feat/secret, worktree /home/beta/wt.
exit=0
--- next --json, same
  "resume_state": "## RESUME STATE\n\nBETA-PRIVATE: branch feat/secret, worktree /home/beta/wt.",
  "resume_state_absent_reason": null,
--- status --resume, same
## RESUME STATE

BETA-PRIVATE: branch feat/secret, worktree /home/beta/wt.
exit=0
--- status --json (typo'd --source, explicit --metrics in beta)
  "metrics": { "records": 1 },
  "metrics_absent_reason": null
exit=0
--- CONTROL: the SAME command with the real --source spelling (p.plan.toml)
the ledger $R/disjoint/beta/docs/plans/b.ledger.md is not under the plan's project root $R/disjoint/alpha; nothing to resume
exit=0
```

THE CONTROL IS WHAT SETTLES IT. In the DISJOINT layout, correcting the `--source` spelling and changing nothing else turns the leak into a refusal. So the cause is the anchor that does not exist, not nesting and not distance. In the NESTED layout BOTH the typo'd run and the control leak, which is the in-root bound behaving as recorded; the disjoint control is the case that is not it. Per the brief's own rule, the disjoint case reproduces, so this is NOT the in-root bound and it is filed.

REASONING.

- IT IS THE INCREMENT'S OWN END PROPERTY FAILING, on the surface the specification cares most about. Specification line 127 calls a leaked `## RESUME STATE` block "not a wrong boundary at all but CONTENT INJECTION into an instruction that the receiving agent has been told is authoritative and to read first". Here the block is printed verbatim on `next`'s human output and carried whole on `next --json`.
- THE MACHINE SURFACE REPORTS THE OPPOSITE OF THE TRUTH, which is the aggravator the round 1 triage named when it upheld ADV-1 at `high`. `"resume_state_absent_reason": null` and `"metrics_absent_reason": null` do not omit an explanation, they positively assert that the block and the record count are this plan's.
- IT FALSIFIES `README.md:236` in the same way ADV-1 did: "Every one of these commands checks that the log (and, for the ledger readers, the ledger) it is about to read lives under the project root of the plan it is about to read." In this configuration none of the three does.
- THE SPECIFICATION DOES NOT COVER IT. Line 159 and line 161 both discuss the typo'd `--source`, and both mean a nonexistent `--source` BESIDE A READABLE `--plan`, where the root comes from the plan that WAS read; I confirmed that case is correctly refused (it is my matrix cases G and G2, both refused on all four surfaces). A typo'd `--source` ALONE is nowhere in the specification. Line 157's "Where NO plan is read there is no root" is written about `validate --workflow`, and the round 1 triage already ruled that it was superseded for the no-plan case by `Q-55-resumepairing`; the fix applied that later policy, and this is the remaining hole in it.
- IT IS NOT THE "NEITHER ANCHOR" CASE, which IS decided. With neither `--source` nor `--plan`, `resume_roots`'s own doc comment and `README.md` both record that there is nothing to anchor to. Here an anchor WAS supplied; it is the derivation that drops it.
- THE FIX DID NOT CAUSE IT, and I say so plainly: this cell of the matrix is byte-identical between `bin-prefix` and `bin-postfix`. It is in scope because the whole containment mechanism is new in `main..HEAD`.

WHY `medium` AND NOT `high`, stated so a triager can move it on grounds rather than taste. There is NO SURFACE-TO-SURFACE CONTRADICTION: `status --resume` leaks identically, so unlike ADV-1 the tool does not give two different answers to one question, and that contradiction is what the round 1 triage called "the sharp part" when it held ADV-1 at `high`. No instruction is fabricated either, since no plan means no steps and no `ACTIVE LOOP`. And `canonical_project_root`'s doc comment records the `None`-on-a-missing-plan behaviour deliberately, so this is a documented derivation meeting an undocumented consequence rather than an oversight. Against that, the harm is the same content injection at the same exit code with the same false `null`, and the reachability profile (a plausible typo plus one explicit flag) is the same one the triage refused to soften ADV-1 for. A triager who weighs the content injection above the missing contradiction should move it to `high`; I would not argue.

WHAT WOULD CLOSE IT, offered because the shape matters for cost. The mechanism the code already has: `resolve_for_containment` (`src/main.rs:1350`) resolves an ARTIFACT path as far as the filesystem allows and re-appends the rest. Applying the same partial resolution to an ANCHOR would give `$R/alpha/docs/plans/TYPO.plan.toml` the root `$R/alpha` and close this with no new concept. That is authored logic and a decision (it changes what "the plan was read" means for the rooting), so it should be routed rather than assumed.

### FV-2: the new README and CHANGELOG sentence names one of the reachable no-plan-read configurations and describes a multi-root rule those two surfaces can never apply

SEVERITY: `low`.

CLAIM. The sentence the fix added to both documents reads: "Where a command reads no plan at all, as `status --resume` always does and as `status` and `next` do with a Markdown-primary `--source` and no `--plan`, the roots come from the `--source` and `--plan` themselves and the artifact must be under every one of them." Two things are off, both measurable.

`file:line`. `README.md:236`, `CHANGELOG.md:23`.

FIRST, THE ENUMERATION IS INCOMPLETE ON A CONFIGURATION THIS VERY FIX CHANGED. `status` and `next` also read no plan with a Markdown-primary `--source` and a `--plan` THAT DOES NOT EXIST, and that cell is one of the exactly two cells whose behaviour the fix altered in my 216-invocation matrix:

```
########## H2 (--source md-primary, --plan NOSUCH.md) / artifacts=FOREIGN
----- status
-metrics: 1 records
+metrics: unavailable, the round log $R/two/docs/metrics/workflow.jsonl is not under the plan's project root $R/one, ...
----- next --json
-  "resume_state": "## RESUME STATE\n\ntwo resume state.",  "resume_state_absent_reason": null,
+  "resume_state": null,                                    "resume_state_absent_reason": "ledger-not-this-project",
```

A reader checking the documented rule against this run finds the behaviour but not the case. The same holds for a `--source` that exists and fails to parse as a `<task>.plan.toml`.

SECOND, "THE ARTIFACT MUST BE UNDER EVERY ONE OF THEM" IS TRUE ONLY OF `status --resume`. As section 2.3 shows, `containment_roots` can never hand `status` or `next` more than one root, so for two of the three surfaces the sentence yokes together, "every one of them" is always exactly one. Measured: under a divergent pairing with the artifacts under the `--plan`'s project, `status --resume` refuses while `status` and `next` accept, which is the two-root rule and the one-root rule giving opposite answers in the same breath the sentence describes them as sharing.

REASONING. Both documents are the increment's own account of what it does, and the round 1 triage filed TRI-1 at `low` for exactly this class (a claim in the increment's own prose that does not hold on a path a reader can drive). This is weaker than TRI-1 because nothing in the sentence is false as literally scoped; it is an incomplete enumeration plus a rule attributed to surfaces that cannot exercise it. `low` is where it belongs, and one clause in each file closes it. The doc comment at `src/main.rs:1317-1318` carries a milder version of the same looseness ("this is the one place they meet", when `run_resume` applies the second policy without passing through `containment_roots`); I note it rather than filing it separately, since it is the same sentence in a third place.

---

## 4. WHAT I ATTACKED AND FOUND NOTHING IN

Recorded so the next reader knows the ground is covered and does not re-run it.

- Every cell of the 12-by-3-by-6 configuration matrix other than the two named, differentially against the pre-fix binary. Byte-identical.
- The project's own explicit `--metrics` and `--ledger-fragment` in each of the twelve anchor configurations: never refused, before or after.
- The relative-spelling invocations from inside the project root, TOML-primary and Markdown-primary: unchanged, still printing the relative paths they always did.
- `validate --workflow` in every no-plan-read configuration: hard-fails on its own arm, so the fix's decision not to route it through `containment_roots` is complete rather than an omission.
- Both directions of the predicate (M1 and M29), so a fix that had over-fired would have shown up in the inc1 file's eight tests.
- The `..` escape, the symlinked leaf, the symlinked directory on both sides, and the string-prefix confusion (M18, M19, M20, M17): all still caught.
- The nested layout, as the control half of FV-1: it leaks with and without the typo, which is the in-root bound behaving as recorded and correctly not filed.

---

## 5. ROUND OUTCOME FROM THIS LENS

THE NINE FINDINGS: **9 VERIFIED CLOSED, 0 NOT CLOSED.** Six by mutation (each mutation the triage named now fails the suite on the prescribed assertion), one by fixture (ADV-1, on both surfaces, with `status --resume` byte-identical), one by reading every clause against a run (TRI-1), and ADV-2's byte-identical claim confirmed by diff.

REGRESSION: **NONE FOUND.** Thirteen previously-caught mutations re-run, all still caught, including the central M2 on the same three tests. The whole configuration matrix is byte-identical except the two cells the fix was meant to change, both moving toward refusal. The test diff is purely additive.

NEW FINDINGS: **2.** `critical` 0, `high` 0, `medium` 1 (FV-1), `low` 1 (FV-2). Neither is caused by the fix pass; FV-1 is a live hole in the increment's containment rule that the fix's own root-supply policy leaves open, and FV-2 is the documentation of that policy.
