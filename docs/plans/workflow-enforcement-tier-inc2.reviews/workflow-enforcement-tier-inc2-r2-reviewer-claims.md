# `workflow-enforcement-tier-inc2`, work review round 2, ISOLATED REVIEWER, CLAIM-ACCURACY lens

ARTIFACT. `git diff main..HEAD` at commit `6bf5280` on the review worktree `r2-claims`, concentrating on the round 1 fix pass `git diff HEAD~1..HEAD`: `README.md`, `CHANGELOG.md`, `src/main.rs`, `src/next.rs`, `tests/unsafe_pairings_are_refused_and_omitted.rs`.

METHOD. Every verdict below was produced by building a fixture on disk under the scratchpad and running the built binary (`target/debug/agent-scaffold`) against it, comparing the observed output to the quoted claim, or by reading the exact code path the claim describes and confirming the code performs (or does not perform) what is claimed. Nothing is concluded from reading a reviewer's or the triager's file alone.

---

## 1. THE ADDED NO-PLAN-READ SENTENCE, CLAUSE BY CLAUSE

The sentence, identical on both surfaces:

`README.md:236` and `CHANGELOG.md:23`:

> "Where a command reads no plan at all, as `status --resume` always does and as `status` and `next` do with a Markdown-primary `--source` and no `--plan`, the roots come from the `--source` and `--plan` themselves and the artifact must be under every one of them."

### Clause 1: "`status --resume` always [reads no plan at all]"

VERDICT: TRUE. `run_resume` (`src/main.rs:1467-1489`) never inspects `args.plan`'s or `args.source`'s CONTENT for a plan projection; it only derives `task` (from the filename), the ledger path, and the containment roots via `resume_roots(&args.source, &args.plan)`. This holds regardless of whether `--source` is TOML-primary, Markdown-primary, or absent, and regardless of whether `--plan` exists. Confirmed by running `status --resume` with a genuinely TOML-primary `--source` (a case where `status`/`next` WOULD read a plan) and observing the ledger containment note still fires exactly as with a Markdown-primary source.

### Clause 2: "`status` and `next` do [read no plan at all] with a Markdown-primary `--source` and no `--plan`"

VERDICT: TRUE as the stated case; not the only case, but the sentence does not claim exhaustiveness (it reads as an illustrative "as ... do", paralleling the `status --resume` clause immediately before it) and every additional path I found to the same "no plan read" state behaves consistently with what the sentence describes:

- Command: `status --source <Markdown-primary-toml, no --plan> --metrics <foreign-log>`. Observed: `metrics: unavailable, the round log <foreign> is not under the plan's project root <source's root>, so its records cannot be paired with this plan`, exit 0. Matches the claim.
- Divergent-anchor construction to test the PLURALITY ("every one of them"): `status --resume --source <alpha, Markdown-primary> --plan <beta/p.md, a genuine Markdown plan> --ledger-fragment <alpha's own, genuine ledger>`. Observed: `the ledger <alpha's ledger> is not under the plan's project root <beta>; nothing to resume`, exit 0. Alpha's own, legitimate ledger is refused solely because it is not ALSO under beta's root, which is exactly "the artifact must be under every one of them" and not merely "under any one of them". Confirmed the plurality is load-bearing, not decorative.
- A NARROWER case than the sentence states: an UNPARSEABLE `--source` (not a declared Markdown-primary, just invalid TOML) with no `--plan` reaches the identical `containment_roots` fallback (`toml_source` returns `None` on a parse failure the same way it does for a genuine Markdown-primary source, so `checked_plan_root` is `None` either way). Verified: `status --source <garbage.toml> --metrics <foreign-log> --json` still reports `"metrics_absent_reason": "log-not-this-project"`. So the sentence's illustrative case does not exhaust the trigger condition, but nothing behaves differently on the paths it omits (this is a LOW-priority precision note, not a separate finding).
- The "neither `--source` nor `--plan`" case is correctly NOT folded into this sentence: `resume_roots` returns an EMPTY vector when both anchors are `None` (`src/main.rs:1445, docstring: "with neither there is no root and the predicate does not fire"`), so no containment happens there at all, consistent with the EARLIER, separate sentence in the same section ("a run with neither `--source` nor `--plan` has nothing to anchor to"). Verified: `status --json` and `next --json` with neither flag read the plain current-directory-relative log with no containment note.

### Clause 3: "the roots come from the `--source` and `--plan` themselves"

VERDICT: TRUE. `containment_roots` (`src/main.rs:1332-1339`) falls back to `resume_roots(source, plan)` exactly when `checked_plan_root` is `None`, and `resume_roots` (`src/main.rs:1445-1454`) canonicalises each of `source` and `plan` independently. Confirmed by the tests above: the reported root in the refusal note is the SOURCE's own project root, not any plan-derived root (there is none to derive).

### Clause 4: "and the artifact must be under every one of them"

VERDICT: TRUE, and exercised (not vacuous) as shown in Clause 2's divergent-anchor test.

### Interaction with `validate --workflow` (the seam the paragraph's own opening invites checking)

The paragraph OPENS, one sentence before the added one (unchanged by this fix pass, pre-existing on `main` before round 1's ADV-1 fix even landed):

`README.md:236` (first sentence): "Every one of these commands checks that the log (and, for the ledger readers, the ledger) it is about to read lives under the project root of the plan it is about to read..."

and CLOSES, one sentence after the added one:

`README.md:236` (third sentence): "Where it does not, `validate --workflow` refuses as above, while `status` and `next` LEAVE THAT PART OUT..."

The added sentence itself is careful: it names exactly THREE commands (`status --resume`, `status`, `next`) as reaching "reads no plan at all", and says nothing about `validate --workflow`. That omission is CORRECT, because `validate --workflow`'s containment root supply was deliberately NOT touched by this fix pass (confirmed: `run_validate` still calls `checked_plan_root` directly at `src/main.rs:977`, not `containment_roots`; `containment_roots` at `src/main.rs:1332` is called only from `run_status` `:1149` and `run_next` `:1545`). The code comment at `src/main.rs:974-976` states this is intentional and matches the specification (`docs/plans/agent-scaffold.steps/workflow-enforcement-tier.md:157`, "on `validate --workflow` that case is the match's own `(None, None, _)` arm, already a hard problem for its own reason") -- a settled, out-of-scope design decision I am not relitigating.

But the FIRST sentence's "Every one of these commands checks that the log ... lives under the project root of the plan it is about to read" is stated WITHOUT the added sentence's qualification, and the THIRD sentence's "`validate --workflow` refuses as above" reads, in context, as the general consequence of the rule the first two sentences just established for all four commands. Read together, a reader has no signal that `validate --workflow`'s "refusal" in the no-plan-read configuration is NOT the containment refusal at all.

I verified this is not merely a wording nuance but a real behavioural gap between the two commands' claimed uniform coverage:

```
$ agent-scaffold validate --source alpha/docs/plans/p.plan.toml --metrics alpha/docs/metrics/workflow.jsonl --workflow
  (Markdown-primary --source, no --plan, metrics log IN the plan's own root)
--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan
exit=1

$ agent-scaffold validate --source alpha/docs/plans/p.plan.toml --metrics foreign/docs/metrics/workflow.jsonl --workflow
  (SAME command, metrics log now OUTSIDE the plan's root)
--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan
exit=1
```

Byte-identical failure message and exit code whether the log is in-root or out-of-root: the containment predicate never runs (`checked_root` is `None`, so `unsafe_pairing` is vacuously `false`, and the four-arm match's pre-existing `(None, None, _)` arm fires instead, for a wholly unrelated reason -- "no plan source resolved", not "log outside project root"). `validate --workflow` therefore does NOT, in this configuration, "check that the log ... lives under the project root of the plan it is about to read" (there is no such root computed at all), contradicting the paragraph's opening claim taken at face value. This is filed as R2C-1 below.

---

## 2. FINDINGS

### R2C-1: the paragraph's opening "Every one of these commands checks..." is false of `validate --workflow` in the no-plan-read case, and the closing "`validate --workflow` refuses as above" invites reading its failure there as the containment refusal

SEVERITY: medium.

CLAIM, verbatim, `README.md:236` (and identically `CHANGELOG.md:23`): "Every one of these commands checks that the log (and, for the ledger readers, the ledger) it is about to read lives under the project root of the plan it is about to read, resolving both through their real on-disk locations so a symlink cannot disguise one as the other." ... "Where it does not, `validate --workflow` refuses as above, while `status` and `next` LEAVE THAT PART OUT with a reason in its place and still exit 0."

COMMAND RUN: see the two `validate --workflow` invocations quoted above (Markdown-primary `--source`, no `--plan`, metrics in-root vs. out-of-root).

OBSERVED: identical output and exit code in both cases. The message named is `"--workflow requested but no plan source resolved: pass a TOML-primary --source or a Markdown --plan"`, never the containment message ("... which is not under the plan's project root ...; pass a `--metrics` under that root, run against the plan's own log, or correct the `--source` and `--plan` pair").

CODE THAT MAKES THE CLAIM FALSE: `src/main.rs:977` (`let checked_root = checked_plan_root(toml_primary.is_some(), &args.source, &args.plan);`) is unchanged by this fix pass and is NOT routed through the new `containment_roots` (`src/main.rs:1332`) that `run_status` and `run_next` now use. In the no-plan-read configuration `checked_root` is `None`, so `unsafe_pairing` at `src/main.rs:981-982` is vacuously `false` and the containment branch at `:983-991` never executes; the pre-existing `(None, None, _)` arm of the four-arm match (below `:995`) fires instead, unconditionally on the metrics path's containment. `validate --workflow` still exits non-zero in this configuration, but never by "checking that the log lives under the project root" -- it always exits non-zero here for the unrelated, pre-existing reason of having no plan to check at all, whether or not a log is supplied and regardless of that log's location.

NARROWEST CORRECTION. Either qualify the opening sentence (for example: "Every one of these commands that reads a plan checks that the log ... lives under the project root of that plan") so it does not implicitly cover the no-plan-read state it never claims to fix for `validate --workflow`, or add one clause after the added sentence noting that `validate --workflow` has no equivalent fallback and instead relies on its pre-existing requirement that SOME plan resolve before `--workflow` runs at all. The fix needs to land on BOTH `README.md:236` and `CHANGELOG.md:23`, since the sentence is duplicated verbatim.

### R2C-2: the `Projection.plan` doc comment's added second sentence rests on a false first sentence: `plan` can be populated with no `--plan` at all

SEVERITY: medium (a load-bearing invariant of the struct central to `status`/`status --json`, in a project whose Writer agents read source comments as ground truth for follow-on changes).

CLAIM, `src/main.rs:570-571`: "The plan projection, present only when a readable `--plan` was given. It carries no reason field: there is exactly one cause, so a reason there would inform nobody."

The first sentence predates this increment (present verbatim on `main`, confirmed via `git show main:src/main.rs:567`) and is out of scope on its own; the SECOND sentence ("It carries no reason field: there is exactly one cause...") is NEW this increment (`git diff main..HEAD -- src/main.rs`) and is the claim in scope. It is presented as a conclusion resting directly on the (false) first sentence's premise, so as written the pair together misstates the field.

COMMAND RUN:
```
agent-scaffold status --source <proj>/docs/plans/p.plan.toml --json
```
with `<proj>/docs/plans/p.plan.toml` a valid TOML-primary source (`[meta] primary = "toml"`) carrying one step, and NO `--plan` flag at all.

OBSERVED:
```
{
  "plan": {
    "steps": [ { "slug": "step-a", "status": "in progress" } ],
    "open_questions": []
  },
  "metrics": null,
  "metrics_absent_reason": "log-absent"
}
```
`plan` is populated (`Some`) with no readable `--plan` anywhere in the invocation.

CODE: `src/main.rs:1120-1128` (`run_status`): `let plan = if let Some(source) = &toml_primary { Some(PlanProjection { steps: source.step_views(), ... }) } else { match &args.plan { ... } };`. The TOML-primary `--source` branch populates `plan` unconditionally on a successful parse, never consulting `args.plan`.

NARROWEST CORRECTION: change `src/main.rs:570` to something like "The plan projection, present when a TOML-primary `--source` parses or a readable `--plan` was given." and drop or rephrase the "exactly one cause" clause accordingly, since presence now has two independent paths (only ABSENCE reduces to the single "neither resolved" cause, which is a narrower and different claim than the one written).

### R2C-3: the test name `a_symlinked_log_leaf_outside_the_root_is_refused` also proves the opposite (quiet omission) behaviour it does not name

SEVERITY: low.

CLAIM (the test's own name), `tests/unsafe_pairings_are_refused_and_omitted.rs:539`: `a_symlinked_log_leaf_outside_the_root_is_refused`.

The test body (`:539-563`) asserts, in this order: (1) `validate --workflow` exits 1 and prints the containment refusal (the "LOUD manifestation", per the test's own comment) and (2) `status` and `next`, on the SAME fixture, exit 0 and print `metrics: unavailable,` (the "QUIET one", per the test's own comment, explicitly the opposite of "refused"). The name captures only the first half; a reader scanning test names for "which test proves the quiet status/next omission on a symlinked leaf" would not find this one under that description.

NARROWEST CORRECTION: rename to something like `a_symlinked_log_leaf_outside_the_root_is_refused_and_omitted`, matching the file's own naming convention (`unsafe_pairings_are_refused_and_omitted.rs`) which already distinguishes the two verbs.

### R2C-4: the doc comment on `a_surface_that_reads_no_plan_is_supplied_a_root` overstates what the test compares as "all three"

SEVERITY: low.

CLAIM, `tests/unsafe_pairings_are_refused_and_omitted.rs:571` (the last line of the test's doc comment): "... exactly as it supplies one to `status --resume`, so all three give the same answer on identical inputs."

"All three" reads as `status --resume`, `status`, and `next` giving identical answers. The test body never runs `status` (non-`--resume`) against the LEDGER at all -- `status`'s `Projection` has no ledger-related field to compare (confirmed: `src/main.rs:569-577`, no `resume_state` on `Projection`). What the test actually shows is: `status --resume` and `next` agree on the LEDGER (both refuse the same fragment with the same note), and `next` and `status` separately agree on the LOG (both report `log-not-this-project` on the same `--metrics`). No single input elicits a directly comparable answer from all three surfaces at once; the sentence conflates "the same root-supply policy now applies to all three" (true) with "all three answer the same question on the same run" (not tested, and not applicable to `status`, which does not read a ledger).

NARROWEST CORRECTION: "... so the two ledger readers agree with each other, and the two log readers agree with each other, on identical anchors" or similar; this is an internal test-file comment, not user-facing prose, so the fix is cosmetic.

---

## 3. CLAIM SURFACES SWEPT AND FOUND ACCURATE

- `README.md:210-232`, the `validate --workflow` refusal paragraph and its worked example (ran the exact `/elsewhere` shape with a real fixture; message and exit code match verbatim, including all three remedies).
- `README.md:238-260`, the `status`/`next`/`status --resume` omission paragraph, the `--json` reason-vocabulary paragraph, and its worked `status --json` example (ran it; output matches the documented shape).
- `CHANGELOG.md:23` "Changed" entry's enum/field enumeration: `metrics_absent_reason` (`log-absent`, `log-not-this-project`), `resume_state_absent_reason` (`ledger-absent`, `no-resume-section`, `ledger-not-this-project`), `no_active_loop_reason` (`no-plan-steps`, `all-steps-terminal`, `metrics-not-this-project`) -- all checked against `src/next.rs:102-144` variant order and names; all accurate.
- CHANGELOG's "TWO BREAKS TO KNOW ABOUT" enumeration (deliberately-mismatched-`--metrics` and symlinked-`docs/plans`-or-`docs/metrics` cases) -- unchanged by this fix pass, both reproduced.
- `containment_roots`'s doc comment (`src/main.rs:1316-1331`) -- every clause verified against the function body and its two call sites (`run_status:1149`, `run_next:1545`); correctly scoped to `status`/`next` only (never claims to cover `validate --workflow`).
- `checked_plan_root`'s doc comment (`src/main.rs:1296-1308`) -- unaffected by this fix pass, still accurate.
- `resolve_for_containment`'s and `is_outside_root`'s doc comments (`src/main.rs:1341-1358`, `1364-1378`) -- unaffected by this fix pass; the in-root-bound description in the latter is accurate and out of scope per the brief.
- `unpairable_log_note` and `unpairable_ledger_note` doc comments (`src/main.rs:1382-1401`) -- call-site claims verified by `grep`: `unpairable_log_note` used only by `run_status:1153` and `run_next:1554`; `unpairable_ledger_note` used only by `run_resume:1479` and `run_next:1574`. `next`'s reuse of `unpairable_log_note`'s result in BOTH the metrics line and the no-loop reason (claimed by the doc comment) confirmed at `src/next.rs:1188-1190`.
- `resume_roots`'s doc comment (`src/main.rs:1439-1454`) -- the "with one anchor alone the anchor is the root" and "with neither there is no root" claims both verified by direct test.
- `NextProjection`'s `metrics_absent_note` doc comment (`src/next.rs:193-198`, the TRI-1 fix from round 1): the deleted clause ("a machine consumer already holds the paths it passed in") is gone; the remaining "Not serialised: `--json` reports the token" is true (`metrics_absent_reason` IS the serialised token) and the "`Some` exactly when `metrics_absent_reason` is `LogNotThisProject`" invariant verified against `run_next`'s `if/else if/else` at `src/main.rs:1554-1563`. Accurate.
- `NextProjection::resume_state_absent_note` doc comment (`src/next.rs:200-202`) -- same invariant verified against `src/main.rs:1574-1581`. Accurate.
- The `--workflow` CLAP help string (`src/main.rs:438`) -- correctly scopes its containment-refusal clause to "a round log that lies outside the project root of the plan being checked" (presupposing a plan is checked), never overclaims into the no-plan-read case. Accurate, no correction needed.
- `run_resume`'s doc comment (`src/main.rs:1456-1465`) -- the "third [ledger-not-this-plan case] is a member of that list rather than an exception to it" claim confirmed against the function body's three note-and-return-Ok branches.
- Test names and bodies added by the fix pass other than the two noted above: `an_explicit_metrics_outside_the_plans_root_is_refused`'s new assertions (the refusal replaces rather than accompanies the four-arm match), `a_divergent_source_and_plan_pairing_is_refused`'s new third-remedy assertion, `status_omits_only_the_unpairable_part`'s new precedence-rule assertion, and the new `status --json` assertions added to `the_machine_surface_separates_the_causes_on_both_commands` and `the_resume_reasons_separate_and_cover_the_default_ledger` -- all match what their enclosing test names claim.
