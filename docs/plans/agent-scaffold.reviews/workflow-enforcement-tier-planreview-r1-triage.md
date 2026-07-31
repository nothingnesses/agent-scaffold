# `workflow-enforcement-tier` plan review, round 1: TRIAGE

Triager model: Claude Opus 5, 1M-context variant. Exact model id `claude-opus-5[1m]`.
Date: 2026-07-31.
Worktree: `.claude/worktrees/triage-q55-r1`, branch `plan/q55-enforcement` at commit `6df032c`.
Artifact triaged against: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (primary), `docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md`, `docs/plans/agent-scaffold.steps/status-resume-ignores-json.md`, and the `[[step]]`/`[[question]]` entries this fold adds to `docs/plans/agent-scaffold.plan.toml`.

Findings files triaged: `workflow-enforcement-tier-planreview-r1-reviewer-executability.md` (EX-1 to EX-10) and `workflow-enforcement-tier-planreview-r1-reviewer-fidelity.md` (F-1 to F-5).

## Result

15 findings triaged. 14 `VALID`, 1 `VALID BUT ACCEPT RESIDUAL`, 0 `DISMISSED`.

Adjusted severity: 0 critical, 1 high, 4 medium, 10 low.

NO FINDING WAS RULED `high` OR `critical` AND DISMISSED, so no backstop re-check is owed by this round.

Severity changes from the reviewers' ratings: EX-4 UPGRADED `medium` -> `high`; EX-6 DOWNGRADED `medium` -> `low`; F-1 DOWNGRADED `medium` -> `low`. Every other rating stands.

| id | reviewer severity | adjusted | verdict |
| --- | --- | --- | --- |
| EX-1 | medium | medium | VALID, structural claim narrowed |
| EX-2 | medium | medium | VALID, one sub-claim replaced |
| EX-3 | medium | medium | VALID |
| EX-4 | medium | high | VALID |
| EX-5 | medium | medium | VALID |
| EX-6 | medium | low | VALID |
| EX-7 | low | low | VALID |
| EX-8 | low | low | VALID |
| EX-9 | low | low | VALID |
| EX-10 | low | low | VALID |
| F-1 | medium | low | VALID |
| F-2 | low | low | VALID |
| F-3 | low | low | VALID |
| F-4 | low | low | VALID |
| F-5 | low | low | VALID BUT ACCEPT RESIDUAL |

## What I reproduced first-hand

Everything below was run in this worktree with `TMPDIR=/tmp/triage-q55-scratch` (outside any repository) against the debug binary, with the fixture rebuilt by the sidecar's own command into `/tmp/triage-q55-scratch/fixture`.

- `cargo build` clean; `cargo test` gives 386 passed, 0 failed (373 in the main binary, 13 across the integration binaries), so check 1's "386 expected" and the same count in `test-tmpdir-repo-assumption.md` are current. The `370 passed; 3 failed` block quoted at `test-tmpdir-repo-assumption.md:28` is the main binary's own result line (373 tests) and is internally consistent, not a contradiction of 386.
- `validate --source docs/plans/agent-scaffold.plan.toml --workflow` exits 0 with `workflow invariants hold`; `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date". The fold is validate-clean and render-clean.
- The fixture reproduces: 30 files, `ls "$SCRATCH/docs"` prints only `plans`.
- The borrowed-slug FALSE PASS reproduces (`workflow invariants hold`, exit 0) and the control reproduces at exit 1 with the W3 message quoted at sidecar line 103.
- The fabricated `next` output reproduces on both the default and the explicit `--metrics` path (`state: converged`, `streak: 1/1`, `rounds: 2/5`, `next: mark the step complete, re-render, and commit`, exit 0), so checks 5, 11, 14b and 15 are genuinely red today.
- `docs/metrics/workflow.jsonl` holds 239 records, not 235.
- The six `Q-55*` receipts all exist, one each, all with `task:"workflow-enforcement-tier"`.
- The exploration line counts (521 + 483 + 510 = 1514) are exact; `#[serde(skip)]` appears exactly once in `src/` (`src/next.rs:116`); `skip_serializing_if` appears zero times in `src/next.rs` and `src/main.rs`; `docs/metrics/workflow.jsonl` appears exactly twice in `pack/AGENTS.md`, at `:61` and `:63`.
- `GOLDEN_JSON` is at `src/next.rs:1705` and `golden_json` at `:1762`, as cited.

Two of the reviewers' own numbers I checked and one is wrong; see "Errors inside the findings files" below.

---

## EX-1. `VALID`. Severity `medium` (unchanged). The structural claim is OVERSTATED: I found escapes the reviewer missed

REPRODUCED. Every code fact the finding rests on is exact. `render_human(projection: &NextProjection) -> String` at `src/next.rs:1017`. `NextProjection` (`src/next.rs:99-118`) carries exactly `task`, `source`, `metrics`, `active_loop`, `resume_state`, `no_active_loop_reason`, and no path or root anywhere. `ledger_path` reaches `LoopContext` (`src/next.rs:532`, `:545`, `:567`) and is consumed only inside the loop-building path, so it is unavailable when `active_loop` is `None`. `status`'s human print is inline in `run_status` at `src/main.rs:1125-1128`, where `args.metrics` is in scope, so the asymmetry the finding draws between the two surfaces is real. The two sidecar requirements are where the finding says they are: line 182 (`status`, the reason "names the resolved log, the derived project root, and that the two do not correspond") and line 224 (`metrics-not-this-project`, "Printed with a reason naming the resolved log and the derived root").

THE STRONG CLAIM DOES NOT SURVIVE. The finding says every escape route is closed by another clause in the same sidecar, which would make the specification unbuildable. I attacked that adversarially and found at least three routes that no clause in the sidecar closes.

- ESCAPE A, A NON-SERIALISED DETAIL FIELD. Add a `#[serde(skip)]` field to `NextProjection` carrying the two paths, and interpolate it in `render_human`. Sidecar line 219 rejects "adding a parallel SERIALISED field"; a skipped field is not serialised, so that clause does not reach it. Check 14e is unaffected (the enum still serialises as the bare token). Check 14h is unaffected: `GOLDEN_JSON` is a byte-compare of `serde_json::to_string_pretty(&golden_projection())` (`src/next.rs:1762`), a skipped field contributes nothing to it, so the golden diff remains exactly the three added reason fields. The sidecar's `#[serde(skip)]`-appears-once sweep at line 206 is a recorded negative result, not a prohibition.
- ESCAPE B, A HAND-WRITTEN `Serialize`. The finding blocks the data-carrying variant with "serde would serialise it as an object, not a bare string". That is true of a DERIVED impl and false in general: `impl Serialize for NoActiveLoopReason` emitting `serializer.serialize_str(self.token())` produces `"no_active_loop_reason": "metrics-not-this-project"` byte-for-byte, satisfying check 14e literally, while the variant carries the paths for the human renderer. This keeps line 219's stated property exactly ("one reason, one type, rendered two ways").
- ESCAPE C, CHANGE `render_human`'s SIGNATURE, or assemble the unsafe-pairing message in `run_next`, which already has both paths in scope. Nothing in the sidecar constrains that function's signature; "`render_human` is a pure function of the projection" is a description of today's code, not a clause of the specification. The finding's own "WHAT SHOULD CHANGE" proposes exactly this ("moving the unsafe-pairing message out of `render_human`"), which contradicts its own every-escape-is-closed framing.

WHAT SURVIVES, AND IT IS STILL WORTH FIXING. The residual is UNDER-SPECIFICATION, not unbuildability: the sidecar requires the human message to name two paths, specifies a carrier that cannot hold them, and never says how the renderer gets them. The path of least resistance (print the bare token) passes every runnable acceptance check, since check 14b asks only for "a reason naming the unsafe pairing", which a bare token arguably satisfies. The shipped result would be `next` printing `no active review loop (metrics-not-this-project)` and a metrics line with no paths, on the exact surface this decision exists to make legible, while `status` names them. That is a real but bounded shortfall plus avoidable implementer churn on the increment's hardest section, which is `medium`. Because escapes exist, it does NOT go above `medium`.

MINIMAL FIX AND SITE COUNT. Two sites carry the requirement: lines 182 and 224 (`grep -c "derived root"` returns 2 across lines 224 and 316, and line 316 is the validator's refusal message, which is correct as written; `derived project root` returns 1, at line 182). One clause naming the carrier fixes both: say that the enum is the machine value and the human message is assembled by the CALLER from the paths it already holds, and reconcile check 14h with whatever that implies. A DELETION-ONLY variant exists (strike "naming the resolved log and the derived root" from lines 182 and 224), but it discards real value; the message stops saying which log was rejected. PREFER THE ONE ADDED CLAUSE OVER A PARAGRAPH: this is a carrier decision, not an explanation.

---

## EX-2. `VALID`. Severity `medium` (unchanged). Two of three sub-claims reproduce as stated; the third is REPLACED by a different real defect on the same input

Sub-claim 1, THE METRICS OVERLAP: DOES NOT REPRODUCE AS STATED. The finding says a `--metrics` path outside the root that does not exist makes both `log-absent` and `log-not-this-project` true with no stated precedence. The sidecar's own definitions make them DISJOINT: line 216 is "no file at the resolved metrics path" and line 217 is "A FILE EXISTS at the resolved path, but it is not under the plan's project root". Two variants that cannot both hold need no precedence rule, so the stated mechanism fails.

A DIFFERENT AND REAL DEFECT SITS ON THE SAME INPUT, which I verified by reading rather than by running. For an explicit `--metrics` outside the root whose leaf does not exist: line 180 makes containment the trigger for the omit on all three surfaces, and line 164 has the guard resolve a non-existent leaf through its longest existing ancestor, so the omit FIRES; then line 217's definition forces `log-absent` (no file exists), while line 232's correlation rule states flatly that on an unsafe metrics pairing `metrics_absent_reason` IS `log-not-this-project`. The definition and the correlation rule contradict each other on that input, and neither variant fits. Worse, the projections then report a bare absence for a log they cannot vouch for, which is the exact conflation line 188 forbids ("UNSAFE IS NOT ABSENT"). So the sub-claim's conclusion (the vocabulary is under-specified here) stands; its stated mechanism does not, and the fix pass must address the collision I describe rather than the precedence the finding asked for.

Sub-claim 2, THE LEDGER OVERLAP: REPRODUCES AS STATED. Line 228 is "no file at the resolved ledger path" and line 230 is "an explicit `--ledger-fragment` resolves outside the plan's project root", with NO existence qualifier on the second. An explicit `--ledger-fragment` outside the root that does not exist satisfies both, and no precedence is stated. Genuine.

Sub-claim 3, THE CORRELATION RULE AGAINST STEP-DERIVED REASONS: REPRODUCES, and I re-ran both demonstrations. A one-step `deferred` plan prints `no active review loop (all steps complete)` and an empty plan prints `no active review loop (no plan steps found)`, both at exit 0. `no_active_loop_reason` is a single `Option`, so on an unsafe metrics pairing over an all-terminal plan, line 232's unconditional "AND `no_active_loop_reason` is `metrics-not-this-project`" is either violated or satisfied by asserting something false (that the log is why there is no loop, when the plan has no work). The rule is stated absolutely and cannot be met absolutely.

MINIMAL FIX AND SITE COUNT. One contiguous section (sidecar lines 214-232) plus optionally check 14f at line 324. The smallest coherent repair is ONE sentence covering both path fields ("where an absent and an unsafe cause both apply, the unsafe variant wins"), plus a NARROWING of line 232 rather than an addition to it: delete or qualify the unconditional clause so the correlation rule binds only when the loop's absence is metrics-derived. The line 232 half is deletion-class. Do not answer this with a paragraph; the section is already the longest in the file.

---

## EX-3. `VALID`. Severity `medium` (unchanged). The sidecar's diagnosis is factually wrong and its instruction would write a NEW false statement into shipped code

REPRODUCED, both by construction and by running. `select_active_loop` (`src/next.rs:589-614`) has a third arm at `:607-611` returning `Some(build_pending_loop(step, LoopState::Blocked, ...))` for any pending step whose blockers are unmet. I built the finding's fixture and ran it:

```
$ agent-scaffold next --source docs/plans/blocked.plan.toml
task: blocked
metrics: no log found

ACTIVE LOOP
  waiter  not started -> -
  state: blocked
  next: resolve the unmet blockers before starting (no spawn)
exit: 0
```

An ACTIVE LOOP, not a `None`. So sidecar line 204's "THAT CASE IS FOLDED INTO THE THIRD STRING" is false: the case is not a cause of `None` at all, and the doc comment at `src/next.rs:108-109` is wrong for a different reason than the sidecar gives. An implementer executing line 352 literally ("reconcile the comment to what the code distinguishes") from that premise would write that the blocked case is reported as "no in-progress or ready step", a fresh false statement replacing an old one, inside the doc comment the same increment is touching to make the enumeration honest.

I confirmed the correct statement independently. `StepPhase` has exactly seven variants (`src/next.rs:388-396`); `is_pending` is `NotStarted | Next` (`:415-417`), `is_terminal` is `Complete | Skipped | Optional | Deferred` (`:421-426`), and `InProgress` is arm 1, so the three arms partition the phase set exhaustively. `active_loop` is `None` ONLY when there are no steps or when every step is terminal.

MINIMAL FIX AND SITE COUNT. Two sites, lines 204 and 352 (`grep -c "every pending step blocked"` returns 2). Correct the diagnosis in one clause and state the target text for the reconciled comment, so the implementer is not deriving it from a wrong premise. This is a correction, not an addition; keep it to the sentence that is wrong.

---

## EX-4. `VALID`. Severity UPGRADED `medium` -> `high`. Acceptance check 7 instructs an implementer to write a decoy over this repository's live, tracked review ledger

REPRODUCED. Sidecar line 312 says, verbatim: "rename a fixture's plan to `agent-scaffold.plan.toml`, put a decoy `docs/plans/agent-scaffold.ledger.md` in the current directory, and run `agent-scaffold status --resume --source "$FIXTURE/docs/plans/agent-scaffold.plan.toml"` from the agent-scaffold root." The current directory is stated by the check itself to be the agent-scaffold root. `git ls-files docs/plans/agent-scaffold.ledger.md` returns the path, the file is 790 lines, and its `## RESUME STATE (compaction checkpoint, read this first)` block begins at line 329. Executed literally, the check overwrites the orchestrator's in-flight workflow state.

THE CHECK IS ALSO SELF-DEFEATING, AND I MEASURED THAT IT DOES NOT NEED THE DECOY. I ran check 7 WITHOUT writing anything: renamed a fixture's plan to `agent-scaffold.plan.toml` and ran `status --resume --source "$FIXTURE/..."` from the repository root. It printed this repository's real `## RESUME STATE` block verbatim, which is exactly the leak the check exists to pin. The repository's own committed ledger IS the decoy. Once a decoy replaces it, the check's own assertion ("must NOT print any line of THIS REPOSITORY'S `## RESUME STATE`") is being made against text the implementer just wrote.

WHY `high` AND NOT `medium`. Severity here is the impact of the instruction, not the size of the wording fix. This is a destructive instruction in a document whose whole purpose is to be executed, aimed at the one file this project's workflow resumes from (`pack/AGENTS.md:63`), and the population most likely to execute it literally is an agent. It stops short of `critical` because the file is tracked and committed, so the committed content is recoverable from git; what is NOT recoverable is any uncommitted RESUME STATE edit in flight when the check runs, and an implementer who does not notice the overwrite carries it into a commit. Note the contrast the finding draws with check 17 (line 329), which correctly scopes its file creation to the borrowed-slug fixture; the standard is already applied elsewhere in the same list.

MINIMAL FIX, DELETION-ONLY, SINGLE-SITE. `grep -c decoy` over the sidecar returns 1, at line 312. Delete the clause "put a decoy `docs/plans/agent-scaffold.ledger.md` in the current directory, and". Nothing else changes: I verified the check still demonstrates the leak with the clause removed. THIS IS THE BEST-SHAPED FIX IN THE ROUND: it deletes, it re-seeds nothing, and it is one clause.

---

## EX-5. `VALID`. Severity `medium` (unchanged). Inc1's stated safety property is false, and the sidecar's own check 4 requires the counterexample

REPRODUCED, measured on this branch. Same fixture, same command line, no `--metrics`, borrowed slug at `complete`, fixture given a log of its own with no evidence for that slug:

```
=== today, run from the agent-scaffold root ===
docs/metrics/workflow.jsonl: 239 records, valid
/tmp/.../fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/tmp/.../fixture/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0

=== the pairing inc1's anchor will produce, same files ===
docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; ...
exit: 1
```

So an invocation that exits 0 today exits 1 after inc1, which falsifies line 272's "NO new failure mode: every invocation that exited 0 before still exits 0". Sidecar line 309 (check 4) demands precisely that flip: "Give the fixture a log of its OWN with no evidence for that slug and expect the correct RED." The document contradicts itself across 37 lines. Explorer A measured the same flip on a real post-anchor build; I opened `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:203-209` and the quoted passage is there verbatim ("the anchored default produces not merely the absence of a green but the correct red: ... exit: 1").

The finding's reasoning for why this matters rather than being a wording slip holds: inc1 is `risky`, its review question is whether the derivation names the right file, and a reviewer or implementer holding line 272 as the invariant can either write an exit-code-preservation test that fails on the legitimate case or read a correct new red as a regression and weaken the derivation to suppress it. The second is the self-concealing failure mode the increment exists to remove.

MINIMAL FIX AND SITE COUNT. Single-site: line 272. (`grep -c "still exits 0"` returns 2; the other, at line 319 in check 14, is a correct statement about the refusal not firing without `--workflow`.) Replace the false clause with the narrow true one: inc1 adds no new REFUSAL mechanism, and any new non-zero exit comes from the pre-existing W3 check finally running against the right project. A pure deletion of the clause is also viable and loses little, since check 9 (byte-identical output on the correct case) already carries the real no-regression property.

---

## EX-6. `VALID`. Severity DOWNGRADED `medium` -> `low`. The inc1 documentation-impact list misses one of five prose sites

REPRODUCED. `grep -rn 'docs/plans/<task>.ledger.md' src/` returns FIVE prose sites: `src/main.rs:461`, `:464`, `:482`, `:1133`, `:1149`, plus the code itself at `:1137`. Sidecar line 341 names `:464-466` and `:482-484`, and line 342 names `:1133-1135` and `:1147-1151` (which covers `:1149`). `src/main.rs:461` is uncovered, and I read it: it is `StatusArgs::resume`'s own help string, three lines above one the list does name, and it states the same superseded default ("from --ledger-fragment, or `docs/plans/<task>.ledger.md` derived from the plan source"). An implementer working the enumerated list literally leaves `status --help` stale after inc1 moves the ledger, against the sidecar's own standard at lines 286 and 336.

WHY `low` AND NOT `medium`. Severity is absolute impact if left unfixed, and that impact is one stale parenthetical in one help string, telling a user the ledger sits at `docs/plans/<task>.ledger.md` when it sits beside the plan. It is a real documentation-currency defect and it is cheap, but it misleads about a path, not about behaviour, and the same class of defect elsewhere in this round is rated `low`. This is a borderline call and it does not change what the fix pass does: fix it regardless.

MINIMAL FIX AND SITE COUNT. Single-site: add `:461` to the line 341 list. This authors no prose; it adds a citation.

---

## EX-7. `VALID`. Severity `low` (unchanged). `no-ready-step` is unreachable, which the section's own governing rule forbids

REPRODUCED by construction and confirmed by exhaustive case analysis. `no_loop_reason` (`src/next.rs:953-961`) is called at exactly one site, `src/next.rs:572-573`, and only when `active_loop.is_none()`. Its third branch requires steps to be non-empty and not all terminal. By the phase partition established under EX-3, `select_active_loop` returns `Some` for every non-terminal phase, so whenever `no_loop_reason` runs, either steps is empty or all steps are terminal. The third string is dead, and `render_human`'s `unwrap_or("no in-progress or ready step")` at `src/next.rs:1031` is dead for the same reason. I confirmed the two reachable answers by running (`no plan steps found` for an empty plan and for a bare `next` with no arguments; `all steps complete` for a one-step `deferred` plan) and confirmed under EX-3 that the blocked case yields a loop instead. No input reaches the third string.

Sidecar line 204 sets the governing rule ("The enum's variant set must match WHAT THE CODE CAN ACTUALLY DISTINGUISH"), and line 223 specifies a variant that violates it. Check 14f cannot exercise it.

MINIMAL FIX AND SITE COUNT. Single-site: line 223. Two acceptable forms, and one of them is DELETION-ONLY: drop the `no-ready-step` bullet (and say the reconciliation collapses `no_loop_reason` to two answers), or retain it with an explicit note that it is a currently-unreachable defensive default. PREFER THE DELETION. Note that this shares a root cause with EX-3 and should be fixed in the same pass.

---

## EX-8. `VALID`. Severity `low` (unchanged). Two acceptance checks pin a record count that is already stale, and one of them is vacuous

REPRODUCED. `grep -c . docs/metrics/workflow.jsonl` returns 239, and I ran `next --source "$FIXTURE/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl` from the repository root and got `metrics: 239 records`. Check 5 (line 310) says the output "must NOT print `metrics: 235 records`", which is ALREADY TRUE of the pre-fix binary, so that clause cannot fail and cannot detect the defect it is written for. Check 14b (line 320) says "Before inc2 this prints `metrics: 235 records`", which is now false as a statement about the pre-fix build. The count grows on every future round, so both will keep drifting. Check 5's remaining clauses (no `state: converged`, no `next: mark the step complete`) still discriminate, which is why this is a weakened check rather than a broken one.

MINIMAL FIX, DELETION-ONLY, TWO SITES. `grep -c 235` over the sidecar returns 8; six of the eight (lines 72, 75, 81, 122, 164, 262) are HISTORICAL observations correctly framed as such, including line 72's explicit acknowledgement that the count drifts, and must not be touched. Only lines 310 and 320 state 235 as an EXPECTATION. Delete "must NOT print `metrics: 235 records`, " from check 5 and "`metrics: 235 records`, " from check 14b's before-list. Both checks keep their discriminating clauses and no prose is authored.

---

## EX-9. `VALID`. Severity `low` (unchanged). The queued-step inheritance note carries a false statement about a production call site, faithfully relayed from a source that was wrong

REPRODUCED. `src/next.rs:1339` is inside `#[cfg(test)] mod tests` (which opens at `src/next.rs:1077-1078`), in `assert_differential` (`:1327-1342`), the differential test whose doc comment at `:1324-1326` says it exists so `next`'s forward verdict and `w3_problems`' backward verdict agree on the same records. `grep -rn "w3_problems(" src/` confirms the only non-test caller in the crate is `src/workflow.rs:217`. So sidecar line 264's "the minimal-diff filter placement does NOT cover `src/next.rs:1339`, which calls `w3_problems` directly" is false about the code: `next` does not call `w3_problems` in production at all.

A POINT NEITHER REVIEWER RESOLVED, AND IT MATTERS FOR THE FIX. The error originates with explorer C, not with the planner. `docs/plans/workflow-enforcement-tier.explorations/metrics-path-independent-map.md:189-196` says "`next`'s own `w3_clean` check (`src/next.rs:1339`) is NOT protected by this filter, since it calls `w3_problems` directly rather than through `check_workflow_toml`", and `:444` repeats it. The fidelity lens is therefore right that the sidecar reports its source faithfully, and the executability lens is right that the statement is false. Both hold: FIDELITY TO A SOURCE IS NOT TRUTH OF THE SOURCE. The conclusion survives for a different reason (a filter placed inside `run_checks` would leave `next` unprotected, because `next` derives convergence independently through `select_active_loop` and the shared accessors documented at `src/next.rs:730`), and carried forward as written it sends the validation-constraints step's implementer to a test looking for a call site to protect.

MINIMAL FIX AND SITE COUNT. Single-site: line 264 (`grep -rn 1339 docs/plans/agent-scaffold.steps/` returns one hit). A DELETION-ONLY form exists: strike ", which calls `w3_problems` directly", which removes the false statement without authoring anything, though it leaves a bare citation to a test. The better form replaces the clause with the true reason in one sentence. Either is acceptable; do not expand it into a paragraph about the call graph.

---

## EX-10. `VALID`. Severity `low` (unchanged). Two inc1 resolution inputs are left to the implementer to guess

BOTH REPRODUCE.

- THE LEDGER WITH NO PLAN SOURCE. Neither `--source` nor `--plan` is required by clap on `status` or `next`. I ran bare `agent-scaffold next` and got `task: task`, `source: no plan source`, exit 0, confirming `derive_task` (`src/next.rs:993-1003`) yields the literal `"task"` and `default_ledger_path` (`src/main.rs:1136-1138`) today returns `docs/plans/task.ledger.md`. Sidecar line 272 says the ledger resolves "BESIDE" the plan source and line 136 says "the source's own directory is the whole rule", neither of which has an answer when there is no source. The metrics rule states its answer for the same case explicitly at line 158; the ledger rule does not.
- SOURCE-VERSUS-PLAN PRECEDENCE. Line 245 says candidate (a) derives its root "from the `--source` OR the `--plan` path" and line 158 says only "the source's parent directory". `derive_task` (`src/next.rs:997-999`) establishes a source-then-plan precedent, while `validate --workflow` selects its plan substrate differently (`src/main.rs:957` filters the source to TOML-primary), so "anchor off the source, check the Markdown plan" is reachable and unreviewed.

Both are genuine gaps with no acceptance check, and both have an obvious right answer that the implementer will probably reach anyway, which is why this is `low` and not `medium`.

MINIMAL FIX. One sentence each in the inc1 description at line 272, or a single sentence covering both. This is the one finding in the round where the fix is unavoidably additive prose; keep it to two clauses.

---

## F-1. `VALID`. Severity DOWNGRADED `medium` -> `low`. "Four constraint attributes" contradicts its own five-item list, and one cited line is off by one

REPRODUCED. `grep -n 'requires = \|conflicts_with = ' src/main.rs` returns exactly five: `:396` (`dry_run` conflicts with `write`), `:442` (`workflow_spec` requires `workflow`), `:465` (`StatusArgs::ledger_fragment` requires `resume`), `:525` (`render --strict` requires `check`), `:557` (`audit --out` conflicts with `json`). `status-resume-ignores-json.md:92` says "Four constraint attributes already exist in `src/main.rs` (`:396`, `:441`, `:465`, `:525`, `:557`)": five numbers behind the word "Four", and the true count is five. Separately, `:441` is the first line of the `workflow_spec` field's doc comment; the attribute is at `:442`. The same file cites the same field correctly as a range (`:441-443`) at line 82, and cites the other precedents as ranges too (`:464-466`, `:524-526`, `:556-558`), so the bare-line list is the outlier.

WHY `low` AND NOT `medium`. The argument the sentence makes is unaffected and is in fact strengthened by the true count. The cost of leaving it is a reader who greps and finds five where the prose says four, which is seconds. This project is right to care about re-derived counts, but the impact scale is about consequences, not about how annoying the error is.

MINIMAL FIX, AND IT IS NOT SINGLE-SITE. The reviewer flagged one site; the construct appears at THREE. `grep -ni "four .*constraint\|constraint attributes" docs/plans/agent-scaffold.steps/status-resume-ignores-json.md` returns lines 92, 120 ("in the style the four existing constraint attributes use") and 125 ("Four constraint attributes already exist and the three cases they cover..."). All three say four. THE FIX PASS MUST CHANGE ALL THREE, or it will fix the flagged one and leave the same false count twice in the same file. The `:441` -> `:442` correction is single-site, at line 92.

---

## F-2. `VALID`. Severity `low` (unchanged). `src/main.rs:560-563` is cited twice for a comment that spans 561-564

REPRODUCED. Line 560 is blank, lines 561 to 564 are the four-line doc comment, line 565 is `#[derive(Serialize)]`, and the quoted sentence ("Every part is optional so a missing plan or metrics file yields a partial projection rather than a failure") sits at 562-563. So the quote IS inside the cited range; the range is imprecise at both ends rather than wrong, including one blank line and excluding the comment's last line. The correct range is `561-564`.

MINIMAL FIX AND SITE COUNT. Two sites in the artifact: `workflow-enforcement-tier.md:202` and `:352`. `grep -rn 560-563 docs/plans/` also returns `docs/plans/agent-scaffold.md:1597` and `:1747` (the GENERATED view, which `render` regenerates and which must never be hand-edited) and `docs/plans/agent-scaffold.ledger.md:437` (out of scope, and it is the ledger's own record rather than the artifact). See the note on re-rendering below. The fix is two number edits and authors no prose.

---

## F-3. `VALID`. Severity `low` (unchanged). The quoted comment is at 992-994, not inside the cited 995-998

REPRODUCED. `src/main.rs:995-998` is exactly the match arm (`(None, None, _) => problems.push(...)`), which is a correct citation for "pushes a hard problem". The quoted comment ("`--workflow` was explicitly requested, so skipping would green-pass while checking nothing; make it a hard problem instead.") begins mid-line 992 and ends at line 994. The quote is word-for-word accurate; only its attributed location is wrong.

NOTE ON THE APPARENT CONFLICT BETWEEN THE TWO LENSES. The executability reviewer listed `:995-998` among citations that "resolve to what the sidecar says is there". Both are right at their own resolution: the arm is where the sidecar says, and the quote is not. This is not a contradiction and neither lens is discredited by it.

MINIMAL FIX AND SITE COUNT. Single-site: line 59, the only place the quote appears. `grep -rn 995-998` also returns line 298, which cites the arm as an in-tree precedent without quoting the comment and is correct as written. Widen line 59's citation to `:992-998`, or split the comment out as `:992-994`. A number edit, no prose.

---

## F-4. `VALID`. Severity `low` (unchanged). `src/main.rs:1150` is cited for a quote spanning 1150-1151

REPRODUCED. Line 1150 ends with "since" and the clause "`status` is a best-effort projection, not a validator." is on line 1151. The full doc comment runs 1147-1151 and is cited correctly as that range elsewhere in the same file.

MINIMAL FIX AND SITE COUNT. Single-site: sidecar line 174 (`grep -rn "src/main.rs:1150"` over the steps directory returns one hit). Change `:1150` to `:1150-1151`. A number edit, no prose.

---

## F-5. `VALID BUT ACCEPT RESIDUAL`. Severity `low` (unchanged). The dangling `validation-constraints` reference is PRE-EXISTING PLANNER DEBT, not a defect of this fold

REPRODUCED, and the fold-versus-pre-existing question resolves against the finding.

- There is no step with that slug. The plan TOML holds 95 `[[step]]` blocks and 95 `slug =` lines; `grep -n '^slug = .*valid'` returns nothing, and `validate` itself reports "95 steps, 69 questions, valid". A reader following the cross-reference from the plan alone finds nothing.
- THE DEBT PREDATES THE FOLD. `git show main:docs/plans/agent-scaffold.plan.toml | grep -c validation-constraints` returns 0, but the handle already exists on main in `docs/plans/agent-scaffold.ledger.md` (7 occurrences, including the human decision at `:633`, "HUMAN DECIDED (2026-07-30) where the three `agent-scaffold next` defects go: FOLD THEM INTO GATE 4, the validation-constraints step"), in `docs/metrics/workflow.jsonl`, and in two of the three exploration records. The step was decided by a human on 2026-07-30 and never entered as a `[[step]]`. This fold cites it (four times in the sidecar, three in the TOML's Q-55 prose) and is the first place the plan TOML treats it as a destination, but it did not create the gap.
- The substance is sound, which the reviewer establishes and I confirm: the queued work is grounded in a decision receipt (`Q-55-mechanism`), not fabricated.

WHY ACCEPT THE RESIDUAL FOR THIS ROUND. Both offered remedies cost more than the defect inside this fold. Adding a stub `[[step]]` means choosing an order, a status and blockers and authoring a body sidecar, which is a plan decision belonging to the human or the orchestrator, not to a fix pass on this fold, and it would enlarge the fold with an unreviewed step. Noting the non-existence at first mention authors prose in four places to describe a gap that the ledger already documents. THE RIGHT DISPOSITION IS TO LOG THE UNDERLYING DEBT AS ITS OWN PLAN ACTION (enter the human-decided `validation-constraints` step), not to patch its symptom here. Flagging this for the orchestrator rather than the fix pass.

---

## Deduplication

- NO FINDING IS RAISED BY BOTH REVIEWERS. The two lenses returned disjoint sets, so nothing in this round has the corroboration that a doubly-raised finding would carry. Each finding stands on its own evidence.
- EX-3 AND EX-7 ARE ONE ROOT CAUSE SEEN TWICE. Both follow from the same wrong model of when `active_loop` is `None`: EX-3 is the wrong diagnosis of the `src/next.rs:108-109` doc comment, EX-7 is the unreachable `no-ready-step` variant that the same wrong model justifies. They are not the same defect (one is a diagnosis, one is a specified variant) but ONE corrected fact resolves both, and they must be fixed in the same pass or the fix to either will look inconsistent with the other.
- EX-1 AND EX-2 SIT IN ONE SECTION (sidecar lines 210-232) and are distinct defects (a carrier gap and a precedence gap). Fix them together, in one pass over that section.
- F-2, F-3, F-4 AND THE `:441` HALF OF F-1 ARE ONE CLASS: citation ranges that are off by one or two lines. They are four separate sites and none subsumes another, but they are one mechanical edit pass.
- EX-9 AND THE FIDELITY LENS'S CLEARED CHECK OF THE SAME TEXT ARE NOT DUPLICATES AND DO NOT CONFLICT. See the note under EX-9: the sidecar is faithful to explorer C and explorer C is wrong about the code.

## Errors inside the findings files

Checked because this project has repeatedly caught misnumbered citations inside findings files.

- F-5 STATES "90 slugs total, via `grep -n '^slug = '`". THE CORRECT COUNT IS 95, which `grep -c '^slug = '`, `grep -c '^\[\[step\]\]'` and `validate --source` ("95 steps") all agree on. The load-bearing claim (no `validation-constraints` slug exists) is correct, so the finding survives, but the count in its evidence paragraph is wrong and should not be carried into the ledger.
- THE FIDELITY LENS'S COVERAGE CLAIM IS OVERSTATED ON ONE POINT. It says it checked "every cross-referenced step (orders 63, 64, 84, 88, 92, 93, 94, 95, 96) against the sidecars' claims about it". It missed one: see the item below.
- Every other quantified claim in both files that I re-derived was correct: the five clap constraint attributes, the 386-test total, the 1514 exploration lines, the six `Q-55*` receipts, the two `pack/AGENTS.md` mentions of the round log, the single `#[serde(skip)]`, the zero `skip_serializing_if`. Every sidecar line number cited by the executability lens (lines 72, 103, 136, 158, 162, 164, 172, 180, 182, 188, 204, 212, 214, 219, 223, 224, 226-232, 245, 254-256, 264, 272, 286, 309-312, 319, 320, 323, 326, 329-331, 336, 341, 342, 352, 362, 371, 375, 378) resolves to what it says.

## One item raised during reproduction, not a triaged finding

Recorded because I found it while reproducing and both reviewers cleared the file it is in. It is NOT one of the fifteen and is not counted above; the orchestrator decides whether it joins this round's fix set.

`docs/plans/agent-scaffold.steps/test-tmpdir-repo-assumption.md:76` describes `checks-runner-worktree-name-collision` as "(order 93, `deferred`)". The step's status in `docs/plans/agent-scaffold.plan.toml:1279` is `complete`. The order is right and the `risk_class = "risky"` cited at line 60 of the same sidecar is right (`plan.toml:1286`); only the status label is wrong. Severity `low`: the point the bullet makes (the two test-isolation defects are deliberately separate) is unaffected. Single-site, and the fix is a one-word deletion or replacement. Both reviewers cleared this sidecar, and the fidelity lens named order 93 among the cross-references it checked, so this is the round's one demonstrated gap in a stated coverage claim.

## Guidance for the fix pass

- FIX SET, IN DESCENDING ORDER OF WHAT IT BUYS: EX-4 (destructive instruction, deletion-only), then EX-5, EX-3 with EX-7, EX-1 and EX-2 together, then the low-cost mechanical set (EX-6, EX-8, EX-9, F-1, F-2, F-3, F-4).
- SEVEN OF THE FOURTEEN VALID FINDINGS ADMIT A DELETION-ONLY OR NUMBER-EDIT-ONLY FIX, which re-seeds nothing: EX-4 (delete the decoy clause), EX-8 (delete two stale literals), EX-7 (drop the unreachable variant), EX-9 (strike the false clause), F-2, F-3, F-4 and the `:441` half of F-1 (citation numbers), EX-6 (add one citation to a list). EX-5 and EX-2's third sub-claim are narrowings, which are the next best shape.
- ONLY THREE FINDINGS REQUIRE NEW PROSE, AND ALL THREE SHOULD BE ONE CLAUSE OR ONE SENTENCE: EX-1 (name the carrier), EX-2 (one precedence sentence), EX-10 (two clauses in the inc1 description). This project has five retrospective and one prospective confirmation that a fix pass which AUTHORS prose manufactures the next round's finding, so the target is the smallest true statement in each case, not an explanation of why it is true.
- F-1 IS THE ONE MULTI-SITE FIX: three sites, not the one flagged. Every other valid finding is one or two sites, and the counts are stated in each entry above.
- AFTER EDITING ANY SIDECAR, RE-RENDER. `docs/plans/agent-scaffold.md` carries the same text (I found the F-2 citation at `:1597` and `:1747`) and is a generated projection that must never be hand-edited. `render docs/plans/agent-scaffold.plan.toml --check` reports "up to date" at this commit, so it will catch a missed re-render.
- NOTHING IN THIS ROUND TOUCHES A DECIDED ITEM. I found no finding that re-litigates the enforcement tier, the one-step/three-increment shape, the anchor-plus-refusal mechanism, the conventionless fallback, the omit-and-exit-0 behaviour, the serialised reason, the two accepted costs, the nearest-wins judgement, or the open TMPDIR fork. EX-1, EX-2 and EX-5 all report the sidecar failing to produce or state the decided behaviour, which the reviewers were correctly told is in scope.
