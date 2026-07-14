# Review: compaction-prep (Q-15, Q-19)

Reviewer: sonnet (independent, adversarial lens)
Diff range: 082280e..226ca33
Scope: pack/AGENTS.md checkpoint section, pack/user-prompts/compaction-prep.md,
pack/user-prompts/resume.md, supporting pack.toml / manifest test / README changes.

---

## S1 - medium

**Thinness violation in compaction-prep.md: body enumerates the procedure steps
it claims not to restate**

Location: `pack/user-prompts/compaction-prep.md`, lines 3-4 and 9-13.

The preamble explicitly states "It triggers the checkpoint procedure defined in
`AGENTS.md`; it does not restate that procedure." The body then enumerates four
concrete sub-steps:

> "flush the plan, the ledger, and the plan's Open Questions queue to current,
> verify the plan's Status line (the resume anchor) is accurate, and commit
> everything"

Compare to `pack/AGENTS.md` (the new section), which says:

> "the orchestrator flushes the plan, the ledger, and the plan's Open Questions
> queue to current, verifies the plan's Status line (the resume anchor) reflects
> where the work actually is, and commits everything"

These are the same four steps, in the same order, with near-identical phrasing.
The prompt body is a paraphrase of the AGENTS.md procedure, not a thin trigger to
it. This contradicts the preamble's own claim, the plan step's explicit requirement
("The user prompts stay thin trigger-prompts so no workflow content is duplicated
between them and the canonical pack/AGENTS.md", user-prompts-dir outcome), and
Principle 1 (one source of truth, no duplication). The practical consequence is
a drift risk: if the procedure in AGENTS.md is updated (say, to also flush
findings files once the findings-files step lands), the prompt will not reflect
it, and an agent following the prompt would miss the new step without any
indication that it is out of date.

The only text in the prompt body that is not a restatement of AGENTS.md is the
completion signal ("Tell me when the tree is clean and it is safe to compact"),
which is legitimately prompt-specific. A thin version would say something like
"Run the pre-compaction checkpoint from the 'Checkpoint and resuming after context
loss' section of `AGENTS.md`. Tell me when the tree is clean and it is safe to
compact."

---

## S2 - low

**Thinness violation in resume.md: body paraphrases the AGENTS.md read-order
despite claiming not to**

Location: `pack/user-prompts/resume.md`, lines 3-4 and 9-13.

The preamble states "It triggers the resume procedure defined in `AGENTS.md`; it
does not restate it." The body says:

> "Reconstruct your state per the checkpoint and resume section of `AGENTS.md`:
> read `AGENTS.md`, then the plan under `docs/plans/` (its Status line is the
> resume anchor), then the review ledger beside it. Act as the orchestrator and
> continue from where the plan and the ledger say the work left off; do not start
> over."

The AGENTS.md section says:

> "reconstruct state from `AGENTS.md`, the plan (its Status line first), and the
> ledger, then continue from where they say the work left off rather than starting
> over."

The prompt restates the read-order (AGENTS.md, then plan, then ledger), the
Status-line-as-resume-anchor concept, and the "do not start over" instruction,
all of which appear verbatim or near-verbatim in AGENTS.md. This is the same
category of violation as S1, but milder: the content is simpler (a read-order
rather than a multi-step procedure), the drift risk is smaller, and there is a
practical argument that a context-lost agent needs some concrete navigation
guidance to reach the right section. The preamble's claim still contradicts the
body, and the plan's design intent still requires thin triggers, so this is a real
finding, but less impactful than S1.

---

## S3 - low

**Harness-agnostic note introduces the phrase "durable notes" without a referent
in the section**

Location: `pack/AGENTS.md` (and generated copies), the closing paragraph of the
new section: "This names the plan, the ledger, and the durable notes, not any
specific harness memory feature, so it works on any harness."

The section body names three artifacts: "the plan, the ledger, and the plan's
Open Questions queue." The harness-agnostic note says the section names "the plan,
the ledger, and the durable notes." "Durable notes" does not appear anywhere else
in the section, in the surrounding AGENTS.md text, or in the two new prompts. The
phrase does not map cleanly to any of the three artifacts named in the procedure
("the plan's Open Questions queue" is a section of the plan, not a separate
"notes" artifact), and it is not defined or introduced elsewhere. The note's
intent is clear (name artifacts by their established names rather than harness
features), but the phrase "durable notes" is imprecise about what the section
actually names, and a reader encountering it for the first time cannot resolve
what it refers to.
