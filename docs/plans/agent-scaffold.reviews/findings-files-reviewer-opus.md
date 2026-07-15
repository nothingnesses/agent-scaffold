# Reviewer findings: `findings-files` (Q-14), lens CORRECTNESS/COMPLETENESS

Range reviewed: `4ebe8a3..a8a75f5` (`git diff -- pack/`). Pack source only; generated mirrors ignored.

## Summary of the assessment

The change captures the decided rule across all four artifacts. Decided elements all present:

- Reviewers and triager write findings to per-agent files under `docs/plans/<task>.reviews/`: reviewer.md new paragraph, triager.md line 9, AGENTS.md new "Findings files" subsection (line 57); the read-only sentence at AGENTS.md line 25 stays consistent (already said "they write only their own findings files").
- Read-directly hand-off: AGENTS.md subsection ("Other agents read those files directly: the triager reads the reviewers' files") and triager.md line 3. Present.
- Ledger-references-by-path: AGENTS.md subsection and orchestrator.md ("reference a finding by its file path rather than copying its text"). Present.
- Parallel-safety rationale: AGENTS.md subsection ("parallel writers never contend for one file"). Present.
- Commit-before-delete cleanup, orchestrator-owned: AGENTS.md subsection and orchestrator.md, both naming the orchestrator as owner and referencing "(the commit-before-delete rule)". Ownership unambiguous.

Consistency checks requested:

- No contradiction with the prior read-only sentence (AGENTS.md line 25); the subsection elaborates it.
- Commit-before-delete agrees with the file-safety section (AGENTS.md line 66), which already lists "a findings file" as a workflow-managed file to commit before deleting. The subsection references that rule by name rather than redefining it (Principle 16, one source of truth). Not harmfully redundant.

No critical, high, or medium findings. Three low completeness nits below.

## Findings

### F1 (low) - Backstop re-check triager has no distinct findings-file name

- Location: pack/AGENTS.md "Findings files" subsection (line 57) and pack/prompts/triager.md line 9, read against the convergence backstop at AGENTS.md line 51.
- Problem: The naming scheme gives one triage file per step (`<step>-triage.md`). The backstop requires a second, independent triager to re-check a dismissed high/critical finding. That second triager, following triager.md line 9, would write to `<step>-triage.md`, colliding with or overwriting the first triager's verdict file. The re-check verdict has no distinct home.
- Principle: Principle 16 (one source of truth) / completeness.

### F2 (low) - Reviewer filename template does not guarantee the uniqueness the parallel-safety claim depends on

- Location: pack/prompts/reviewer.md new paragraph (`<step>-<your-role-or-model>.md`) vs AGENTS.md subsection guarantee that "parallel writers never contend for one file".
- Problem: Parallel-safety holds only if each concurrent writer gets a distinct filename. `<step>-<your-role-or-model>.md` does not guarantee that: two reviewers sharing model and role/lens resolve to the same path. The plan's own example (line 366) uses `<step>-<agent>.md`, implying a per-agent unique id; "role-or-model" is weaker. Rare in practice (distinct lenses), hence low.
- Principle: completeness of the parallel-safety guarantee (plan line 366).

### F3 (low) - Triager prompt does not instruct creating the reviews directory

- Location: pack/prompts/triager.md line 9 vs pack/prompts/reviewer.md new paragraph, which says "(create the directory if it does not exist)".
- Problem: The reviewer creates the dir if absent; the triager is not told to. Harmless in the normal flow (reviewers write first), a defect only in an off-nominal ordering. Noted for completeness; mitigated in practice.
- Principle: completeness; low because ordinary control flow guarantees the directory exists.

## Considered and not raised

- Dangling working-tree path after cleanup: after deletion, a ledger reference-by-path resolves only via git history. By design (committed-deletion choice, plan line 368); not a finding.
- Duplication between the subsection and the file-safety / ledger paragraphs: the subsection cross-references rather than redefines; no Principle 16 violation.
