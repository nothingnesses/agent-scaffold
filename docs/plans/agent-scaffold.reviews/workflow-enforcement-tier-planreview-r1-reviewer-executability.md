# `workflow-enforcement-tier` plan review, round 1, reviewer: EXECUTABILITY lens

Reviewer model: Claude Opus 5, 1M-context variant. Exact model id `claude-opus-5[1m]`.
Date: 2026-07-31.
Worktree: `.claude/worktrees/rev-q55-r1-exec`, branch `plan/q55-enforcement` at commit `6df032c`.
Artifact: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md` (primary), plus `test-tmpdir-repo-assumption.md`, `status-resume-ignores-json.md`, and the `[[step]]`/`[[question]]` entries this fold adds to `docs/plans/agent-scaffold.plan.toml`.

Lens: read the sidecar as INSTRUCTIONS TO FOLLOW. I simulated each of the three increments as the implementer, ran every acceptance check that describes CURRENT behaviour, checked the closed enums from a consumer's position, checked the increment ordering claims, and checked every command the sidecar tells an implementer to run.

## What reproduced, so the negative results are grounded

Everything below was run in this worktree with `TMPDIR=/tmp/rev-exec-scratch` (outside any repository) and the debug binary at `target/debug/agent-scaffold`. Fixture rebuilt per the sidecar's own command into `/tmp/rev-exec-scratch/fixture`.

- `cargo build`, `cargo test` (386 passed, 0 failed), `render --check` ("up to date"), and `validate --source docs/plans/agent-scaffold.plan.toml --workflow` (exit 0, `workflow invariants hold`) are all clean on this branch. Acceptance check 1's "386 expected" is accurate.
- The fixture reproduces: 30 files written, `ls "$SCRATCH/docs"` prints only `plans` (acceptance check 2).
- Defect A reproduces byte-for-byte, including the two stderr notes and the single stdout line, at exit 0.
- Defect B reproduces, and the borrowed-slug FALSE PASS reproduces (`workflow invariants hold`, exit 0, for a `complete` foreign step with no evidence of its own).
- The control at sidecar line 103 reproduces verbatim, at exit 1.
- The fabricated `next` instruction reproduces (`state: converged`, `streak: 1/1`, `rounds: 2/5`, `next: mark the step complete, re-render, and commit`, exit 0), on both the default path and the explicit `--metrics` path, so checks 5, 11, 14b and 15 are all genuinely RED today.
- All 60-odd `file:line` citations I checked resolve to what the sidecar says is there, including `src/main.rs:958-1004`, `:995-998`, `:999-1003`, `:1007-1011`, `:823-847`, `:1136-1138`, `:1200-1205`, `:1208-1212`, `src/next.rs:99-118`, `:114-115`, `:116`, `:953-961`, `:1017-1043`, `:1705`, `:1762`, `src/workflow.rs:180-195`, `:448-449`, `src/plan/source.rs:102`, `:480-495`, `src/plan/render.rs:167-169`, `:296`, `pack/AGENTS.md:61`, `:63`, `:93`, `:116`, `justfile:46-48`, `README.md:210`, `:212-224`, `:226`, `:228-237`, and the whole `test-tmpdir-repo-assumption` citation set. The six `Q-55*` receipts all exist with `task:"workflow-enforcement-tier"`. The exploration line counts (521 / 483 / 510 = 1514) are exact. The `#[serde(skip)]`-appears-once and no-`skip_serializing_if`-in-those-two-files sweeps are both correct. One citation is misattributed; see EX-9.

Answer to "could a competent implementer execute this from the sidecar alone": QUALIFIED NO for inc2. Inc1 and inc3 are executable with two small guesses. Inc2's JSON-reason section boxes the implementer into a choice the sidecar forbids on every side (EX-1) and leaves three overlapping-cause precedences unstated (EX-2).

---

## EX-1. `medium`. The specified enum cannot carry the reason text the sidecar requires the human renderer to print, and every escape route is closed by another clause

THE INSTRUCTION. Two places require the human-surface message to name paths. Sidecar line 182: "`status`. The metrics half is omitted: print `metrics: unavailable, <reason>` in place of `metrics: <n> records`, where the reason names the resolved log, the derived project root, and that the two do not correspond." Sidecar line 224, on the `no_active_loop_reason` variant: "`metrics-not-this-project`, the NEW case: ... Printed with a reason naming the resolved log and the derived root."

WHY IT CANNOT BE DONE FOR `next`. `render_human` is a pure function of the projection: `pub(crate) fn render_human(projection: &NextProjection) -> String` at `src/next.rs:1017`. `NextProjection` (`src/next.rs:99-118`) carries exactly `task`, `source`, `metrics`, `active_loop`, `resume_state`, `no_active_loop_reason`. There is no metrics path, no ledger path and no root anywhere in it; `ledger_path` reaches `LoopContext` (`src/next.rs:532`, `:545`, `:567`) and is consumed only as a prompt slot at `src/next.rs:879`, so it is unavailable when `active_loop` is `None`. A unit-variant enum therefore gives the renderer a token and nothing to interpolate.

THE FOUR ESCAPES, EACH BLOCKED BY THE SIDECAR ITSELF.

- Add a parallel serialised reason string beside the enum: explicitly rejected at sidecar line 219, "Retyping is preferred over adding a parallel serialised field: one reason, one type, rendered two ways, rather than two representations of the same fact that can disagree (One source of truth)."
- Make the variant carry the paths (a newtype or struct variant): serde would serialise it as an object, not a bare string, which fails acceptance check 14e's literal assertion `"no_active_loop_reason": "metrics-not-this-project"` (sidecar line 323) and breaks the `LoopState` kebab-case precedent the vocabulary is told to follow (sidecar line 212, `src/next.rs:187-189`).
- Add path fields to `NextProjection` and `Projection` so the renderer can interpolate: not in the specified field set (sidecar lines 214-232 name exactly three new fields), and acceptance check 14h says "The `GOLDEN_JSON` diff (`src/next.rs:1705`) must be exactly the added fields and nothing else" (sidecar line 326).
- Drop the paths and print the bare token: contradicts lines 182 and 224, and is the outcome that would actually ship, because it passes every runnable acceptance check. The user-visible result would be `next` printing `no active review loop (metrics-not-this-project)` with no indication of which log was rejected or against which root, on the exact surface this decision exists to make legible.

`status` is not affected the same way: its human print is inline in `run_status` (`src/main.rs:1125-1128`) where the resolved path is in scope, so `status` can name the paths and `next` cannot. That asymmetry is itself worth stating, because the sidecar treats the two surfaces as one rule.

WHAT SHOULD CHANGE. Decide the carrier explicitly in the sidecar. The smallest resolution consistent with the rest of the file is to say that the enum is the machine value and the human message is assembled by the CALLER from data the caller already has, which for `next` means either passing the two paths into `next::project` and onto `NextProjection` as fields (and saying so, so check 14h expects them) or moving the unsafe-pairing message out of `render_human`. Whichever is chosen, say it, and reconcile check 14h's "exactly the added fields and nothing else" with it.

---

## EX-2. `medium`. No precedence rule for overlapping causes, in all three reason vocabularies; the stated correlation rule is unsatisfiable in one of them

THE INSTRUCTION. Sidecar line 214: "`metrics_absent_reason` ... `Some` exactly when `metrics` is `None`", with two variants `log-absent` and `log-not-this-project` (lines 216-217). Line 226: "`resume_state_absent_reason` ... `Some` exactly when `resume_state` is `None`", with `ledger-absent`, `no-resume-section`, `ledger-not-this-project` (lines 228-230). Line 232, THE CORRELATION RULE: "On an unsafe metrics pairing, `metrics_absent_reason` is `log-not-this-project` AND `no_active_loop_reason` is `metrics-not-this-project`, both set in the same output."

THREE REAL CASES SATISFY TWO VARIANTS AT ONCE, AND THE SIDECAR NAMES NO WINNER.

- `--metrics <path outside the plan's root that does not exist>`. The file is absent AND outside the root. Both `log-absent` and `log-not-this-project` are true. The natural implementation follows today's code shape, `if args.metrics.exists()` at `src/main.rs:1090` and `src/main.rs:1200`, which tests existence FIRST and would report `log-absent`. That is exactly the conflation the sidecar forbids at line 188 ("UNSAFE IS NOT ABSENT: absent means 'this project has no rounds', unsafe means 'this tool cannot tell you anything about this project's rounds'"), and it splits the surfaces: `validate --workflow` would REFUSE the same inputs, because the guard is specified to run before the four-arm match and to resolve a non-existent leaf through its longest existing ancestor (sidecar line 164), while `status` would report a bare absence. That contradicts line 180's "One predicate, three consumers" and "The predicate is never re-implemented per surface (One source of truth)".
- The same overlap on `resume_state_absent_reason`: an explicit `--ledger-fragment` outside the root that also does not exist satisfies both `ledger-absent` and `ledger-not-this-project`.
- An unsafe metrics pairing on a plan with NO steps, or with all steps terminal. `active_loop` is `None` for a reason that has nothing to do with the log. Demonstrated: a one-step `deferred` plan prints `no active review loop (all steps complete)` and an empty plan prints `no active review loop (no plan steps found)` (both run in this worktree against `/tmp/rev-exec-scratch/vocab/docs/plans/allterm.plan.toml` and `nosteps.plan.toml`). If the implementer keeps the existing reason, the correlation rule at line 232 is violated (`metrics_absent_reason` is `log-not-this-project` while `no_active_loop_reason` is `all-steps-complete`); if the implementer forces `metrics-not-this-project`, the correlation rule holds but the output asserts the log is why there is no loop when it is not. Either way a consumer that "recognises the token needs no lookup table to correlate them" (line 232) is told something false in one of the two arrangements.

WHAT SHOULD CHANGE. State the precedence explicitly for each field, in the vocabulary section: which variant wins when a cause overlaps, and whether `metrics-not-this-project` outranks the step-derived reasons on `no_active_loop_reason`. Given line 188's rule, the unsafe variant should almost certainly win over the absent variant on both path fields, and that is the one sentence the section is missing. Add a fourth run to acceptance check 14f pinning the overlap, since 14f's whole claim is that the vocabulary separates the causes and it currently tests only three non-overlapping ones.

---

## EX-3. `medium`. The sidecar's account of the pre-existing `src/next.rs:108-109` doc defect is factually wrong, and executing its instruction literally writes a NEW false statement into shipped code

THE INSTRUCTION. Sidecar line 204: "`src/next.rs:108-109` says `active_loop` is `None` when 'all steps complete, every pending step blocked, or no plan source', while `no_loop_reason` (`src/next.rs:953-961`) can only produce three strings ... There is no distinct 'every pending step blocked' answer; THAT CASE IS FOLDED INTO THE THIRD STRING." Repeated as an implementer instruction at line 352: "reconcile the comment to what the code distinguishes rather than adding a variant to satisfy the comment."

WHY IT IS WRONG. "Every pending step blocked" does not produce `active_loop: None` at all, so it is not folded into any string. `select_active_loop` (`src/next.rs:589-614`) has a THIRD arm at `:607-611` that returns `Some(build_pending_loop(step, LoopState::Blocked, ...))` for any pending step whose blockers are unmet. Demonstrated in this worktree:

```
$ cat /tmp/rev-exec-scratch/vocab/docs/plans/blocked.plan.toml   # step `gate` deferred, step `waiter` not-started, blocked_by = ["gate"]
$ agent-scaffold next --source docs/plans/blocked.plan.toml
task: blocked
source: docs/plans/blocked.plan.toml
metrics: no log found

ACTIVE LOOP
  waiter  not started -> -
  state: blocked
  ...
  next: resolve the unmet blockers before starting (no spawn)
exit: 0
```

An ACTIVE LOOP, not a `None`. So the doc comment at `src/next.rs:108-109` is wrong for a DIFFERENT reason than the sidecar gives: the case it names is not a cause of `None` in the first place. An implementer following line 352 literally would rewrite the comment to say the blocked case is reported as "no in-progress or ready step", which is a fresh false statement replacing an old one, in a doc comment the same increment is touching precisely to make the enumeration honest.

WHAT SHOULD CHANGE. Correct the diagnosis in the sidecar: `active_loop` is `None` only when there are no steps or when every step is terminal (`is_terminal` covers `Complete | Skipped | Optional | Deferred`, `src/next.rs:421-426`), and the blocked case yields `LoopState::Blocked` with a loop. State that as the target text for the reconciled comment rather than leaving the implementer to derive it from a wrong premise.

---

## EX-4. `medium`. Acceptance check 7 instructs the implementer to write a decoy over this repository's live, tracked review ledger

THE INSTRUCTION. Sidecar line 312, acceptance check 7: "rename a fixture's plan to `agent-scaffold.plan.toml`, PUT A DECOY `docs/plans/agent-scaffold.ledger.md` IN THE CURRENT DIRECTORY, and run `agent-scaffold status --resume --source "$FIXTURE/docs/plans/agent-scaffold.plan.toml"` FROM THE AGENT-SCAFFOLD ROOT."

The current directory is stated to be the agent-scaffold root, and `docs/plans/agent-scaffold.ledger.md` there is this repository's real ledger: `git ls-files docs/plans/agent-scaffold.ledger.md` returns the path, and it carries the live `## RESUME STATE (compaction checkpoint, read this first)` block at line 329. Following the instruction literally overwrites the orchestrator's in-flight workflow state, which by this project's own guidance (`pack/AGENTS.md:63`) is the thing the whole loop resumes from. It is recoverable from git, but only if the implementer notices; and any uncommitted RESUME STATE edit is not.

The decoy is also unnecessary and self-defeating. The real ledger already sits at that path, so it is already the leak target the check needs; and once a decoy replaces it, the check's own assertion ("must NOT print any line of THIS REPOSITORY'S `## RESUME STATE`") is being made against text the implementer just wrote, not against this repository's. Note the contrast with acceptance check 17 (line 329), which correctly scopes its file creation to "the borrowed-slug fixture".

WHAT SHOULD CHANGE. Delete the decoy instruction; the repository's own committed ledger is the decoy, and the check works as written without it. If a synthetic ledger is genuinely wanted, put it in a throwaway directory and run from there, and say so explicitly.

---

## EX-5. `medium`. Inc1's stated safety property, "every invocation that exited 0 before still exits 0", is false, and acceptance check 4 requires the counterexample

THE CLAIM. Sidecar line 272, the inc1 description: "NO new failure mode: every invocation that exited 0 before still exits 0, and only WHICH FILE is read changes."

THE CONTRADICTION IN THE SAME FILE. Acceptance check 4 (line 309): "rerun the borrowed-slug demonstration (fixture step `complete` with slug `triager-runs-only-on-findings`) from the agent-scaffold root. Before the fix it exits 0 with `workflow invariants hold`. After, no green. GIVE THE FIXTURE A LOG OF ITS OWN WITH NO EVIDENCE FOR THAT SLUG AND EXPECT THE CORRECT RED." A red is exit 1, from a command line that exits 0 today.

MEASURED, in this worktree, same fixture, same plan source, no `--metrics`:

```
=== today, run from the agent-scaffold root (fixture has its own empty log) ===
docs/metrics/workflow.jsonl: 239 records, valid
/tmp/rev-exec-scratch/fixture/docs/plans/TEMPLATE.plan.toml: 1 steps, 0 questions, valid
/tmp/rev-exec-scratch/fixture/docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
exit: 0

=== the pairing inc1's anchor will produce (same files, run from inside the fixture) ===
docs/plans/TEMPLATE.plan.toml vs docs/metrics/workflow.jsonl: Roadmap step `triager-runs-only-on-findings` is `complete` but has no round records and no covering waiver; ...
exit: 1
```

Explorer A measured the same flip on a real post-anchor build, at `docs/plans/workflow-enforcement-tier.explorations/metrics-path-anchor-to-source.md:203-209`: "When the foreign project HAS a log of its own, the anchored default produces not merely the absence of a green but the correct red: ... exit: 1".

WHY IT MATTERS RATHER THAN BEING A WORDING SLIP. Inc1 is classified `risky` and its review question is "does the derivation name the right file, in every layout". A reviewer or implementer who takes line 272 as the increment's invariant has two bad moves available: write a test asserting exit-code preservation across the anchor (which will fail on the legitimate case), or read a new, correct exit 1 as a regression introduced by the anchor and weaken the derivation to suppress it. The correct property is narrower and should be stated: inc1 adds no new REFUSAL mechanism, and any new non-zero exit it produces comes from the pre-existing W3 check finally running against the right project. That is also what makes acceptance check 9 (byte-identical output on the correct case) the right no-regression test rather than an exit-code sweep.

WHAT SHOULD CHANGE. Replace the "every invocation that exited 0 before still exits 0" clause with the narrower true claim, and say plainly that inc1 can turn a contaminated green into a correct red, which is check 4's whole point.

---

## EX-6. `medium`. The INC1 documentation-impact list misses one of the three help strings that state the superseded ledger convention

THE INSTRUCTION. Sidecar line 341 enumerates the ledger-default statements inc1 makes stale: "`src/main.rs:464-466` (`StatusArgs::ledger_fragment`) and `:482-484` (`NextArgs::ledger_fragment`), both of which say the default is `docs/plans/<task>.ledger.md`; after inc1 it is `<task>.ledger.md` BESIDE the plan source." Line 342 adds `default_ledger_path`'s doc comment (`:1133-1135`) and `run_resume`'s (`:1147-1151`).

THE MISS. `grep -rn 'docs/plans/<task>.ledger.md' src/` returns FIVE prose sites, not four: `src/main.rs:461`, `:464`, `:482`, `:1133`, `:1149` (plus the code itself at `:1137`). The list names `:464`, `:482`, `:1133-1135` and `:1147-1151`, which covers four of the five.

The uncovered one is `src/main.rs:461`, `StatusArgs::resume`'s own help string, three lines above the one the list does name: "Print the ledger's `## RESUME STATE` block verbatim (from --ledger-fragment, or `docs/plans/<task>.ledger.md` derived from the plan source) instead of the state projection." It states the same superseded default. An implementer working the enumerated list literally leaves `status --help` telling users the ledger is at `docs/plans/<task>.ledger.md` after inc1 has moved it. The sidecar's own standard forbids this: line 286, "shipping behaviour with stale prose beside it is the defect that step exists to remove", and line 336, "each item travels with the increment that makes it stale".

WHAT SHOULD CHANGE. Add `src/main.rs:461` to the INC1 documentation-impact list.

---

## EX-7. `low`. `no-ready-step` is an unreachable variant, which the sidecar's own governing rule for the vocabulary forbids

THE RULE THE SIDECAR SETS. Line 204: "The enum's variant set must match WHAT THE CODE CAN ACTUALLY DISTINGUISH."

THE VARIANT. Line 223 specifies `no-ready-step`, "printed as today's 'no in-progress or ready step'", as one of four `no_active_loop_reason` variants.

WHY IT CANNOT BE REACHED. `no_loop_reason` (`src/next.rs:953-961`) is called at exactly one site, `src/next.rs:572-573`, and only when `active_loop.is_none()`. Its third branch requires steps to be non-empty and not all terminal. But `select_active_loop` (`src/next.rs:589-614`) returns `Some` for every non-terminal phase: `InProgress` at arm 1 (`:595-599`), and `NotStarted`/`Next` at arm 2 or arm 3 (`:600-611`), since `is_pending` is `NotStarted | Next` (`src/next.rs:415-417`) and `is_terminal` is `Complete | Skipped | Optional | Deferred` (`src/next.rs:421-426`), which partitions the seven phases exhaustively. So whenever `no_loop_reason` runs, either steps is empty or all steps are terminal, and the third string is dead. Confirmed by construction: I could produce `no plan steps found` (empty plan, and no `--source` at all) and `all steps complete` (one `deferred` step), and the blocked case yields an ACTIVE LOOP instead (see EX-3), leaving no input that reaches the third string. `render_human`'s `unwrap_or("no in-progress or ready step")` at `src/next.rs:1031` is dead for the same reason.

Acceptance check 14f cannot exercise this variant, so the vocabulary ships one value no consumer can ever observe, in a section whose stated purpose is that "a consumer can TELL THE CAUSES APART".

WHAT SHOULD CHANGE. Either drop `no-ready-step` from the specified variant set (and say the reconciliation of `no_loop_reason` collapses to two answers), or state deliberately that it is retained as a defensive default with a note that it is currently unreachable. Do not leave it unlabelled, since the section's own rule says the set must match what the code distinguishes.

---

## EX-8. `low`. Acceptance checks 5 and 14b pin a literal record count that is already stale on this branch, and check 5's first assertion is satisfied by the pre-fix build

THE INSTRUCTION. Check 5 (line 310): "must NOT print `metrics: 235 records`". Check 14b (line 320): "Before inc2 this prints `metrics: 235 records`, `state: converged`, `streak: 1/1`, `rounds: 2/5` and `next: mark the step complete, re-render, and commit` at exit 0."

MEASURED TODAY on this branch, from the agent-scaffold root against the fixture:

```
$ agent-scaffold next --source "$SCRATCH/docs/plans/TEMPLATE.plan.toml" --metrics docs/metrics/workflow.jsonl
task: TEMPLATE
source: /tmp/rev-exec-scratch/fixture/docs/plans/TEMPLATE.plan.toml
metrics: 239 records
...
```

239, not 235: this fold's own commits added four records to `docs/metrics/workflow.jsonl` (`grep -c . docs/metrics/workflow.jsonl` returns 239), and the count grows on every future round. The sidecar acknowledges the drift once, in the defect B section at line 72, but then restates 235 as an EXPECTATION in two acceptance checks. Check 5 is the worse of the two: "must NOT print `metrics: 235 records`" is ALREADY TRUE of the pre-fix binary, so that clause can never fail and cannot detect the defect it is written for. Its remaining clauses (no `state: converged`, no `next: mark the step complete`) still discriminate, so the check is weakened rather than broken.

WHAT SHOULD CHANGE. Replace the literal counts in checks 5 and 14b with the property ("this repository's own record count, whatever it is on the day, rather than the fixture's"), or state the count as illustrative in the same way line 72 does.

---

## EX-9. `low`. The queued-step inheritance note cites test code as a production `next` call site

THE CLAIM. Sidecar line 264: "the minimal-diff filter placement does NOT cover `src/next.rs:1339`, which calls `w3_problems` directly".

`src/next.rs:1339` is inside `#[cfg(test)] mod tests`, which opens at `src/next.rs:1077-1078`. The line is part of `assert_differential` (`src/next.rs:1327-1342`), a differential TEST helper whose doc comment at `:1324-1326` says it exists so "`next`'s forward `converged` verdict must agree with `w3_problems`' backward 'no shortfall' verdict on the same records". A sweep confirms the only non-test caller of `w3_problems` in the whole crate is `src/workflow.rs:217`: `grep -rn "w3_problems(" src/` returns `src/next.rs:1339` plus twenty-odd `src/workflow.rs` hits, of which only `:217` (the call) and `:437` (the definition) sit above that file's `#[cfg(test)]` at `src/workflow.rs:623`, and `src/next.rs:1339` sits below `src/next.rs:1077`.

The note's CONCLUSION happens to survive (a project filter placed inside `run_checks` would indeed leave `next` unprotected), but for a different reason: `next` derives convergence itself, through `select_active_loop` and the shared accessors documented at `src/next.rs:730`, and never calls `w3_problems` in production at all. Carried into the validation-constraints step as written, this sends that step's implementer to a test looking for a call site to protect.

WHAT SHOULD CHANGE. Restate the inheritance as "`next` derives its forward convergence verdict independently of `w3_problems` (`src/next.rs:730` and `select_active_loop`), so a filter placed inside `run_checks` leaves it unprotected; `src/next.rs:1339` is the differential TEST that pins the two against each other."

---

## EX-10. `low`. Two inc1 resolution inputs are left for the implementer to guess: the ledger default with no plan source, and which of `--source`/`--plan` anchors when both are given

BOTH ARE FORCED GUESSES, and neither has an acceptance check.

- THE LEDGER WITH NO PLAN SOURCE. Inc1 says (line 272) "`default_ledger_path` takes the plan source and resolves the ledger BESIDE it", and line 136 says "the ledger NEEDS NO ROOT DERIVATION AT ALL, because it lives BESIDE the plan, so the source's own directory is the whole rule". Neither `--source` nor `--plan` is required by clap on `status` or `next`, and with both absent there is no directory to be beside. Today `default_ledger_path` (`src/main.rs:1136-1138`) returns `docs/plans/task.ledger.md`, and `derive_task` (`src/next.rs:993-1003`) yields the literal `"task"` in that case, which I confirmed by running `agent-scaffold next` with no arguments (`task: task`, `source: no plan source`). The metrics rule states its answer for this case explicitly (line 158, "With neither a `--source` nor a `--plan` there is nothing to anchor to, so the historical CWD-relative path stands unchanged"); the ledger rule does not, so the implementer chooses silently between keeping `docs/plans/<task>.ledger.md` and collapsing to `<task>.ledger.md` in the current directory.
- SOURCE-VERSUS-PLAN PRECEDENCE. Line 245 says candidate (a) "derives its root from the `--source` OR the `--plan` path" and line 158 says only "the source's parent directory". When both flags are given and point at different trees, which one anchors is unstated. `derive_task` (`src/next.rs:997-999`) establishes a source-then-plan precedent an implementer would probably follow, but `validate --workflow` selects its plan substrate differently (a `--source` drives the check only when TOML-primary, `src/main.rs:957`), so "anchor off the source, check the Markdown plan" is a reachable and unreviewed combination.

WHAT SHOULD CHANGE. Add one sentence each to the inc1 description: the ledger's no-source fallback, and that the anchor follows the same source-then-plan order `derive_task` uses (or the substrate-selection order, if that is meant instead).

---

## What I checked and found nothing wrong with

Recorded because a negative result is worth having, and because a later round should not re-run these.

- EVERY COMMAND THE SIDECAR TELLS AN IMPLEMENTER TO RUN, for repo damage. Only check 7 is destructive (EX-4). The regeneration instruction at line 362, `cargo run -- scaffold --output-dir . --write --force --principles default --instrument`, is byte-identical to `justfile:47`, and the accompanying "Do NOT run `just scaffold-self`" warning correctly identifies `justfile:48` as the `nix fmt` line. The fixture commands all scope their writes to `$SCRATCH`. Check 17 correctly scopes its file creation to the fixture.
- THE INCREMENT ORDERING CLAIMS. Inc1 -> inc2 -> inc3 holds under simulation. After inc1 alone, an explicit `--metrics` at a foreign log genuinely does survive (the anchor changes only the default), so the cost stated at line 284 is consistent with what inc1 is specified to do, and check 5's "THE EXPLICIT-`--metrics` CASE IS STILL OPEN HERE BY DESIGN" matches it. Accepted cost (i) also survives inc2 correctly: the wrong path stays inside the right root, so the canonical guard cannot fire, exactly as line 254 says.
- THE RED CASES. Checks 4, 5, 11, 14b and 15 are all genuinely red today; I ran each. Check 14e's red is red by construction (`#[serde(skip)]` at `src/next.rs:116` and no reason field on `status`'s `Projection` at `src/main.rs:561-568`, both confirmed by running `next --json` and `status --json`). Check 17's control reproduces verbatim at exit 1. Check 10 reproduces (exit 0 plus the stderr note, both with a `--source` and bare).
- THE DECIDED ITEMS. I found no place where the sidecar misrepresents a decision. The `Q-55-refusalscope` third option is described consistently in the step file, in the `[[question]]` body, and in the "Scope" section; the two accepted costs are stated as costs in all three places they appear (lines 254-256, acceptance checks 18 and 19 at lines 330-331, and the scope list at line 375); the exit-0-for-projections rule is stated identically at lines 172, 319, 320, 371; and the nearest-wins case is labelled a judgement rather than a measurement at lines 162 and 378, matching explorer A's own framing at `metrics-path-anchor-to-source.md:152` and `:497`.
- THE TOML ENTRIES. Three `[[step]]` blocks at orders 94, 95, 96 with `blocked_by = []`; increment ids and `risk_class` values match the three sidecars exactly; `Q-55` flips to `decided` with `folded_into` and `receipt` set. `validate --source docs/plans/agent-scaffold.plan.toml --workflow` and `render docs/plans/agent-scaffold.plan.toml --check` both pass on the fold.
- THE TWO BACKLOG SIDECARS. `test-tmpdir-repo-assumption.md` and `status-resume-ignores-json.md` are executable as written; every citation in both resolves; both correctly leave their fix fork open with a recommendation and a re-classification trigger; `status-resume-ignores-json.md:96-97` gets the `conflicts_with`-versus-`requires` distinction right against the in-repo precedents at `src/main.rs:465` and `:557`, which I verified. `status-resume-ignores-json.md:101`'s reasoning for an empty `blocked_by` is sound and its dependency on `workflow-enforcement-tier` inc2's vocabulary is stated in prose as it says. I found nothing to raise in either file.
- THE SUITE AND THE `TMPDIR` CLAIM. `cargo test` gives 386 passed, 0 failed with `TMPDIR` outside any repository, so the count in check 1 and in `test-tmpdir-repo-assumption.md` is current.
