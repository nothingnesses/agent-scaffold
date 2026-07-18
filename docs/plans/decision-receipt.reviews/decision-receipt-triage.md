# decision-receipt triage verdicts

Triager, independent of the reviewers and orchestrator. Read-only: this file is the only artifact written. Adjudicates the two reviewers' findings on `impl/decision-receipt` (`70903bd` vs main `db3ecd1`), deduplicating the shared QUEUE_FOLD_PREFIX finding. Claims marked "confirmed by running" were reproduced against the built worktree binary, not read.

Artifact risk class: RISKY (extends the enforcement backstop like `workflow-invariants`/W3; the plan pins two consecutive clean rounds). This shapes must-fix scope below, not the absolute severity of any one finding.

---

## V1. W4's derived min-index boundary is unsound in both directions (opus F1) [VALID, high]

Verdict: VALID. Final severity: high (confirmed, not downgraded).

The false-pass is real and I reproduced it. `w4_problems` (`src/workflow.rs:120-151`) derives its boundary as `decisions.iter().filter_map(question_index).min()` (`:124`) and exempts every decided item whose index is `< boundary` (`:140`). Because the boundary is the minimum index among the receipts that EXIST, a forgotten receipt for a lower-id decided item moves the boundary above that item and exempts it, which is exactly the item W4 is meant to catch.

Reproduced (isolated to W4, `step-a` marked `trivial` and a Step-Detail heading added so no unrelated W3/plan problems fire):

- False-pass: plan with `Q-44` and `Q-45` both `decided -> folded into`, log with a receipt for `Q-45` only. Result: `workflow invariants hold`, exit 0. The genuinely-missing `Q-44` receipt slips through silently. Nothing else backstops it: there is no malformed record for the strict `validate_log` to report (the receipt simply does not exist), so the E11 split does not catch it.
- False-flag: same plan, log with a single receipt for `Q-1`. Result: both `Q-44` and `Q-45` flagged as missing receipts, exit 1. Against the real plan (Q-1..Q-42 historical, no receipts) one mistyped or re-decided low `q_id` would pull ~41 explicitly-exempt historical items into scope.

Why high: this is the enforcement backstop and the whole point of the pilot (the human asked for a checkable guarantee that every decision was contract-presented). The false-pass defeats that guarantee silently for a current decided item; the false-flag floods the check with false positives on a single typo and could block convergence. Impact if unfixed: the check does not deliver its stated guarantee and is actively misleading. Not critical: it is an internal workflow/tooling self-check, not security/data/money-sensitive, humans are present at every decision, and the strict `validate_log` still catches malformed records; the blast radius is bounded to this enforcement layer.

Does the fix genuinely require a declared `baseline` cutoff (vs the derived min)? Yes. The defect is structural, not a `<` vs `<=` off-by-one: ANY boundary derived from the set of existing receipts is circular, because the quantity W4 checks (is a receipt missing?) is the same quantity that determines the boundary, so a missing receipt can always move the boundary to exempt itself. Min is unsound (shown); max is equally unsound (a forgotten highest-id receipt among several exempts itself); no derived function of the receipt set escapes the circularity. A sound forward-looking boundary must be an INDEPENDENT declared marker: a single cutoff id (the regime start) that W4 requires receipts at/after, recorded as data rather than inferred from the receipts. That is precisely the `type:"baseline"` marker planned for `waiver-model` (pilot 2), which the plan already names as the shared historical-exemption primitive for both W3 and W4 (`docs/plans/agent-scaffold.md:671-673`).

Coupling note: fixing F1 properly does couple pilot 1 to the baseline primitive, but the coupling is inherent to soundness, not scope creep, and it is aligned with the plan's own design (baseline is the intended shared W3/W4 primitive). Importantly, the derived-min approach F1 refutes is the exact candidate the step spec proposed (`docs/plans/agent-scaffold.md:669`: "require a receipt only for decided items whose `q_id` is >= the minimum `q_id` among existing `decision` records"). So this finding invalidates a spec-level decision, and the fix must revisit that spec line, not just the code. One-line fix note: replace the derived-min boundary with an independent declared cutoff (pull a minimal `baseline` marker into this step, or bring `waiver-model`'s baseline forward), so a forgotten receipt can no longer move its own exemption boundary.

---

## V2. Duplicated `QUEUE_FOLD_PREFIX` has no cross-check guard; drift silently disables W4 (opus F2 = sonnet F-2, deduplicated) [VALID, low]

Verdict: VALID. Final severity: low. Both reviewers raised the same defect; adjudicated once.

`src/workflow.rs:48` and `src/plan.rs:87` both define `const QUEUE_FOLD_PREFIX: &str = "decided -> folded into ";`, byte-identical today. The plan drift guard (`src/plan.rs`, `plan_template_documents_every_accepted_status`) pins `plan.rs`'s copy against the template, but nothing ties `workflow.rs`'s copy to `plan.rs`'s. If the fold-status vocabulary is changed in `plan.rs` + template without updating `workflow.rs`, `question.status.starts_with(QUEUE_FOLD_PREFIX)` (`src/workflow.rs:131`) matches nothing, every folded item falls out of W4's scope, and W4 silently requires no receipts at all. The tests that would catch this are only the ones asserting a NON-empty problems list; every `is_empty()` test (zero-record, matching receipt, before-boundary, non-decided) stays green with W4 fully disabled.

Low is correct: a silent total disablement is a serious failure MODE, but the trigger (a deliberate change to a stable vocabulary that touches one copy and not the other) is unlikely, and the header comment at `src/workflow.rs:43-47` already flags the duplication. Fix note: add a one-line test asserting the two constants are equal (or expose one `pub(crate)` and reuse it), pinning the `workflow.rs` copy the way the template guard pins the `plan.rs` copy.

---

## V3. W4 accepts a malformed-but-present receipt (opus F3) [INVALID]

Verdict: INVALID (the E11 strict/best-effort split working as intended, not a defect).

The observation is factually correct: `parse_decisions` (`src/metrics.rs:482-506`) projects any record with `type:"decision"` and a string `q_id`, ignoring the `chosen in options` and well-formed-`options` constraints, so `w4_problems` treats a malformed receipt as satisfying. But this is the deliberate strict-validate vs best-effort-projection split the step spec explicitly requires preserving (`docs/plans/agent-scaffold.md:669`: "`parse_decisions` drops malformed lines; `validate_log` reports them; both run"). I confirmed both run in the same `--workflow` invocation: `validate_log` is invoked whenever the metrics file is present (`src/main.rs:541`) and `check_workflow` under `--workflow` (`src/main.rs:600`), both pushing into the same `problems` list, so a malformed `decision` record (missing/wrong-typed field, or `chosen` not in `options`) makes the whole command exit non-zero via `validate_log` regardless of W4 accepting it. The guarantee holds at the command level, which is the level `validate --workflow` operates at. W4's job is presence/cross-reference; well-formedness is `validate_log`'s job, and no invocation runs one without the other. Opus itself framed this as "worth noting rather than a defect." There is no scenario in the actual command where a malformed decision record is invisible to both checks, so there is nothing to fix. (Contrast with V1, which IS a real hole precisely because a forgotten receipt produces NO malformed record for `validate_log` to report.)

---

## V4. Non-array `options` path is untested (sonnet F-1) [VALID, low; downgraded from medium]

Verdict: VALID. Final severity: low (reviewer proposed medium).

Confirmed: `require_options` (`src/metrics.rs:277-298`) has a `.as_array()` wrong-type branch (`:282-284`) returning "field `options` has wrong type (expected array)", and the decision tests cover missing (`a_decision_missing_options_is_reported`), empty (`a_decision_with_an_empty_options_array_is_reported`), and non-string-element (`a_decision_with_a_non_string_option_is_reported`) but NOT a non-array value such as `"options":"A,B"`. The parallel `reviewers` field does test its wrong-type path (`a_reviewers_field_of_wrong_type_is_reported`), so the gap is a real inconsistency against the project's own testing standard (Principle 11).

Downgraded to low because the code is correct on that path (opus verified it returns an error, no panic; I read the branch and agree), so the only impact if unfixed is a latent regression risk on a currently-correct branch of a helper whose sibling branches are all tested, with a one-line fix. Severity is absolute impact-if-unfixed, and an untested-but-correct branch is a low-severity gap, not a medium one; the reviewer's medium was defensible on Principle-11 grounds but overstates the impact. Fix note: add one test feeding `"options":"A,B"` and asserting `field \`options\` has wrong type (expected array)`.

---

## V5. `recommendation` prose implies `options` membership the validator does not enforce (sonnet F-3) [VALID, low]

Verdict: VALID. Final severity: low.

`pack/instrument.md:9` describes `recommendation` as "the orchestrator's recommended option," while the validator only requires it be a present string (`src/metrics.rs:366`, `require_str`); the `chosen in options` constraint is enforced for `chosen` alone (`:367-373`). The word "option" invites a reader to assume membership that is not enforced, and the asymmetry between `chosen` (enforced) and `recommendation` (unenforced) is undocumented, so the docs are not self-contained on this point (Principle 20). Low: a doc-clarity nuance with no correctness impact.

Scope note for the implementer/planner: not enforcing `recommendation in options` is a deliberate design decision, since the step spec names `chosen in options` as "the one genuinely new cross-field constraint" (`docs/plans/agent-scaffold.md:669`). So the fix is the documentation clarification the reviewer proposes (a parenthetical such as "any string; not required to be a member of `options`"), NOT adding a code constraint; enforcing membership would be a scope change to the converged substrate and is the planner's call, not required by this finding.

---

## Roll-up

Deduplicated findings: 5. Valid: 4 (1 high, 3 low). Invalid: 1.

- V1 (opus F1): VALID, high.
- V2 (opus F2 = sonnet F-2): VALID, low.
- V3 (opus F3): INVALID (E11 split working as intended).
- V4 (sonnet F-1): VALID, low (downgraded from medium).
- V5 (sonnet F-3): VALID, low.

Must-fix before this RISKY artifact can converge (two consecutive clean rounds required): V1 is the blocking must-fix; it defeats the pilot's stated guarantee and is confirmed in both failure directions. The three low findings (V2, V4, V5) are all cheap and should be addressed in the same fix pass rather than accepted as residual risk; a round does not count clean while any valid finding is unaddressed. So the next implement pass should: (1) replace W4's derived-min boundary with an independent declared cutoff, (2) pin the two `QUEUE_FOLD_PREFIX` copies with an equality test, (3) add the non-array `options` test, and (4) clarify the `recommendation` prose in `pack/instrument.md` (and regenerate the self-scaffold).

No high/critical finding is being DISMISSED (V1 is VALID and will be fixed; V3 is the only dismissal and is low), so no second-triager backstop re-check is required for this round.

F1/baseline coupling recommendation: YES, the F1 fix should pull the `baseline` marker (or a minimal declared-cutoff form of it) into this step. A sound forward-looking boundary cannot be derived from the receipt set (the derivation is circular), so an independent declared cutoff is required, and that cutoff is exactly the `baseline` primitive the plan already earmarks as the shared W3/W4 historical-exemption mechanism (`docs/plans/agent-scaffold.md:671-673`). This means revisiting the step spec's derived-min candidate (`docs/plans/agent-scaffold.md:669`), which F1 refutes, so the orchestrator should route V1 back through the planner (spec change), not only the implementer.
