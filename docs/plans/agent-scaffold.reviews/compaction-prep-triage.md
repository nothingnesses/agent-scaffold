# Triage: compaction-prep (Q-15) and resume prompt (Q-19)

Triager: independent adjudicator (not the producer, not the orchestrator).
Artifact: `git diff 082280e 226ca33` (HEAD 226ca33) plus the `compaction-prep`
step detail in `docs/plans/agent-scaffold.md`.
Reviews adjudicated: `compaction-prep-reviewer-opus.md` (R1, R2) and
`compaction-prep-reviewer-sonnet.md` (S1, S2, S3).

Deduplication applied: opus R2 and sonnet S1 + S2 are the same underlying
finding (the two prompts claim not to restate the AGENTS.md procedure, but their
bodies enumerate it). They are adjudicated once below as Group 1. R1 and S3 are
distinct single findings.

---

## Group 1 (covers R2, S1, S2): the prompts claim they do not restate the procedure, but they do

Verdict: VALID.
Severity: medium (the `compaction-prep.md` instance), low (the `resume.md`
instance). Fixed together by one trim. Confirms opus R2 (medium) and sonnet S1
(medium) / S2 (low).

### Reasoning

The self-contradiction is real and sits in shipped template text:

- `pack/user-prompts/compaction-prep.md` header (line 5) states it "does not
  restate that procedure," and the body (lines 9-13) then enumerates the exact
  four sub-steps the AGENTS.md checkpoint bullet owns (flush plan + ledger + Open
  Questions queue, verify the Status line, commit everything), in the same order
  and near-identical phrasing.
- `pack/user-prompts/resume.md` header (line 5) states it "does not restate it,"
  and the body (lines 9-13) then paraphrases the AGENTS.md read-order (AGENTS.md,
  then the plan with Status line as resume anchor, then the ledger) and the "do
  not start over" instruction.

Two independent harms, both grounded:

1. Coherence (Principle 1, one source of truth). The preamble's own description
   of the artifact is factually inaccurate: it says the body does not restate the
   procedure, and the body restates it. This is a defect in the shipped text
   regardless of drift.
2. Drift (Principle 4, idempotent / no drift). The AGENTS.md section is the
   canonical source; the prompt bodies are an uncatchable copy. The asset-list
   test in `src/manifest.rs` checks only dest paths, not contents, so if the
   checkpoint procedure gains or drops a durable artifact the prompt list goes
   stale silently. This is not hypothetical: the queued `findings-files` step
   would add findings files as a durable artifact to flush, at which point
   `compaction-prep.md`'s enumerated list is wrong with nothing catching it.

The design intent confirms the finding rather than excusing it: the plan's
`user-prompts-dir` outcome (line 509) requires these prompts to "stay thin
triggers that do not duplicate workflow content," and the sibling `kickoff.md`
already meets that bar without enumerating any procedure.

Severity split within the group: the `compaction-prep.md` four-step enumeration
is a genuine multi-step paraphrase with the concrete `findings-files` drift path,
so medium. The `resume.md` read-order is a simpler three-item navigation list
with smaller drift surface, so low. Same category, one fix.

### The thinness bar (as asked)

The correct bar is the one `kickoff.md` sets: name the role, point at where the
workflow lives (`AGENTS.md` / the orchestrator prompt), and give a task slot,
WITHOUT restating any workflow steps. Applied here:

- The prompts MAY name what they trigger (a pre-compaction checkpoint; a resume)
  and keep their prompt-specific framing, because that framing is not a copy of
  the AGENTS.md procedure:
  - compaction-prep: the human-facing intent and completion signal, "tell me when
    the tree is clean and it is safe to compact."
  - resume: the role-and-continuation framing, "act as the orchestrator ...
    continue from where the plan and the ledger say the work left off; do not
    start over."
- The prompts MUST drop the enumerated steps that the AGENTS.md section owns: the
  flush / verify-Status-line / commit list in compaction-prep, and the
  AGENTS.md -> plan (Status line) -> ledger read-order in resume. A brief recap is
  NOT acceptable here: any recap of those steps is exactly the duplication that
  Principle 1 and the `user-prompts-dir` outcome forbid, and it is what creates
  the drift. Naming the target section by its title is enough navigation; the
  agent reads `AGENTS.md` first anyway, so sonnet's "needs concrete navigation"
  caveat (S2) is satisfied by the section name without the read-order copy.

Keep the "does not restate" claim in both headers, and trim the bodies to honour
it. Do not soften the claim to admit a restatement: softening would legitimise
the duplication and leave the drift path in place, which defeats the thin-trigger
design. The claim is the correct target; the bodies are what is wrong.

### Recommended minimal fix (trimmed wording)

`pack/user-prompts/compaction-prep.md` body (replace lines 9-13):

> I am about to compact the context. Before I do, run the pre-compaction
> checkpoint from the "Checkpoint and resuming after context loss" section of
> `AGENTS.md`, so the task can resume after the context is lost. Tell me when the
> tree is clean and it is safe to compact.

`pack/user-prompts/resume.md` body (replace lines 9-13):

> Continue the in-progress task. Reconstruct your state per the "Checkpoint and
> resuming after context loss" section of `AGENTS.md`. Act as the orchestrator and
> continue from where the plan and the ledger say the work left off; do not start
> over.

Both keep the pointer (naming the exact section) and the legitimate
prompt-specific framing, and drop the enumerated steps. After editing the
`pack/` sources, re-run `just scaffold-self` so the byte-identical
`.agents/user-prompts/` mirrors are regenerated (these are `reference` assets).

---

## R1 (over-claim): "the clean-tree-before-writer discipline covers this"

Verdict: VALID. Severity: low. Confirms opus R1.

### Reasoning

At `pack/AGENTS.md` line 256 the pre-compaction bullet says the orchestrator
"commits everything (the clean-tree-before-writer discipline covers this)." The
cited rule (lines 213-215) is scoped by its own wording to a different trigger,
"Commit pending work ... before spawning a writer agent." A pre-compaction flush
is not spawning a writer; a compaction is a distinct event. What the two share is
the commit-before-risk mechanic, not the rule's trigger, so a reader who follows
the pointer finds no compaction trigger there. The over-claim is documentation
precision only: the section states the pre-compaction commit requirement
independently, so nothing is functionally missing. Hence low. The plan step
frames the relationship correctly as reuse (line 405, "reuses the durability
rules ... rather than inventing new ones"); only the shipped parenthetical
over-reaches by pointing at one specific rule whose trigger excludes compaction.

### Recommended minimal fix

Replace the parenthetical at line 256:

- from: `(the clean-tree-before-writer discipline covers this)`
- to: `(the same commit-before-risk durability discipline as before a writer,
  applied to a distinct trigger)`

Re-run `just scaffold-self` to update the generated mirrors (`AGENTS.md`,
`.agents/AGENTS.reference.md`).

---

## S3 (durable-notes referent): closing note names an artifact the body does not

Verdict: VALID. Severity: low. Confirms sonnet S3.

### Reasoning

The section's closing note (line 263) says "This names the plan, the ledger, and
the durable notes, not any specific harness memory feature." The section body
names three artifacts: the plan, the ledger, and the plan's Open Questions queue.
"Durable notes" appears nowhere else in the section and does not map cleanly to
any of the three (the Open Questions queue is a section of the plan, not a
separate notes artifact), so a first-time reader cannot resolve the referent. The
harness-agnostic intent is fine; only the phrase is imprecise. Documentation
precision only, hence low.

Note on provenance: the plan step (line 410) uses the same "durable notes"
phrase, so the wording is inherited. The referent breaks in the shipped section
because the section body enumerates the Open Questions queue instead; the fix
belongs in `pack/AGENTS.md` (the shipped artifact). Aligning the plan's line 410
is optional and cosmetic, not required for the fix.

### Recommended minimal fix

Replace at line 263:

- from: `This names the plan, the ledger, and the durable notes`
- to: `This names the plan, the ledger, and the plan's Open Questions queue`

Re-run `just scaffold-self` to update the generated mirrors.

---

## Summary of verdicts

| Finding group | Covers | Verdict | Severity |
| --- | --- | --- | --- |
| Group 1 (prompts restate the procedure they claim not to) | R2, S1, S2 | VALID | medium (compaction-prep), low (resume) |
| R1 (clean-tree-before-writer over-claim) | R1 | VALID | low |
| S3 (durable-notes referent) | S3 | VALID | low |

No findings dismissed; no high-or-above dismissals to escalate. All fixes are
documentation-level and edit only `pack/AGENTS.md` and the two
`pack/user-prompts/*.md` sources, then regenerate the `.agents/` mirrors with
`just scaffold-self`.
