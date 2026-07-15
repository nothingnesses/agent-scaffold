# Triage verdicts: state-schema increment 3

Artifact: INCREMENT 3 of `state-schema` (plan parsing + `validate --plan` + `status`), commit range `61ba68f..fdd3774`, grounded in `src/plan.rs`. Risk classification: LOW (small, self-contained parser/CLI; a defect is a lenient gate, not data loss or an irreversible change; expected to converge in one clean round). Principle references are to the plan's Project Principles (P1 cleaner-architecture, P2 minimal-by-default, P5 illegal-states-unrepresentable, P6 evidence) unless noted; "parse-don't-validate" and "fail fast" are the AGENTS.md workflow principles the reviewers cited.

Reviewers read: `state-schema-3-reviewer-opus.md` (O-F1..O-F6), `state-schema-3-reviewer-sonnet.md` (S-F1..S-F3). Both reviewers verified 72 tests pass, clippy clean, and no panic on any malformed input; the real plan validates and projects correctly. The findings are all about the gate being too lenient (false-negatives), not wrong behaviour on well-formed input. That is consistent with the ground truth in `src/plan.rs`.

Dedup map: {O-F1, O-F2, O-F3} -> TRI-1 (leniency). {O-F4, S-F2} -> TRI-2 (status exact-match). O-F5 -> TRI-3. O-F6 -> TRI-4. S-F1 -> TRI-5. S-F3 -> TRI-6.

---

## TRI-1 (O-F1 + O-F2 + O-F3) - Skip-on-mismatch parser where the design chose HARD-FAIL

Verdict: VALID. Severity: medium (anchored by O-F1; O-F2 and O-F3 are valid-low components folded into the same fix). Fix-now.

Reasoning. The design decision is explicit: `validate` HARD-FAILS and "reports on any schema or cross-reference violation (a malformed metrics record, a broken Roadmap table, a queue id with no target, a Roadmap slug with no Step Detail)" (`docs/plans/agent-scaffold.md`:390, strictness bullet), so "CI or an agent can gate on it". The parser in `src/plan.rs` is instead skip-on-mismatch: any Roadmap or queue line that does not parse is silently dropped (`parse_roadmap` src/plan.rs:90-112 `continue`s on `cells.len() < 2` and on no-backtick; `parse_questions` src/plan.rs:129-164 `continue`s on a malformed status group). The consequence is a false-negative gate:

- O-F1 (the medium anchor): a `## Roadmap` whose table has no `| --- |` delimiter row validates to exit 0. That is not a GFM table (a human sees literal pipe text), and the design names "a broken Roadmap table" as a violation `validate` must report. An agent that mangles or deletes the delimiter while editing gets a green gate, so the check does not protect the table's structure it is meant to protect (fail-fast; parse-don't-validate; P5).
- O-F2 (low): a data row with a missing status cell (`| `a` |`) or an unbackticked slug (`| a | complete |`) is dropped, so a fat-fingered step vanishes with exit 0 rather than being flagged.
- O-F3 (low): a `- ` item whose first backticked id matches `Q-<n>` but whose status parens are malformed (no `(`, or unclosed) is dropped, so a live-queue item with a status typo passes and disappears from the projection.

All three are the same defect (structurally-recognisable-but-unparseable input is dropped instead of reported), which is why they dedup into one fix. This is the core purpose of the gate, so it is what keeps this round from being clean.

Fix (three parts, one implementer pass). Detect a line that is structurally a Roadmap data row or a live `Q-<n>` item but fails to parse, and REPORT it as a validation problem, rather than dropping it. Concretely:

1. Model the Roadmap region as GFM does: within the `## Roadmap` section, take the contiguous run of pipe-lines as header (row 0, any content), delimiter (row 1, cells all dashes), then data rows (rows 2..). If the section contains pipe-lines but no delimiter row of dashes, report `Roadmap table is missing its \`| --- |\` delimiter row` (catches O-F1). This structural model is what lets the parser tell the header apart from a malformed data row, so tightening data-row parsing does not spuriously flag the header.
2. For each data row (row 2..), if it does not yield a `Step` (fewer than two cells, or no backticked slug in the first cell), report it as a malformed Roadmap row instead of `continue` (catches O-F2).
3. In `parse_questions`, once `is_question_id` confirms a live `Q-<n>` id, require a well-formed `(...)` status group; if the `(` is missing or the `)` is absent, push a problem rather than `continue` (catches O-F3). Historical `OQ-<letter>` lines and non-`Q-<n>` list items must still be ignored, so gate the new strictness on `is_question_id` being true, exactly where the current code already knows the line is a live item.

Scope guard (P2 vs P5). Stop at the GFM header+delimiter+rows model and the confirmed-`Q-<n>` requirement above; do not build a general Markdown table validator or handle arbitrary GFM corner cases. That bounded model catches every enumerated design violation without over-engineering, and matches P1 (the cleaner structural model is worth more than the smallest local patch).

---

## TRI-2 (O-F4 + S-F2) - `question_status_ok` accepts terminal statuses as prefixes

Verdict: VALID. Severity: low. Fix-now.

Reasoning. `question_status_ok` (src/plan.rs:200-204) uses `starts_with("open")` and `starts_with("superseded")`. Per the Documentation Protocol (`docs/plans/agent-scaffold.md`:36, 73; `pack/plan-template.md`:24) `open` and `superseded` are exact terminal statuses that take no parameter, so `(openfoo)`, `(supersededbar)`, `(open (typo))` all validate as ok. This is both a false-negative and an inconsistency with `roadmap_status_ok` (src/plan.rs:192-195), which exact-matches its enumerated set via `ROADMAP_STATUSES.contains` and reserves `strip_prefix` for the one genuinely parametric form (`blocked on <slug>`). Same class as TRI-1 (a lenient gate), but narrow and self-contained, hence low.

Fix. Exact-match the two parameterless statuses (`status == "open" || status == "superseded"`) and keep `starts_with("decided -> folded into ")` only for the parametric fold form. That makes the queue vocabulary as strict as the Roadmap vocabulary.

---

## TRI-3 (O-F5) - Fold-into target not cross-referenced against the Roadmap

Verdict: VALID. Severity: low. Fix-now.

Reasoning. A `decided -> folded into <slug>` item whose target is not a Roadmap step validates to exit 0 (evidence: `Q-1 (decided -> folded into \`ghost\`)`with no`ghost`step passes). The reviewer flagged an interpretation ambiguity in "a queue id with no target" (plan:390). I rule the cross-reference reading is the correct one: the scope paragraph says`validate`"checks the plan/ledger structured regions parse and their cross-references hold" (plan:393), and a present-but-nonexistent fold target is exactly a dangling cross-reference, parallel to the already-implemented "Roadmap slug with no Step Detail heading" check (src/plan.rs:229-234). The empty-target case is already caught (the trailing-space prefix in`question_status_ok` rejects it, as O-F5 notes), so only the dangling case is missing. Low: it would not affect the real plan (all live fold targets resolve), but it is a designed check that is absent.

Fix. In `validate_plan`, for each `Question` whose status begins `decided -> folded into `, extract the target (the backticked value via `first_backtick`, or the trimmed remainder) and check it against the set of Roadmap slugs already available from `parse_roadmap`; report a dangling target if it does not resolve. Reuse the step set already computed in `validate_plan`; do not re-scan.

---

## TRI-4 (O-F6) - Reverse cross-reference (detail block with no Roadmap row) not checked

Verdict: VALID. Severity: low. Fix-now.

Reasoning. Only the forward direction is validated (every Roadmap slug has a detail heading, src/plan.rs:229-234); an orphan `### \`slug\`` detail block whose slug is not in the Roadmap passes. The Documentation Protocol states the invariant in both directions: "every Roadmap slug has a detail block and vice versa" (`docs/plans/agent-scaffold.md`:34), so O-F6 has a direct spec basis, not just a reviewer's inference. The reviewer's caution was a false-positive risk (the detail-slug set is scanned document-wide, so a stray backticked heading could trip the reverse check). I checked this against the real plan (P6, evidence): all 36 backticked `###`/`####`detail slugs are present as Roadmap rows and vice versa (36 == 36, set difference empty;`workflow-viz`, the deferred block, is in the Roadmap). So the reverse check produces zero false-positives on the real plan and would not break `validate` on it. With the false-positive concern retired by evidence and a clear spec mandate, this is worth closing now to make the gate symmetric.

Fix. In `validate_plan`, for each slug in `detail_slugs` not present in the Roadmap slug set, report an orphan Step Detail block. Umbrella headings already contribute no slug (they lack a leading backtick, src/plan.rs:181-185), so they are unaffected.

---

## TRI-5 (S-F1) - No drift-guard test for the plan/queue status vocabulary

Verdict: VALID. Severity: low. Fix-now.

Reasoning. The plan status vocabulary lives in code (`ROADMAP_STATUSES` src/plan.rs:47-48; the queue set in `question_status_ok`) and in prose (`pack/plan-template.md`:24 for queue statuses, backticked; :30 for Roadmap statuses, a comma list), with no test guarding against divergence. The metrics module has the exact precedent: `instrument_prose_documents_every_accepted_schema_value` (src/metrics.rs:409-470) iterates the validator's own accepted set and asserts each value appears verbatim in `pack/instrument.md`. This is the same one-source-of-truth drift class (P5; AGENTS.md P16), and increment 2 spent three rounds on precisely it, so the risk is demonstrated, not hypothetical.

Feasibility (the reviewer flagged this as a judgment call, and the prompt asked whether the prose is too loose to guard). It is not: `pack/plan-template.md` enumerates the vocabulary tightly enough for a verbatim-substring guard analogous to the metrics one. Line 30 lists the Roadmap statuses as a comma list (`not started, in progress, complete, skipped, next, optional, deferred, blocked on <slug>`) and line 24 lists the queue statuses backticked (`open`, `decided -> folded into <slug>`, `superseded`). So a guard that iterates `ROADMAP_STATUSES` (plus the queue statuses and the two parametric prefixes) and asserts each appears in `pack/plan-template.md` is low-effort and mirrors the metrics test. Target `pack/plan-template.md` (the shipped pack asset), not `docs/plans/agent-scaffold.md` (the task-local plan), to match how the metrics guard targets `pack/instrument.md`.

With a direct precedent, a demonstrated cost, and a feasible cheap implementation, "note it for later" is hard to defend (P1); fix it in the same pass. Low, because the two sources currently agree and the guard prevents future drift rather than fixing a present divergence.

Fix. Add a test mirroring `instrument_prose_documents_every_accepted_schema_value`: iterate `ROADMAP_STATUSES`, the queue terminal statuses, and the parametric prefixes (`blocked on `, `decided -> folded into `), asserting each appears verbatim in `include_str!("../pack/plan-template.md")`. Anchor the backticked queue statuses with backticks (as the metrics test does) to avoid a substring false-positive on `open`/`superseded`; the multi-word Roadmap phrases (`not started`, `in progress`) are safe as plain substrings.

---

## TRI-6 (S-F3) - README does not document `status` or `validate --plan`

Verdict: VALID. Severity: low. Fix-now.

Reasoning. The README Usage section (`README.md`:116-215) documents only `scaffold`; neither `validate` (with `--plan` / `--metrics`) nor `status` (with `--plan` / `--json` / `--metrics`) is mentioned, though both are now live subcommands. `--help` is self-documenting so discovery is not absent, but the README is the expected home for usage examples of a non-trivial subcommand, and this is the increment that gates marking `state-schema` complete, so the now-live verbs should be documented before the step closes. No incorrect documentation, only missing; hence low.

Fix. Add short Usage subsections for `validate` (what it hard-fails on: metrics-record schema, Roadmap/queue schema and cross-references) and `status` (best-effort projection; `--json` shape), each with a one-line example and their `--plan` / `--metrics` flags.

---

## Round outcome

All six deduplicated findings are VALID; none dismissed, so no high/critical dismissal exists and the backstop re-check is not triggered.

This round is NOT clean: it carries new valid findings (one medium, five low), so the consecutive-clean streak stays at zero. Route all six to the implementer as one pass (they are small and cohesive: TRI-1 through TRI-4 tighten the same `src/plan.rs` gate, TRI-5 adds one test, TRI-6 edits the README). The artifact is classified LOW-risk, which requires ONE consecutive clean round to converge; after the fixes land, expect a single follow-up review round to confirm clean and converge. No finding is being accepted-as-residual-risk and none is deferred.

Not re-raised, per brief: the increment-2 duplicate-JSON-key (last-wins) metrics behaviour (accepted-as-is, out of this increment's scope) and ledger round-summary parsing (deliberately deferred this increment). Line length / prose wrapping is never a finding.
