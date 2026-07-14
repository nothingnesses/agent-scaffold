# Triage: `file-safety-rules` (Q-17)

Artifact: `git diff 742bcae d86ec0f` (the `file-safety-rules` step). Judged against
the step detail (docs/plans/agent-scaffold.md lines 346-356) and the numbered
Project Principles (lines 18-24). Reviews adjudicated: reviewer-opus (R1) and
reviewer-sonnet (S1, S2, S3).

Verdict summary: three finding-groups, all VALID at low severity. No high or
critical dismissals (nothing to escalate for re-check). No contradictions found;
every issue is a coherence/redundancy polish against Principle 1, and every
recommended fix is a subtraction or a one-clause addition, consistent with
Principle 2 (minimal).

---

## Group A (covers S1, S2): "writer agent" is a loose category term

Verdict: VALID. Severity: low (both S1 and S2 downgraded from medium).

Reasoning:

- S1 is correct that "writer agent" is never explicitly tied to roles. The Roles
  section (pack/AGENTS.md lines 21-41) enumerates orchestrator, planner,
  reviewers, triager, implementer but does not classify any as writer vs
  read-only. So the term that gates the clean-tree rule ("before spawning a writer
  agent") and frames the whole section is used without a definition.
- But the classification is inferable from the existing role descriptions: the
  planner "Drafts the plan" and the implementer "Makes small, reviewable changes"
  clearly write; reviewers "review" and the triager "judges" clearly do not. The
  clean-tree bullet's own "especially the plan and the ledger" confirms the
  planner-writes-the-plan case is in scope. So the term is loose, not undefined to
  the point of blocking the rule. The plan's validation criterion ("each writer
  role and the orchestrator carry the rule that applies to it") is verifiable by
  reading those role descriptions. That is why this is low, not medium: no
  operational failure follows; a reader lands on the right classification.
- S2's "umbrella contradicts its own bullets" claim is the weaker half and is
  only partly right. The intro states an invariant ("Every writer agent's damage
  must stay a visible, committed-or-recoverable diff"), then "The rules:". The
  reviewer reads "The rules:" as "the rules every writer performs", under which
  the orchestrator-named "Recover on interrupt" bullet is contradictory. The
  alternative reading, "the rules that keep this invariant true, owned by the role
  each bullet names", is coherent: clean-tree and recover-on-interrupt are the
  orchestrator's mechanisms for bounding a writer's blast radius, not writer
  duties. The text genuinely admits both readings, so there is a minor framing
  ambiguity, but it is not an operational contradiction: every bullet that is not
  a plain writer duty names its owner ("the orchestrator", "An implementer"), and
  both role prompts carry the correct duty to the correct actor in this same diff.
  So S2 is a tightness issue, low, and shares the same underlying defect as S1
  (the section leans on an unclassified "writer agent" notion).

Recommended minimal fix (single fix for both):

1. Add one sentence to the Roles section classifying the roles, e.g. "The planner
   and the implementer are the writer roles; the reviewers and the triager are
   read-only." This gives "writer agent" a definition (S1) and is reused by the
   later `agent-isolation` step, which needs the same read-only carve-out.
2. Reword the file-safety intro's lead-in from "The rules:" to make ownership
   explicit, e.g. "The rules, each carried out by the role it names:". This
   removes the S2 ambiguity without restructuring the section.

Both are small; neither adds machinery (Principle 2). Do not add a separate
writer/read-only registry or per-bullet role tables; the one-sentence
classification plus the lead-in tweak is sufficient.

---

## Group B (covers R1): incidental-reformatting duty is deferred to a role that "does not implement"

Verdict: VALID. Severity: low (confirms R1's rating).

Reasoning:

- The format-only rule tells the implementer to "leave incidental reformatting to
  the orchestrator" (pack/AGENTS.md line 199; implementer.md line 16), matching
  the step spec (plan line 352). But the orchestrator prompt never mentions
  formatting and opens "You do not plan, implement, review, or triage yourself"
  (orchestrator.md line 3); the Roles section repeats "It does not plan,
  implement, review, or triage itself" (AGENTS.md line 25). Reformatting a file is
  a content change, i.e. implementing, so a reader can reasonably conclude the
  orchestrator is barred from it too. The duty the implementer hands off then has
  no clearly-permitted owner: a real coherence gap against Principle 1.
- Severity is low: incidental reformatting is cosmetic, the format-only rule's
  safety purpose (preventing the fmt-plus-checkout clobber) is fully served
  regardless of who later reformats, and the repo already runs `nix fmt` as a
  pre-commit convention (plan line 3), so drift gets corrected anyway.
- The gap originates in the decided spec wording ("leave incidental reformatting
  to the orchestrator"), so the implementation is faithful to the step; the
  finding is nonetheless valid because faithfulness to a spec clause that itself
  conflicts with the orchestrator's stated non-implementing role is still a
  coherence defect worth recording.

Recommended minimal fix (either resolves it; prefer the first to honour the
decided spec):

1. Add one clause to the orchestrator prompt clarifying it owns tree-hygiene
   formatting, e.g. that after a writer completes the orchestrator may run the
   repo-wide formatter to normalise incidental drift, distinct from implementing a
   step. The orchestrator already owns committing and cleanup, so this is a
   one-clause clarification, not a new responsibility.
2. Alternative (cleaner against "orchestrator does not implement", but diverges
   from the spec wording): change the implementer's deferral from "to the
   orchestrator" to "flag it", routing repo-wide reformatting through the
   implementer prompt's existing "flag anything else rather than doing it
   silently" so the orchestrator schedules it as an implementer task. Flag this
   choice to the planner rather than silently reinterpreting the decided spec.

R1's related sub-low (how an implementer scopes `nix fmt` to its own files) is
not a doc defect; the reviewer agrees it is a practical detail left to the agent.
No fix.

---

## Group C (covers S3): commit-before-delete stated twice

Verdict: VALID. Severity: low (confirms S3's rating).

Reasoning:

- The pre-existing ledger paragraph says "delete it, committing the deletion, when
  the task closes" (AGENTS.md lines 178-179; orchestrator.md line 24). The new
  commit-before-delete rule covers the same ledger case with broader scope and
  names the ledger explicitly as its example ("the ledger at task close"). So the
  ledger's commit-before-delete requirement now appears twice in AGENTS.md and
  twice in the orchestrator prompt. The step's own validation criterion asks for
  rules "stated once and consistently"; for this rule they are stated twice.
- The two phrasings are consistent in intent (no contradiction), which is why this
  is low, not medium. But they differ in wording ("delete it, committing the
  deletion" vs "commit ... before deleting it"), a small drift surface against
  Principle 1: a future edit to one may not track the other.
- This is a genuine redundancy, not a justified in-context restatement: the new
  rule already cites the ledger case by name, so the ledger paragraph's own
  "committing the deletion" clause is now fully subsumed.

Recommended minimal fix (a subtraction, consistent with Principles 1 and 2):

- In the ledger paragraph of both AGENTS.md and orchestrator.md, drop the
  "committing the deletion" clause, leaving "delete it when the task closes", and
  let the canonical commit-before-delete rule supply the commit requirement (it
  already names the ledger). No information is lost, the duplication and the drift
  surface both go away, and no new text is added.

---

## Notes

- No settled-finding ledger was supplied for this artifact, so all findings were
  adjudicated fresh; none are re-openings.
- Reviewer-opus's verification context (all five rules present, role assignment
  matches the spec's fold-in, generated mirrors in sync, ASCII-clean) was
  spot-checked against the diff and holds; not a finding.
