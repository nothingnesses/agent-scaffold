# Inc 1 (structured-skeleton) triage verdicts

Triager: independent (not the producer, not the orchestrator). Change under review: commit `aa42412` on base `a780541`. Contract: `docs/plans/agent-scaffold.md` line 684 (`structured-skeleton-inc1`), billed LOW risk / pure addition.

Method: read `git diff a780541 aa42412` and `git show aa42412:src/plan/source.rs`; built the worktree binary read-only and ran `validate --source` against crafted fixtures for every reproducible finding (Principle 6). Every reviewer reproduction below was independently confirmed: an `open` question with `folded_into = "ghost"`, a `superseded` question with no `superseded_by`, duplicate waiver `id`, duplicate `principle.n`, a `blockd_by` typo, self-`blocked_by`/`folds`, an `orphan_tasks` token equal to a step slug, and a `decided` question carrying `superseded_by` all validate clean (exit 0).

Severity scale: `low` / `medium` / `high` / `critical`. ASCII: `->`, `<->`, `>=`, `!=`.

---

## Consolidated: O1 + O2 + S7 -> question status/cross-reference integrity (VALID, medium)

The three findings are one issue in one code region (`src/plan/source.rs:491-513`). Ruled together per the orchestrator's instruction.

- O1 (folded_into resolved only inside the `Decided` arm): confirmed. `o1.toml` (`open` question, `folded_into = "ghost"`) validates clean.
- O2 (superseded does not imply superseded_by): confirmed. `o2.toml` (`superseded`, no `superseded_by`) validates clean.
- S7 (no bidirectional status <-> field enforcement): confirmed. `s7b.toml` (`decided` with both `folded_into` and `superseded_by`) validates clean.

Verdict: VALID, severity medium. The medium driver is a literal contract miss: plan line 684 lists `folded_into` among the cross-references that must "resolve" (unconditionally), and the module doc (`src/plan/source.rs:423-426`) claims `validate_source` catches the dangling cases, yet a dangling `folded_into` on a non-`decided` question is neither resolved nor rejected. This is exactly the class of authoring error the validator exists to catch (Principle 13/14). The `superseded_by`-presence and forbid-unless-status rules are not named in the contract, so they are the low-severity illegal-states (Principle 13) portion folded into the same fix; they do not raise the ceiling above medium (nothing consumes these fields in Inc 1 and the module is unconsulted pre-cutover).

Fix must achieve:

1. (Required, closes the contract gap) `folded_into`, when present, resolves to a real step regardless of question status, so a dangling target is always flagged. Mirror the unconditional treatment already given to `superseded_by`.
2. (Recommended, illegal-states) Enforce status <-> field consistency: `superseded` implies a resolving `superseded_by`; `folded_into` is present only when `decided`; `superseded_by` is present only when `superseded`. Decide and document the chosen rule set.

---

## O3 (waiver id) reconciled with S2 -> waiver-id uniqueness (VALID, medium)

Opus (O3) rated waiver-id uniqueness low; sonnet (S2) rated it medium. Confirmed: `o3.toml` (two `step`-unit waivers both `id = "dup"`) validates clean.

Reconciled severity: medium. Reasoning: (a) the validator already enforces id uniqueness uniformly for step slugs, increment ids, and question ids (`src/plan/source.rs:452-470`), so omitting waiver id is an inconsistency in the validator's own contract, not a deliberate narrowing; and (b) Inc 4's W5 read-path joins on waiver identity (AGENTS.md Instrumentation, `type: "waiver"`; plan Inc 4), so an undetected duplicate id becomes an ambiguous cross-substrate join, i.e. a malformed document passing the very boundary check meant to reject it (Principle 14). "Impact if left unfixed" therefore lands on Inc 4, which justifies medium over low.

Fix must achieve: waiver `id` uniqueness enforced, plan-wide (matching how Inc 4 will look them up), plus a well-formedness (`is_well_formed_token`) check, so waiver ids get the same treatment as the other ids.

---

## O3 (second part) -> principle.n uniqueness (VALID, low)

`src/plan/source.rs` never checks `principle.n`. Confirmed: `prin.toml` (two principles both `n = 1`) validates clean. VALID, low: principles are display data, not join keys, so the blast radius is small, but two principles sharing a number is an internal inconsistency the uniqueness pattern should cover.

Fix must achieve: reject duplicate `principle.n` values.

---

## O4 -> deny_unknown_fields (DEFERRED to human decision)

`src/plan/source.rs:402-404` (`toml::from_str`, no `deny_unknown_fields`). Reproduction confirmed: `o4.toml` (a step with `blockd_by = ["ghost"]`, a typo for `blocked_by`) validates clean, exit 0; the intended blocking edge is silently dropped and the dangling `ghost` is never seen.

Technical assessment: the reproduction is correct. Required-field typos are caught (the field goes missing -> parse error), but a typo in any OPTIONAL key is silently ignored and the field defaults. Blast radius: every optional key in a hand-authored file, including the correctness-bearing cross-reference keys `blocked_by`, `folds`, `folded_into`, `superseded_by`, `superseded_by`, `receipt`, `increment`, `evidence`, `orphan_tasks`, and the `[meta.sidecars]` keys. For the cross-reference keys the failure is silent data loss of a correctness edge, not a forward-compat tolerance. The JSONL log's permissive stance is not a true parallel: JSONL records are checked field-by-field with explicit presence checks, so a typo there still shows up as a missing required field, whereas the TOML `#[serde(default)]` optionals have no such backstop.

Recommendation (non-binding): add `#[serde(deny_unknown_fields)]`. The skeleton is hand-authored and versioned in-repo (Inc 3-6 will edit the structs anyway), so the forward-compat argument is weak and catching a typo that silently drops a cross-reference is the safer failure direction. Per the orchestrator's instruction this is DEFERRED to the human as a schema-strictness design choice; the human's decision governs and this triage issues no binding remediation.

---

## O5 -> self-referential / cyclic blocked_by / folds (VALID, low; cycle-detection out of scope)

`src/plan/source.rs:473-488`. Confirmed: `o5.toml` (step `a` with `blocked_by = ["a"]` and `folds = ["a"]`) validates clean.

Split verdict:

- Self-reference: VALID, low. A step that blocks itself can never unblock and a step that folds itself is self-contradictory; both are illegal states (Principle 13). The contract says "resolve" and a self-edge technically resolves (the target exists), so this is a Principle-13 extension rather than a literal contract miss, hence low.
- Cycle detection (multi-step `blocked_by` cycles): DISMISSED as out of scope for Inc 1. The contract says "resolve", not "acyclic", and a topological/ordering pass belongs to whichever later increment orders steps. Raising it here would be scope expansion (Principle 8).

Fix must achieve: reject `target == step.slug` for `blocked_by` and `folds`. Cycle detection deferred to the increment that orders steps.

---

## O6 -> orphan_tasks not unique and may collide with a real slug (VALID, low)

`src/plan/source.rs:531-536`. Confirmed: `o6.toml` (`orphan_tasks = ["a", "a"]` with a step `slug = "a"`) validates clean. VALID, low: the field's own definition (`src/plan/source.rs:244-247`, "tasks that appear in the round log but own no Roadmap step") is contradicted by an orphan token that equals a declared slug; a duplicate orphan token is a mild collision. Low because orphan handling is not consumed in Inc 1.

Fix must achieve: reject duplicate tokens within `orphan_tasks`, and reject any orphan token that equals a step slug (enforcing the field's own contract).

---

## S1 -> workflow.rs W5 refactor in a "pure addition" increment (VALID, low; keep-and-document)

`src/workflow.rs:411-415`: the inline `match waiver.reason { ... }` pairing check was replaced by `waiver.reason.required_tier() == waiver.evidence_tier`, and the `WaiverReason` import was dropped.

Two weighed facts: (1) opus verified the refactor is BYTE-FAITHFUL for every (reason, tier) pair (`WaiverReason::required_tier` at `src/metrics.rs:203-213` encodes exactly the old mapping, and `required_tier() == evidence_tier` is equivalent to the old per-arm equality because `EvidenceTier` has exactly two variants); and (2) the change was DISCLOSED, in the `aa42412` commit message and the implementer's report. So Principle 8's operative word, "silent", does not apply: the scope was flagged, not quietly done.

Verdict: VALID (the observation that a pure-addition increment touched live W5 code is correct and worth recording), re-severitised from medium to LOW. Rationale for the down-rate: byte-faithful + disclosed + covered by the passing 229-test suite means the "invisible regression" scenario the finding raises is already mitigated; the residual impact if left as-is is essentially nil.

Remediation is KEEP-AND-DOCUMENT, not revert. The refactor exists to single-source the `reason <-> evidence_tier` pairing shared between W5 and the new TOML check (Principle 16); reverting it would either duplicate the pairing (reintroducing the drift risk the refactor removes) or force the TOML check to inline its own copy. The fix must achieve only that the disclosed scope is captured in the round ledger so the Principle-16 improvement is an explicit, signed-off deviation from the "pure addition" billing. No code change.

---

## S3 -> parse_toml returns PlanToml, not the plan's (Vec<Step>, Vec<Question>, Meta) tuple (VALID, low)

`src/plan/source.rs:175-193` and `388-396`. The plan bullet specifies "a strict `plan::parse_toml -> (Vec<Step>, Vec<Question>, Meta)`"; the implementation returns the named `PlanToml { meta, steps, questions, principles }`.

VALID, low. This is a literal deviation from an explicit contract signature. The implementer's rationale is correct (a bare tuple would silently drop the `principles`, which the schema defines; Principle 16) and the named struct is a strict superset and the better design for Inc 3/4 callers (Principle 17), so the code is right and the contract text is stale. Nothing depends on the signature yet, hence low.

Fix must achieve: update the plan's Inc 1 contract line so it names the `PlanToml` superset (or "a struct carrying steps, questions, meta, and principles") rather than the retired tuple, so Inc 3/4 plan authors reference the type the code actually returns (Principle 16, one source of truth for the contract). Keep the struct.

---

## S4 -> meta.sidecars sub-table not in the plan's [meta] field list (DISMISSED)

`src/plan/source.rs:209-247`; fixture `[meta.sidecars]`. The plan bullet lists "the front-and-tail prose-sidecar references" within `[meta]` without prescribing a flat-vs-nested shape.

DISMISSED: because the contract does not prescribe the field layout, grouping `front`/`tail` under a `[meta.sidecars]` sub-table is an in-scope design choice, not a deviation from an explicit contract (contrast S3, where the plan gave a concrete signature). The struct is self-documenting via its doc comments, and the fixture exercises the sub-table form. Optionally the plan's `[meta]` prose could mention the sub-table as a reader nicety, but no defect requires action.

---

## S5 -> is_well_formed_token allows uppercase in slugs and increment ids (VALID, low)

`src/plan/source.rs:398-407`. The validator permits ASCII uppercase; the plan calls a step slug "kebab-case" (conventionally lowercase), and increment ids follow the same convention (`structured-skeleton-inc1`). A slug `Alpha` passes. The looseness is deliberate because orphan-task tokens carry an uppercase suffix (the fixture uses `round-log-core-incA`), so the shared validator cannot be lowercase-only.

VALID, low. The concrete harm is small (a cross-reference is a case-sensitive string match, so a mixed-case mismatch would be flagged as dangling), but the validator is genuinely looser than the stated kebab-case contract.

Fix must achieve: enforce lowercase for step slugs and increment ids while still allowing the uppercase orphan-task suffix, e.g. split the check or pass an `allow_uppercase` flag for the orphan-task call site.

---

## S6 -> fixture coverage gaps in status variants (VALID, low)

`src/plan/testdata/skeleton.plan.toml`. Step statuses `not-started`, `skipped`, `optional`, `deferred` and question status `exploring` are never exercised by the round-trip test (Principle 11).

VALID, low, with a caveat: the finding's concrete "serialisation-name typo in `StepStatus::Optional`" scenario is largely unfounded, because `StepStatus`/`QuestionStatus` use `#[serde(rename_all = ...)]` (mechanical), not per-variant `rename` tokens, so there is no hand-written token to mistype (unlike the metrics enums). The residual valid point is the plain Principle-11 gap: those variants never round-trip through a test.

Fix must achieve: add fixture entries or a small round-trip/parse test covering the unexercised step and question statuses. Cheap; the enums are small.

---

## S8 -> no test for folded_into pointing to a non-existent step (VALID, low)

`src/plan/source.rs:490-494`. The dangling-`folded_into` arm (`Some(target) if !slugs.contains(...) => problems.push(...)`) is exercised by zero tests; the existing test covers only decided-with-no-`folded_into`. Principle 11 coverage gap.

VALID, low. Fix must achieve: add a test for a `decided` question whose `folded_into` names an absent slug. This overlaps the O1/O2/S7 remediation: once `folded_into` resolution is made unconditional, the test should also cover the non-`decided` dangling case, so the two fixes are best done together.

---

## Ruled out (agree with reviewers, no verdict needed)

Both reviewers' "cleared"/"ruled out" sections are sound and independently spot-checked: the `metrics.rs` serde derives do not route through the JSONL path (that path reads `serde_json::Value` via hand-written `Xxx::parse`, not these derives); the W5 refactor is byte-faithful (see S1); exit-code wiring is correct; `options`/`chosen` are correctly absent from `Question`; and `validate --plan` on the live repo stays green (the Markdown path is untouched).

---

## Tally

Valid findings by severity:

- critical: 0
- high: 0
- medium: 2 (question status/field integrity [O1+O2+S7]; waiver-id uniqueness [O3/S2])
- low: 8 (principle.n uniqueness [O3]; self-reference in blocked_by/folds [O5]; orphan_tasks uniqueness/collision [O6]; W5 refactor keep-and-document [S1]; parse_toml return-type contract text [S3]; uppercase in slugs/increment ids [S5]; fixture status-variant coverage [S6]; dangling-folded_into test [S8])

Dismissed: 1 (S4, meta.sidecars sub-table; in-scope design choice, no defect). Deferred to human: 1 (O4, deny_unknown_fields; reproduction correct, real silent-data-loss path, non-binding recommendation to add deny_unknown_fields).

## Remediation list (what each valid fix must achieve)

1. [medium] Resolve `folded_into` to a real step whenever present, independent of question status; and enforce status <-> field consistency (`superseded` implies `superseded_by`; `folded_into` only when `decided`; `superseded_by` only when `superseded`).
2. [medium] Enforce waiver `id` uniqueness plan-wide plus well-formedness, matching the other ids.
3. [low] Reject duplicate `principle.n`.
4. [low] Reject self-reference (`target == step.slug`) in `blocked_by` and `folds`; defer multi-step cycle detection to the ordering increment.
5. [low] Reject duplicate `orphan_tasks` tokens and any orphan token equal to a step slug.
6. [low] Keep the W5 `required_tier()` refactor; record its disclosed scope in the round ledger as an accepted Principle-16 improvement over the "pure addition" billing (no code change).
7. [low] Update the plan's Inc 1 contract line to name the `PlanToml` superset instead of the retired `(Vec<Step>, Vec<Question>, Meta)` tuple.
8. [low] Enforce lowercase for step slugs and increment ids while still allowing the uppercase orphan-task suffix.
9. [low] Add test coverage for the unexercised step/question status variants.
10. [low] Add a test for a `decided` question whose `folded_into` names an absent slug (do together with remediation 1).

Deferred (human decides, not counted above): add `#[serde(deny_unknown_fields)]` to close the silent optional-key-typo data-loss path (O4).
