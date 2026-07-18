# Architecture audit (Q-44), Phase 1: THE DATA MODEL

Read-only audit. Maps every distinct datum the workflow operates on: WHERE it lives, STRUCTURED vs PROSE, the CODE that produces/parses it, whether it is HAND-MAINTAINED or DERIVED, and whether it is DUPLICATED across homes. Ends with two flagged lists: (A) hand-maintained data that could be derived by projection, and (B) duplicated-across-homes data (the single-source-of-truth smells). This is raw material for the later design pass; it maps and flags, it does not design the target format.

Scope note on evidence: `src/plan.rs` states its own boundary in the header comment (lines 4-15): the plan's Roadmap table and Open Questions list "parse deterministically; the free narrative (Step Details, motivations) holds no machine state and is not parsed." `src/metrics.rs` (lines 1-11) owns the JSONL schema. `src/workflow.rs` (lines 1-24) cross-references the plan Roadmap against the JSONL round log and explicitly does NOT parse the ledger ("there is no ledger parse", line 9). So today: two files are machine-parsed (the plan's two structured regions, and the JSONL); the ledger is parsed by nothing; the rest of the plan is prose.

Homes in play:

- Plan: `docs/plans/agent-scaffold.md` (permanent).
- Ledger: `docs/plans/agent-scaffold.ledger.md` (per-task, committed-then-deleted at task close; template `pack/LEDGER.template.md`).
- Metrics log: `docs/metrics/workflow.jsonl` (permanent, cross-task, append-only).
- Pack format definitions (what scaffolded projects inherit): `pack/plan-template.md`, `pack/LEDGER.template.md`, `pack/instrument.md`.
- Code schema owners: `src/plan.rs`, `src/metrics.rs`, `src/workflow.rs`, `src/main.rs`.

--

## DATA-MODEL MAP (one entry per datum)

### D1. Plan Status line (the resume anchor)

- WHERE: `docs/plans/agent-scaffold.md:3` (a single ~1 screen-long `Status: ...` paragraph directly under the title).
- STRUCTURED or PROSE: PROSE. It opens `Status: in progress; ...` then runs as one free narrative sentence-stream naming per-step status, decided/deferred clusters, crates.io publication, and the "next work" set.
- CODE that produces/parses it: NONE. No parser reads this line; `src/main.rs run_status` (lines 643-695) and `src/plan.rs` never touch it. `pack/plan-template.md:3` seeds it as `Status: draft.`
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED, entirely by the orchestrator/human.
- DUPLICATED: YES. It restates, in prose, information that is authoritatively structured elsewhere: per-step status (the Roadmap table, D2), which questions are decided/deferred (the Open Questions queue, D3), and the "next" pointer. The plan's own Documentation Protocol (`agent-scaffold.md:33`) says the Roadmap "is the single source of truth for status and for implementation order", so this narrative is a second, hand-synced copy that can (and visibly does) drift (for example it asserts "no open questions remain" while `Q-42`/`Q-44` are `open`/`exploring` in the queue).

### D2. Plan Roadmap table (slug + status), one row per step

- WHERE: `docs/plans/agent-scaffold.md:130-179`, a GFM pipe-table under `## Roadmap`. Template: `pack/plan-template.md:26-33`.
- STRUCTURED or PROSE: STRUCTURED. A two-column pipe-table (`| Step | Status |`) with a backticked slug in cell 1 and a status string in cell 2.
- CODE: `src/plan.rs parse_roadmap` (lines 235-257) projects each data row into `Step { slug, status }`; `roadmap_table_problems` (lines 159-192) validates table well-formedness; `validate_plan` (lines 360-428) checks the status vocabulary (`ROADMAP_STATUSES`, lines 59-69, plus the parametric `blocked on <slug>`, line 74), slug uniqueness, and the slug<->Step-Detail cross-reference. `src/main.rs run_status` (lines 671-686) projects it to a per-status count summary; `PlanProjection.steps` (main.rs:419-424) serialises it for `status --json`. `src/workflow.rs check_workflow` (lines 77-86) reads it for W3.
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED (orchestrator edits the row on each step transition). It is the closest thing to a machine source of truth in the plan.
- DUPLICATED: PARTIAL. The status per step is single-sourced HERE by design, but it is re-narrated in the Status line (D1) and, for completed steps, cross-checked against the JSONL round log (D9) by W3 (workflow.rs:150-212). The Roadmap-slug-to-round-record link is itself carried only as a lexical convention (see D14).

### D3. Plan Open Questions queue items (id, status, ask, and the prose body)

- WHERE: `docs/plans/agent-scaffold.md:79-124` under `## Open Questions, Decisions, Issues and Blockers`, each a `- `Q-<n>` (<status>) <ask + body>` list line. Template: `pack/plan-template.md:22-24`.
- STRUCTURED or PROSE: MIXED. The id, the parenthesised status, and the leading one-line ask are STRUCTURED (parsed). The remainder of each item is PROSE (often very long: `Q-42`/`Q-43`/`Q-44` are multi-hundred-word decision narratives with lettered option lists (a)/(b)/(c), recommendations, and cross-references embedded in free text).
- CODE: `src/plan.rs parse_questions` (lines 274-309) projects `Question { id, status, ask }`; `queue_structure_problems` (lines 201-229) validates the `(status)` group; `validate_plan` (lines 401-425) checks the status vocabulary (`QUEUE_EXACT_STATUSES` = `open`/`exploring`/`superseded`, line 82; plus the parametric `decided -> folded into <slug>`, `QUEUE_FOLD_PREFIX`, line 87) and cross-references each fold-into target to a Roadmap slug. `is_question_id` (lines 262-267) accepts only `Q-<n>`, ignoring the historical `OQ-<letter>` provenance prose (agent-scaffold.md:77). The body after the ask is NOT parsed. `PlanProjection.open_questions` (main.rs:419-424) serialises id/status/ask only.
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED. The ledger (`agent-scaffold.ledger.md:99`) records this queue is error-prone by hand ("evidence that the hand-maintained living queue is error-prone").
- DUPLICATED: YES, in two ways. (i) The `decided -> folded into <slug>` status duplicates the fact-of-decision that also lives in the target Step Detail (the pointer target). (ii) The decision OPTION LABELS and CHOICE inside the prose body (e.g. `Q-42`'s (a)/(b)/(c), `Q-44`'s numbered sharp-edges list) are exactly the "soft prose/JSONL duplication of decision option-labels" that `Q-44` sharp-edge (7) and the whole `Q-42`/`Q-43` receipt saga flag: there is no structured home for option-labels+choice today, so they live only in this prose.

### D4. Plan Step Details (design/outcome reasoning, headed by slug)

- WHERE: `docs/plans/agent-scaffold.md:181-715` under `## Step Details`; one `### `<slug>`: <title>` block per Roadmap step (e.g. `core-assets` at :191, sample bodies :193-220). Template: `pack/plan-template.md:34-41`.
- STRUCTURED or PROSE: The heading SLUG is STRUCTURED (a cross-reference key); the block BODY is PROSE (free design/decision/outcome narrative, ~80% of the document per `Q-44` and `Q-43`).
- CODE: `src/plan.rs detail_slugs` (lines 316-333) collects only the backticked slug from each `###`-or-deeper heading (umbrella headings without a leading backtick are excluded); `validate_plan` (lines 384-399) enforces the bidirectional Roadmap-slug<->detail-slug cross-reference. The body text is parsed by nothing.
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED.
- DUPLICATED: The heading slug duplicates the Roadmap slug (D2) by design, and this duplication is exactly what `validate_plan` enforces as a cross-reference (so it is a checked, not a drifting, duplication). The body prose is the genuine single-source PROSE PAYLOAD (`Q-44` framing: "Step Details reasoning narrative ... likely stays PROSE PAYLOADS").

### D5. Project Principles (numbered list, incl. the new Principle 8)

- WHERE: `docs/plans/agent-scaffold.md:14-27` under `## Project Principles` (P1-P8; P8 the architecture pivot at :25). Template: `pack/plan-template.md:12-16`.
- STRUCTURED or PROSE: PROSE (a numbered list referenced by number, e.g. "Principle 8", but the numbering is positional prose, not a parsed field).
- CODE: NONE parses the plan's principles. NOTE the separate, unrelated machine principle data: `pack/principles.toml` parsed by `src/pack.rs` (`Principle`, `parse_principles`) for scaffolding AGENTS.md; that is the SCAFFOLD-INPUT principle set, distinct from a plan's own governance principles.
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED. `pack/plan-template.md:14` instructs seeding from AGENTS.md principles in order, then adding project-specific ones.
- DUPLICATED: YES (partial). By construction the leading principles are copied from the scaffolded `AGENTS.md` (which is itself rendered from `pack/principles.toml` via `pack.rs render_principles`), so principle text has two homes (the plan's list and AGENTS.md), reconciled only by hand. Code cross-references cite principle NUMBERS in comments/messages (e.g. "Principle 16" throughout plan.rs/metrics.rs), and the ledger records past numbering-drift incidents (round-2 finding G10, ledger:55, and H1, ledger:94, both "AGENTS.md-numbering leak").

### D6. Plan Documentation Protocol status/queue vocabulary enumerations

- WHERE: `docs/plans/agent-scaffold.md:33` (Roadmap statuses in prose) and `:37` and `:75` (queue statuses in prose). Template equivalents: `pack/plan-template.md:28` (Roadmap statuses) and `:24` (queue statuses).
- STRUCTURED or PROSE: PROSE (inline comma-lists of the allowed status words).
- CODE: The authoritative sets are the code constants `ROADMAP_STATUSES`/`ROADMAP_BLOCKED_PREFIX` (plan.rs:59-74) and `QUEUE_EXACT_STATUSES`/`QUEUE_FOLD_PREFIX` (plan.rs:82-87). A drift-guard test `plan_template_documents_every_accepted_status` (plan.rs:753-784) asserts the code sets appear verbatim in `pack/plan-template.md` ONLY. The live plan's own prose enumerations are NOT drift-guarded.
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED, in three places (code, template, live plan).
- DUPLICATED: YES, and DRIFTING TODAY. The plan's Documentation Protocol (:33) lists "not started, in progress, complete, skipped, next, optional, deferred, or blocked on <slug>" but OMITS `trivial` and `grandfathered`, which the code accepts (plan.rs:66-68) and the Roadmap actively uses (agent-scaffold.md:132-143 are `grandfathered`). Likewise the queue-status prose at :37 and :75 lists `open`/`decided -> folded`/`superseded` but OMITS `exploring`, which the code accepts (plan.rs:82) and `Q-44` actively uses (:124). Only `pack/plan-template.md` is kept current (it documents `exploring` at :24 and `trivial`/`grandfathered` at :28). So the dogfooded plan's own prose is a stale third copy of a vocabulary whose real source is the code.

### D7. Plan Repository-Layout-and-Current-Architecture section

- WHERE: `docs/plans/agent-scaffold.md:39-71` under `## Repository Layout and Current Architecture`.
- STRUCTURED or PROSE: PROSE (a hand-written index of `src/` modules, key types, the pack format, and the data flow).
- CODE: NONE parses it. It is not in the pack template (`pack/plan-template.md` has no such section; it is a project-specific addition).
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED.
- DUPLICATED: YES (soft). It restates the shape of `src/main.rs`, `src/pack.rs`, `src/manifest.rs`, `src/tui.rs` (type names, flag lists, field lists), so it drifts from the code as the code changes; the section itself says "The code in the repository is the source of truth; this is an index into it" (:41), acknowledging the duplication.

### D8. Ledger RESUME STATE block (compaction checkpoint)

- WHERE: `docs/plans/agent-scaffold.ledger.md` `## RESUME STATE` section; template `pack/LEDGER.template.md:11-13`.
- STRUCTURED or PROSE: PROSE (a pointer-plus-transient-state paragraph).
- CODE: NONE. `workflow.rs` does not parse the ledger (workflow.rs:9). NOTE `Q-28`/`state-queries` (agent-scaffold.md:109) PLANS a "verbatim `## RESUME STATE` section extract" in `status --resume`, but that step is `not started` (Roadmap :174), so no code reads it today.
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED; per-task and DELETED at task close (template :3, :13).
- DUPLICATED: YES, and by design mixed. The template (`pack/LEDGER.template.md:13`) tells it to be "a pointer plus transient state, not a restatement of the Roadmap or the Open-Questions queue; do not copy step statuses or decisions here" so it explicitly tries to AVOID duplicating the plan Status/Roadmap (D1/D2). But it holds the genuinely non-plan-derivable transient counters (current round number, consecutive-clean streak, pending dismissal re-check) that exist NOWHERE structured; those same counters are ALSO the values the JSONL round records carry as `consecutive_clean` (D9), so the streak has two hand-written homes (prose here, integer there).

### D9. Ledger round-records narrative (per-round prose)

- WHERE: `docs/plans/agent-scaffold.ledger.md:25-131+` (e.g. `## Round 2 (plan review) reviewer findings`, per-round paragraphs); template `pack/LEDGER.template.md:7-9`.
- STRUCTURED or PROSE: PROSE (per-round narrative: what was reviewed, risk classification, reviewers/triager, verdicts, changed-since-prev, outcome, convergence decision).
- CODE: NONE parses it (template :9 and :3: "holds no machine-parsed tables"; workflow.rs:9).
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED; per-task, deleted at close.
- DUPLICATED: YES. This is the HEADLINE single-source-of-truth smell, and it is openly acknowledged: `pack/LEDGER.template.md:9` says "When instrumentation is on, the orchestrator ALSO appends a `round` record for the same round to `docs/metrics/workflow.jsonl`; the core counting reads this narrative, not that log." So the SAME per-round facts (artifact, phase, outcome, streak, risk class, findings) are hand-authored TWICE: as prose here and as a `round` JSON object in the JSONL (D10). `Q-34` (agent-scaffold.md:114) already found and partly resolved a related instance (it removed the ledger's structured `## Round summaries`/findings-index TABLES for being a stale duplicate of the JSONL), leaving only this prose narrative as the residual duplicate; `Q-44` sharp-edge (4) lists "the ledger round-narrative vs the JSONL round-record duplication" as an edge to resolve.

### D10. Metrics JSONL `round` record

- WHERE: `docs/metrics/workflow.jsonl` (one JSON object per line; ~80 round records). Schema prose: `pack/instrument.md:5`.
- STRUCTURED or PROSE: STRUCTURED (JSON), except `artifact` (a free-text label, e.g. "CLI subcommand restructure") which is prose-in-a-field.
- CODE: `src/metrics.rs check_record`/`validate_log` (lines 276-440) validate the schema; fields: common `type`, `task`, optional `ts` (lines 281-285); `round`-specific `artifact`, `phase` (enum, lines 58-69), `changed_since_prev` (bool), `outcome` (enum `clean`/`new_valid`, lines 71-81), `valid_findings` (count), `severities` (array of the four-level `Severity` enum, lines 140-148 + `require_severities` 206-226), `consecutive_clean` (count), `risk_class` (enum `low_risk`/`risky`, lines 107-118), optional `reviewers` (D12). `src/metrics.rs parse_rounds` (lines 372-415) projects the W3-relevant subset into `Round { line, task, artifact, outcome, consecutive_clean, risk_class }`. Read by `src/workflow.rs` (D15, D16) and counted by `count_records` (metrics.rs:337-339) for `status`/`validate`.
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED by the orchestrator LLM (metrics.rs:5-11: "hand-write one JSON object per line ... records are not guaranteed well-formed"; validation is detection, not prevention).
- DUPLICATED: YES. (i) The whole record duplicates the ledger round narrative (D9). (ii) INTERNAL redundancy: `consecutive_clean` is fully determined by the prior `outcome` sequence within an increment, so it is a stored copy of a derivable value (workflow.rs:96-133 recomputes it and reports disagreement, `round_log_consistency_problems`). (iii) `valid_findings` (the count) and `len(severities)` (the list) both encode the finding count for a round and are not cross-checked against each other.

### D11. Metrics JSONL `escalation`, `dismissal_recheck`, `intake` records

- WHERE: `docs/metrics/workflow.jsonl` (e.g. the `escalation` record at line 82). Schema prose: `pack/instrument.md:6-8`.
- STRUCTURED or PROSE: STRUCTURED (JSON).
- CODE: `src/metrics.rs check_record` (lines 311-328): `escalation` -> `artifact` + `human_decision` (enum `decision`/`resume`, lines 83-89); `dismissal_recheck` -> `artifact` + `result` (enum `upheld`/`overturned`, lines 91-97); `intake` -> `classification` (enum `trivial`/`non_trivial`, lines 99-105) + `replanned` (bool). `parse_rounds` skips these (metrics.rs:370-371, non-`round` types carry no convergence data).
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED (orchestrator LLM).
- DUPLICATED: PARTIAL / cross-referenced. These are the "partial receipts" `Q-42` (agent-scaffold.md:122) names: `escalation.human_decision:"decision"` is the record `Q-40`/`escalation-exempt` (agent-scaffold.md:120, :659) would have W3 read to authorise a below-streak convergence; `intake.classification` mirrors, in structured form, an intake decision also narrated in prose in the ledger (ledger:42 etc.). The `intake` enum `trivial`/`non_trivial` is a DIFFERENT axis from the round `risk_class` `low_risk`/`risky` and from the Roadmap `trivial` status (all three deliberately separate; metrics.rs:99-118 documents the `Classification` vs `RiskClass` distinction).

### D12. Metrics JSONL `reviewers[]` per-reviewer attribution sub-record

- WHERE: nested array inside `round` records (e.g. workflow.jsonl:47-85). Schema prose: `pack/instrument.md:5`.
- STRUCTURED or PROSE: STRUCTURED (array of objects `{role, model, raw_findings, valid_findings, harness?}`).
- CODE: `src/metrics.rs require_reviewers` (lines 235-270): required string `role`/`model`, non-negative `raw_findings`/`valid_findings`, optional string `harness`; a present array must be non-empty (lines 246-248). `harness` added by `reviewer-harness-field` (`Q-37`, agent-scaffold.md:117).
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED (orchestrator LLM).
- DUPLICATED: Mostly NOT (deliberately). instrument.md:5 documents that per-reviewer `valid_findings` may sum to more than the round-level `valid_findings` (dedup across reviewers), so they are intentionally distinct rather than a duplication to reconcile. The same reviewer/model/finding facts are ALSO narrated in the ledger round prose (D9).

### D13. Metrics/plan SCHEMA definitions (the format contracts themselves)

- WHERE, metrics schema: authoritative in `src/metrics.rs` (the `enum_field!` macro constants + `check_record`, lines 34-332); documented in prose in `pack/instrument.md`. WHERE, plan schema/vocabulary: authoritative in `src/plan.rs` constants (lines 59-87); documented in prose in `pack/plan-template.md` and (stale) in the live plan (D6).
- STRUCTURED or PROSE: the code side is STRUCTURED (typed constants); the pack-doc side is PROSE.
- CODE: two drift-guard tests pin the prose to the code: `instrument_prose_documents_every_accepted_schema_value` (metrics.rs:673-741) and `plan_template_documents_every_accepted_status` (plan.rs:753-784). Both iterate the code constants and assert each appears verbatim in the pack markdown.
- HAND-MAINTAINED or DERIVED: BOTH homes hand-maintained; kept in sync by the drift-guard tests (not by projection).
- DUPLICATED: YES, by design and TEST-GUARDED. The schema/vocabulary lives in code AND in `pack/instrument.md` / `pack/plan-template.md`; the tests catch drift for the pack templates but (see D6) not for the live plan's own prose.

### D14. The round-record <-> Roadmap-step relationship (encoded lexically in `task`)

- WHERE: the JSONL `task` field (e.g. `state-schema-inc1`, `round-log-core-incB`, workflow.jsonl:37,55) and the Roadmap slug (D2).
- STRUCTURED or PROSE: STRUCTURED string, but the RELATIONSHIP (this round belongs to that step, and this is which increment) is encoded by a naming convention inside the string, not as a typed link.
- CODE: `src/workflow.rs leading_slug` (lines 63-71) strips a trailing `-inc<alnum>` suffix lexically to map a `task` back to its Roadmap slug; W3 (`w3_problems`, lines 150-212) then groups by that. The code itself flags this as fragile: the header comment (workflow.rs:55-62, "Latent over-strip risk (T3)") notes the strip is purely lexical and a slug ending `-inc<alnum>` would be mis-routed.
- HAND-MAINTAINED or DERIVED: HAND-MAINTAINED (the orchestrator types the `task` string with the right suffix).
- DUPLICATED: YES (soft). The step-slug is duplicated as the prefix of every `task`, and the increment identity is a bare string convention with no structured home; `Q-44` sharp-edge (6) explicitly lists "the alphanumeric `-inc<x>` leading-slug strip in W3" as an edge to resolve.

### D15. Cross-reference invariant: W3 (Roadmap `complete` step -> converged round records)

- WHERE: computed, not stored. Reads D2 (plan) + D10 (JSONL).
- CODE: `src/workflow.rs w3_problems` (lines 150-212), driven by `src/main.rs run_validate --workflow` (lines 593-622). Every `complete` Roadmap step must have matching round records whose per-increment peak `consecutive_clean` reaches the `risk_class` required streak (`low_risk` 1, `risky` 2; `RiskClass::required_streak`, metrics.rs:124-129). `trivial`/`grandfathered`/`skipped` are exempt (workflow.rs:135-137).
- HAND-MAINTAINED or DERIVED: DERIVED (a check, produces no stored datum).
- DUPLICATED: n/a (it is the consistency check BETWEEN the two hand-maintained homes D2 and D10). Relevant to the exemption sharp edge: `Q-44` sharp-edges (1)/(2) name `grandfathered`+`trivial` (and the proposed `escalation-exempt`) as proliferating special-case escape hatches on the status enum to be UNIFIED.

### D16. Cross-reference invariant: round-log internal consistency

- WHERE: computed over D10 alone.
- CODE: `src/workflow.rs round_log_consistency_problems` (lines 96-133): recompute the implied `consecutive_clean` from the `outcome` sequence per `task` and report any stored value that disagrees.
- HAND-MAINTAINED or DERIVED: DERIVED (a check over the hand-stored `consecutive_clean`).
- DUPLICATED: this check exists precisely BECAUSE `consecutive_clean` is a redundant stored copy of a derivable value (see D10.ii).

--

## (A) HAND-MAINTAINED DATA (candidates to derive by projection)

Each line: the datum, why it is hand-maintained, and a one-line note on the structured-source + projection direction (NOT a design; just the direction).

- A1. Plan Status line (D1). Fully hand-written narrative that re-states per-step status + decided/deferred clusters + the "next" pointer. Direction: derive it (or its structured half) from the Roadmap table (D2) and the Open Questions queue (D3) as a projected summary, the way `run_status` already prints a status breakdown (main.rs:671-686); keep only any genuinely non-derivable framing as prose.
- A2. Ledger RESUME STATE (D8). Hand-written pointer + transient counters, deleted per task. Direction: the pointer half is derivable from the plan (Roadmap active step, Open Questions), and `state-queries`/`Q-28` already plans a `status --resume` extract; only the transient counters (round number, streak) need a live home, and the streak already exists as JSONL `consecutive_clean` (D10).
- A3. Roadmap status per step (D2). Hand-edited on each transition; authoritative today. Direction: this is the least-bad hand-maintained source (it already IS the structured skeleton); the projection direction is the reverse, other views (A1) should derive FROM it rather than restate it.
- A4. Open Questions id/status/ask (D3). Hand-maintained list the ledger itself calls error-prone (ledger:99). Direction: the id/status/ask are already the parsed structured skeleton; the candidate to move to a structured home is the OPTION-LABELS + CHOICE currently trapped in each item's prose body (see B4), which a projection could render back into the human view.
- A5. Plan Documentation Protocol / Open Questions status enumerations (D6). Hand-copied vocabulary lists, currently STALE (missing `trivial`/`grandfathered`/`exploring`). Direction: project the allowed-status list from the single code source (`ROADMAP_STATUSES`/`QUEUE_EXACT_STATUSES`) into the rendered human view instead of hand-listing it in three places.
- A6. Repository-Layout section (D7). Hand-written index of `src/` that drifts from the code. Direction: candidate to derive from the code (module/type/flag inventory) or drop from the plan; lower priority than the workflow-data items, and it is project-specific (not in the pack template).
- A7. Ledger round narrative (D9) and JSONL `consecutive_clean` (D10.ii). Hand-authored per round; the streak is derivable. Direction: `consecutive_clean` could be computed from the `outcome` sequence rather than stored (workflow.rs already recomputes it), and the ledger narrative could be a projection of the JSONL round records plus any non-structured commentary.
- A8. Project Principles list numbering (D5). Hand-maintained and cited by number in code, with a history of numbering drift. Direction: candidate to give principles stable ids (like `pack/principles.toml` already does) and project the numbered view, so code/prose cite an id, not a positional number.

## (B) DUPLICATED-ACROSS-HOMES DATA (single-source-of-truth smells)

Each line: the two-or-more homes, how they can drift, and the structured-source + projection direction.

- B1. HEADLINE: ledger round narrative (D9) vs JSONL `round` record (D10). Two hand-authored homes for the SAME per-round facts (artifact, phase, outcome, streak, risk class, findings); `pack/LEDGER.template.md:9` admits the orchestrator writes both. Drift: the prose and the JSON can disagree on outcome/streak/risk (the core counting reads the prose, the tooling reads the JSON). This is `Q-44` sharp-edge (4); `Q-34` already killed the ledger's structured round TABLE for the same reason, leaving this prose residue. Direction: make the JSONL the single structured source and derive the ledger's round section (or the parts of it that are structured) by projection.
- B2. RESUME STATE vs plan Status/Roadmap (D8 vs D1/D2). The transient in-flight state and pointer are hand-written in the ledger AND narrated in the plan Status line, both restating the Roadmap. Drift: any status change must be echoed in up to three places by hand. Direction: single-source status in the Roadmap; derive both the Status-line summary and the RESUME-STATE pointer by projection; give the streak counter one home (JSONL).
- B3. Status-vocabulary in code vs pack template vs live plan (D6/D13). Three homes: `plan.rs` constants (source), `pack/plan-template.md` (drift-guarded), the live plan's Documentation Protocol/Open Questions prose (NOT guarded, currently STALE, missing `trivial`/`grandfathered`/`exploring`). Drift: demonstrated, live. Direction: the code is the source; project the allowed-status list into any human-readable doc rather than hand-listing it, or extend the drift-guard to the live plan.
- B4. Decision option-labels + choice: queue prose (D3) with NO structured home. The (a)/(b)/(c) option labels and the human's choice live only in each `Q-<n>` prose body; the `decided -> folded into <slug>` status and the target Step Detail also assert the decision. Drift: option-labels can be re-narrated inconsistently (this is exactly the SSOT hole that reopened `Q-42`->`Q-43`->`Q-44`; sharp-edge (7)). Direction: give option-labels+choice a single structured home (the proposed `type:"decision"` receipt is the candidate substrate the receipt-encoding passes converged on) and project them into the queue view, so the prose carries only rationale + a pointer.
- B5. Metrics schema in code vs `pack/instrument.md` (D13). Two homes; drift-guarded by `instrument_prose_documents_every_accepted_schema_value` (metrics.rs:673-741). Drift: caught by the test, so contained, but it is still two hand-maintained copies. Direction: the code is the source; a projection could render the human schema doc from the code constants instead of hand-writing and test-pinning it.
- B6. Roadmap step-slug duplicated into the JSONL `task` prefix, and the round<->step link carried only lexically (D14 vs D2). Drift: a renamed slug orphans its round records; the `-inc<x>` strip can mis-route (workflow.rs T3 note). Direction: a structured link (round record names its step id / increment id as fields) instead of a lexical convention parsed by string-stripping; `Q-44` sharp-edge (6).
- B7. Step-detail slug vs Roadmap slug (D4 vs D2). Duplicated by design but CHECKED (validate_plan bidirectional cross-reference, plan.rs:384-399), so this is the model of a duplication kept honest by enforcement, not a drifting smell; listed for completeness as the contrast case.
- B8. Plan Principles vs scaffolded AGENTS.md principles (D5). The plan's leading principles are hand-copied from AGENTS.md (itself rendered from `pack/principles.toml`). Drift: numbering/text divergence, with recorded incidents (ledger G10 :55, H1 :94). Direction: cite stable principle ids and project the numbered list; single-source the principle text in the TOML.

--

## Summary of the terrain for the design pass

Structured/machine-parsed today: only the plan Roadmap table (D2), the plan Open Questions id/status/ask (D3), the Step-Detail heading slugs (D4), and the JSONL records (D10-D12). Everything else in the plan and the entire ledger is prose that no code reads. The schema/vocabulary contracts (D13) already live twice (code + pack doc) and are held together by drift-guard tests, not projection. The heaviest single-source-of-truth smells, in rough priority: B1 (ledger prose vs JSONL round record, the acknowledged double-write), B2/A1/A2 (Status line + RESUME STATE re-stating the Roadmap), B4 (decision option-labels+choice with no structured home, the very hole that produced Q-44), B3 (the live plan's status vocabulary drifting stale against the code), and B6 (the round<->step link carried only as a lexical `-inc` string). The natural structured skeleton the pivot points at (Roadmap, queue metadata, decisions/receipts, round log, cross-references) already exists in fragments; the prose payloads (Step Details bodies, motivations, per-round commentary) are the parts that plausibly stay prose linked from that skeleton.
