# Triage: `agent-isolation` (Q-18)

Artifact: implementation diff `57739c3..032964a` (the writer-isolation rule in
`pack/AGENTS.md` + mirrors, and the tier-selection paragraph in
`pack/prompts/orchestrator.md`). Adjudicated against the step detail
(`docs/plans/agent-scaffold.md`, `agent-isolation` and `optional-modules`) and the
plan's numbered Project Principles (Principle 1 cleaner/coherent architecture,
Principle 2 minimal by default). Reviews: `agent-isolation-reviewer-opus.md` (R1,
R2, R3) and `agent-isolation-reviewer-sonnet.md` (S1, S2). R2 and S1 are the same
underlying finding and are ruled on together.

None of the findings is high or critical, so no dismissal triggers the
second-triager re-check requirement.

## R1 - Tier list restated in the orchestrator prompt as well as AGENTS.md

Covered ids: R1.
Verdict: INVALID (not a defect).
Severity: low (as raised), but downgraded to non-issue.

Reasoning: this is the established, intended style of this codebase, not a
duplication defect. The orchestrator prompt is written as a self-contained actor
prompt that restates each AGENTS.md rule it must carry out, each with a "see
`AGENTS.md`" pointer to the authoritative source. The pre-existing file-safety
block in the same prompt (`pack/prompts/orchestrator.md` lines 26-36) does exactly
this: "(see the file-safety rules in `AGENTS.md`)" followed by an operational
restatement of clean-tree-before-writer, commit-before-delete, and the recovery
protocol. The new isolation paragraph (lines 38-42) follows that pattern
identically: a cross-reference plus a one-line operational restatement of the tier
order. The step detail explicitly assigns the tier selection to this actor ("the
orchestrator selects the tier when spawning a writer"), so the restatement lives in
the prompt of the role that performs it. AGENTS.md remains the single authoritative
source (Principle 16 in the shipped set); the prompt derives an operational summary
from it and points back to it, which is derivation, not a competing source. The
reviewer itself rated it "borderline rather than a defect" and required no fix; the
sonnet reviewer did not raise it. The divergence risk is real but is inherent to
the prompt-mirrors-AGENTS.md design already in use throughout the pack, and is not
introduced or worsened by this change.

Recommended fix: none.

## R2 + S1 - "Structural upgrade" framing stated in both the file-safety and isolation sections

Covered ids: R2, S1 (same underlying finding).
Verdict: VALID.
Severity: low (confirming opus's low; correcting sonnet's medium down to low).

Reasoning: the same conceptual claim, that isolation is a structural upgrade
over/on the file-safety baseline, is asserted in two places:
`pack/AGENTS.md` lines 192-194 (file-safety section: "running writers under
isolation is a structural upgrade layered on top of these rules, not a replacement
for them") and lines 225-227 (isolation section: "Isolation is the structural
upgrade over the file-safety baseline: ..."). The isolation section adds the "why"
(blast radius contained rather than only recoverable), but it leads with the same
characterization the file-safety sentence already makes. This is a genuine
one-source-of-truth violation (Principle 1 coherence; Principle 2 minimal): the two
sentences are currently consistent, but the claim has two homes, so a later edit to
one can silently drift from the other.

Severity is low, not medium: there is no present incoherence, no functional impact,
and the blast radius is two adjacent sentences in a single document. The impact if
left unfixed is a future drift surface, which is a low-severity documentation
coherence issue on the absolute scale, not the material-impact level that medium
implies.

Sonnet's proposed direction is the minimal correct one and respects "do not
over-edit": the isolation section is the natural owner of the isolation<->file-safety
relationship (it already carries the rationale), so it keeps the "structural
upgrade" claim unchanged. The file-safety section should retain only its
load-bearing point, that the baseline rules are not replaced by isolation (a reader
landing there must not conclude isolation obviates the rules), and drop the
duplicated "structural upgrade" characterization.

Recommended minimal fix: in `pack/AGENTS.md`, edit only the file-safety sentence
(lines 192-194) to drop the "structural upgrade layered on top of these rules"
wording while keeping the not-a-replacement point and pointing forward, e.g. "This
is the always-on baseline; running writers under isolation (the writer-isolation
rule below) does not replace these rules." Leave the isolation section (lines
225-227) unchanged as the sole owner of the "structural upgrade" claim. Regenerate
the mirrors (`AGENTS.md`, `.agents/AGENTS.reference.md`).

## S2 - Isolation section re-enumerates role membership instead of reusing the Roles-section definition

Covered ids: S2.
Verdict: VALID.
Severity: low (as raised).

Reasoning: the Roles section (`pack/AGENTS.md` lines 43-47) already defines the
membership and mints the reusable term: the planner and implementer are writers,
the reviewers and triager are read-only, and '"Writer agent" below means a spawned
writer role.' The isolation section then re-enumerates that membership
parenthetically: "Run each writer agent (the planner and the implementer)..." and
"Read-only agents (the reviewers and the triager)...". The `file-safety-rules`
Outcome in the plan (line 358) records the intended pattern explicitly: the Roles
section "defines 'writer agent', which `agent-isolation` reuses for its read-only
carve-out", i.e. reuse the defined term, not restate its members. Re-enumeration is
a minor drift surface (Principle 2, minimal; Principle 1, coherence): adding or
renaming a writer role would require updating both the Roles section and these
parentheticals. It also cuts against the document's own stated convention that
later sections use the bare term "writer agent". Principle 20 (self-contained docs)
does not justify the parentheticals here, because the term is defined earlier in the
same document and the doc signals that later use is by bare term.

This is low severity: the parentheticals are not wrong today and carry no functional
risk; the cost is only the extra place to keep in sync.

Recommended minimal fix: in `pack/AGENTS.md`, drop the "(the planner and the
implementer)" parenthetical from the isolation section's opening sentence so it
reads "Run each writer agent in the strongest isolation the harness supports,...",
relying on the Roles-section definition of "writer agent" (this is the exact reuse
the step detail calls for). The read-only carve-out's "(the reviewers and the
triager)" may be trimmed for the same consistency, but is the lower-value half
because the Roles section names those roles without minting a single reusable
"read-only agent" term; trimming it is optional. Regenerate the mirrors.

## R3 - Plan Roadmap status and Outcome not updated in this diff

Covered ids: R3.
Verdict: INVALID (expected artifact of reviewing the pre-convergence diff; not a
defect in the reviewed content).
Severity: low (as raised), reclassified as a pending process action rather than a
finding.

Reasoning: the orchestrator's workflow flips a step's Roadmap status to `complete`
and writes its Outcome paragraph only at convergence, after the review loop, not in
the implementation commit. This is confirmed by `pack/prompts/orchestrator.md`
lines 83-85 ("have the implementer make it, review it to convergence, then mark it
complete") and by the sibling `file-safety-rules` step, whose Outcome is dated
"converged at `aeea55b`", after review. The diff under review (`032964a`) is the
pre-convergence implementation commit, so the Roadmap still reading `next` and the
absent Outcome paragraph are exactly the expected state at this point in the
workflow, not an omission in the artifact. The status flip and Outcome are a real
pending action for the orchestrator at task close, but they are not a defect against
this diff. The opus reviewer reached the same conclusion ("almost certainly
intentional deferral ... not a defect in the reviewed content itself").

Recommended fix: none against this diff. Process note for the orchestrator: at
convergence, flip `agent-isolation` to `complete` in the Roadmap and add its Outcome
paragraph (with the converged commit), as done for prior steps. Do not close the
task without that update.

## Summary

- Valid: 2 groups, both low severity (R2+S1 structural-upgrade duplication; S2
  role re-enumeration).
- Invalid: 2 (R1 intended prompt-mirrors-AGENTS.md style; R3 expected
  pre-convergence artifact, with a status-update process note).
- No high or critical findings; no dismissal required a second-triager re-check.

Concrete fixes (both in `pack/AGENTS.md`, then regenerate the mirrors):
1. File-safety sentence (lines 192-194): drop the "structural upgrade layered on
   top of these rules" wording, keep the not-a-replacement point (point forward to
   the isolation rule); isolation section keeps sole ownership of the claim.
2. Isolation section opening sentence: drop the "(the planner and the implementer)"
   parenthetical and use the defined term "writer agent" bare; optionally trim the
   read-only "(the reviewers and the triager)" parenthetical for consistency.
