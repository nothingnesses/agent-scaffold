# Reviewer findings: `ledger-template` step, round 2 (verification)

Reviewer: opus. Role: round-2 verification of the four round-1 low fixes (fix range `9d7c859..7e2b11b`), plus a fresh coherence read of `pack/LEDGER.template.md` against `pack/AGENTS.md` and `pack/prompts/orchestrator.md`. Judged against AGENTS.md Principles and the `ledger-template` step detail. Line length ignored per the no-wrap convention (`Q-22`).

## Summary verdict

CLEAN. All four round-1 fixes landed correctly and are internally consistent. The fresh coherence read found no new defect at any severity. The tree is a stable fixed point (`just scaffold-self` then `git status --short` shows no changes) and `just test` passes (46 passed, 0 failed).

Counts: 0 critical, 0 high, 0 medium, 0 low.

## Fix verification

- V1 (Findings note, finding-file paths cited in the Round records narrative, not a table column): LANDED. The note now reads "The full findings live in each agent's file under `docs/plans/<task>.reviews/`; cite those file paths in the Round records section below rather than copying their text, so this table stays a compact index." It no longer implies a Findings-table path column. The Findings table columns (ID, Round, Severity, Triager verdict, Reasoning, Action) have no path column, which is now consistent with the note. The Round records section note (the section the fix points to) already invites recording "which reviewers and which separate triager ran (with their findings-file paths)", so the pointer resolves and is coherent.

- V2 (AGENTS.md "Preventing relitigation" paragraph and orchestrator.md ledger paragraph trimmed to defer to the template): LANDED. Both now say "in the format the scaffolded template pins (see below)". The old partial schema restatement ("one row per finding: the round it was raised in, the triager's verdict, the reasoning, and the action taken (fixed in `<commit>`, or dismissed because `<reason>`)") is fully removed from both files; no divergent schema restatement remains. The "(see below)" reference resolves within each file: in AGENTS.md, the later same-paragraph sentence "The ledger's format is pinned by the scaffolded template `.agents/LEDGER.template.md`, which the orchestrator copies to `docs/plans/<task>.ledger.md` at task start if that file does not yet exist..."; in orchestrator.md, the later same-paragraph sentence "At task start, create the ledger by copying `.agents/LEDGER.template.md` to `docs/plans/<task>.ledger.md` if that file does not already exist...". AGENTS.md keeps the outcome-vocabulary gloss "(new valid findings, or clean)" while orchestrator.md drops it; this is not a divergence (AGENTS.md is the canon that defines the vocabulary, orchestrator.md defers to the template), so it is not a finding.

- V3 (Consecutive clean column labelled a derived convenience view, Outcome authoritative): LANDED. The Round summaries note now ends "The Consecutive clean column is a convenience view derived from the Outcome column, which is authoritative." Coherent with one-source-of-truth (AGENTS.md Principle 16): Outcome is the authoritative field, the streak is derived.

- V4 (Round summaries example row reads `new valid findings`): LANDED. The example row now reads `<clean \| new valid findings>`, matching the defined term in the note's first sentence ("Outcome is exactly one of `clean` or `new valid findings`").

## Fresh coherence read (template vs workflow docs)

No disagreement found on any of the checked axes:

- Round-outcome schema: template partitions each round into exactly `clean` or `new valid findings` (zero findings counts as clean); AGENTS.md convergence bullets and orchestrator.md step 1/2 use the same two-value vocabulary. Agree.
- Countable counts, per-artifact: template says the total-round count and consecutive-clean streak are countable from the Round summaries table and both reset to zero when the loop moves to a new artifact or step; AGENTS.md "Tracking progress" and orchestrator.md say the same. Agree.
- No cross-round finding identity: template states the id is a within-round label and a round is scored by its outcome, not by tracking a finding across rounds; AGENTS.md convergence text agrees. Agree.
- Four-level severity: template Findings note uses `low`/`medium`/`high`/`critical`; matches reviewer.md, triager.md, and AGENTS.md. Agree.
- Findings-files reference-by-path rule: template (post-V1) tells the orchestrator to cite findings-file paths in the Round records narrative rather than copy text; AGENTS.md "Findings files" and orchestrator.md ("reference a finding by its file path rather than copying its text") agree. Agree.
- RESUME STATE / checkpoint procedure: template has a "RESUME STATE (compaction checkpoint, read this first)" block whose note defers to the checkpoint procedure in AGENTS.md; AGENTS.md "Checkpoint and resuming after context loss" names the plan, the ledger, and the plan's Open Questions queue as the durable state. Agree.

Nothing new was introduced by the fixes: the two pointer-style references (V1's "Round records section below", V2's "(see below)") both resolve, and no dangling or contradictory reference results.

## Verification commands (run for this review)

- `direnv exec . just scaffold-self && git status --short`: scaffold-self ran (15 changed, 0 left untouched by the render; `nix fmt` reported 0 changed) and `git status --short` returned no lines. Stable fixed point.
- `direnv exec . just test`: 46 passed, 0 failed, 0 ignored.

## Settled findings (not re-raised)

Round-1 findings V1-V5 are settled in the ledger (`docs/plans/agent-scaffold.ledger.md`, `ledger-template` round-1 entry). V5 (RESUME STATE subsection naming) was dismissed as invalid in round 1; I found no new evidence its verdict was wrong, so it is not re-raised.
