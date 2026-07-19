# Inc 4 triage: TOML-source swap for W3/W4/W5 + `status`

Artifact: `structured-skeleton-inc4` (RISKY), commit `36a872f` on base `3f29a81`. Triager independent of the implementer and orchestrator. Verdicts below are judged against the Inc 4 contract (`docs/plans/agent-scaffold.md`, tag `structured-skeleton-inc4`), the Inc 5 / Inc 6 contracts (for defers), and the numbered Project Principles in `AGENTS.md`.

Evidence checked directly (36a872f, not the pre-Inc-4 main working tree):

- `git show 36a872f:src/main.rs` line 380: `--workflow` still carries `#[arg(long, requires = "plan")]`; the TOML branch comment reads "TOML-sourced: needs only the metrics log (the plan comes from the TOML)".
- `git diff 3f29a81 36a872f -- src/workflow.rs`: `waivers_from_toml` sets `line: position`, where `position` is a per-`[[step.waiver]]` counter incremented BEFORE the drop `continue`, so it is TOML document order (kept waivers get unique but non-contiguous positions). `w5_problems` prints every violation as `round log line {}` using `waiver.line`.
- `metrics::Waiver` (`src/metrics.rs:812`) has no `id` field. The TOML `source::Waiver` (`src/plan/source.rs:237`) HAS `id: String` (unique, enforced by `validate_source`); `waivers_from_toml` drops it.
- Stale doc strings confirmed in 36a872f: `--workflow` help "Requires --plan" (no TOML path); `ValidateArgs --source` help "When omitted, no source is validated" (silent about driving `--workflow`); `run_validate` doc "With `--workflow` (which requires `--plan`)..." (no TOML arm). The `StatusArgs --source` help WAS updated correctly.

Consolidation: C1 (correctness, low) + L3 (liveinvariance, low) + S1 (consistency, medium) are ONE issue (the W5 message locator); ruled once below as MSG-LOCATOR.

---

## Verdicts

### MSG-LOCATOR (C1 + L3 + S1): W5 message says "round log line N" for a TOML waiver

- Verdict: VALID.
- Final severity: LOW (reconciling S1's medium down to C1/L3's low). ORCHESTRATOR-DECISION INPUT (see below).
- Rationale: Confirmed real. For a TOML-primary plan, `w5_problems` prints `round log line {N}` where N is TOML document-order position, so a user looking in `workflow.jsonl` line N finds an unrelated record or none. It fires on WELL-FORMED-but-mis-evidenced waivers too (both Inc 4 W5 negatives, `..._w5_rejects_a_mis_tiered_waiver` and `..._w5_rejects_a_wrong_escalation_waiver`), not only malformed plans, so it is a realistic path. But `waiver.line` is used ONLY inside `format!` strings (workflow.rs 420/431/466/478); it never enters a comparison, join, or enforcement decision, and the substantive text after the locator (the reason, the cited evidence pointer, the tier) is correct, so the waiver stays identifiable by content. The verdict is always right; only the locator prefix asserts a false JSONL provenance.
- Why LOW, not medium: severity is impact-if-unfixed. The impact is bounded to debugging friction on a correct verdict; no wrong pass/fail, no hidden violation, no data loss, nothing blocked. S1's "only user-visible TOML error path that names a misleading file+line" is true but is a diagnostic-quality argument, not a correctness or enforcement one, so it does not lift the impact above low. I record the medium argument for the orchestrator but rule low.
- Fix-now vs defer: FIX-NOW in Inc 4. ORCHESTRATOR-DECISION INPUT. Reasoning: (a) the fix is small and entirely inside the code under review; (b) Inc 4 is the increment that introduces the defect, and its own `waivers_from_toml` doc comment already admits `line` is "a stable disambiguator for the shared W5 message rather than a real log line" -- the code knows the string is wrong; (c) the "no real user hits it yet" mitigation expires at the very NEXT increment (Inc 5 makes THIS repo TOML-primary), so deferring buys almost nothing and risks the fix being lost inside Inc 5's heavy migration commit. Fixing it with the code that creates it is the cleaner-long-term choice (Principle 17) and keeps the increment self-consistent. The only argument to defer is "it is low," which the low cost of the fix outweighs.
- What the fix must achieve: a W5 problem message must not claim JSONL provenance for a waiver that came from the TOML. Carry the waiver's stable `id` (present in `source::Waiver`, unique per `validate_source`) through `waivers_from_toml` into `metrics::Waiver` (add an `id`/`locator` field, populated in BOTH `parse_waivers` as `format!("log line {}", n)` and `waivers_from_toml` as `format!("waiver `{}`", id)` or similar), then replace the four `round log line {}` prefixes in `w5_problems` with the substrate-neutral locator. Keep one `w5_problems` (Principle 16). Update both W5 TOML negatives to assert the new substrate-correct locator so the message is pinned (Principle 11).

### L1: `--workflow` `requires = "plan"` blocks a TOML-only project; branch comment contradicts it

- Verdict: VALID, but SPLIT (see below). Final severity: LOW as an Inc-4-actionable finding (reconciling liveinvariance's medium and consistency's "not a finding" -- both partly right). ORCHESTRATOR-DECISION INPUT.
- Rationale: The behavior is confirmed: `--workflow` still carries `requires = "plan"`, so `validate --workflow --source <toml-primary>` without a `--plan` exits 2, even though the TOML branch ignores `plan_contents` (`_` in the match) and its comment says it "needs only the metrics log". Two separable defects hide in this one finding:
  1. The RELAXATION (make `--workflow` accept plan-OR-source): NOT required by the Inc 4 contract. The Inc 4 bullet scopes the increment to pointing the checks at the TOML source behind the `primary == "toml"` gate and explicitly leaves the live repo Markdown-primary; it says nothing about the CLI arity. The "validate passes on a TOML-sourced project with no Markdown plan" requirement is written into the Inc 6 acceptance, which lists Inc 4 as a dependency. So the relaxation is legitimately deferred, not an Inc 4 shortfall. Consistency's "accepted deferred wart" is right on this half.
  2. The MISLEADING COMMENT: the branch comment "TOML-sourced: needs only the metrics log" asserts, inside the same file whose clap constraint forbids it, a capability the CLI does not have. That IS an Inc 4 accuracy defect (liveinvariance is right that something is wrong). It is low severity and folds into the S2 doc pass. This is why I downgrade the finding's medium to low: the only Inc-4-actionable part is a doc/comment correction.
- Fix-now vs defer, for the RELAXATION: DEFER. Recommended target: Inc 6. ORCHESTRATOR-DECISION INPUT.
  - Inc 6 argument (recommended): Inc 6's acceptance literally requires `validate` to pass "on a TOML-sourced project with no Markdown plan," and a fresh scaffolded project is the first context that genuinely has no hand-authored plan to pass. Inc 4 and Inc 5 are NOT blocked: through Inc 5 this repo retains a rendered `<task>.md` usable as the (ignored) `--plan`, so the current `requires` is satisfiable everywhere until Inc 6. Pulling the relaxation into Inc 4 would be scope creep on a risky increment whose contract does not call for it (Principle 8).
  - Inc 5 argument (viable alternative): Inc 5 is the live cutover that makes THIS repo TOML-primary and wires `validate --workflow` into `checks.toml` + CI. If the orchestrator wants that CI invocation to be clean (`--workflow --source` with no placeholder `--plan`), landing the relaxation at Inc 5 rather than Inc 6 is reasonable. The cost of NOT doing so is a redundant `--plan <rendered.md>` in the Inc 5 CI call, which works but is ugly.
  - Recommendation: defer to Inc 6 (contract-faithful owner), with Inc 5 as an acceptable earlier home if the orchestrator prefers a placeholder-free cutover invocation. Either way it is NOT fix-now in Inc 4.
  - If/when relaxed, the fix must achieve: `--workflow` accepts a plan OR a TOML source and errors clearly (not clap exit 2 with a misleading message) when NEITHER is present, e.g. a clap `required_unless_present`/arg-group tying `--workflow` to (`--plan` OR a TOML-primary `--source`).
- The MISLEADING-COMMENT half: FIX-NOW in Inc 4, folded into S2.

### L2: `status --source <malformed>.toml` falls back silently with no diagnostic

- Verdict: VALID. Final severity: LOW.
- Rationale: Confirmed. `toml_source` swallows a parse error to `None`, so `status --source <broken>` projects from `--plan` (or empty) with no signal the source failed to parse. This does NOT weaken enforcement (`validate --source` reports the malformed source, and its workflow gate fails closed to a non-zero exit), so it is a projection-diagnostic gap, not a correctness hole. But a user who explicitly passed a `--source` and silently gets a different projection is a mild Principle 12 (fail loudly) violation.
- Fix-now vs defer: FIX-NOW in Inc 4 (lowest-priority of the fix-now items; discretionary). `status --source` is introduced by Inc 4, so the silent fallback is this increment's behavior. The fix is a one-line stderr note and aligns with Principle 12. It does not make `status` enforcing (that stays `validate`'s job); it just informs. If the orchestrator wants Inc 4 kept minimal, this is the one fix-now item safe to drop, but I recommend including it.
- What the fix must achieve: when a `--source` path is given to `status` but fails to parse, emit a one-line stderr note (e.g. "note: --source <path> did not parse; projecting from --plan") before falling back, so the fallback is visible rather than silent. Do not change the exit code or make `status` enforcing.

### S2: stale `--workflow` / `--source` help + `run_validate` doc after the TOML swap

- Verdict: VALID. Final severity: LOW.
- Rationale: Confirmed in 36a872f. (a) `--workflow` help says "Requires --plan" and does not mention the TOML path; (b) `ValidateArgs --source` help says "When omitted, no source is validated" and never says a TOML-primary source ALSO drives `--workflow`; (c) `run_validate`'s doc says "With `--workflow` (which requires `--plan`)..." with no TOML arm. Plus the L1 branch comment "needs only the metrics log" actively contradicts the same file's `requires = "plan"`. These are accuracy defects on the code that shipped in this increment (Principles 19, 20).
- Fix-now vs defer: FIX-NOW in Inc 4. Docs and comments describing code under review should be corrected with that code; the branch comment is self-contradicting today. Cheap and local.
- What the fix must achieve: the four strings must state the CURRENT truth given L1's deferral -- namely that when `--source` is TOML-primary the check reads steps/questions/waivers/baseline from it, AND that `--plan` is still syntactically required by the CLI (relaxation deferred to Inc 6/Inc 5), so no string asserts a capability clap forbids. Specifically: correct the `--workflow` help and `run_validate` doc to mention the TOML arm and the still-required `--plan`; correct the `ValidateArgs --source` help to note it drives `--workflow` when `[meta].primary == "toml"`; and correct the "needs only the metrics log" branch comment to note `--plan` is still required (ignored, but present) until the relaxation lands.

### S3: imprecise test comment ("isolating the W5 rejection" when two W5 problems fire)

- Verdict: VALID. Final severity: LOW (trivial).
- Rationale: Confirmed. In `check_workflow_toml_w5_rejects_a_mis_tiered_waiver`, the fixture's record-backed+`predates-logging` waiver trips TWO W5 checks (the evidence-join failure AND the reason<->tier mismatch); the assertion targets the tier-mismatch one and is correct, but the comment says "isolating the W5 rejection" as if only one fires. The test is sound; the comment overstates isolation (adjacent to Principle 11's "tests exercise what they claim").
- Fix-now vs defer: FIX-NOW in Inc 4 (trivial, in the code under review).
- What the fix must achieve: correct the comment to note that two W5 problems fire (missing-escalation join AND reason-tier mismatch) and that the assertion covers the reason-tier one; or restructure the fixture (give the evidence a real escalation) so only the pairing problem fires. The comment correction is the minimal fix.

---

## Tally

- Findings raised: 7 (C1, L1, L2, L3, S1, S2, S3). Distinct issues after consolidating C1+L3+S1 into MSG-LOCATOR: 5.
- VALID: 5 of 5 (MSG-LOCATOR, L1, L2, S2, S3). DISMISSED: 0.
- Severities (final): all LOW. Reconciled DOWN from the reviewers: S1 medium -> low (MSG-LOCATOR); L1 medium (liveinvariance) / not-a-finding (consistency) -> low as the Inc-4-actionable comment defect, with the relaxation deferred.
- No high/critical findings, so no dismissal re-check is owed.
- Fix-now in Inc 4: 4 items. Deferred: 1 (the L1 CLI relaxation -> Inc 6, or Inc 5 at the orchestrator's discretion).

## Fix-now remediation list (all in Inc 4)

1. MSG-LOCATOR: carry the TOML waiver `id` through `waivers_from_toml` into `metrics::Waiver` (new `id`/`locator` field, populated in both `parse_waivers` and `waivers_from_toml`) and make the four `w5_problems` prefixes substrate-neutral; update both W5 TOML negatives to assert the new locator. Keep one `w5_problems`.
2. S2 (+ L1 comment half): correct the `--workflow` help, the `ValidateArgs --source` help, the `run_validate` doc, and the "needs only the metrics log" branch comment to state that a TOML-primary `--source` drives the check AND that `--plan` is still required by clap until the deferred relaxation lands.
3. L2: emit a one-line stderr note when a `status --source` path fails to parse, before the silent fallback (lowest-priority; droppable if keeping Inc 4 minimal).
4. S3: correct the `check_workflow_toml_w5_rejects_a_mis_tiered_waiver` comment (two W5 problems fire), or restructure the fixture to isolate the tier-mismatch problem.

## Orchestrator-decision inputs (my recommendation attached)

- MSG-LOCATOR severity: low vs medium. Recommendation: LOW (diagnostic-only, verdict always correct). Fix-now recommended regardless of the low/medium call.
- L1 CLI relaxation: fix-now vs defer, and Inc 5 vs Inc 6. Recommendation: DEFER, target Inc 6 (its acceptance owns "no Markdown plan"; Inc 4/Inc 5 retain a rendered `.md` so nothing is blocked). Inc 5 is an acceptable earlier home if a placeholder-free cutover invocation is wanted. The self-contradicting comment is fixed now via item 2 either way.
