# Triage: task-entry-regrounding-inc1 (Part A prose discipline)

Triager: independent, read-only. Adjudicates the reviewer findings in
`task-entry-regrounding-inc1-reviewer.md`.
Diff range: main..impl/ter-inc1 (main b949a1c, HEAD a611491).
Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/ter-inc1`.

## Verdict table

| id   | verdict                         | actionable this round? | one-line fix |
|------|---------------------------------|------------------------|--------------|
| I1-1 | VALID                           | yes                    | Delete the four-element parenthetical in the orchestrator pointer so it matches the build plan's shorter gloss. |
| I1-2 | VALID (dangling ref part); INVALID (name-the-section part) | yes | Cite the always-present Open Questions decision and qualify the `type:"decision"` round-log record with "when instrumentation is on". |

---

## I1-1 (reviewer severity: medium)

Verdict: VALID. Actionable this round: yes.

Evidence:

- The pointer at `pack/prompts/orchestrator.md:23` (and the generated
  `.agents/prompts/orchestrator.md:23`) enumerates the brief's four elements:
  "(what it is, why it exists, the cited evidence, and what you are about to
  do)". The same list is the load-bearing definition of the brief's scope in the
  AGENTS.md subsection at `pack/AGENTS.md:104`: "what the task is, why it exists
  (its provenance), the cited evidence, and what it is about to do". These are
  two copies of one scope list, so a later change to the AGENTS.md list (a fifth
  element, a reworded element) leaves the pointer stating a stale scope. That is
  a genuine Principle 8 (one source of truth) second source, not a cosmetic
  overlap: the pointer's whole job is to say "run the discipline in AGENTS.md",
  which does not require it to reproduce what AGENTS.md defines.
- The agreed design prescribed the shorter form. Build plan
  `docs/plans/task-entry-regrounding.build-plan.md:40` gives the gloss as
  "(brief from durable artifacts + go/no-go per the human-input contract, scaled
  to stakes)" with no enumeration, and `:40` also says "referencing the AGENTS.md
  subsection rather than restating it". The implementer went beyond the plan by
  adding the four-element list. So the finding aligns with the decided design,
  not a fresh reviewer preference.
- The reviewer's "self-contradiction" framing (enumerate, then say "rather than
  restating that discipline here") is weaker and somewhat overstated: a pointer
  giving a one-line gloss of what it points to is normal, and "that discipline"
  can be read as the full multi-sentence rule rather than the gloss. The
  substantive defect is the duplicated scope list (the drift source), not the
  rhetorical tension. The finding stands on the duplication alone.

Adversarial check: does removing the enumeration lose necessary meaning? No. The
pointer still names the discipline ("the task-entry re-grounding in `AGENTS.md`"),
its trigger ("before starting each step (and again on resume)"), its sourcing
("from durable artifacts"), and its gate ("go/no-go per the human-input contract,
scaled to stakes"). The four-element scope lives in `pack/AGENTS.md:104`, which
the pointer already directs the reader to. Nothing operational is lost.

Smallest correct fix (edit the pack source, then re-run the dogfood sync recipe
so the generated `.agents/prompts/orchestrator.md` matches): in
`pack/prompts/orchestrator.md:23`, delete the parenthetical
" (what it is, why it exists, the cited evidence, and what you are about to do)".
Resulting clause: "run the task-entry re-grounding in `AGENTS.md`: brief the step
from durable artifacts and push it for a go/no-go per the human-input contract,
scaled to stakes, rather than restating that discipline here." This removes the
second source and, incidentally, the enumerate-then-disclaim tension, in one
deletion.

Severity note: the reviewer's "medium" is arguably high for a prose-gloss drift
risk in guidance (real impact is bounded: a stale scope hint, not a broken code
path). It is best read as low-to-medium. It is fixed now regardless, because the
fix is a one-token deletion that restores the agreed build-plan wording.

---

## I1-2 (reviewer severity: low)

The finding has two sub-claims; they split.

### Sub-claim A: unconditional reference to an instrumentation-conditional artifact

Verdict: VALID. Actionable this round: yes.

Evidence:

- The `type: "decision"` record and its `chosen` field are defined ONLY in the
  instrument section: `pack/instrument.md:9`. Nowhere else in the pack defines
  them (grep for `"decision"` across `pack/` returns only `instrument.md:9` and
  the new subsection at `AGENTS.md:104`).
- The instrument section is emitted via the `{{instrument}}` placeholder at
  `pack/AGENTS.md:114`, which substitutes to empty when `--instrument` is off
  (`src/manifest.rs:318-321`: "a variable that substitutes to empty (for example
  `{{instrument}}` when `--instrument` is off)"). So a non-instrumented scaffold
  renders no round log and no `type: "decision"` record.
- The new subsection at `pack/AGENTS.md:104` sits in the UNCONDITIONAL body
  (before the `{{instrument}}` placeholder at `:114`) and cites "the round log's
  `type: \"decision\"` record with the human's recorded `chosen`" with no
  qualifier. In a non-instrumented scaffold that reference is dangling: the
  section it points to does not exist in the rendered `AGENTS.md`.
- The precedent for such references is explicit and consistent: the other
  unconditional-body mentions of the round log qualify it. `pack/AGENTS.md:61`
  and `:63` both say "When instrumentation is on ... the round log
  (`docs/metrics/workflow.jsonl`)"; `:69` says "When instrumentation is on it
  appends one `round` record". Line 104 omits that qualifier, so it is the
  inconsistent one.
- Severity is correctly low: the dogfood repo IS instrumented
  (`docs/metrics/workflow.jsonl` exists; the generated `AGENTS.md` contains the
  `type: "decision"` spec), so the live reference is accurate today. This is a
  pack-template coherence bug that only bites a non-instrumented consumer, and
  the pack is authored to be scaffolded into both modes.

Note on provenance of the defect: the build plan itself cited the receipt at
`build-plan.md:34` as "the `type:\"decision\"` receipt with the human's `chosen`,
`pack/AGENTS.md:141`" (an instrument-section line). So the implementer followed
the plan; the instrumentation-conditional gap is inherited from the design, not
an implementer deviation. That explains it but does not excuse it: the pack
should be correct for a non-instrumented scaffold, and there is an
always-present durable home for the choice, namely the resolved decision recorded
in the plan's Open Questions section (`pack/AGENTS.md:41`: "A resolved decision
is recorded in the plan's Open Questions section and folded into the step it
affects").

Smallest correct fix (edit `pack/AGENTS.md:104`; then re-render/sync the
generated `AGENTS.md` and `.agents/AGENTS.reference.md`): make the primary
citation the always-present artifact and qualify the round-log record. Replace
"a decision receipt by `q_id` (the round log's `type: \"decision\"` record with
the human's recorded `chosen`)" with wording of the form: "a decision by `q_id`
(the resolved decision recorded in the plan's Open Questions section, and, when
instrumentation is on, its `type: \"decision\"` round-log record carrying the
human's `chosen`)". This removes the dangling reference in the non-instrumented
case while staying accurate in the instrumented case, and it follows the exact
qualifier pattern already used at `:61`/`:63`/`:69`.

### Sub-claim B: inconsistent citation style (not naming the "Instrumentation" section heading)

Verdict: INVALID (cosmetic, and following it would reduce consistency).
Actionable this round: no.

Reasoning: the reviewer compares this reference to the subsection's other three
cross-refs (which name section headings). But the relevant precedent for a
ROUND-LOG reference is `pack/AGENTS.md:61`/`:63`/`:69`, and none of those name
the "Instrumentation (metrics logging)" heading; they identify the round log by
file path (`docs/metrics/workflow.jsonl`) plus the "when instrumentation is on"
qualifier. So requiring a section-heading name here would make line 104 MORE
inconsistent with the established round-log-reference style, not less. Once
sub-claim A's qualifier is added, the reference is coherent; adding a section
name is optional polish and is not worth a separate change.

---

## Shared fix locus?

Independent. I1-1 edits `pack/prompts/orchestrator.md:23`; I1-2 edits
`pack/AGENTS.md:104`. Different pack source files. Both require the same
follow-on step (re-run the dogfood sync/render recipe and commit the pack source
together with its regenerated outputs: for I1-1 the generated
`.agents/prompts/orchestrator.md`; for I1-2 the generated `AGENTS.md` and
`.agents/AGENTS.reference.md`), but the edits themselves do not overlap and can
be made in one round as two small prose changes.
