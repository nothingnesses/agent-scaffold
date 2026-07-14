# Review: compaction-prep (Q-15) and resume prompt (Q-19)

Reviewer: independent reviewer (opus), mechanics-and-correctness lens.
Diff reviewed: `git diff 082280e 226ca33` (HEAD 226ca33).

Scope confirmed clean (no findings): pack.toml asset entries, manifest test and
order, byte-identity of generated mirrors, idempotency, README layout, ASCII
cleanliness. Details under "Verified" below. Two content findings follow.

## R1 (low): "clean-tree-before-writer discipline covers this" is a trigger-scope over-claim

Location: `pack/AGENTS.md` line 256 (and the generated mirrors `AGENTS.md`,
`.agents/AGENTS.reference.md` at the same line).

Evidence: the new checkpoint bullet says the orchestrator "commits everything
(the clean-tree-before-writer discipline covers this)." The rule it cites, at
`pack/AGENTS.md` lines 213-215, is scoped by its own wording to a different
trigger: "Clean tree before a writer. Commit pending work ... before spawning a
writer agent". A pre-compaction flush is not spawning a writer; it is a distinct
event (context loss / compaction). So the referenced rule does not literally
"cover" the compaction case: what is shared is the commit-before-risk mechanic,
not the rule's trigger. The plan step frames this correctly as reuse ("This
reuses the durability rules from `file-safety-rules` rather than inventing new
ones"), but the shipped parenthetical names one specific rule whose trigger does
not include compaction, so a reader who follows the pointer finds no
compaction trigger there. Wording like "the same commit-before-risk durability
discipline applies" would be accurate without over-claiming. Impact is
documentation precision only; the section states the pre-compaction commit
requirement independently, so nothing is functionally missing. Hence low.

## R2 (medium): the two prompts claim they do not restate the procedure, but they do

Location: `pack/user-prompts/compaction-prep.md` (header line 4, body lines
9-13) and `pack/user-prompts/resume.md` (header line 5, body lines 9-13); same
in the `.agents/user-prompts/` mirrors.

Evidence: `compaction-prep.md` header states "It triggers the checkpoint
procedure defined in `AGENTS.md`; it does not restate that procedure." The body
then enumerates that procedure step by step: "flush the plan, the ledger, and
the plan's Open Questions queue to current, verify the plan's Status line (the
resume anchor) is accurate, and commit everything" -- i.e. the exact four
sub-steps of the AGENTS.md checkpoint bullet (lines 253-257). `resume.md` is the
same pattern: header says "it does not restate it", body enumerates the
reconstruct order (AGENTS.md, then plan/Status line, then ledger). This is a
self-contradiction in shipped template text, and it diverges from the
established thin-trigger pattern: the sibling `kickoff.md` names *where* the
workflow lives ("the workflow and its rules live in `AGENTS.md`, so this prompt
only points the agent there") without enumerating any procedure, and truthfully
claims it "deliberately does not restate the workflow." The `user-prompts-dir`
step's stated goal (plan line 509) is that these prompts "stay thin triggers
that do not duplicate workflow content"; the enumerated steps here duplicate it.
Consequences: (a) the shipped self-description is factually inaccurate, and (b)
the duplication is a silent drift vector against Principle 4 -- if the AGENTS.md
checkpoint procedure later gains or drops a durable artifact, the prompt's
enumerated list goes stale with no test catching it (the asset-list test checks
only dest paths, not contents). Fix: either make the prompt bodies genuine thin
triggers (point at the AGENTS.md section, drop the step enumeration, as kickoff
does), or drop the "does not restate" claim from the headers. Medium: coherence
(Principle 1) plus a real maintenance-drift risk, though impact is
documentation-level, not functional.

## Verified (no findings)

- `pack/pack.toml`: two new `[[asset]]` entries, sources `user-prompts/
  compaction-prep.md` and `user-prompts/resume.md`, dests under
  `.agents/user-prompts/`, `ownership = "reference"`, no `render` key (defaults
  false, matching the kickoff entry). Correct.
- `src/manifest.rs`: `builtin_manifest_lists_the_expected_assets` adds both new
  dests immediately after `kickoff.md`, matching the pack.toml asset order
  (kickoff -> compaction-prep -> resume). `just test` passes 46/46. No other
  asset-count/list assertion is affected (`assets.len(), 1` at line 354 is a
  synthetic single-asset pack; `principles.len(), 1` at main.rs:476 is
  unrelated).
- Reference ownership => verbatim: `diff` shows `pack/user-prompts/*.md`
  byte-identical to their `.agents/user-prompts/*.md` mirrors. `just
  scaffold-self` leaves a clean tree (`git status` empty), so the generated
  mirrors are in sync and the run is idempotent (Principle 4).
- AGENTS.md section content matches the plan step: before = flush plan + ledger
  + Open Questions queue, verify Status line (resume anchor), commit; on resume
  = reconstruct from AGENTS.md + plan (Status line first) + ledger, continue not
  restart. It names the plan/ledger/notes rather than a harness memory feature,
  so it is harness-agnostic as required. Placement (after Writer isolation,
  before `## Principles`) sits within the durability cluster. Complete.
- README layout entries (`compaction-prep.md`, `resume.md` under
  `user-prompts/`) are accurate and correctly ordered.
- ASCII-clean: no non-ASCII bytes in any changed file (`pack/AGENTS.md`, both
  prompts, `README.md`, `pack/pack.toml`, `src/manifest.rs`).

## Severity counts

critical: 0; high: 0; medium: 1 (R2); low: 1 (R1).
