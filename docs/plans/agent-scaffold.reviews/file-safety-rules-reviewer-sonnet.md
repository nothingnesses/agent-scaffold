# Review: file-safety-rules (d86ec0f)

Diff range: `742bcae..d86ec0f`. Reviewed `pack/AGENTS.md`, `pack/prompts/implementer.md`,
and `pack/prompts/orchestrator.md` in full, plus the pre-existing ledger paragraph and
the step detail in `docs/plans/agent-scaffold.md`.

---

## S1 - medium

**"Writer agent" is undefined; it gates the clean-tree rule on an undeclared term**

Location: `pack/AGENTS.md` lines 184-192 (the section intro and the clean-tree bullet).

The section opens "Every writer agent's damage must stay a visible, committed-or-recoverable
diff" and the clean-tree rule says "before spawning a writer agent". Neither here nor
anywhere else in AGENTS.md is "writer agent" defined. The Roles section enumerates
orchestrator, planner, reviewers, triager, and implementer but does not classify any of
them as writer vs. read-only.

An orchestrator reading the document cannot determine from the text alone whether
"writer agent" includes the planner (which writes the plan document) or only the
implementer. If the planner is a writer, the clean-tree rule requires a commit before
spawning every planner pass. If it is not, the rule is narrower than stated. The
distinction is implied by context (the `agent-isolation` step, not yet implemented,
names reviewers and the triager as read-only) but not derivable from AGENTS.md alone.

The plan's own validation criterion for this step is "each writer role and the
orchestrator carry the rule that applies to it, stated once and consistently". That
criterion cannot be verified without knowing which roles are writers.

---

## S2 - medium

**The recovery rule is an orchestrator rule placed inside a section framed as writer-agent rules**

Location: `pack/AGENTS.md` lines 184-188 (umbrella sentence) and lines 202-204
(the "Recover on interrupt" bullet).

The section is introduced as rules for writer agents: "Every writer agent's damage must
stay a visible, committed-or-recoverable diff." But "Recover on interrupt" explicitly
names the orchestrator: "the orchestrator inspects `git status` and the diff, reverts
stray temporary artifacts, discards or completes partial work, and confirms a known-good
tree before continuing." The orchestrator is not a writer agent. AGENTS.md says it "does
not plan, implement, review, or triage itself; you spawn the roles that do."

This framing mismatch creates two failure modes:

1. An orchestrator that reads the umbrella ("these are writer-agent rules") may not
   register "Recover on interrupt" as applying to itself, since it does not consider
   itself a writer agent.

2. An implementer reading the section may be confused about whether the recovery
   protocol is the implementer's responsibility or the orchestrator's (the bullet names
   "the orchestrator" but the umbrella says these are writer-agent duties).

The orchestrator's own prompt receives the recovery rule in this same diff (lines 31-33
of `pack/prompts/orchestrator.md`), so the orchestrator will act correctly from its
prompt. The AGENTS.md framing is still the canonical reference for any agent reading the
guidance, and the mismatch is a structural error there.

Two of the five bullets explicitly name specific roles ("An implementer" for format-only,
"the orchestrator" for recover-on-interrupt), while the umbrella claims all five are
writer-agent rules. The umbrella is inconsistent with its own bullets.

---

## S3 - low

**Commit-before-delete is stated twice in AGENTS.md and twice in the orchestrator prompt, with different emphasis**

Location: `pack/AGENTS.md` lines 178-179 (ledger section) and lines 193-195 (new
file-safety section); `pack/prompts/orchestrator.md` line 24 (ledger paragraph) and
lines 29-30 (new paragraph).

The pre-existing ledger section already says: "delete it, committing the deletion, when
the task closes." The new commit-before-delete rule covers the same ledger case with
broader scope, and explicitly calls out the ledger as an example: "Commit any
workflow-managed file (a findings file, the ledger at task close, any transient artifact)
before deleting it."

For the ledger specifically, both sections of AGENTS.md, and both paragraphs of the
orchestrator prompt, now say the same thing. The step's own validation criterion requires
the rules to be "stated once and consistently." For commit-before-delete, they are stated
twice in each document.

The two phrasings also emphasise different orderings. The ledger section treats deletion
as the primary act with committing as a modifier ("delete it, committing the deletion"),
which could be read as "delete, and in doing so commit" rather than "commit first, then
delete." The new rule makes the pre-condition explicit ("commit... before deleting it"),
which is unambiguous about order. Both intend the same meaning, but the two formulations
are not word-for-word consistent, creating a surface on which they could drift in future
edits.

The finding is low rather than medium because the two phrasings are consistent in intent
(no contradiction) and a writer following either would do the right thing. The issue is
redundancy and the minor phrasing divergence, both against Principle 1.
