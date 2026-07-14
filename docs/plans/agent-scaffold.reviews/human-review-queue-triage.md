# Triage: `human-review-queue` step, round 1 (`3599756..fbd3ffa`, `pack/`)

Adjudicating the two reviewers' findings: R1, R2 (opus, correctness lens) and
S1, S2, S3 (sonnet, consistency lens). Principle numbers below are normalised to
the plan's own numbered Project Principles (1-7): the plan consolidated the
shipped AGENTS.md principles, so both shipped Principle 16 (one source of truth)
and shipped Principle 20 (self-contained documentation) map to the plan's
Principle 1 (cleaner long-term architecture: correctness, internal coherence,
maintainability). There are no high or critical findings, so no dismissal
triggers the backstop.

Deduplication note. S1, S2, and R2 all touch the AGENTS.md/template split and
the duplication/placement theme, but they are distinct edits, not one finding:
- S1 is format leaking INTO the behaviour file (the field list is enumerated in
  `pack/AGENTS.md` when it should live only in the template).
- S2 is behaviour leaking INTO the format file (the push-at-checkpoint rule is
  stated in the template's Open Questions placeholder).
  S1 and S2 are the two complementary halves of one "clean the split" fix and
  should be done together, but they are separate edits in separate files.
- R2 is a different mechanism: a pre-existing section (the compaction section)
  omits a behaviour that the new Checkpoints paragraph asserts it takes.
All three are counted as distinct valid findings.

---

## S1 - VALID (severity: medium)

Reasoning. The queue item field list (stable id, one-line ask, status, pointer)
is stated in full in both `pack/AGENTS.md` (new Checkpoints paragraph, line 231)
and `pack/plan-template.md` (Open Questions placeholder, line 36). AGENTS.md
names all four fields and then adds "(the plan template defines the item
format)", designating the template as canonical, yet has already given the
reader the complete field list. The step's decided scope (plan line 335) splits
the two files deliberately: AGENTS.md carries "the workflow ... a required
push-at-checkpoint step" (behaviour) and the template "adopts the queue format"
(format), and sets the validation criterion "the queue's format and the required
push step are stated once and consistently." Stating the field list in two files
fails "stated once": the format is not stated once, and a field rename would
require editing both files, which can drift. This is the one finding that fails
the step's own validation criterion, which is why it rates above the other lows;
medium (not high) because the two statements are currently consistent, so the
harm today is latent drift risk rather than an active contradiction. Confirms the
reviewer's medium. Principle 1 (internal coherence / one source of truth).

Recommended fix. Trim the field enumeration from the AGENTS.md Checkpoints
paragraph: describe the queue behaviourally (what it is, resolved-marked-not-
deleted, and the push) and defer to the template for the item format via the
existing parenthetical, so the field list and enum live once in
`pack/plan-template.md`. Pairs with the S2 fix (below) to realise the split
cleanly: AGENTS.md = behaviour + pointer to template for format; template =
format + pointer to AGENTS.md for behaviour.

---

## S2 - VALID (severity: low)

Reasoning. The template's Open Questions placeholder ends with a behavioural
orchestrator instruction, "The orchestrator updates this queue at every
checkpoint and pushes the open items to you" (`pack/plan-template.md` line 36).
That rule is already authoritative in `pack/AGENTS.md` (Checkpoints paragraph)
and `pack/prompts/orchestrator.md` (checkpoint paragraph). Placing it in the
template's format placeholder means it propagates into every concrete plan (it
did in `docs/plans/agent-scaffold.md`, Documentation Protocol line 36 and Open
Questions line 73), creating per-plan copies of a behaviour rule that only
AGENTS.md should own; if the cadence, trigger, or scope of the push changes, the
copies go stale. Low because it is a template-seeding duplication with no current
inconsistency, and the concrete plan legitimately documents its own maintenance;
the defect is that the behaviour is sourced from the template rather than pointed
to. Confirms the reviewer's low. Principle 1 (one source of truth).

Recommended fix. Replace the behavioural sentence in the template placeholder
with a pointer to the AGENTS.md rule (for example "maintained per the push-at-
checkpoint rule in `AGENTS.md`"), keeping the format definition (fields, enum,
pointer) but not restating the orchestrator's duty. Do together with S1.

---

## R2 - VALID (severity: low)

Reasoning. The new Checkpoints paragraph (`pack/AGENTS.md` line 233) asserts "The
compaction checkpoint below ... take[s] the same queue push", and
`pack/prompts/orchestrator.md` line 97 lists a compaction-prep flush as a
checkpoint that pushes open items. But the dedicated "Checkpoint and resuming
after context loss" section's before-context-loss bullet enumerates flushing the
plan, ledger, and Open Questions queue and committing, and omits the push, even
though it enumerates the other pre-compaction actions. A reader landing on that
section in isolation (the likely case during a real compaction) would not learn
the push applies there; the linkage exists only via the forward-reference from
line 233. This incompleteness is introduced by this change (the new paragraph
makes a claim the older section does not back up), so it is a new-valid finding
rather than pre-existing. Low: a completeness / cross-linking gap, not a
contradiction, since the orchestrator prompt (the actor) is unambiguous. Confirms
the reviewer's low. Principle 1 (internal coherence; self-contained
documentation).

Recommended fix. Add the push to the before-context-loss bullet, or a short
clause noting the compaction checkpoint takes the queue push per the Checkpoints
paragraph, so the section that enumerates the pre-compaction actions is complete.

---

## R1 - VALID (severity: low)

Reasoning. The Q-9-follow-up rewording of the human-facing "Getting started"
section introduces the term "checkpoint" (`pack/AGENTS.md` line 16: "at each
checkpoint the orchestrator raises the open items with you"). The old text did
not use the word; the new text does, as its first occurrence in the document,
~215 lines before its definition at line 231. The `human-onboarding` round-1
review had already caught and fixed an "orchestrator"/"checkpoint" jargon gloss
in this same onboarding section (plan line 319), so reintroducing the bare term
is a mild regression against a settled outcome: a newcomer reading Getting
started meets an undefined term. Low because the surrounding sentence still
conveys the actionable point (the orchestrator raises open items with you) even
without the precise definition. Confirms the reviewer's low. Principle 1 (self-
contained documentation).

Recommended fix. Gloss "checkpoint" inline at first use (for example "at each
checkpoint, such as a step boundary or a compaction pause, the orchestrator
raises the open items with you"), or reword to avoid the bare term, keeping the
onboarding section jargon-light as the earlier fix intended.

---

## S3 - VALID (severity: low)

Reasoning. The autonomous cadence option reads "run autonomously to acceptance"
in `pack/user-prompts/kickoff.md` (line 19) but "run autonomously through to
acceptance" in both `pack/AGENTS.md` (line 233) and `pack/prompts/orchestrator.md`
(line 97); "through" is dropped in the kickoff prompt. All three phrasings were
authored in this same change, and the sibling cadence work (Q-21) was
deliberately synchronised across the three files (the opus reviewer noted the
cadence is stated "with matching wording"), so a one-word drift in the autonomous
option is a real, self-introduced consistency gap in a set meant to agree. Low
and cosmetic: the meaning is identical, the phrases are prose descriptions rather
than an exact-match keyword the human types, so there is no behavioural risk; the
value is only coherent naming of one named option across the three documents.
Confirms the reviewer's low. Principle 1 (internal coherence / consistent
vocabulary). Note this is a vocabulary-consistency finding, not a line-length /
wrapping finding, so the standing "line length is never a finding" convention
does not dismiss it.

Recommended fix. Align the three files on one phrasing (either add "through" to
kickoff.md or drop it from the other two); "run autonomously through to
acceptance" everywhere is the smaller edit.

---

## Round outcome

New valid findings (5 of 5 valid: S1 medium; S2, R2, R1, S3 low). The round is
NOT clean; the consecutive-clean streak does not advance. The implementer should
address all five (S1+S2 together as the split-cleanup, R2, R1, S3), after which a
fresh round is spawned on the revised artifact. No high/critical findings, so no
backstop re-check is triggered.
