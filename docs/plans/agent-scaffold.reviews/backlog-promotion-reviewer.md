# Backlog-promotion review: `impl/backlog-plan`

Reviewer: independent adversarial reviewer (low_risk pass).
Branch HEAD: `1ca6c33`. Base: `main` at `0f12590`.
Pass: promotion of loose backlog items into structured plan entries (10 deferred steps, 3 open questions).

---

## B-1 (low) -- sidecar-ref-empty-string.md:3: misattributed finding source

`docs/plans/agent-scaffold.steps/sidecar-ref-empty-string.md`, line 3, says "Deferred cleanup (issue I2-2, from the structured-skeleton implementation reviews)."

I2-2 was found during the `task-entry-regrounding` inc2 review, not during the structured-skeleton reviews. The ledger entry for `task-entry-regrounding` (around the 20h/20i/20j anchors) records I2-2 as a round-1 finding of that step's inc2: "BACKLOG ADDED (I2-2, triager-deferred): `is_safe_sidecar_ref("")` accepts an empty string." The structured-skeleton reviews use different finding-ID prefixes (MSG-LOCATOR, L1-L3, S1-S3). The L1 symlink issue in `sidecar-ref-symlink.md` is correctly attributed (it originated in `structured-skeleton.reviews/inc3-round2-reviewer-opus.md`), which makes the wrong attribution of I2-2 more likely to mislead a reader tracing back.

The technical content of the step is accurate; only the attribution is wrong. Low severity: no finding is lost and the technical description fully identifies the gap, but a reader following the cited source would look in the wrong review set.

---

## What was probed (clean areas)

- **Schema and validation**: ran `validate --source`, `validate --workflow`, and `render --check` (corrected arg form) against the worktree. All three exited 0. Output: 66 steps, 57 questions, valid; workflow invariants hold; render up to date.
- **Additions only**: `git diff main..HEAD --name-only` and a scan of `-` lines in the TOML diff confirm no existing step, question, principle, or prose was altered. Q-44's `ask` is byte-identical to main.
- **Ledger, src/, metrics untouched**: `git diff main..HEAD -- docs/plans/agent-scaffold.ledger.md docs/metrics/ src/` produced no output.
- **Order continuity**: prior max step order on main was 56 (the `human-input-gate-reinforce` step). New steps are 57-66. No renumbering of existing steps.
- **Question ID continuity**: prior max was Q-54. New questions are Q-55, Q-56, Q-57, in sequence.
- **Q-55 open vs exploring**: confirmed correct. SE-3 is an "independent sharp edge" explicitly carved out from Q-44's design pass, three concrete candidate options are stated in the ask, and no design pass is in progress. `exploring` would imply an active or owed design pass; `open` is accurate.
- **Empty question body convention**: sampled Q-1, Q-10, Q-20, Q-30, Q-44, Q-51, Q-54 sidecars; all are 0 bytes. The three new question sidecars (Q-55, Q-56, Q-57) are also empty. Convention confirmed and correctly followed.
- **Provenance accuracy (SE steps)**: `docs/plans/architecture-audit.explorations/audit-sharp-edges.md` exists on disk and covers SE-1 through SE-17 including all six SE items cited. The task-relative path `architecture-audit.explorations/audit-sharp-edges.md` is correct (relative to `docs/plans/`, the TOML's directory). Q-51 is a real decided question about the workflow driver (`agent-scaffold next` / `status --resume`); the link in `repoint-resume-prompts` provenance is apt.
- **sidecar-ref-empty-string / sidecar-ref-symlink split**: I2-2 (empty string passes lexical check) and L1 (symlink escapes lexical check) are distinct technical gaps in `is_safe_sidecar_ref` that require different fixes; split is sound.
- **SE-1/SE-17 grouping**: SE-17 ("Module header citations mix two principle namespaces") is the code-level symptom of SE-1 (the two-namespace collision). The audit itself (line 133, 161) lists SE-17 as cosmetic and directly tied to SE-1; grouping is sound.
- **Sidecar headings vs TOML titles**: all ten step sidecar headings match their TOML `title` fields exactly.
- **Style**: no em-dashes, en-dashes, unicode characters, or emoji in any new file. `--` occurrences in sidecar bodies are CLI flag syntax (`--resume`, `--instrument`), not dash substitutes. Prose is not hard-wrapped.
- **All deferred steps**: each sidecar says "No open decision, a known change" and the task descriptions are self-consistent with the cited audit findings or review records.
