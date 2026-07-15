# state-schema increment 2, triage

Adjudicates round-1 findings for INCREMENT 2 of `state-schema` (the `validate` verb, `src/metrics.rs` schema validator, and the V-4 `task`/`ts` fields). Commit range `4de2b35..bdc0955`. Reviewer files read directly: `state-schema-2-reviewer-opus.md` (correctness), `state-schema-2-reviewer-sonnet.md` (schema consistency).

Artifact risk classification: LOW (single-user dev-tool calibration side-channel, no user data, changes are reversible). Required consecutive clean rounds to converge: 1.

Judged against AGENTS.md Principles 1-22, in particular Principle 16 (one source of truth), Principle 5 (make illegal states unrepresentable / drift detectable), Principle 14 (parse, don't validate), and Principle 12 (fail fast and loudly). Per the brief: the absence of `status` / plan-parsing (increment 3) is out of scope and not treated as a finding, and line length / prose wrapping is never a finding.

---

## F1 (sonnet) - schema lives in two places with no automated alignment guard

Verdict: VALID.

Severity: LOW (corrected down from the reviewer's MEDIUM).

Reasoning: the finding is accurate. The record schema genuinely exists in two authoritative places by deliberate design: the prose in `pack/instrument.md` (what the orchestrator LLM authors from) and the Rust validator in `src/metrics.rs` (what enforces it). Both reviewers confirmed the two are in exact sync at this commit, so there is no current defect. Nothing, however, guards against future drift: an edit to one side that is not mirrored in the other passes CI silently and only shows up when a human notices a spurious validation failure (or a gap) at runtime. This is precisely the single-source-of-truth failure class this project keeps hitting, and the Principle 16 tension is real.

On severity, the reviewer's MEDIUM is one level too high for the impact-if-unfixed rating (severity is absolute impact, not likelihood or fix cost). The blast radius is bounded and recoverable: drift would make the validator reject otherwise-valid hand-written records (exit 1) or accept a record shape the LLM never actually writes. Neither corrupts already-committed calibration data, and the failure is loud and caught at the next validate run, the same fail-loud path the module is built around. No user-facing bug, no data loss, fully reversible. That is a LOW absolute impact. The elevated likelihood from the recurring failure class does not raise the severity; it justifies fixing now (see below), which is a separate axis.

This is an upheld (valid) finding with a corrected severity, not a dismissal, so it does not invoke the high/critical dismissal-recheck backstop.

Recommended fix: add a unit test in `src/metrics.rs` that reads the prose via `include_str!("../pack/instrument.md")` and asserts, for every accepted on-disk spelling, that the spelling appears in the prose. The `enum_field!` macro already centralizes each enum's spellings in its `VARIANTS` slice, so the test can iterate `Phase::VARIANTS`, `RoundOutcome::VARIANTS`, `HumanDecision::VARIANTS`, `RecheckResult::VARIANTS`, `Classification::VARIANTS`, and `Severity::VARIANTS`, plus the four record-type names (`round`, `escalation`, `dismissal_recheck`, `intake`) and the field names the validator checks (for example `task`, `artifact`, `changed_since_prev`, `valid_findings`, `severities`, `consecutive_clean`, `human_decision`, `result`, `classification`, `replanned`), asserting each substring is present in the `include_str!`'d prose. This is a code-to-prose containment check: a rename or removal on either side breaks it (rename in code -> new spelling absent from prose; rename in prose -> code still holds the old spelling, now absent from prose), turning a silent drift risk into a compile-time-adjacent test failure. It does not parse the Markdown structure, so it stays cheap and low-maintenance. This directly serves Principle 16 and Principle 5 (make the drift detectable rather than latent). Build it now: the fix is small and targets the project's documented recurring failure class, so deferring buys nothing.

Note on the reviewer's weaker alternative (per-arm comments pointing at the instrument.md bullet): that improves traceability but is not a guard, since a comment cannot fail CI. Prefer the test.

---

## F2 (sonnet) - `ts` documented as ISO 8601 but validated as any string

Verdict: VALID.

Severity: LOW (agrees with the reviewer).

Reasoning: real prose-versus-code mismatch. `pack/instrument.md` line 3 calls `ts` "an ISO 8601 timestamp"; `check_record` (lines 198-200) only calls `require_str`, so `"ts":"not-a-date"` passes. The prose promises a format the validator does not enforce. Impact is small: `ts` is optional, LLM-written, and the primary calibration signals (counts, severities, outcomes, phase) do not depend on it, so a malformed timestamp only degrades any later time-based analysis. LOW is correct.

Recommended fix: soften the prose rather than tighten the validator. Change the instrument.md description from "an ISO 8601 timestamp" to a timestamp string with ISO 8601 as a recommendation (for example "may carry `ts`, a timestamp string, ISO 8601 recommended"), so prose and code agree that any string is accepted while still steering the LLM toward the format. Rationale: full ISO 8601 is a large grammar and even the RFC 3339 subset is fiddly to match correctly; on an optional, low-value, LLM-written field the false-rejection risk of a strict shape check outweighs the marginal benefit of catching a malformed timestamp, and a wrongly-rejected record would block validation for no calibration gain. Softening restores one consistent source of truth (Principle 16) at near-zero cost and keeps the boundary honest about what it actually enforces (Principle 14: the parsed type is "string", so the prose should claim exactly that).

Tightening the validator to check an RFC 3339 shape is a defensible alternative if the project later decides the format must be guaranteed; if taken, keep it a shape check (a light regex-style pattern), not a full calendar-validity parse, and update the prose to match. The primary recommendation remains soften-the-prose. Either way, prose and code must end up saying the same thing.

---

## L1 (opus) - duplicate JSON keys silently resolved last-wins

Verdict: VALID (low, informational); disposition: accept the residual risk, no fix required this increment.

Severity: LOW (agrees with the reviewer's informational framing).

Reasoning: the behavior is real and accurately described. `serde_json::from_str::<Value>` keeps the last value for a duplicated key, so a hand-written record with a repeated key validates against whichever value came last, with no signal. But this is not a violation of the increment-2 spec: `pack/instrument.md` says nothing about duplicate keys, standard JSON permits them (RFC 8259 leaves duplicate handling to the implementation), and serde's last-wins is the conventional resolution. The reviewer itself labels it informational and defensible as-is. Rejecting duplicate keys would be a stricter-parsing design choice (it would need a custom deserializer or a raw-token pass, since `Value` has already collapsed duplicates by the time `check_record` sees it), which belongs to an increment-3 parsing-strictness decision, not to what increment 2 was specified to do. Flagging it as a defect against this increment would be scope expansion (Principle 8).

Disposition: accept as recorded residual risk. It does not block convergence and needs no change this increment. Worth carrying forward as a note for the increment-3 parsing design: if stricter detection of subtly-corrupt hand-written data is wanted there, rejecting duplicate keys is one option to weigh, and the module's stated purpose (detecting corrupt hand-written records) makes it a reasonable candidate. That is a future design choice, not a fix owed here.

---

## Round outcome

Round 1 is NOT clean. Two new valid findings need addressing:

- F1 (valid, low): add the `include_str!`-based drift-guard test in `src/metrics.rs`.
- F2 (valid, low): soften the `ts` prose in `pack/instrument.md` to match the validator (any string, ISO 8601 recommended).

L1 is valid-informational with its residual risk accepted; it requires no fix and does not block convergence.

No critical, high, or medium finding stands after triage (F1 corrected medium -> low). No dismissal of a high/critical finding occurred, so the dismissal-recheck backstop is not triggered.

Because this artifact is LOW-risk, one clean round converges it. The implementer should apply F1 and F2, then the orchestrator spawns one fresh round; if that round is clean, the increment has converged.

Off-cycle versus fresh round: F2 is a trivial prose edit and could be folded in off-cycle. F1 adds a test and is a code change; it should go through the implementer and be confirmed by the next round, which must verify the test actually fails on induced drift (Principle 11: the test must exercise what it claims, so check it by temporarily mutating one spelling and seeing red). Since a confirming round is needed for F1 regardless, fold both fixes in and let that single round both verify F1 and serve as the converging clean round.
