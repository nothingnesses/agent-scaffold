# Triage: triager-independence (diff 0c95ab9..bd483e1)

Triager: independent adjudicator, did not produce the artifact and is not the
orchestrator. Judged against the numbered Project Principles in
`docs/plans/agent-scaffold.md` (1-7). Reviewer references to "Principle 16" map
to Principle 1 (correctness, internal coherence, maintainability, one source of
truth); "coherence" maps to Principle 1; "make illegal states unrepresentable"
maps to Principle 5.

Five finding-groups after dedup. Result: 3 valid (1 medium, 2 low), 2 invalid.

---

## Group A - covers R1: AGENTS.md fallback still says "one agent plays the roles in sequence"

Verdict: VALID. Severity: LOW (correcting R1's MEDIUM down).

Reasoning. `pack/AGENTS.md` (and its mirrors) keeps the pre-change fallback
sentence "Where sub-agents are unavailable, one agent plays the roles in
sequence...", which literally includes the triager among "the roles". The
parallel sentence in `orchestrator.md` was tightened this same commit to "perform
the other roles yourself in sequence", grammatically excluding the triager. So the
canonical document now diverges from its own orchestrator prompt on the exact
invariant this step hardens (Principle 1), and leaves the collapse-the-triager
reading textually present in the top-level rule (Principle 5).

Why LOW, not MEDIUM: the very next sentence is an explicit override ("The triager
is the one exception to collapsing: it is always a separate agent, never merged
into the producer or the orchestrator..."). "X applies. The triager is the one
exception to X" is a valid general-rule-plus-exception construction; a reader who
reaches the exception is not left concluding the triager may be collapsed. The
residual defect is a stylistic/coherence asymmetry with orchestrator.md, not a
functional hole, so impact if unfixed is small. The fix is free and aligns the two
files, so it is still worth doing.

Minimal fix: in `pack/AGENTS.md` (regenerate the root `AGENTS.md` and
`.agents/AGENTS.reference.md` mirrors), change "one agent plays the roles in
sequence" to "one agent plays the other roles in sequence", matching
orchestrator.md, and keep the following triager-exception sentence.

---

## Group B - covers R2, S1, S3: AGENTS.md (and triager.md) omit the "(or a human)" allowance

Verdict: VALID. Severity: MEDIUM (confirming S3's MEDIUM; correcting S1's HIGH
down and R2's LOW up).

These three findings are one defect. `orchestrator.md`'s primary triager rule says
"always a separate agent (or a human)", and both files' convergence backstop says
"a second, independent triager (or a human)". But the primary triager rule in
`pack/AGENTS.md` (both the collapse-exception sentence and the Triager role bullet)
says only "always a separate agent", with no human allowance; `pack/triager.md`'s
opening rule ("You are always a separate agent... you must not be either") has the
same omission while its own backstop reference (line 19) includes "(or a human)".
So there are two inconsistencies of the same kind:
- Cross-file (S1): the canonical, read-first AGENTS.md states a stricter rule than
  orchestrator.md enforces. An agent reading only AGENTS.md would conclude a human
  triager is not permitted, because a human is not "a separate agent".
- Intra-file (S3): within AGENTS.md the backstop permits "(or a human)" but the
  primary triager rule does not, an asymmetry with no stated reason. The same
  asymmetry exists inside triager.md.

This is a genuine coherence defect (Principle 1) on the invariant this step
exists to harden, and the design clearly intends humans to be allowed (both the
backstop and orchestrator.md say so), so the omission is an oversight, not an
intended distinction.

Why MEDIUM, not HIGH (auditable reasoning for correcting S1 down): this is
guidance-text incoherence, not a broken mechanism. The intended reading is
unambiguous from two sibling statements (the backstop and orchestrator.md both
permit a human), so a reader who cross-references resolves it; nothing in the
running workflow is blocked purely by this wording (the deadlock S2 posits needs
the additional no-human precondition, addressed in Group C). HIGH overstates a
one-clause documentation omission whose correct value is already stated elsewhere
in the same document set. It is above LOW because it sits in the canonical,
read-first document, touches the step's core invariant, and appears in two files.

Minimal fix: add "(or a human)" to the primary triager rule everywhere it is
currently missing so all statements match: the Triager role bullet in
`pack/AGENTS.md` ("...it is always a separate agent (or a human), independent
of..."), and the opening rule in `pack/triager.md`. Optionally also the
collapse-exception sentence in the opening paragraph of `pack/AGENTS.md` for full
symmetry. Regenerate the root `AGENTS.md` / `.agents/` mirrors. (Editing both
AGENTS.md locations at once is also what Group D asks for; do them together.)

---

## Group C - covers S2: "undefined behavior on a single-agent, no-human harness"

Verdict: INVALID as stated (not a regression introduced by this change).
Severity if it were valid: LOW, and only as an optional clarity note, not the
HIGH claimed.

Auditable reasoning for dismissing a HIGH finding:

1. The state S2 describes is reachable only on a harness with no sub-agents AND
   no human at all. That is the precondition that makes the "(or a human)" escape
   unavailable.

2. This change did not introduce the gap. Before the commit, the primary rule was
   "The triager must not be the agent that produced the artifact under review."
   On a genuinely single-agent harness the sole agent is the producer, so the
   pre-change "one agent plays the roles in sequence" path already violated the
   triager-not-producer constraint for the triager specifically. The single-agent
   no-human case was never cleanly handled; this commit makes the existing
   constraint explicit ("never played by you") rather than creating a new hole.

3. The "no human anywhere" configuration is out of scope for this workflow
   independent of the triager rule. The workflow depends on a human at several
   points: the orchestrator "escalates to a human on impasse", the total-round cap
   "escalate[s] to a human", and acceptance shortfalls route back through human
   decision. A harness with no human available cannot complete the workflow
   regardless of triager independence, so triager-independence is not what breaks
   it.

4. Given (2) and (3), the change closes a path (orchestrator plays the triager)
   that was already illegitimate for the triager, in a configuration the broader
   design already does not support. Closing an illegitimate path in an unsupported
   configuration is not a defect, and certainly not a HIGH regression.

The kernel of truth is that the documents do not spell out an explicit "if
neither a sub-agent nor a human is available, escalate/block" clause for the
triager step. That is already implied by the general escalation model (no path
forward -> human), so at most it is a LOW, optional clarity addition, not a defect
in this change. No fix required.

---

## Group D - covers S4: rule stated twice within AGENTS.md

Verdict: VALID (marginal). Severity: LOW.

The exception is stated in the opening role-separation paragraph ("The triager is
the one exception to collapsing... (see the Triager role below)") and again in the
Triager role bullet ("The triager is always a separate agent... it is never
collapsed into another role"). Both are normative and the two use slightly
different phrasing for the same rule. This is largely the document's normal
overview-then-role-detail structure, and the forward reference already points to
the bullet, so the divergence risk is modest, hence LOW rather than higher. It is
not INVALID because the two statements are worded differently and both are
normative, which is a real (if small) future-divergence risk on Principle 1, and
the Group B fix must touch both AGENTS.md locations, so keeping them aligned has
practical value.

Minimal fix (optional, best done together with Group B): keep the opening-paragraph
sentence as the pointer and let the Triager role bullet carry the full normative
statement, and phrase both consistently (e.g. both "separate agent (or a human),
independent of the producer and the orchestrator; never collapsed into another
role") so a future edit to one is obviously mirrored in the other.

---

## Group E - covers S5: backstop's "independent" has ambiguous referent scope

Verdict: INVALID (reviewer over-reach). Severity: N/A (at most a LOW wording
polish).

The backstop text "a second, independent triager (or a human)" was not changed by
this commit. The new general rule in AGENTS.md and triager.md establishes that
every triager is independent of the producer and the orchestrator; that rule binds
the backstop triager too. So the backstop's "independent" is, if anything, now
redundant with the general rule, not contradictory or under-specified: the second
triager inherits full producer/orchestrator independence from the general rule,
and "independent" in the backstop still carries its plain second-opinion sense (a
different agent from the first triager). S5 itself concedes "a careful reader will
apply the general rule". There is no reachable interpretation under which the
backstop triager is licensed to be non-independent, so there is no real ambiguity
or audit gap to fix. If desired, the word could be dropped or expanded as pure
polish, but this is not a defect in the change.

---

## Summary

- Valid: Group A (LOW, R1), Group B (MEDIUM, R2/S1/S3), Group D (LOW, S4).
- Invalid: Group C (S2, dismissed HIGH with reasoning above), Group E (S5).

Recommended fixes, minimal, all in `pack/` with the root `AGENTS.md` and
`.agents/` mirrors regenerated:
1. Group B (do first, MEDIUM): add "(or a human)" to the primary triager rule in
   `pack/AGENTS.md`'s Triager role bullet and in `pack/triager.md`'s opening rule
   (optionally also the collapse-exception sentence), matching orchestrator.md and
   the backstop.
2. Group A (LOW): change "one agent plays the roles in sequence" to "one agent
   plays the other roles in sequence" in `pack/AGENTS.md`, matching orchestrator.md.
3. Group D (LOW, optional, fold into 1): align the wording of the two AGENTS.md
   triager statements and keep the opening paragraph as the pointer.
No changes needed for Groups C and E.
