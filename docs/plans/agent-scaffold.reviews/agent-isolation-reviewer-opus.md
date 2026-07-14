# Review: `agent-isolation` (Q-18) - reviewer-opus

Diff range: `57739c3..032964a`. Files changed: `pack/AGENTS.md`,
`pack/prompts/orchestrator.md` (SOURCE) and their generated mirrors
`AGENTS.md`, `.agents/AGENTS.reference.md`, `.agents/prompts/orchestrator.md`.

Reviewed adversarially against the decided step detail in
`docs/plans/agent-scaffold.md` (`agent-isolation`, lines 360-367), the
`file-safety-rules` section and Roles classification it builds on, the
`optional-modules` step, and the numbered Project Principles.

## Summary of correctness against the spec

The core spec is met:

- Tier order is exactly container > worktree > file-safety fallback
  (`pack/AGENTS.md` lines 212-221), matching Q-18's ordered list.
- Writers are named as the planner and the implementer (line 212-213),
  matching the Roles classification added by `file-safety-rules`
  (`pack/AGENTS.md` lines 43-47: planner + implementer are writers).
- The read-only carve-out ("Read-only agents (the reviewers and the triager)
  need no isolation", lines 223-224) is present, cites the same rationale
  (minimal by default), and is consistent with the read-only classification.
- The change implements ONLY the rule, not the mechanism: it uses "if
  available" / "if that is what the harness offers", never claims the
  container/worktree integration exists, and explicitly frames the mechanism
  as an optional module deferred until built ("The isolation mechanism ... is
  an optional module; this rule ... holds whether or not that module is built,
  resolving to the file-safety fallback until it is", lines 226-231).
- `pack/prompts/orchestrator.md` carries the tier selection when spawning a
  writer (lines 38-43) and cross-references the AGENTS.md rule; it is
  consistent with the AGENTS.md rule.
- The fallback tier is exactly the file-safety discipline ("The file-safety
  discipline above, as the fallback", line 220).
- Generated mirrors are in sync: `AGENTS.md` and `.agents/AGENTS.reference.md`
  differ from `pack/AGENTS.md` only in the expected `{{principles}}`
  substitution (a pre-existing render difference, not introduced here); the
  added isolation section is byte-identical across all three, and the
  orchestrator mirror is identical to source.

## R1 - Tier list is restated in the orchestrator prompt as well as AGENTS.md

Severity: low

Location: `pack/prompts/orchestrator.md` lines 38-43 vs `pack/AGENTS.md`
lines 212-221.

Evidence: the orchestrator paragraph both cross-references the AGENTS.md rule
("see the writer-isolation rule in `AGENTS.md`") and restates the full ordered
list ("container isolation ... if available, else a worktree, else the
file-safety discipline as the fallback"). This partially duplicates the
authoritative list, so the two could drift (Principle 16, one source of truth;
Principle 2, minimal). This is borderline rather than a defect: the plan
explicitly directs that "the orchestrator selects the tier when spawning a
writer", an operational one-line restatement in the actor's own prompt is
reasonable, and the wording is currently consistent with AGENTS.md
("agent-box / agent-images", container > worktree > file-safety). No fix
required; noted only as a divergence risk.

## R2 - "Structural upgrade over the file-safety baseline" is stated in two adjacent sections

Severity: low

Location: `pack/AGENTS.md` file-safety section lines 192-194 ("running writers
under isolation is a structural upgrade layered on top of these rules, not a
replacement for them") and isolation section lines 224-227 ("Isolation is the
structural upgrade over the file-safety baseline ...").

Evidence: the same framing appears in both adjacent sections. The isolation
section adds the "why" (blast radius contained rather than only recoverable),
so it is elaboration rather than pure duplication, and each section stays
self-contained (Principle 20). Minor redundancy against Principle 2; acceptable
as written.

## R3 - Plan Roadmap status and Outcome paragraph not updated in this diff

Severity: low

Location: `docs/plans/agent-scaffold.md` line 121 (`| agent-isolation | next |`)
and the `agent-isolation` step (lines 360-367, no `Outcome` paragraph), vs the
completed `file-safety-rules` step which carries an `Outcome (complete; ...)`
line.

Evidence: the diff range `57739c3..032964a` touches no plan file, so the
Roadmap still marks `agent-isolation` as `next` and the step lacks the
completion/Outcome record that sibling completed steps carry (Principle 9,
durable notes). This is almost certainly intentional deferral: the change is
mid-workflow (under review, pre-acceptance), and the implementer flips status /
writes the Outcome at task close, as done for prior steps. Noted so the
orchestrator does not close the task without that update; not a defect in the
reviewed content itself.

## Severities with no findings

No critical, high, or medium findings. The tier order, the writer/read-only
classification, the rule-only (mechanism-deferred) scoping, the orchestrator
tier selection, and the generated-mirror sync are all correct and complete
against Q-18.
