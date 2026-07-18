# Plan review: structured-skeleton fold (reviewer-sonnet)

Summary: 2 defects, 1 low finding, 2 nits. Both validates exit 0. ASCII clean, no prose hard-wrap, diff confined to `docs/plans/agent-scaffold.md`. The umbrella-with-increments structure is coherent and the Q-45/Q-46 provenance is preserved. The primary defect is an internal contradiction between the stated sub-question resolution and the Inc 1 schema; the secondary defect is a stale Status narrative.

---

## Defects

### PS1 (medium) - options/chosen: claimed JSONL-only but TOML schema contradicts this

Section: `structured-skeleton` umbrella + Inc 1 schema + Inc 1 acceptance test.

Defect: The umbrella paragraph states "Smaller sub-questions resolved on the synthesis recommendations (... queue `options`/`chosen` stay in the JSONL receipt ...)". This records synthesis 3(c) as resolved: JSONL receipt only, not in the TOML. But Inc 1 then lists `[[question]]` fields as including `options`/`chosen`/`receipt`, and its `validate --source` acceptance test checks "a `chosen` outside `options`" - a cross-field constraint that only exists if both fields live in the TOML. These two claims are mutually exclusive.

Why it matters: an implementer reading the umbrella will omit `options`/`chosen` from the TOML schema; one reading Inc 1 will include them. The resulting schemas are incompatible. The validate --source cross-field check is unimplementable if options/chosen are JSONL-only.

Root cause: the planner chose to honor synthesis 3(c) in the umbrella text but silently included the full design-B `[[question]]` fields (which design-B does include options/chosen) in Inc 1. These represent two different choices and neither was made explicit.

Fix: pick one path and make both places agree.

- Option A (JSONL receipt only, as 3(c) says): remove `options`/`chosen` from the Inc 1 `[[question]]` schema and drop the "chosen outside options" acceptance check from `validate --source` (it becomes a `validate --workflow` concern, crossing to the JSONL receipt via W4). The umbrella text is then correct as written.
- Option B (in TOML per full design-B): revise the umbrella sub-question-resolution sentence to say "queue `options`/`chosen` are ALSO structured in `[[question]]` (overriding synthesis 3(c), since the human's full design-B choice carries this)" and leave Inc 1 as-is.

Option A is more consistent with the stated 3(c) resolution and keeps the TOML schema minimal (a separate `receipt` pointer field in `[[question]]` still lets render display a link). Option B requires explaining that the human's design-B choice overrides 3(c), which is a legitimate position but must be stated.

---

### PS2 (low) - Status narrative line still says planning pass is "OWED"

Section: document Status line (line 3 of the plan, the "Status: in progress..." paragraph).

Defect: The diff updates the Step Detail to say "the planning pass is DONE (2026-07-18)" but does not touch the Status narrative (line 3), which still says "it OWES a PLANNING PASS (turn `target-arch-B-cleanslate.md` into a staged, reviewed Roadmap) before any build." The two sections now give contradictory information about whether the planning pass has been completed.

Why it matters: the Status line is a resume anchor. An implementer resuming from line 3 alone would be misled into thinking the planning pass is still needed; one reading the Step Detail would see it is done. The plan's own Documentation Protocol says the Status line and the Step Details must stay in sync.

Fix: update the Status narrative's `NEXT:` fragment to say the planning pass is now done and the next step is the per-increment review loop, mirroring the new Step Detail's opening sentence.

---

## Low finding

### PS3 (low) - Synthesis 3(f) Status-narrative sidecar resolution not reflected in Inc 5

Section: Inc 5 acceptance test / expected diffs.

Defect: Synthesis 3(f) includes a specific resolution: "Preserve the current ~1,200-word hand-authored Status prose as a sidecar during migration (`_status-narrative.md`): PRESERVE during migration, prune later." The umbrella paragraph does not list this among the sub-questions resolved on synthesis recommendations (its list stops at "synthetic-pilot-first before the live plan"). Inc 5 mentions "the meta prose" as part of the sidecar split, which may implicitly cover this, but does not name the Status narrative explicitly, does not call out a `_status-narrative.md` (or equivalent) sidecar, and does not list it among the expected fidelity-diff items.

Why it matters: the Status narrative is the document's prose resume anchor and the largest non-step, non-question block (~1,200 words). If the prose-split script omits it or the migration plan treats it as part of the generated Status line (which it is NOT - the derived Status line is a step-distribution summary, not this editorial narrative), a data-loss slip would occur at the one stage the plan itself describes as "the single place a data-loss slip would hide."

Fix: name the Status narrative sidecar explicitly in the umbrella's sub-question-resolution list and add it to Inc 5's expected-fidelity-diff items (or explicitly note that "meta prose" covers it and name the sidecar path). If it is intentionally pruned at migration rather than preserved, say so and explain why diverging from the synthesis recommendation.

---

## Nits

### PS4 (nit) - "per Q-45(d)" is a misleading citation

Section: Inc 3 prose, "warn-local, fail-CI per Q-45(d)".

Q-45 is the skeleton-depth question (B vs C vs A), a single-item human decision with no lettered sub-parts. The "(d)" refers to synthesis section 3(d) (the render-check severity sub-question). Writing "Q-45(d)" invents a notation that does not exist in the Open Questions queue and suggests Q-45 has sub-parts it does not.

Fix: write "warn-local, fail-CI per the synthesis sub-question resolution (section 3(d))" or simply drop the citation since the umbrella already records this resolution.

---

### PS5 (nit) - Inc 2 has no corresponding Success Criterion

Section: Success Criteria additions.

Each of Inc 1, Inc 3, Inc 4, Inc 5, and Inc 6 maps to at least one new Success Criterion. Inc 2 (the structured step/increment link on JSONL records, retiring the lexical `-inc<x>` strip, closing SE-10/B6) has no matching SC. The other five increments' combined SCs do not cover "new `round`/`escalation` records carry a structured step/increment id, and W3/W5 join on it for post-migration records," which is Inc 2's observable, verifiable outcome.

Fix: add a SC for Inc 2 along the lines of "New round and escalation records carry a structured step/increment id; W3 and W5 use it in preference to the lexical strip for post-migration records, so the over-strip fragility (SE-10/B6) no longer applies to newly logged increments, and pre-migration records continue to join via the leading-slug shim." Whether the project considers this worth its own SC is a judgment call - noting it here as a nit rather than a defect.

---

## Verified correct

- `validate --plan` exits 0 on the worktree plan.
- `validate --workflow` exits 0 on the worktree plan + JSONL.
- Diff is confined to `docs/plans/agent-scaffold.md` (single file, confirmed with `git diff --name-only`).
- No em-dashes, en-dashes, unicode symbols, or emoji in the new text. ASCII arrows (`->`) used throughout.
- Prose not hard-wrapped; new paragraphs are single long lines matching the existing plan style.
- Roadmap table shows `structured-skeleton` as `next`, consistent with the new Step Detail ("the umbrella stays `next` until an increment starts").
- Q-45 entry correctly shows `decided -> folded into structured-skeleton`; Q-46 entry likewise.
- Q-45/Q-46 rationale is preserved verbatim in the umbrella Step Detail; no decision-history loss.
- The umbrella-with-increments structure matches the `optional-modules` precedent (one Roadmap row, `-inc<x>` task suffixes), and the rationale for this choice is recorded.
- All six increments state a risk class.
- Dependency order is stated and coherent: Inc 1 -> Inc 3 -> Inc 4 -> Inc 5, Inc 2 parallel to Inc 1/Inc 3, Inc 6 after Inc 3 (sensibly after Inc 5).
- The six increments collectively cover schema (Inc 1), structured JSONL link (Inc 2), render engine (Inc 3), enforcement swap including Q-46 waivers/baseline (Inc 4), live migration synthetic-pilot-first (Inc 5), and template/scaffold (Inc 6).
- The append-only break (pruning 16 waiver + 1 baseline JSONL lines at cutover) is explicitly named and cited as sanctioned by Q-46 at this pre-adoption stage.
- The dogfooding recursion is documented: Inc 2 retires the lexical strip, so Inc 3+ of this initiative log under the new structured id.
- Synthesis sub-question resolutions reflected: (a) prune waivers/baseline overridden by Q-46 MOVE decision and correctly reflected; (b) Principles structured as `[[principle]]`, Success Criteria as prose sidecars; (d) render-check warn-local/fail-CI; (e) RESUME STATE stays ledger prose; (f) orphan task slugs declared in `[meta]`, `[meta] primary` cutover bit adopted, synthetic-pilot-first, question-body relocation implied by expected-diff list.
- New Success Criteria (6 additions) match the existing prose-bullet style: one full sentence per bullet, terminates with a period, no hard-wrap.
- Inc 5 acceptance explicitly requires `git revert` of the single cutover commit to restore both the hand-authored `.md` and the pruned JSONL lines.
- Inc 4's cross-substrate W5 join is called out and risk-classified as a larger surface than the synthesis's keep-in-JSONL variant (correctly attributing the added risk to Q-46).
