# `workflow-enforcement-tier-inc2`, work review ROUND 2, ISOLATED TRIAGE

ARTIFACT. `git diff main..HEAD` at commit `dd163f2` ("fix: supply a root to the surfaces that read no plan, and pin six unguarded clauses"), reviewed in the worktree `.claude/worktrees/triage-inc2-r2` with `main` at `66e13c5`. The round 1 fix pass alone is `git diff HEAD~1..HEAD`. Specification: `docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md`.

SOURCES TRIAGED. Three round 2 reviewer files in `docs/plans/workflow-enforcement-tier-inc2.reviews/`:

- `workflow-enforcement-tier-inc2-r2-reviewer-adversarial.md` (R2A-1 `high`, R2A-2 `low`).
- `workflow-enforcement-tier-inc2-r2-reviewer-fixverify.md` (FV-1 `medium`, FV-2 `low`, plus a nine-finding closure verification and a regression sweep).
- `workflow-enforcement-tier-inc2-r2-reviewer-claims.md` (R2C-1 `medium`, R2C-2 `medium`, R2C-3 `low`, R2C-4 `low`).

METHOD. Every verdict below carries a command I ran in this worktree and output I observed. Two binaries were built for differential work: `target/release/agent-scaffold` at HEAD, and a second build of the tree exported from `HEAD~1` (`git archive HEAD~1 | tar -x -C <scratch>/prev`, then `cargo build --release` there), so every "unchanged by the fix" claim below is a diff of two real runs. Four mutations were applied in this worktree and reverted one at a time, each with a full suite run. Nothing was taken on any reviewer's word; where a reviewer's own account is wrong I say so and give the measurement. `git status --short` is empty at the end of this file's writing except for this file.

BASELINE. `cargo test`: 415 passed, 0 failed, across 9 binaries (378 + 5 + 1 + 1 + 9 + 3 + 15 + 1 + 2). `cargo clippy --all-targets -- -D warnings`: clean. `render docs/plans/agent-scaffold.plan.toml --check`: up to date. `validate --source docs/plans/agent-scaffold.plan.toml --workflow`: 264 records, invariants hold, exit 0.

---

## 1. THE RULING ON R2A-1 AND FV-1

### 1.1 They are ONE finding. Grouped as G-EMPTYROOT.

R2A-1 and FV-1 describe the same code path (`containment_roots` at `src/main.rs:1332` returning an empty vector because `resume_roots` at `src/main.rs:1445` drops every anchor that `canonical_project_root` at `src/main.rs:1289` cannot canonicalise), reached by the same trigger (an anchor that does not exist), reproduced with the same fixture shape (two disjoint projects, an explicit foreign `--metrics` or `--ledger-fragment`), producing the same three symptoms on the same three surfaces. They dedupe completely. Everything below is one finding with two independent discoveries, which raises confidence in the reproduction rather than the count.

### 1.2 What I ran, and what I observed

I built the adversarial lens's fixture myself from scratch (two top-level sibling projects `alpha` and `beta`, `alpha` holding a Markdown-primary `p.plan.toml` and no `--plan`, `beta` holding its own three-record `workflow.jsonl` and its own ledger with a private `## RESUME STATE` block) and ran eight invocations against the HEAD binary.

CONTROL, the anchor spelled correctly, which is the fix working:

```
task: p
source: no plan source
metrics: unavailable, the round log <R>/beta/docs/metrics/workflow.jsonl is not under the plan's project root <R>/alpha, so its records cannot be paired with this plan

no active review loop (no plan steps found)

the ledger <R>/beta/docs/plans/b.ledger.md is not under the plan's project root <R>/alpha; nothing to resume
exit=0
```

ATTACK, one character changed in the anchor (`p.plan.toml` becomes `q.plan.toml`, a file that does not exist), everything else identical:

```
task: q
source: no plan source
metrics: 3 records

no active review loop (no plan steps found)

RESUME STATE (verbatim from the ledger):
## RESUME STATE

BETA PRIVATE RESUME STATE.
exit=0
```

The machine surface on the same inputs:

```
{
  "task": "q",
  "source": "no plan source",
  "metrics": { "records": 3 },
  "metrics_absent_reason": null,
  "active_loop": null,
  "resume_state": "## RESUME STATE\n\nBETA PRIVATE RESUME STATE.",
  "resume_state_absent_reason": null,
  "no_active_loop_reason": "no-plan-steps"
}
exit=0
```

`status --resume` on the same anchors prints beta's block verbatim at exit 0, while the same command with the correct spelling prints the containment note. `status --json` prints `"metrics": {"records": 3}` beside `"metrics_absent_reason": null`. A nonexistent `--plan` instead of a nonexistent `--source` reproduces identically, and so does the no-anchor-at-all case. Reproduced in full, on all three surfaces, at exit 0.

### 1.3 Dispute one: is the behaviour identical before and after the fix pass? YES. The fix-verification lens is right and the adversarial lens's causal framing is wrong.

I ran the identical script against the `HEAD~1` binary and diffed the two transcripts. The diff is EXACTLY ONE BLOCK, and it is the CONTROL:

```
 === CONTROL: anchor EXISTS (p.plan.toml). next ===
 task: p
 source: no plan source
-metrics: 3 records
+metrics: unavailable, the round log <R>/beta/docs/metrics/workflow.jsonl is not under the plan's project root <R>/alpha, ...
-RESUME STATE (verbatim from the ledger):
-## RESUME STATE
-
-BETA PRIVATE RESUME STATE.
+the ledger <R>/beta/docs/plans/b.ledger.md is not under the plan's project root <R>/alpha; nothing to resume
```

Every ATTACK block is byte-identical between `HEAD~1` and `HEAD`. The mechanism is visible in the fix pass's own diff: before it, `run_status` and `run_next` held `checked_root: Option<PathBuf>` and used `.as_ref().filter(...)`, which is `None` when the root is `None`; after it they hold `checked_roots: Vec<PathBuf>` and use `.iter().find(...)`, which is `None` when the vector is empty. `Option::None` and an empty `Vec` behave the same under both spellings, so the empty case did not change.

RULING ON CAUSE. FV-1's "THE FIX DID NOT CAUSE IT, and I say so plainly" is correct and measured. R2A-1's framing that the empty quantifier "silently restores the exact defect the fix exists to close" is wrong on the verb: nothing was restored, because this configuration was never closed. What R2A-1 gets right, and what matters more than the verb, is that `containment_roots` is now the SINGLE PLACE where the hole lives, and that the function's own doc comment (`src/main.rs:1317-1331`) asserts it does not exist: "Where NO plan is read, `checked_plan_root` has nothing to derive from, so the rule SUPPLIES a root from the anchors instead" is FALSE for an anchor that does not canonicalise, and the very next clause describes the exact leak I reproduced as something this function prevents.

### 1.4 Dispute two: does `run_resume` have the same hole independently? YES, and both lenses actually agree on this.

`run_resume` (`src/main.rs:1467`) calls `resume_roots(&args.source, &args.plan)` directly at `:1475` and is not touched by the fix pass at all (`git diff HEAD~1..HEAD -- src/main.rs` shows no change inside it). Measured above: `status --resume --source <alpha>/docs/plans/q.plan.toml --ledger-fragment <beta>/.../b.ledger.md` prints beta's block at exit 0, and the same command with `p.plan.toml` prints the note. So all three surfaces are vacuous together. The brief framed this as an adversarial claim in dispute; it is not in dispute, since FV-1 states the same thing ("`status --resume` leaks identically") and uses it as its ground for `medium`. Both lenses measured the same fact and drew opposite severity conclusions from it.

A CONSEQUENCE FOR THE FIX'S LOCATION, which follows from that fact: this is a gap in ONE shared derivation, not an inconsistency between surfaces, so the fix belongs in `canonical_project_root` or in `resume_roots`, not in the three consumers. That is the opposite of round 1's ADV-1, whose fix belonged in the two consumers.

### 1.5 The discriminating control for the in-root bound, run by me

The brief requires this before any in-root attribution is accepted or rejected. I ran ONE fixture in TWO arrangements, changing only where `beta` sits, with a control invocation in each that differs from the leaking one in one character of the `--source`:

```
===== DISJOINT (alpha and beta are top-level siblings)
ATTACK  (--source .../q.plan.toml, missing) -> beta's block echoed verbatim, exit 0
CONTROL (--source .../p.plan.toml, exists)  -> "the ledger ... is not under the plan's project root <R>/disjoint/alpha; nothing to resume", exit 0

===== NESTED (beta moved to alpha/vendor/beta)
ATTACK  (--source .../q.plan.toml, missing) -> beta's block echoed verbatim, exit 0
CONTROL (--source .../p.plan.toml, exists)  -> beta's block echoed verbatim, exit 0
```

THE DISJOINT ROW SETTLES IT. In a layout with no containment relationship in either direction, correcting the anchor spelling and changing nothing else turns the leak into a refusal. The variable is whether the anchor canonicalises, not whether the artifact is nested. The nested row is the in-root bound behaving exactly as recorded (both cells leak), and it is correctly not filed. Per the brief's own rule, the disjoint case reproduces, so THIS IS NOT THE IN-ROOT BOUND and it is filed.

### 1.6 Severity: `high`. FV-1 is re-severitised UP from `medium`; R2A-1's `high` is upheld.

I take the round 1 triage's own standard as binding, because applying two standards within one step makes the severities incomparable: it reserved `high` for "a defect a user can hit today" and moved EVI-1 down to `medium` on the ground that "THE SHIPPED CODE IS CORRECT" there. This defect is live on the shipped code.

The decisive test is calibration against round 1's ADV-1, which was rated `high` and upheld. Compare them fact by fact, all of which I verified myself rather than reading across:

| | ADV-1 (round 1, `high`) | G-EMPTYROOT (this finding) |
| --- | --- | --- |
| Payload | Another project's `## RESUME STATE` echoed verbatim by `next`, plus a foreign record count | Identical, on `next`, `status` and `status --resume` |
| Machine surface | `"resume_state_absent_reason": null` positively asserting the block is this plan's | Identical, plus `"metrics_absent_reason": null` |
| Exit code | 0 | 0 |
| Fabricated `next:` instruction | No (no plan read, so no steps, so no `ACTIVE LOOP`) | No, same reason, verified: `"active_loop": null`, `"no_active_loop_reason": "no-plan-steps"` |
| Trigger | Markdown-primary `--source`, no `--plan`, one explicit flag | Anchor that does not exist, one explicit flag |
| Suite visibility | None | None |
| Documentation falsified | `README.md:236` sentence 1 | `README.md:236` sentences 1 AND 2, the second authored by the fix pass |
| A surface that tells the truth | Yes, `status --resume` refused | No, all three leak |

FV-1's ground for holding at `medium` is that there is no surface-to-surface contradiction, which the round 1 triage called "the sharp part" of ADV-1. I do not accept that as a downgrade, for three reasons.

- THE CONTRADICTION WAS ALSO A MITIGATION. In ADV-1 an operator who ran `status --resume` got the correct refusal, so one surface told the truth and the disagreement was itself the signal. Here no surface does. Treating the loss of the honest surface as a reduction in severity inverts the harm.
- THE AGGRAVATORS THE ROUND 1 TRIAGE NAMED ALL SURVIVE. It named three: the machine surface asserting the opposite of the truth, documentation falsified in the same commit, and the suite unable to see it. All three hold here, and the second holds MORE strongly, because the fix pass's own new sentence is among the falsified text (section 6).
- THE "NO FABRICATED INSTRUCTION" MITIGATION IS NOT A DIFFERENCE. It applied to ADV-1 identically, for the identical structural reason, and did not stop that finding being `high`.

Against `high`, FV-1 offers that `canonical_project_root`'s doc comment records the `None`-on-a-missing-plan behaviour deliberately. It does (`src/main.rs:1289` and the comment above it), but a documented derivation whose consequence is undocumented content injection is a reason the defect is understandable, not a reason it is less harmful.

RULING: `high`. NO `high` WAS DISMISSED OR DOWNGRADED ANYWHERE IN THIS ROUND. The only severity movement on this finding is upward.

### 1.7 Scope: IN SCOPE for inc2 as a defect, with the REMEDY SHAPE routed as a human decision.

I applied the boundary in both directions, as the brief requires.

WHY IT IS NOT PRE-EXISTING IN ANY SENSE THAT REMOVES IT FROM THIS INCREMENT. `git show main:src/main.rs | grep -n "fn is_outside_root\|fn checked_plan_root\|fn resume_roots\|fn canonical_project_root\|fn resolve_for_containment\|fn unpairable"` returns nothing: the ENTIRE containment mechanism is new in `main..HEAD`. "Pre-existing" here can only mean "not introduced by the round 1 fix pass", which is a statement about a commit inside the increment, not about the increment. FV-1 reaches the same conclusion and states it plainly; I confirm it by measurement.

WHY THE SPECIFICATION'S BOUNDARIES DO NOT COVER IT.

- The four accepted costs (line 251) are (i) a bare filename from inside `docs/plans`, (ii) a symlink on the plan's or the log's path, (iii) a same-project `--plan` outside any `docs/plans`, and (iv) `status --resume` on that same pair. None is an anchor that does not exist.
- "What this step does not fix" (line 265) names the IN-ROOT BOUND (excluded by the discriminating control in section 1.5), PROJECT IDENTITY (a data-model question; this is a path question), and at line 271 the shared root cause of costs (iii) and (iv), which is `project_root_of_source`'s fallback to the plan's own parent. That is a DIFFERENT function and a different failure: `project_root_of_source` runs only after `fs::canonicalize` has SUCCEEDED, and here it never runs at all. Line 271 does not reach this.
- Inc2's own scope statement (line 280) sets the review question as "does the predicate identify an unsafe pairing correctly, and does each consumer give the right answer to it, on both surfaces". Three consumers give the wrong answer on both surfaces.
- Line 303 requires of this increment specifically that "passing the increment's own tests is explicitly NOT sufficient evidence here; the review has to include adversarial construction". This finding is the product of exactly that mandated construction, found independently by two lenses.

WHY IT IS NOT A SPECIFICATION DEFECT I SHOULD BOUNCE. `Q-55-resumepairing` (line 182) decides the no-plan-read rooting as "a `--source` and a `--plan` that BOTH EXIST must resolve to the SAME root ... and with one alone the anchor is the root". The decision conditions on existence and is silent on a nonexistent anchor, so the specification does not sanction the leak and does not forbid a remedy. The code had to answer the question one way or another and answered it by leaking; that is a code decision made silently, not a specification defect the implementer could not proceed past.

WHERE I STOP. The remedy is not mechanical and I do not prescribe it as one. Every candidate changes behaviour that no decision covers, and one of them touches the implementation of a human-decided policy (`Q-55-resumepairing`). The FINDING must be closed or explicitly deferred by the human; the SHAPE is routed in section 8, item 1, with options and a recommendation. Do not let an implementer pick.

---

## 2. VERDICT TABLE

| Raw id | Verdict | Reviewer severity | MY severity | Dedup group | One-line prescription |
| --- | --- | --- | --- | --- | --- |
| R2A-1 | VALID | `high` | `high` (upheld) | G-EMPTYROOT | Give a root to an anchor that does not canonicalise, or refuse when one was supplied and no root could be derived; `src/main.rs:1289` (`canonical_project_root`) or `src/main.rs:1445` (`resume_roots`); AUTHORED LOGIC, and the shape is a human decision (section 8, item 1). |
| FV-1 | VALID, DUPLICATE of R2A-1 | `medium` | `high` (UP) | G-EMPTYROOT | Same fix. Counts once. |
| R2A-2 | INVALID (absorbed by specification line 271) | `low` | n/a | none | No fix. Carry the measurement into the queued `project_root_of_source` work; see section 8, item 2 for the confirmation the human may want. |
| R2C-1 | VALID | `medium` | `low` (DOWN) | G-NOPLANPROSE | Qualify the paragraph's opening clause so it does not cover `validate --workflow` in the no-plan-read case; `README.md:236` and `CHANGELOG.md:23`; ONE-LINE CHANGE on each, one clause. |
| FV-2 | VALID | `low` | `low` (upheld) | G-NOPLANPROSE | Stop the added sentence enumerating one configuration as though it were the trigger, and stop attributing "every one of them" to `status` and `next`; same two lines; ONE-LINE CHANGE on each. |
| R2C-2 | INVALID for inc2 (pre-existing text, out of scope) | `medium` | n/a | none | No fix owed here. The underlying claim IS false and is confirmed; route to the documentation-currency step, or take the one-line freebie by explicit choice (section 8, item 3). |
| R2C-3 | VALID | `low` | `low` (upheld) | G-TESTNAME | Rename to `a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted`; `tests/unsafe_pairings_are_refused_and_omitted.rs:539`; ONE-LINE CHANGE. |
| R2C-4 | VALID | `low` | `low` (upheld) | G-TESTNAME | Reword the last doc-comment line so it does not claim all three surfaces answer one question; `tests/unsafe_pairings_are_refused_and_omitted.rs:571`; ONE-LINE CHANGE. |

DEDUPLICATION NOTES. R2A-1 and FV-1 collapse completely into G-EMPTYROOT. R2C-1 and FV-2 do NOT collapse: they attack different sentences of the same paragraph (R2C-1 the opening and closing sentences, FV-2 the middle sentence the fix pass added), and both must be answered for the paragraph to be true, so they are grouped for editing convenience and counted separately. R2C-3 and R2C-4 are grouped as test-text accuracy but are separate lines with separate corrections.

---

## 3. PER FINDING

### 3.1 G-EMPTYROOT (R2A-1 and FV-1): VALID, `high`

Covered in full in section 1. Summary of the record: reproduced from scratch on my own fixture, eight invocations, all three surfaces; differential against the `HEAD~1` binary shows the empty-root cells byte-identical and only the control cell changed; the discriminating control run in both the disjoint and nested arrangements; `run_resume`'s independent hole confirmed by code path (`src/main.rs:1475`) and by run.

`file:line` FOR THE FIX. The cause is at `src/main.rs:1289` (`canonical_project_root`, which is `fs::canonicalize(plan).ok().map(...)`) and it reaches the consumers through `src/main.rs:1445` (`resume_roots`, whose `filter_map` silently drops a `None`) and `src/main.rs:1332` (`containment_roots`). Consumed at `src/main.rs:1150` (`run_status`), `:1551` and `:1571` (`run_next`), and `:1475` (`run_resume`). AUTHORED LOGIC, small, in ONE place; see section 8, item 1 for the shape.

TESTS OWED WITH IT. The increment's own new test `a_surface_that_reads_no_plan_is_supplied_a_root` (`tests/unsafe_pairings_are_refused_and_omitted.rs:578`) only ever writes an anchor that EXISTS (`:586` writes the file it then passes at `:587`), so nothing in the suite pins the empty-vector case in either direction. Whatever shape is chosen owes at least: `next` and `status --resume` with a nonexistent `--source` and an out-of-root explicit artifact, asserting the chosen answer on both the human and the `--json` surface; and a no-regression assertion that the neither-anchor case still keeps its documented current-directory-relative behaviour.

CORRECTIONS OWED TO PROSE WITH THE SAME FIX, so the code and its account land together: `src/main.rs:1317-1331` (`containment_roots`'s doc comment, which currently asserts the leak cannot happen), and the added sentence at `README.md:236` and `CHANGELOG.md:23` (section 6).

### 3.2 R2A-2: INVALID, absorbed by specification line 271

WHAT I RAN. The adversarial lens's second script, rebuilt from scratch: ONE project whose Markdown-primary plan source is at `proj/notes/n.plan.toml`, with its own `proj/docs/metrics/workflow.jsonl` and its own `proj/docs/plans/n.ledger.md`, both named EXPLICITLY, invoked from `proj` with no `--plan`. Run against both binaries and diffed.

WHAT I OBSERVED. It reproduces exactly as filed. At `HEAD~1`: `metrics: 1 records`, and the ledger echoed as `THIS PROJECT'S OWN RESUME STATE.`. At `HEAD`:

```
metrics: unavailable, the round log docs/metrics/workflow.jsonl is not under the plan's project root <R>/proj/notes, so its records cannot be paired with this plan
the ledger docs/plans/n.ledger.md is not under the plan's project root <R>/proj/notes; nothing to resume
```

All exit 0. I also ran the two comparison cases the lens describes and confirmed both: a source at the project ROOT with no `docs/plans` anywhere still reads its own log at HEAD (`metrics: 1 records`), and the cost (iii) shape produces the same message with the same `notes` root.

REASONING. The derived root is `<R>/proj/notes`, which is `project_root_of_source`'s fallback to the plan's own parent directory when no `docs/plans`-shaped ancestor exists. That is precisely and only the root cause the specification names at line 271: "COSTS (iii) AND (iv) SHARE ONE ROOT CAUSE, `src/main.rs:project_root_of_source`'s fallback to the plan's own parent, and treating it ONCE IS QUEUED TO THE SAME STEP RATHER THAN ACCUMULATING A FRESH ACCEPTED COST ON EVERY NEW SURFACE." The lens is right that the POPULATION is new (cost (iii) is a `--plan` outside `docs/plans`, this is a `--source` outside one with no `--plan`), and right to check that before filing. But line 271's clause about new surfaces is written for exactly this situation, and it decides it: a new surface of the queued root cause is not a new accepted cost and is not a fresh finding. Three further facts hold the ruling: the note NAMES the derived root, so the user is told what happened rather than left guessing; the behaviour follows from the README's own text at `:235` ("the source's own directory when it has no such ancestor") plus the added sentence at `:236`; and the documented layout it might be confused with is unaffected, which I measured.

PRESCRIPTION: none. Record the measurement so the queued step inherits a second surface of the same cause. If the human disagrees with my reading of line 271, section 8 item 2 states the alternative.

### 3.3 R2C-1: VALID, `low`, RE-SEVERITISED DOWN from `medium`

WHAT I RAN. The paragraph's opening claim tested against `validate --workflow` in the no-plan-read configuration, three ways, exit codes captured directly rather than through a `|| true`:

```
validate --source <alpha md-primary p.plan.toml> --metrics <alpha's OWN log>  --workflow -> exit 1
validate --source <alpha md-primary p.plan.toml> --metrics <beta's FOREIGN log> --workflow -> exit 1
validate --source <alpha NONEXISTENT q.plan.toml> --metrics <beta's FOREIGN log> --workflow -> exit 1
```

WHAT I OBSERVED. All three print `--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan` and nothing else on the first two; the third adds the `no source plan at ...` note. The message and exit code are byte-identical whether the log is in-root or out-of-root, so the containment predicate demonstrably never runs. Confirmed against the code: `run_validate` uses `checked_plan_root` directly at `src/main.rs:977` and is not routed through `containment_roots`, so `unsafe_pairing` at `:981` is vacuously false and the pre-existing `(None, None, _)` arm fires for an unrelated reason.

REASONING. The finding is real. `README.md:236`'s first sentence says "Every one of these commands checks that the log (and, for the ledger readers, the ledger) it is about to read lives under the project root of the plan it is about to read", the added second sentence supplies a fallback for exactly three of the four commands and pointedly not for `validate --workflow`, and the third sentence then says "Where it does not, `validate --workflow` refuses as above", inviting the reader to carry the containment story into a case where it does not apply.

A PROVENANCE CORRECTION THAT DECIDES THE SCOPE, and it goes AGAINST the reviewer's own account. The claims lens wrote that this sentence is "pre-existing on `main` before round 1's ADV-1 fix even landed" and treated it as out of scope on its own. That is FALSE. `git show main:README.md | grep -c "lives under the project root"` returns 0, and `git show main:README.md | grep -c "Anchoring changes where the DEFAULT log resolves"` returns 0: the ENTIRE paragraph is new in `main..HEAD`. The sentence is the increment's own prose, authored by an earlier commit of this increment rather than by the fix pass, and it is squarely in scope. The lens filed a valid finding while giving the wrong reason for why it might not have been.

WHY `low` AND NOT `medium`, stated as a ground. No behaviour is wrong; `validate --workflow` does exit non-zero in this configuration, so a reader who acts on the misreading is not led into an unsafe outcome, only into an inaccurate mental model of WHY the tool refused. Round 1 filed TRI-1, a false clause in the increment's own doc comment, at `low`, and FV-2 is filed at `low` for the same class in the same paragraph. Holding R2C-1 at `medium` would make three near-identical documentation defects carry two different severities inside one step.

PRESCRIBED FIX. `README.md:236` and `CHANGELOG.md:23`, ONE CLAUSE on each, no authored logic. Narrow the opening quantifier to the commands that read a plan, for example "Every one of these commands that reads a plan checks that the log ... lives under the project root of THAT plan", so the added sentence carries the no-plan-read case for the three commands that have a fallback and `validate --workflow` is not implicitly claimed to have one. The two files must change together, since the sentence is duplicated.

### 3.4 FV-2: VALID, `low`, UPHELD

WHAT I RAN. Both configurations the lens says the added sentence omits, on the `--json` surface where the answer is unambiguous:

```
status --source <alpha md-primary> --plan <alpha/docs/plans/NOSUCH.md> --metrics <beta's log> --json
  -> {"plan": null, "metrics": null, "metrics_absent_reason": "log-not-this-project"}
status --source <alpha/docs/plans/garbage.plan.toml, unparseable> --metrics <beta's log> --json
  -> note: ... did not parse as a `<task>.plan.toml`; projecting from --plan
     {"plan": null, "metrics": null, "metrics_absent_reason": "log-not-this-project"}
```

WHAT I OBSERVED. Both reach the containment fallback and are correctly refused, and neither is the configuration the sentence names ("a Markdown-primary `--source` and no `--plan`"). The second claim is structural and I verified it by reading rather than by construction: `containment_roots` reaches `resume_roots` only when `checked_plan_root` is `None`; with `toml_primary` true the source was read so its root is `Some`, and with it false a `--plan` that yields a root prevents the fallback while a `--plan` that yields none is dropped by `resume_roots`'s `filter_map` too. So `status` and `next` can hold at most ONE root and "every one of them" is always exactly one for them.

REASONING. Neither half is false as literally scoped; the sentence's own clauses are true, which both lenses verified and I agree with. What is wrong is that a reader checking the documented rule against a run finds behaviour the rule does not name, and a rule attributed jointly to three surfaces of which two can never exercise it. That is the TRI-1 class at the TRI-1 severity.

PRESCRIBED FIX. `README.md:236` and `CHANGELOG.md:23`, one clause on each, no authored logic. Name the trigger by its condition rather than by one example ("where no plan is read, which is always so for `status --resume` and is so for `status` and `next` whenever neither a TOML-primary `--source` nor a readable `--plan` resolves"), and note that `status --resume` is the surface that can hold two roots. This edit and R2C-1's edit land in the same sentence group and should be made together.

### 3.5 R2C-2: INVALID for this increment, out of scope. The underlying claim IS false and is confirmed.

WHAT I RAN. First with a hand-written TOML fixture, which did not parse and gave a misleading `"plan": null`; I discarded that run and used a real TOML-primary plan instead:

```
agent-scaffold status --source <worktree>/docs/plans/agent-scaffold.plan.toml --json
```

with no `--plan` flag anywhere.

WHAT I OBSERVED. `"plan": { "steps": [ ... ] }`, fully populated. So `Projection.plan` IS `Some` with no `--plan` given, and the first sentence of its doc comment at `src/main.rs:570` ("The plan projection, present only when a readable `--plan` was given") is FALSE. The claim-accuracy lens is right on the fact.

REASONING FOR RULING IT OUT OF SCOPE, and I apply the boundary carefully because the brief asks me to be rigorous in both directions.

- THE FALSE SENTENCE IS GENUINELY PRE-EXISTING, verified: `git show main:src/main.rs` carries it verbatim, and unlike `README.md:236` it is not the increment's own text. The increment added a SECOND sentence beside it.
- THE ADDED SENTENCE IS TRUE ON ITS OWN TERMS AND DOES NOT REST ON THE FALSE ONE. "It carries no reason field: there is exactly one cause, so a reason there would inform nobody" is about the cause of ABSENCE, which is the only thing a `*_absent_reason` field could ever report, and absence does reduce to one cause ("no readable plan source was given"). It is a faithful transcription of specification line 237, whose own parenthetical says SOURCE and not `--plan`. The false first sentence is about PRESENCE. The reviewer's framing that the new sentence is "presented as a conclusion resting directly on the (false) first sentence's premise" is a reading of the adjacency, not a demonstrated dependency, and the two sentences are about opposite arms of the same `Option`.
- THE SPECIFICATION LISTED THE DOC COMMENTS THIS INCREMENT OWED (line 197 to line 204: four falsified claims, of which `status`'s `Projection` struct comment is one). The implementer changed that struct comment and I verified it now names all three causes. The FIELD comment's presence clause was not among them.
- THE PROJECT HAS A STANDING RULE that documentation currency work is scheduled at the step close after inc3 rather than folded into each increment, and the specification says so directly at line 293 ("this project already carries a step about documentation currency"). Folding a pre-existing false comment into inc2 because the increment happened to edit an adjacent line would widen a settled increment on a technicality, which is the failure mode the brief names.

PRESCRIPTION: none owed in inc2. The correction, if taken, is one line at `src/main.rs:570`, for example "The plan projection, present when a TOML-primary `--source` parses or a readable `--plan` was given." Section 8 item 3 puts the choice to the human rather than deciding it for them, since the cost is a single line and the argument for taking it now is not silly.

### 3.6 R2C-3: VALID, `low`, UPHELD

WHAT I RAN. Read `tests/unsafe_pairings_are_refused_and_omitted.rs:533-566`, and checked the file's naming convention with `grep -n "metrics: unavailable\|Some(0)\|Some(1)"` over the whole file to see which tests assert which manifestation, and `git diff HEAD~1..HEAD -- tests/` to establish provenance.

WHAT I OBSERVED. The test asserts exit 1 plus the containment refusal at `:553-555` (its own comment calls this "the LOUD manifestation") and then, on the SAME fixture, exit 0 plus `metrics: unavailable,` on both `status` and `next` at `:558-563` ("the QUIET one"). The name covers only the first. The convention argument holds: the two other tests in the file that assert both manifestations are named neutrally (`accepted_cost_two_the_symlinked_layouts_are_pinned` at `:921`, `accepted_costs_three_and_four_are_pinned` at `:978`), and `an_explicit_metrics_outside_the_plans_root_is_refused` at `:165`, which is named `_is_refused`, asserts refusals only. This one test is the sole place where a `_is_refused` name sits over a body that also pins the omission, in a file whose own name distinguishes the two verbs. It is new in the fix pass (`git diff HEAD~1..HEAD -- tests/` adds exactly two `#[test]` functions, this and `a_surface_that_reads_no_plan_is_supplied_a_root`), so it is in scope.

PRESCRIBED FIX. `tests/unsafe_pairings_are_refused_and_omitted.rs:539`, rename to `a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted`. ONE-LINE CHANGE, no assertion changes.

### 3.7 R2C-4: VALID, `low`, UPHELD

WHAT I RAN. Read `tests/unsafe_pairings_are_refused_and_omitted.rs:568-643` and `src/main.rs:568-577`.

WHAT I OBSERVED. The doc comment's last line at `:571` reads "... exactly as it supplies one to `status --resume`, so all three give the same answer on identical inputs." The body runs `status --resume` against the ledger (`:592`), `next` against the ledger (`:602`) and `next --json` (`:612`), then `next` and `status` against the LOG (`:622-640`). `status` is never run against a ledger, and cannot be: `Projection` (`src/main.rs:569-577`) has no ledger-related field. So no single input elicits a comparable answer from all three. The sentence conflates "the same root-supply policy now reaches all three surfaces", which is true and is what the test shows, with "all three answer the same question on the same run", which is not tested and is not applicable.

PRESCRIBED FIX. `tests/unsafe_pairings_are_refused_and_omitted.rs:571`, reword to something like "... so the two ledger readers agree with each other and the two log readers agree with each other on identical anchors". ONE-LINE CHANGE, internal test prose, cosmetic.

---

## 4. SPOT-CHECK OF THE FIX-VERIFICATION LENS'S POSITIVE RESULTS

The brief calls a false "all closed" the most expensive error available this round, so I re-ran the central regression mutation, two of the nine closures, and the guard on the fix itself, and I checked the two structural claims that the matrix result rests on.

### 4.1 The central regression mutation, M2: CONFIRMED

Applied at `src/main.rs:1313`, replacing the `toml_primary` branch with `source.as_ref().or(plan.as_ref())` so `checked_plan_root` roots on the ANCHOR, which is the defect `Q-55-endproperty` exists to prevent. Full suite:

```
test result: FAILED. 12 passed; 3 failed
failures:
    a_divergent_source_and_plan_pairing_is_refused
    accepted_costs_three_and_four_are_pinned
    the_resume_reasons_separate_and_cover_the_default_ledger
```

Exactly the three tests the lens names, and the same three round 1 recorded. STILL CAUGHT. Reverted, tree clean.

### 4.2 Closure EVI-2, mutation M26: CONFIRMED

Applied at `src/main.rs:995`, replacing `} else {` with a closing brace and a bare block so the four-arm match runs BESIDE the refusal rather than instead of it. This is the mutation round 1 recorded as SURVIVING with a green 413-test suite.

```
test result: FAILED. 14 passed; 1 failed
failures:
    an_explicit_metrics_outside_the_plans_root_is_refused
```

RED, on the prescribed assertion, on the prescribed test. VERIFIED CLOSED. Reverted.

### 4.3 Closure EVI-5, mutation M28: CONFIRMED

Applied at `src/main.rs:1606`, passing `resume_state_absent_note: None` into the projection while still computing the reason.

```
test result: FAILED. 13 passed; 2 failed
failures:
    a_surface_that_reads_no_plan_is_supplied_a_root
    the_resume_reasons_separate_and_cover_the_default_ledger
```

Exactly the two tests the lens names. VERIFIED CLOSED. Reverted.

### 4.4 Closure ADV-1, by fixture: CONFIRMED

The CONTROL block of my own section 1.2 run IS the ADV-1 closure, and the differential against the `HEAD~1` binary shows it moving from `metrics: 3 records` plus beta's block echoed verbatim, to the containment note on both halves. Both surfaces, human and `--json`. VERIFIED CLOSED, with the important qualification that it is closed only where the anchor canonicalises, which is G-EMPTYROOT.

### 4.5 Closure TRI-1: CONFIRMED

`grep -rn "already holds the paths" src/` returns nothing. The false clause is gone from the tree rather than moved.

### 4.6 The guard on the fix itself: CONFIRMED

Reverting `containment_roots` to `map_or_else(Vec::new, |root| vec![root])` (the pre-fix behaviour) turns the suite RED on `a_surface_that_reads_no_plan_is_supplied_a_root` and nothing else, exactly as the lens claims. So the fix is pinned, by one test, in one place. Reverted.

### 4.7 The test diff is purely additive: CONFIRMED

`git diff --numstat HEAD~1..HEAD -- tests/` is `214 0`. Zero deletions, so no existing assertion was edited, relaxed or removed.

### 4.8 ONE POSITIVE RESULT DOES NOT SURVIVE, and it matters

The lens's section 2.2 states, as "the load-bearing negative" of its 216-invocation matrix: "NOTHING THAT WORKED BEFORE NOW REFUSES. The two newly-rooted configurations with the project's OWN explicit artifacts do not appear in the diff." THAT GENERALISATION IS FALSE, and the counterexample is the OTHER lens's R2A-2, which I reproduced in section 3.2 on this same commit: one project, its own explicit `--metrics` and `--ledger-fragment`, read at `HEAD~1` and omitted at `HEAD`.

The cause is a gap in the matrix, not an error in its arithmetic. The twelve anchor configurations vary WHICH anchors are supplied and whether they exist; none varies WHERE the source sits relative to `docs/plans`. Every cell therefore has the project's own artifacts under the derived root by construction, which is why the negative held inside the matrix and fails outside it. The lens's "exactly two cells changed" is a true statement about its matrix; its stated conclusion is not a true statement about the fix.

WHAT I CONCLUDE FROM THE SPOT-CHECK. The nine closures and the thirteen re-run mutations are sound on everything I could check (six independent measurements, all matching, including the two the round 1 triage recorded as surviving). The regression sweep's COVERAGE is narrower than its conclusion, and a reader should carry "no regression was found in these 216 invocations" rather than "nothing that worked before now refuses". That does not change any verdict in this round, since the one behaviour change it missed is R2A-2, which I rule absorbed for an unrelated reason.

---

## 5. WHAT I BELIEVE ALL THREE LENSES MISSED

- THE FIX PASS'S OWN NEW SENTENCE IS FALSIFIED BY G-EMPTYROOT, not merely incomplete. FV-1 lists `README.md:236`'s FIRST sentence among the text its finding falsifies, and FV-2 charges the ADDED sentence only with an incomplete enumeration. Neither says that the added sentence is FALSE in FV-1's own configuration. It is: "the roots come from the `--source` and `--plan` themselves and the artifact must be under every one of them" tells a reader that `--source <alpha>/docs/plans/q.plan.toml` yields the root `<alpha>` and that beta's ledger, not being under it, is omitted. Measured, beta's ledger is printed verbatim. The two round 2 findings that the two lenses filed separately are, on this point, the same defect seen in prose and in code, and the prose correction cannot be written until the code question in section 8 item 1 is decided.
- `containment_roots`'S DOC COMMENT ASSERTS THE ABSENCE OF THE DEFECT. `src/main.rs:1317-1331` says "Where NO plan is read ... the rule SUPPLIES a root from the anchors instead" and then describes the leak ("both filters go vacuous and an explicit `--metrics` or `--ledger-fragment` naming another project is read with nothing to reject it") as the thing this function prevents. The claim-accuracy lens swept this exact comment and reported "every clause verified against the function body and its two call sites". That sweep is wrong on the clause that matters most, and the miss is instructive: verifying a comment against the body it sits on cannot catch a claim that is false only on an input the verifier did not supply.
- THE TWO LENSES' RESULTS CONTRADICT EACH OTHER AND NOBODY RECONCILED THEM. Section 4.8: the fix-verification lens's headline negative and the adversarial lens's R2A-2 cannot both be true, and the adversarial one is right. A round that files two reviewer reports without a cross-check would have carried both into the record.
- THE LEAKED OUTPUT MISLABELS ITSELF. In the G-EMPTYROOT runs the human surface prints `task: q` and the JSON prints `"task": "q"`, a task name derived from the anchor that does not exist, above data belonging to a different project entirely. A consumer correlating on `task` gets a coherent-looking record that is wrong in both halves. Minor beside the content injection, but it removes the last cue a careful reader might have caught.
- `source: no plan source` DOES NOT DISCRIMINATE. The line printed in the leaking configuration is the same line the supported Markdown-primary configuration prints, so nothing in the output distinguishes "no plan was read because none was asked for" from "no plan was read because the one you named is not there". The adversarial lens says this in passing inside its severity argument; it deserves to be a named property of any remedy, since a remedy that only omits the artifact still leaves the operator unaware that their anchor was a typo.

---

## 6. ROUND OUTCOME

**NEW VALID FINDINGS. NOT CLEAN.**

VALID COUNT AFTER DEDUPLICATION: **5**, in 3 fix groups.

- `critical`: 0.
- `high`: 1 (G-EMPTYROOT, from R2A-1 and FV-1 deduplicated).
- `medium`: 0.
- `low`: 4 (R2C-1, FV-2, R2C-3, R2C-4).

INVALID: 2 (R2A-2, R2C-2), on the grounds recorded in sections 3.2 and 3.5.

SEVERITY MOVEMENT. One upgrade: FV-1 `medium` to `high`, merged into R2A-1's upheld `high`. Two downgrades, both from `medium`: R2C-1 to `low`, and R2C-2 to INVALID on scope. **NO `high` OR `critical` WAS DISMISSED OR DOWNGRADED.** The only movement touching the `high` band is upward.

WORK SHAPE. One piece of authored logic in one function, whose shape is a human decision; two documentation clauses duplicated across `README.md` and `CHANGELOG.md`; two one-line test-text corrections. The round cannot converge on this artifact: a live `high` is open.

---

## 7. ITEMS NEEDING A HUMAN DECISION

### Item 1 (REQUIRED before the fix pass): the remedy shape for G-EMPTYROOT

The finding is valid and `high` and must be closed or explicitly deferred. Every remedy changes behaviour no existing decision covers, and one of them touches the implementation of `Q-55-resumepairing`, so the implementer must not choose.

- OPTION A (RECOMMENDED): PARTIALLY RESOLVE THE ANCHOR, reusing the mechanism the code already has. `resolve_for_containment` (`src/main.rs:1350`) already absolutises a path, canonicalises its longest existing ancestor, and re-appends the rest; applying the same treatment to an ANCHOR in `canonical_project_root` (`src/main.rs:1289`) gives `<alpha>/docs/plans/q.plan.toml` the root `<alpha>` and closes the leak with no new concept and no new vocabulary. Trade: it changes what "a root could be derived" means, so a nonexistent anchor now yields containment where it yielded none; the only invocations whose output changes are ones that are currently leaking or currently reading an artifact outside the anchor's project. It does not touch the decided neither-anchor case, which still yields an empty vector and keeps the current-directory-relative paths `README.md:235` documents. Judged against Safe on existing projects it is the narrowest of the three; judged against One source of truth it reuses the existing resolution rather than adding a second; judged against Fail loudly it is the weaker option, since the operator is still not told their anchor was a typo.
- OPTION B: TREAT "AN ANCHOR WAS SUPPLIED AND NO ROOT COULD BE DERIVED" AS UNPAIRABLE, omitting the artifact with a reason on the projections. Trade: strictly safer and it satisfies Fail loudly, since the omission note can say the anchor does not exist; but it is a wider behaviour change, because it also omits an artifact that legitimately belongs to the anchor's own directory (a run against a plan file that has not been written yet loses its own log), and it needs a new reason variant or a stretched meaning for `log-not-this-project`, which is a `Q-55-jsonreason` vocabulary change.
- OPTION C: ACCEPT AND RECORD IT as a fifth cost or a residual, and defer the close. Trade: it leaves a `high` content-injection path live in a shipped increment whose whole purpose is to close that class, and specification line 303 says passing the increment's own tests is not sufficient evidence for this increment. I do not recommend it, but it is a legitimate answer if the human judges the trigger population too small to pay for now, in which case it should be recorded in "What this step does not fix" rather than left implicit.

MY RECOMMENDATION: OPTION A, with the anchor-does-not-exist condition additionally surfaced on the human line so Fail loudly is not simply traded away (`next` and `status` already print a `note:` to stderr for a `--source` that does not parse; the same shape fits a `--source` that does not exist). Whatever is chosen, the prose corrections in section 3.3, section 3.4 and section 5 must be written to match it, and the test named in section 3.1 is owed with it.

### Item 2 (CONFIRMATION, low cost): is R2A-2 absorbed by specification line 271, as I ruled?

The adversarial lens raised it as possibly needing a human decision, so I put my ruling up rather than burying it.

- OPTION A (RECOMMENDED): ABSORBED. Line 271 tells reviewers and implementers not to accumulate a fresh accepted cost on every new surface of `project_root_of_source`'s fallback; this is a new surface of exactly that fallback. Record the measurement against the queued step and file nothing.
- OPTION B: RECORD IT AS A FIFTH ACCEPTED COST and pin it with a test beside check 19b, on the reasoning that the four costs are pinned precisely so nobody re-litigates them and an unpinned fifth invites exactly that.

MY RECOMMENDATION: OPTION A. Option B costs a test and a specification edit for a population line 271 already anticipated in writing.

### Item 3 (SCOPE, one line): the false first sentence of `Projection.plan`'s doc comment

- OPTION A (RECOMMENDED): LEAVE IT to the documentation-currency step scheduled after inc3, per the standing rule and specification line 293. It is pre-existing text, the increment's own added sentence is true, and folding it in widens a settled increment.
- OPTION B: TAKE THE ONE-LINE CORRECTION NOW at `src/main.rs:570`, on the reasoning that the increment edited this very comment, the correction is a single line with no behaviour attached, and a Writer agent reading the comment as ground truth is a stated risk in this project.

MY RECOMMENDATION: OPTION A, but Option B is cheap enough that I would not argue if the human prefers currency over boundary here. What must not happen is the implementer taking Option B on their own judgement, since that is precisely the scope-widening the increment boundary exists to prevent.
