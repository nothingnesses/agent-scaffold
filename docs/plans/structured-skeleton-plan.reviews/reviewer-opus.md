# Plan review: structured-skeleton six-increment fold (correctness / coherence / sequencing lens)

Reviewer: opus (independent plan reviewer). Scope: the uncommitted edit to `docs/plans/agent-scaffold.md` on branch `plan/structured-skeleton` (the `structured-skeleton` Step Detail rewrite, the six-increment list, and the six added Success Criteria), checked against `synthesis.md`, the decided `Q-45`/`Q-46` queue entries, `design-B-rich-schema.md`, and `src/workflow.rs`.

SUMMARY: NOT CLEAN. One HIGH internal contradiction (options/chosen duplicated into the TOML against decided sub-question 3(c)), three MEDIUM sequencing/scoping defects, three LOW under-specifications, two nits. The core Q-45 + Q-46 fold (waivers/baseline MOVE to TOML, JSONL events-only, prune-at-cutover, cross-substrate W5 join, render/warn-local-fail-CI, principles structured / success-criteria prose) is realized correctly, and the Inc 4 primary-gate correctly avoids a live-enforcement window. Findings below.

---

## Real defects

### PO1 (HIGH) - Inc 1 schema + Inc 1 acceptance + Success Criteria: `options`/`chosen` duplicated into the TOML, contradicting decided sub-question 3(c) and the plan's own resolution line

The plan's own structured-skeleton block states the resolved sub-question verbatim: "queue `options`/`chosen` stay in the JSONL receipt". Synthesis 3(c) decided this unambiguously: "Do the queue `options`/`chosen` live in the TOML ... or only in the JSONL decision receipt? JSONL RECEIPT ONLY", reasoning that copying them into the TOML "recreates a two-homes-plus-cross-check pattern the audit was shrinking".

But three places in the fold put them back into the TOML:
- Inc 1 schema: "`[[question]]` with `id`/`status`/`ask`/`options`/`chosen`/`folded_into`/`superseded_by`/`receipt`" (lists `options` and `chosen`).
- Inc 1 acceptance: "`validate --source` flags ... a `chosen` outside `options`" (a check that only exists if both fields live in the TOML).
- Success Criteria bullet: "the Open-Questions queue (id, status, one-line ask, options, choice, receipt pointer, ...)".

This is a direct internal contradiction. Worse, Inc 4 does NOT add a W4 cross-check that the TOML `options`/`chosen` agree with the JSONL receipt (Inc 4 only lists the decided-gate and the baseline read), so the fold would create duplicated `options`/`chosen` in two homes with NO integrity check tying them together, exactly the anti-pattern 3(c) rejected.

Why it matters: this misdirects the schema, the validator, and the migration into building and populating fields the decided design says do not exist, and it re-introduces an unchecked two-homes duplication for the queue's decision data.

Suggested fix: drop `options` and `chosen` from the `[[question]]` schema; drop "a `chosen` outside `options`" from the Inc 1 acceptance (validate `receipt` resolves instead); drop "options, choice" from the Success Criteria queue bullet. Render sources the displayed options/choice from the JSONL `decision` receipt keyed by the question's `receipt`, matching design-B's rendered queue without a TOML copy. If the planner instead intends to override 3(c) for render-locality, say so explicitly, revise the "stay in the JSONL receipt" resolution line, AND add the W4 receipt-vs-TOML cross-check so the duplication is at least integrity-checked; but the default should be the decided JSONL-only form.

### PO2 (MEDIUM) - Inc 4: "W3 joins rounds by the structured increment id" presupposes Inc 2, but Inc 4 depends only on Inc 1

Inc 4 says "W3 joins rounds by the structured increment id and reads `[[step.waiver]]`" and declares "DEPENDS ON: Inc 1" with Inc 2 as a parallel track. The structured increment id on `round` records is exactly what Inc 2 adds (with `leading_slug` fallback). Synthesis section 4's Inc 4 did NOT claim a structured-id join; the planner added that phrasing, creating the tension. If Inc 4 lands before Inc 2 (allowed by the declared graph), rounds carry no structured id and W3 must join by the `round.task` string against the declared `[[step.increment]].id` (which works via string equality for pre-migration rounds).

Why it matters: the described join mechanism and the declared dependency disagree; a builder could take Inc 4 first (per "Inc 2 parallel") and find the "structured increment id" it references does not yet exist on round records.

Suggested fix: either add Inc 2 as a hard dependency of Inc 4, or reword Inc 4 to "joins rounds to the declared `[[step.increment]].id` (by `round.task` equality; Inc 2's structured id is the orthogonal replacement for the `leading_slug` strip)".

### PO3 (MEDIUM) - Inc 5: "risky-but-reversible" has no mapping to the binary W3 `risk_class`, so its convergence bar and its round-record class are unspecified

The fold says "risky increments need two consecutive clean rounds, low ones need one", but Inc 5 is classified "risky-but-reversible", a planning label with no member in the binary W3 `risk_class` set (`low_risk` / `risky`) that round records carry and W3 enforces (risky requires peak `consecutive_clean >= 2`). So it is unstated whether Inc 5's rounds are logged `risky` (two clean rounds) or `low_risk` (one), and W3 has no third bucket to encode "reversible".

Why it matters: the implementer cannot log Inc 5's round records or know its done-condition without inventing the mapping; the convergence gate for the single most-consequential increment (the live cutover) is ambiguous.

Suggested fix: state that Inc 5's round records are logged `risky` (two clean rounds required), with "reversible" describing only the blast-radius/rollback story (single revertible commit), not a lighter convergence bar; or explicitly justify logging it `low_risk` and reconcile with the "cuts over the LIVE plan" framing.

### PO4 (MEDIUM) - Inc 4/Inc 5 acceptance covers only the `optional-modules` record-backed waiver, not the `waiver-model` self-waiver

There are two record-backed `accepted-at-escalation` waivers in the live JSONL (verified: line 105 `optional-modules-inc2cii`, joining escalation line 82; line 112 the `waiver-model` self-waiver on increment `waiver-model`, joining escalation line 111). Both are `unit:increment` and join by string equality (`waiver.increment == escalation.task`), so both survive the substrate split. Inc 4's acceptance names only "the `optional-modules` accepted-at-escalation record-backed waiver ... reading the TOML waiver joined to the JSONL escalation". The `waiver-model` self-waiver is the second cross-substrate case and is the self-referential dogfooding one (a step waiving its own increment via its own escalation). Inc 5 migrates all 16 waivers generically but neither Inc 4 nor Inc 5 acceptance explicitly exercises the `waiver-model` cross-substrate join.

Why it matters: the two record-backed waivers are the only cases the new cross-substrate join actually runs on in the live repo; leaving the self-waiver out of the acceptance means the most self-referential join is unverified by a stated check.

Suggested fix: extend Inc 4's acceptance to exercise BOTH record-backed waivers (add the `waiver-model` self-waiver on increment `waiver-model` joining escalation `task:"waiver-model"`), and add to Inc 5's acceptance that after cutover W5 passes for both migrated record-backed `[[step.waiver]]` entries reading their evidence from the retained JSONL escalations.

## Under-specifications (LOW)

### PO5 (LOW) - Inc 6 hard-depends on Inc 4, not just Inc 3

Inc 6's acceptance is "a fresh `agent-scaffold` run ... and `validate` passes on it" for a template that sets `primary = "toml"` and has no Markdown plan. `validate --workflow` on a `primary=toml`, no-Markdown project requires Inc 4's TOML-reading checks; Inc 3 (render) alone cannot make `validate` pass. Inc 6 declares "DEPENDS ON: Inc 3 (render); sensibly follows Inc 5". In practice "sensibly follows Inc 5" (which is after Inc 4) covers it, but the stated hard dependency understates it.

Suggested fix: list Inc 4 as a hard dependency of Inc 6 (in addition to Inc 3), keeping "follows Inc 5" for the final-schema reasoning.

### PO6 (LOW) - Inc 1's enumerated `[meta]` fields omit the render's sidecar references and hash field

Inc 1 enumerates `[meta]` as `title`/`w4_baseline`/`primary`/the orphan-`task` list. Inc 3's render must locate the front/tail prose sidecars (intro/motivations/repo-layout, and the Success-Criteria prose sidecar) and, if the check uses the `[meta]`-hash mechanism (Inc 3 offers "byte/golden or `[meta]` hash"), a render-hash field. None of these appear in the Inc 1 schema. Either the front/tail sidecars are convention-named (state it, as steps/questions are) and the check is byte/golden (no hash field), or Inc 1's "full schema" is incomplete and Inc 3 silently extends it.

Suggested fix: in Inc 1, state that the front/tail prose sidecars are convention-named paths (no `[meta]` refs) OR add their `[meta]` ref fields; pick the check mechanism so it is clear whether a `[meta]` hash field belongs in the Inc 1 schema.

### PO7 (LOW) - `[meta].primary` default / no-TOML behavior unstated

Inc 4 gates on "`[meta].primary == "toml"` (else the Markdown ... fallback)". During Inc 1-4 the live repo has no `.plan.toml` at all, so there is no `[meta]` to read; the gate relies on "no TOML present routes to Markdown", which is never stated. Inc 1 introduces `primary` as a schema field but does not give its default or the absent-file semantics.

Suggested fix: state in Inc 1/Inc 4 that the checks read the TOML source only when a `.plan.toml` exists AND `[meta].primary == "toml"`; otherwise the Markdown + JSONL path, so the live repo (no TOML) stays on Markdown until Inc 5.

## Nits

- N1: Inc 5 says "~51 step bodies"; the live plan has ~49-55 Step Detail blocks (design-B counts ~55). Approximate and harmless; matches synthesis's own "~51".
- N2: Synthesis 3(f) noted `[meta].primary` "can be retired after cutover once the `.md` parsers are removed". The fold keeps the Markdown fallback (Inc 4) and the scaffold template permanently sets `primary = "toml"` (Inc 6), so `primary` persists and the `.md` parsers are never retired. Not a contradiction (the plan never claims to remove them), but the post-cutover fate of the Markdown parsers / the `primary` bit is a loose end worth a sentence.

---

## What I verified is correct

- Q-46 realized faithfully: waivers MOVE to `[[step.waiver]]` and the baseline to `[meta].w4_baseline`; the JSONL becomes events-only (`round`/`escalation`/`decision`/`intake`/`dismissal`); the 16 historical `waiver` lines + 1 `baseline` line are pruned at the one-time cutover (verified counts against `docs/metrics/workflow.jsonl`: 16 waivers lines 91-105 + 112, 1 baseline line 86). The plan follows Q-46 over synthesis's KEEP recommendation and correctly DROPS synthesis invariant 5's "append-only ... no line rewritten" claim rather than falsely asserting it.
- Inc 2 correctly EXCLUDES waivers from the structured-id JSONL field (they move to the TOML), a correct Q-46 adaptation of synthesis Inc 2 (which still added the field to waivers).
- Cross-substrate W5 join specified in Inc 4 (TOML `[[step.waiver]]` -> immutable JSONL `escalation`); both live record-backed waivers are `unit:increment` joining by `waiver.increment == escalation.task` string equality (verified in `w5_problems`, lines 401-405), so the join survives the split; escalations stay in the JSONL and are read by W5 regardless of `primary`.
- The Inc 4 `primary` gate genuinely avoids a live-enforcement window: the live repo stays `primary=markdown` + JSONL-waivers until Inc 5's single atomic cutover commit, which simultaneously sets `primary=toml`, generates the TOML with waivers/baseline, renders, and prunes the now-unread JSONL waiver/baseline lines; nothing reads the pruned lines once `primary=toml`; the cutover is one `git revert`-able commit. `primary` is introduced in the Inc 1 schema and consumed in Inc 4, consistently.
- The `-inc<x>` self-reference is coherent: `structured-skeleton-inc<x>` strips via `leading_slug` to `structured-skeleton` (verified `leading_slug` strips `-inc<alnum>`), so per-increment rounds join to the umbrella and the pause.md catch is satisfied once increments have rounds; Inc 1/Inc 2's own rounds predate Inc 2's field and use the strip, while later increments dogfood the new structured id. The one-umbrella-step decision matches the `optional-modules` precedent.
- Principles structured `[[principle]]`, Success Criteria kept PROSE: Inc 1's schema correctly omits `[[success_criterion]]`/`satisfied_by` (per synthesis 3(b)), unlike design-B section 2.6.
- Risk classes match synthesis section 4 (Inc 1/2 low, Inc 3/4/6 risky, Inc 5 risky-but-reversible), and Inc 4 is correctly flagged as a larger risky surface than synthesis's keep-in-JSONL variant because Q-46 adds the waiver/baseline swap.
- Dependency order Inc 1 -> Inc 3 -> Inc 4 -> Inc 5 with Inc 2 parallel and Inc 6 after Inc 3/Inc 5 is sound (modulo PO2 and PO5); Inc 5 correctly hard-depends on both Inc 3 (shadow render) and Inc 4 (TOML-sourced checks at cutover); Inc 6 sensibly follows Inc 5 for the final schema.
- Render invariants preserved: strict failure writes nothing on a broken source, do-not-edit banner names the real sources, status line + status vocabulary generated (B3), `render --check` warn-local / fail-CI (Q-45(d)); no prose round-trip (opaque sidecar splice). W4's decided-gate becomes `status == "decided"` reading `folded_into` and reads the cutoff from `[meta].w4_baseline`, matching synthesis section 1.
- Acceptance checks are concrete and verifiable for every increment (parse round-trip, injected-fault flags, byte-stable render, `render --check` failure modes, post-cutover green + `git revert` restore, fresh-scaffold `validate` green + no `w4_baseline`).
