# Design notes: `workflow-invariants` W3, trivial declaration + grandfather boundary (explorer A)

Scope: the two coupled sub-decisions owed on `Q-27` / `workflow-invariants`. W3 checks that every Roadmap step marked `complete` has, in `docs/metrics/workflow.jsonl`, `round` records whose `task` leading-slug equals the step slug and whose per-artifact consecutive-clean streak reaches the class count (`low_risk` 1, `risky` 2). A `complete` step with no matching records is the `pause.md` catch, UNLESS it is a declared trivial completion (Decision A) or falls before the grandfather boundary (Decision B). Both decisions are exemptions from the same base rule, so they are designed together (see section 4).

References read: plan `workflow-invariants` step detail, `Q-27`/`Q-34` open questions, Project Principles, `src/plan.rs` (status vocabulary), `src/metrics.rs` (round schema), and the full `docs/metrics/workflow.jsonl` (60 records). Principle numbers below are from the plan's "## Project Principles" (P1 cleaner-long-term, P2 minimal, P3 safe-on-existing, P4 idempotent, P5 illegal-states-unrepresentable, P6 evidence, P7 reproducible) plus P16 single-source-of-truth (cited by `plan.rs` and `metrics.rs` in-code and by `Q-34`).

## Decision A: how a trivial (review-skipped) completion is DECLARED

| Option | How it records triviality | Pros | Cons | Principles |
| --- | --- | --- | --- | --- |
| A1. `trivial` Roadmap status (add to `ROADMAP_STATUSES` in `plan.rs`) | Step's status cell reads `trivial` instead of `complete`; a distinct terminal status meaning "done, authorized skip of the review loop". W3: `trivial` step requires no round records. | Step status stays single-sourced in the plan (P16); no fabricated round record for a review that never ran; the only legitimate way to be "done without rounds" is to say so, so a bare `complete` with no rounds past the cutoff is unambiguously the catch (P5); one enum entry (P2), exactly the precedent `exploration-mode` set when it added `exploring` as "one code line"; human authorizes by writing the status, visible and reviewable in the plan diff; `skipped` already exists as a sibling non-`complete` terminal, so the vocabulary shape is established. | Any consumer that currently means "done" by matching `complete` must also accept `trivial` (small ripple; `status --resume` / `next` / any progress count). Folds two facts (done + review-skipped) into one cell, but status cells are already single-fact terminal markers, so this is consistent, not a regression. | P16, P5, P2, P1 |
| A2. Distinct JSONL record `type: "trivial_completion"` | A non-round line in the round log naming the step. | Declaration sits in the same file W3 reads. | The JSONL is a per-round review log; a non-round record pollutes its schema and forces a new type + validator branch in `metrics.rs`; step status now lives in two files (plan says `complete`, log says trivial) -> drift (violates P16); human authorizes by hand-editing JSONL (least reviewable, most error-prone). | Against P16, P2 |
| A3. `trivial`-flavored `round` record (new outcome/flag on a `round`) | A round line with e.g. `outcome:"trivial"`. | Reuses the round type. | It is a fake round: no review happened, exactly what the orchestrator wants to avoid; corrupts convergence accounting and the clean/new_valid streak semantics; needs a new `RoundOutcome` variant in `metrics.rs`. | Against P5, P16 |
| A4. Separate declarations file (e.g. `docs/plans/trivial-completions.txt`) | A list of trivially-completed slugs the parser reads. | Keeps the round log clean. | A new artifact + new parser (against P2); step done-ness now split across plan + declarations file -> drift (P16); no gain over putting it in the plan. | Against P2, P16 |
| A5. Plan-side prose annotation (a note in the Step Detail, not a status) | Free-text "completed as trivial" in the detail block. | No status-vocabulary change. | Prose is not reliably machine-checkable; this is the exact failure `Q-29` documented (nuance that lived only in body prose could not be validated, so a structured status was added). W3 could not check it deterministically. | Against P5 |

Recommendation: A1, a `trivial` Roadmap status. Add `"trivial"` to `ROADMAP_STATUSES` in `src/plan.rs` as a terminal status parallel to `complete` and `skipped`, meaning "completed as an authorized trivial change, review loop deliberately skipped". W3 iterates `complete` steps and requires their rounds; a `trivial` step is exempt by declaration and W3 skips the round requirement for it. The human authorizes triviality by writing `trivial` in the Roadmap cell, which turns a silent skip into a visible, diff-reviewable one, which is exactly the strictness resolution `Q-27` settled on ("trivial changes are not forced through review, they are forced to be DECLARED trivial; the check confirms the declaration").

Strongest reason: it keeps step status single-sourced in the plan (P16) and makes the illegal state unrepresentable (P5), because the only way to be legitimately done-without-rounds is the `trivial` status; a `complete` with no rounds (past the cutoff) can then be flagged with no false-positive ambiguity. Every alternative either duplicates step status into a second file (drift) or writes a record for a review that never happened.

Implementation note for the builder: A1 requires deciding whether downstream "is this step done" checks match a set `{complete, trivial, skipped}`. Recommend a single `is_terminal`/`is_done` helper in `plan.rs` so the done-set is defined once and cannot drift (P16). W3 specifically keys off `complete` (must have rounds) vs `trivial` (need not), so those two must stay distinct values, not be collapsed.

## Decision B: the GRANDFATHER boundary for pre-logging complete steps

The base rule flags a `complete` step when it has no rounds (b1) or has rounds that never reach the required streak (b2). Both b1 and b2 are historical artifacts of pre-discipline logging and must be exempt, while a real future skip must still fail. Critical constraint from the data (section 3): b1 (no records) is indistinguishable from the `pause.md` skip by record-presence alone (both have zero records), so a rule that keys only on record presence/streak CANNOT tell a legitimate pre-logging step from a future illegal skip. A durable boundary is unavoidable.

| Option | How the exemption is expressed | Pros | Cons | Principles |
| --- | --- | --- | --- | --- |
| B1. Forward epoch cutoff at the migration frontier: a single named boundary slug in the plan; W3 governs Roadmap steps from that slug onward, everything earlier is grandfathered | One slug (`round-log-core`) parsed from the plan; "before" / "after" is by Roadmap row order, which `plan.rs` already reads in order. | Exempts ALL of b1 and b2 without enumerating them (they are all earlier than the frontier); cannot grow (one fixed slug); catches every future skip (frontier step and everything after must comply); minimal (P2) and cleanest long-term (P1); single source (P16); the frontier step (`round-log-core`) itself passes W3, so the check has one real historical step it validates, proving it works (P6). | Gives up retroactive validation of the genuinely-converged historical steps (they are never re-checked by W3). Acceptable: they are frozen, and `metrics.rs` independently checks each record's well-formedness and clean/new_valid streak monotonicity. | P2, P1, P16, P5 |
| B2. Cutoff at "the first fully-logged task" (the plan's other sketch) | Boundary set at the first task that reaches a streak (`workflow-doc-fixes`). | Same single-slug shape as B1. | WRONG on the data: logging discipline was not monotonic. `pack-rebuild-tracking` and `user-prompts-dir` are b2 stragglers that sit AFTER `workflow-doc-fixes` in Roadmap order and after converged neighbours (`triager-independence`, `file-safety-rules`, `agent-isolation`), so a cutoff there fails to exempt them. Moving the cutoff later to cover them would exempt the converged neighbours too and miss a real skip among them. No single early cutoff separates b2 from converged. | Fails correctness |
| B3. Pure streak-presence rule ("flag only if records exist AND at least one reaches the required streak; else exempt") | No boundary; decide purely from the records. | No plan marker needed. | Cannot distinguish b1 (no records, legitimate) from the `pause.md` skip (no records, illegal), which is the whole reason W3 exists; and a future step that logs a couple of `new_valid` rounds then stops (never converged) would be exempted identically to b2, so a real skip slips through. | Fails correctness (defeats the catch) |
| B4. Explicit closed list of every grandfathered slug (b1 + b2) in the plan | A parsed list section enumerating ~14 historical slugs. | Correct (exempts exactly the historical set); still catches future skips; closed at migration, does not grow; lets W3 re-check the converged historical steps. | Larger drift surface (14 hand-listed slugs vs one), more to keep in sync with the Roadmap, more to review; buys only retroactive re-validation of already-frozen steps, which is low marginal value and partly covered by `metrics.rs`. Heavier than B1 for little gain (against P2). | P5, P16, but heavier than P2 |
| B5. Per-step `logged_from` marker on each governed step | Mark every step that must comply. | Explicit per step. | The exemption/obligation set grows forever: every future step needs the marker. This is the maintainability failure the prompt warns about. A forward boundary marks once and everything after inherits. | Against P2, P4-spirit |

Recommendation: B1, a single forward epoch cutoff, boundary = `round-log-core` (inclusive). Express it as one durable marker in the plan (the parser reads a named boundary slug, e.g. a one-line "W3 enforces from `round-log-core` onward; earlier complete steps predate disciplined round-logging and are grandfathered" that `plan.rs` exposes, keyed to the Roadmap row index). W3 logic per `complete` step S:

1. If S.status is `trivial` -> pass (Decision A).
2. Else if S is before the boundary row -> pass (grandfathered: covers both b1 and b2).
3. Else require matching rounds with a consistent `risk_class` and a per-artifact streak reaching the class count; fail otherwise (this is the `pause.md` catch and the future-skip catch).

Why `round-log-core` as the boundary and not "first fully-logged task": disciplined logging became mandatory AT `round-log-core` (the `Q-34` migration that promoted the JSONL to the always-present round log). Placing the cutoff at that frontier, rather than at the first converged task (B2), is what makes the cutoff correct: all six b2 stragglers and all eleven b1 steps are strictly earlier in the Roadmap than `round-log-core`, so they are exempted wholesale, while `round-log-core` (which genuinely converged, `incA` reaches 1 and `incB` reaches 2) and everything after it must comply. The interleaving that kills B2 is entirely below the frontier, so it never bites.

Strongest reason: it is the only expression that provably exempts both b1 and b2 with a single, non-growing marker while still catching every future skip; the interleaved-discipline data (section 3) rules out every earlier cutoff and rules out any records-only rule.

## 3. Data appendix (grounds Decision B)

Source: `docs/metrics/workflow.jsonl`, 60 `round` records. Streak per leading-slug = max `consecutive_clean` seen for that slug (leading-slug = `task` with a trailing `-inc<n>` stripped). Parser caveat for the builder: increment tokens are alphanumeric in this log (`-inc1`/`-inc2`/`-inc3` for `state-schema`, but `-incA`/`-incB` for `round-log-core`), so the strip must be `-inc<alnum>+`, not `-inc<digit>+`.

Max consecutive_clean by leading-slug, in first-seen log order (required streak: `low_risk` 1, `risky` 2):

```
 0  workflow-hardening        b2  (only new_valid rounds; not in current Roadmap)
 0  convergence-accounting    b2  (Roadmap: complete)
 0  plan-maintenance          b2  (not in current Roadmap)
 1  workflow-doc-fixes        converged (low_risk)
 0  pack-rebuild-tracking     b2  (Roadmap: complete)
 0  consolidate-plan          b2  (not in current Roadmap)
 1  triager-independence      converged
 1  file-safety-rules         converged
 1  agent-isolation           converged
 0  user-prompts-dir          b2  (Roadmap: complete)
 1  human-onboarding          converged
 1  gate-prompt-clarity       converged
 1  compaction-prep           converged
 2  deliberation-mode         converged (risky)
 1  human-review-queue        converged
 2  no-wrap-convention        converged (risky)
 1  findings-files            converged
 1  ledger-template           converged
 1  instrument-flag           converged
 1  state-schema              converged
 1  optional-modules          converged, but Roadmap status is `in progress` -> W3 does not check it
 1  metrics-fields            converged (orphan: not a current Roadmap slug)
 1  exploration-mode          converged
 1  session-preflight         converged
 2  round-log-core            converged (incA reaches 1, incB reaches 2)
```

(b2) the six tasks whose rounds never reach `consecutive_clean:1`: `workflow-hardening`, `convergence-accounting`, `plan-maintenance`, `pack-rebuild-tracking`, `consolidate-plan`, `user-prompts-dir`. This matches the six named in the plan exactly. What distinguishes them from properly-converged tasks: they have ONLY `new_valid` rounds logged (no `clean` round was ever recorded), because they converged informally before per-round logging was disciplined; the terminating clean round exists in reality but never made it into the log. It is not that they failed review; it is that the converging round was not written.

(b1) complete Roadmap steps with NO log records at all (the contiguous earliest prefix, predating logging): `core-assets`, `file-dropper`, `idempotency-safety`, `selection-ui`, `mode-enum`, `tag-selection`, `available-filter`, `pack-manifest`, `external-packs`, `pack-owned-principles`, `init-vcs` (11 steps). (`include-all-visible` is `skipped`, not `complete`, so W3 ignores it regardless.)

Interleaving finding (the decisive one for B): among Roadmap-complete steps, the b2 stragglers are NOT a clean prefix. In Roadmap order the sequence around the boundary is `... init-vcs, convergence-accounting(b2), workflow-doc-fixes(converged), pack-rebuild-tracking(b2), triager-independence(converged), file-safety-rules(converged), agent-isolation(converged), user-prompts-dir(b2), human-onboarding(converged) ...`. So `pack-rebuild-tracking` and `user-prompts-dir` are b2 but sit AFTER converged steps. Any cutoff placed at the first converged task (`workflow-doc-fixes`, option B2) leaves them flagged; any cutoff late enough to exempt them also exempts genuinely-converged neighbours. Only a boundary at the migration frontier (`round-log-core`, second-to-last complete step) sits above every b2/b1 straggler, which is why B1 works and B2 does not.

Orphan note: `workflow-hardening`, `plan-maintenance`, `consolidate-plan`, `metrics-fields` appear in the log but are not current Roadmap slugs (folded/consolidated away). W3 iterates Roadmap `complete` steps, so these orphans are simply not visited. A separate optional lint ("log task with no Roadmap step") could catch drift, but it is out of W3's scope.

## 4. Interaction between A and B

A (`trivial` status) and B (grandfather cutoff) are two exemptions from the same base rule, and they must stay SEPARATE channels:

- They mean different things. `trivial` = a forward-looking, per-step human decision that a specific step's change was low enough stakes to skip review; it can legitimately recur for future steps. Grandfather = a one-time historical fact that a step predates disciplined logging; it is closed and never applies to new work. The plan is explicit that b1/b2 steps must NOT be mislabelled `trivial`. B1's forward cutoff satisfies this automatically: grandfathered steps keep their real `complete` status and are exempt by the boundary, not by pretending they were trivial. So choosing B1 keeps A's `trivial` semantically pure (only genuinely-trivial future steps ever carry it).
- Check ordering matters and is clean: W3 checks `trivial` first (status-driven, any position), then the grandfather boundary (position-driven), then the round requirement. A `trivial` step after the boundary is fine (exempt by status); a `complete` step before the boundary is fine (exempt by cutoff) without needing a status change; a `complete` step after the boundary with no/short rounds and no `trivial` status is the catch. The two exemptions do not overlap or contend.
- One shared consumer concern: both A and B feed the same W3 iteration over "steps that claim to be done". Define the done-set (`complete` + `trivial`, and how `skipped` is treated) and the boundary once in `plan.rs`, so A's status extension and B's boundary marker are each single-sourced (P16) and W3 reads them rather than re-deriving.

## Riskiest assumptions

- Decision B: that Roadmap ROW ORDER is a faithful proxy for COMPLETION order, so "earlier than the `round-log-core` row" reliably means "completed before disciplined logging". The log has no per-step completion timestamp the parser can key on, so the boundary is expressed positionally. If a future step were inserted into the Roadmap at an early row but completed late, the positional cutoff would wrongly grandfather it. Mitigant: the plan is append-mostly and the boundary sits at a fixed historical row; new work is appended after it. This is the load-bearing assumption of B1.
- Decision A: that the human/orchestrator reliably writes `trivial` (not `complete`) for authorized skips. That is the same discipline the whole scheme rests on, and making it a visible, diff-reviewable status is precisely the mitigation.
