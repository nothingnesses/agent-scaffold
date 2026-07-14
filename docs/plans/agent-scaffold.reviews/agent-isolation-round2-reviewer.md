# Review round 2: `agent-isolation` (Q-18)

Diff range reviewed: `57739c3..ef875bd` (cumulative step diff). Adjudicated against
the numbered Project Principles in `docs/plans/agent-scaffold.md` (Principle 1
coherent architecture, Principle 2 minimal by default). `pack/` is source; root
`AGENTS.md` and `.agents/` are generated mirrors.

## Round-1 fix verification

Both round-1 VALID findings are fixed as described.

R2+S1 (structural-upgrade duplication): the file-safety section
(`pack/AGENTS.md` line 192-194) now reads "running writers under isolation builds
on it rather than replacing it (see Writer isolation below)." The "structural
upgrade" phrasing is gone from the file-safety section, and the Writer-isolation
section (lines 224-226) solely owns the claim: "Isolation is the structural
upgrade over the file-safety baseline: a killed or misbehaving isolated writer
cannot touch the main tree ...". No duplication remains; the not-a-replacement
point is preserved and now points forward.

S2 (role re-enumeration): the Writer-isolation section no longer carries the
"(the planner and the implementer)" or "(the reviewers and the triager)"
parentheticals. The opening reads "Run each writer agent in the strongest
isolation the harness supports" (line 212) and the carve-out reads "Read-only
agents need no isolation" (line 223), relying on the Roles-section definitions
(`pack/AGENTS.md` lines 43-47: writers = planner + implementer; reviewers +
triager = read-only; '"Writer agent" below means a spawned writer role.'). Both
terms resolve unambiguously against that section.

Mirror sync confirmed: `sed`-range diff of `pack/AGENTS.md` vs `AGENTS.md`
(lines 190-229) is identical, and `pack/prompts/orchestrator.md` vs
`.agents/prompts/orchestrator.md` is byte-identical. `.agents/AGENTS.reference.md`
carries the same edits per the cumulative diff.

## Coherence pass (file-safety + Writer-isolation sections; orchestrator prompt)

- Tier order is intact and correct: container (1) > worktree (2) >
  file-safety-fallback (3), `pack/AGENTS.md` lines 215-221. The orchestrator
  prompt (`pack/prompts/orchestrator.md` lines 38-42) restates the same order
  ("container ... if available, else a worktree, else the file-safety discipline
  as the fallback") with a see-`AGENTS.md` pointer, consistent with the rule.
- Read-only carve-out intact and consistent with the Roles classification
  (line 223, "no blast radius to contain", minimal by default).
- Mechanism still correctly deferred: "The isolation mechanism ... is an optional
  module; this rule is the always-applicable selection policy and holds whether or
  not that module is built, resolving to the file-safety fallback until it is"
  (lines 226-229). No claim that the container/worktree integration exists.
- "writer agent" / "read-only agents" remain unambiguous after the parentheticals
  were dropped; both are anchored by the Roles section earlier in the same file.

No NEW contradiction is introduced by the edits. The dropped parentheticals do
not orphan any reference, and the forward pointer from the file-safety section
lands on the Writer-isolation heading that now exists below it.

(Non-finding, cosmetic only, not reported as a defect per the review scope: the
S2 trim left the "Isolation is the structural upgrade ..." sentence appended to
the read-only carve-out line, so line 224 in `pack/AGENTS.md` runs long relative
to the surrounding ~80-column wrap. This is a source-file line-wrap artifact with
no rendered or semantic effect, and the two sentences already shared a paragraph
before the fix, so it is not a new defect.)

## Result

Fixes verified; no new findings.
