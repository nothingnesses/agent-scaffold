# Confirming plan review: structured-skeleton fold (fix verification)

Reviewer: confirming reviewer (independent). Scope: the uncommitted edit to `docs/plans/agent-scaffold.md` on branch `plan/structured-skeleton` (worktree `plan-structured-skeleton`), verifying the planner's fixes against the prior findings in `reviewer-opus.md` (PO1-PO7) and `reviewer-sonnet.md` (PS1-PS5), plus a fresh whole-read for new contradictions.

SUMMARY: ONE UNRESOLVED finding (PS2, medium, carried over, out of my assigned verify list but a genuine still-live contradiction). All ELEVEN assigned findings (PO1/PS1, PO2, PO3, PO4, PO5, PO6, PO7, PS3, PS4, PS5) are VERIFIED CLOSED. No NEW contradiction was introduced by the fixes. Both validators exit 0; diff is confined to `docs/plans/agent-scaffold.md` and is ASCII-clean.

---

## Assigned findings: verified closed

- PO1 / PS1 (HIGH) - CLOSED. `options`/`chosen` are gone from the TOML in every location. The skeleton paragraph now reads "the queue's `options`/`chosen` staying in the JSONL decision receipt rather than the TOML"; the sub-question-resolution list reads "queue `options`/`chosen` stay in the JSONL decision receipt (3(c)), where W4 reads them unchanged, and are NOT copied into the TOML"; the Inc 1 `[[question]]` schema lists `id`/`status`/`ask`/`folded_into`/`superseded_by`/`receipt` with an explicit note "the queue's `options`/`chosen` live ONLY in that JSONL receipt, per 3(c), and are NOT carried in the TOML"; the Inc 1 acceptance no longer contains the "`chosen` outside `options`" check; the Success Criteria queue bullet reads "id, status, one-line ask, a `receipt` pointer ... the queue's `options`/`chosen` stay in the JSONL decision receipt, not the TOML". No remaining place puts them in the TOML. Because the fix chose the JSONL-only path (not the duplicate-plus-cross-check path), no W4 receipt-vs-TOML integrity check is needed, and none is claimed. Internally consistent.

- PO2 (MEDIUM) - CLOSED. Inc 4 now reads "W3 joins rounds to the declared `[[step.increment]].id` (by Inc 2's structured increment id when the round record carries it, else the `leading_slug` strip, so Inc 4 uses whatever join exists and does not hard-depend on Inc 2)". DEPENDS ON stays "Inc 1". Join mechanism and declared dependency now agree.

- PO3 (MEDIUM) - CLOSED. Inc 5's Risk line now states "its W3 `risk_class` is `risky` (its round records are logged `risky` and its convergence bar is two consecutive clean rounds); 'reversible' is a descriptive blast-radius property, not a third `risk_class` value." Explicit `risky` mapping with "reversible" demoted to a descriptive note.

- PO4 (MEDIUM) - CLOSED. Both record-backed cross-substrate waivers are now exercised in both increments. Inc 4 acceptance: "BOTH cross-substrate accepted-at-escalation record-backed waivers behave identically ... the `optional-modules-inc2cii` waiver ... and the `waiver-model` self-waiver on increment `waiver-model` (joined to the `waiver-model` escalation, the self-referential dogfooding case)". Inc 5 acceptance: "both record-backed waivers (`optional-modules-inc2cii` and the `waiver-model` self-waiver) are translated into `[[step.waiver]]` entries ... and W5 passes for both across the substrate split".

- PO5 (LOW) - CLOSED. Inc 6 DEPENDS ON now reads "Inc 3 (render) and Inc 4 (the TOML-source `validate` path, since the template sets `primary = "toml"` and needs `validate` to pass on a TOML-sourced project with no Markdown plan); sensibly follows Inc 5 (the final schema)."

- PO6 (LOW) - CLOSED. Inc 1's `[meta]` now enumerates "the front-and-tail prose-sidecar references (the intro/motivations/repo-layout front matter and the Success-Criteria tail prose that render splices)" and "`render_sha256` (where the render hash lives: the digest `render --check` re-computes and compares, Inc 3)". Inc 3's check is consistently phrased "byte/golden or the `[meta].render_sha256` hash".

- PO7 (LOW) - CLOSED. Inc 1 states `primary` "(default `markdown`; when no `.plan.toml` is present, or `primary != "toml"`, the Markdown parser is the source, so the live repo stays Markdown-sourced until Inc 5's cutover)". Inc 4 restates the fallback ("else the Markdown + JSONL-waiver fallback, so the live repo is unaffected until Inc 5").

- PS3 (LOW) - CLOSED. The Status narrative sidecar is named in the sub-question-resolution list ("the ~1,200-word hand-authored Status narrative is preserved verbatim as a `_status-narrative.md` sidecar during migration and pruned later (3(f))") and in Inc 5 twice: once in the prose-split lift and once in the expected-fidelity-diff list ("the `_status-narrative.md` editorial narrative, distinct from the derived Status line, spliced back verbatim and expected to round-trip with no diff").

- PS4 (nit) - CLOSED. Inc 3 now cites "warn-local, fail-CI per synthesis section 3(d)"; the invented "Q-45(d)" notation is gone.

- PS5 (nit) - CLOSED. A new Success Criterion covers Inc 2: "New `round` and `escalation` records carry a structured step/increment id, and W3 and W5 join on it in preference to the lexical `leading_slug` strip for post-migration records, so the over-strip fragility (SE-10/B6) no longer applies to newly logged increments, while pre-migration records continue to join via the leading-slug shim." All six increments now map to at least one SC.

---

## Unresolved finding (carried over, not in the assigned verify list)

### PS2 (MEDIUM) - UNRESOLVED: the planning pass is "DONE" in the Step Detail but still "OWED" in the document Status line and the Q-45 queue entry

The fix touched only the `structured-skeleton` Step Detail block (which now opens "the planning pass is DONE (2026-07-18)") and the Decided-design Success Criteria. It did NOT update:

- The document Status line (line 3, the resume anchor), whose `NEXT:` fragment still reads "the `structured-skeleton` initiative ... it OWES a PLANNING PASS (turn `target-arch-B-cleanslate.md` into a staged, reviewed Roadmap) before any build."
- The Q-45 Open-Questions queue entry (line 125), which still reads "ROUTES TO A PLANNING PASS (owed, not yet run) to turn target-arch-B into a staged, reviewed Roadmap ... before any build."

Both now contradict the Step Detail's "planning pass is DONE". An implementer resuming from the Status line or the queue alone would believe the planning pass is still owed; one reading the Step Detail would see it is done. This is the same contradiction reviewer-sonnet raised as PS2, and it remains open (the diff never touches lines 3 or 125). The validators do not police this prose consistency, so both still exit 0.

Note: PS2 was not in the assigned verify list, so this may be a deliberately deferred out-of-scope item. Flagging it because the assignment asked for a fresh whole-read that does not rubber-stamp, and this is a genuine live internal contradiction in the plan.

Fix: update the Status-line `NEXT:` fragment and the Q-45 queue entry's "ROUTES TO A PLANNING PASS (owed, not yet run)" to say the planning pass is done and the next step is the per-increment build/review loop, mirroring the Step Detail. (If the intent is to keep these edits out of this fix round, say so; but as it stands the document contradicts itself.)

---

## Fresh whole-read: no NEW contradiction from the fixes

- Q-45 clean-slate + Q-46 move-waivers-to-TOML remain faithfully realized: waivers to `[[step.waiver]]`, baseline to `[meta].w4_baseline`, JSONL events-only, prune-at-cutover, cross-substrate W5 join, render warn-local/fail-CI, principles structured / Success Criteria prose. The synthesis sub-question resolutions (3(b),(c),(d),(e),(f)) are all reflected.
- Inc 1 `primary` default `markdown` and Inc 6 template `primary = "toml"` are consistent (new scaffolds are TOML-sourced; the live repo migrates at Inc 5).
- Inc 4's "does not hard-depend on Inc 2" is consistent with its DEPENDS ON Inc 1 and with Inc 2 being parallel.
- Inc 5's "~51 step bodies + ~46 question bodies" now matches the validator's live counts (51 steps, 46 open-questions items).
- All seven added Success Criteria map cleanly to increments 1-6 with no double-count or gap.
- Minor loose end (not a defect, not new): the increment-list intro sentence summarizes Inc 6 as "after Inc 3 (sensibly after Inc 5)" while the Inc 6 bullet (correctly, per the PO5 fix) hard-depends on Inc 3 AND Inc 4. The bullet is authoritative and correct; "after Inc 5" transitively covers Inc 4 (Inc 5 depends on Inc 4). No action needed.

## Mechanical checks

- `cargo run -- validate --plan docs/plans/agent-scaffold.md` -> exit 0 (51 steps, 46 open-questions items, valid).
- `cargo run -- validate --workflow --plan docs/plans/agent-scaffold.md --metrics docs/metrics/workflow.jsonl` -> exit 0 (114 records valid; workflow invariants hold).
- `git diff --name-only` -> `docs/plans/agent-scaffold.md` only (17 insertions, 1 deletion).
- Non-ASCII scan of the diff -> none.
