# Review findings: ledger-template step

Reviewer: sonnet (consistency / principles lens) Commit range: 691fcbc..062d8cf Artifact: pack/LEDGER.template.md (new), pack/pack.toml (asset entry), src/manifest.rs (test entry), pack/AGENTS.md (reference sentence added), pack/prompts/orchestrator.md (create-from-template sentence added) Artifact classification: low-risk (workflow-doc and asset addition; no logic change)

---

## M1 - medium

**Location:** pack/LEDGER.template.md lines 19-22 (Findings table); pack/prompts/orchestrator.md line 7; pack/AGENTS.md line 55

**Problem:** The template's Findings section instruction says "Reference each agent's findings file by its path under `docs/plans/<task>.reviews/` rather than copying its text." orchestrator.md says the same: "reference a finding by its file path rather than copying its text." But the Findings table the template defines has no column for that path:

```
| ID | Round | Severity | Triager verdict | Reasoning | Action |
```

The six columns give no place to record which findings file a row came from. The Round records section (template line 26) does capture "findings-file paths" in narrative form, but the per-finding row in the Findings table - the relitigation record an orchestrator uses to decide whether to re-raise - has no designated field for the path reference. An orchestrator filling in the table per the column headers has no clear way to comply with the "reference by path" instruction. The two sources (table schema, instruction) are internally inconsistent in the template.

**Principle:** Principle 16 (one source of truth - the template is meant to pin the schema, but the schema it defines does not match the behaviour it instructs).

---

## L1 - low

**Location:** pack/AGENTS.md line 55 ("Preventing relitigation" paragraph); pack/prompts/orchestrator.md line 7; pack/LEDGER.template.md lines 19-22

**Problem:** Both AGENTS.md and orchestrator.md describe ledger finding entries as tracking "the round it was raised in, the triager's verdict, the reasoning, and the action taken" - a four-field summary that predates the template and was not updated to match it. The template's Findings table defines six columns: ID, Round, Severity, Triager verdict, Reasoning, Action. The prose omits Severity and ID. The Severity omission is the more consequential one: the convergence rule depends on knowing a dismissed finding's severity to decide whether to trigger the high/critical backstop check (AGENTS.md: "before a dismissed finding of high or critical severity... counts towards a clean round, have a second, independent triager confirm the dismissal"). If the orchestrator's only description of what to record is the AGENTS.md/orchestrator.md prose, it might not include a Severity column, breaking that check.

**Principle:** Principle 16 (the template is now the single source of the schema, but the prose description in two files still partially re-specifies it in a way that omits a field with operational significance).

---

## L2 - low

**Location:** pack/AGENTS.md line 55; pack/prompts/orchestrator.md line 7; pack/LEDGER.template.md line 19

**Problem:** The template's Findings section instruction lists three valid Action values: "fixed in `<commit>`", "dismissed because `<reason>`", and "accepted residual risk". Both AGENTS.md's "Preventing relitigation" paragraph and orchestrator.md's ledger description list only the first two: "fixed in `<commit>`, or dismissed because `<reason>`". The "accepted residual risk" value is mentioned elsewhere in AGENTS.md (convergence section: "A valid finding may instead be resolved by consciously accepting its residual risk and recording that; an accepted risk does not block convergence") and in orchestrator.md (line 19: same phrasing), but neither the AGENTS.md ledger-field description nor the orchestrator.md per-finding row description includes it as an action value. An orchestrator writing up a finding where risk was accepted has no guidance from those descriptions on how to record it.

**Principle:** Principle 16 (three sources now describe action values differently).

---

## L3 - low

**Location:** pack/LEDGER.template.md line 15

**Problem:** The template's Round summaries instruction defines "Outcome is exactly one of `clean` or `new valid findings`" (emphasis on the full phrase). The example row in the same table shows `<clean \| new valid>` - dropping "findings" from the second option. An orchestrator writing the table from the example rather than the instruction might record "new valid" in the Outcome column rather than "new valid findings". The rest of the corpus (AGENTS.md, orchestrator.md) consistently uses the full phrase "new valid findings" as the outcome term. The shorthand in the example row contradicts the definition two lines above it.

**Principle:** Principle 16 (the authoritative term and the example disagree within the same document).

---

## L4 - low

**Location:** pack/LEDGER.template.md lines 28-30; pack/prompts/orchestrator.md line 23; pack/AGENTS.md lines 79-84

**Problem:** The template has a "RESUME STATE (compaction checkpoint, read this first)" section that is the key artifact a re-spawned orchestrator reads on resume. The template's own instruction says to flush and commit this section before a compaction. However, neither orchestrator.md's checkpoint procedure (line 23: "At each checkpoint, sync the durable state before moving on... update the plan's Open Questions queue and push its open items to the human") nor AGENTS.md's "Checkpoint and resuming after context loss" section (which says "the orchestrator flushes the plan, the ledger, and the plan's Open Questions queue to current") names the RESUME STATE block specifically. An orchestrator following those checkpoint instructions would flush the ledger (correct) but might not know to update the RESUME STATE block within it, since no external pointer calls it out. The template is self-describing, but an orchestrator must read and remember that section to maintain it; the checkpoint procedure is where that reminder belongs.

**Principle:** Principle 9 (leave durable notes that survive context loss - the mechanism exists but is not wired into the checkpoint procedure that should trigger it).

---

## Nothing at high or critical severity

No high or critical findings. The template's content is internally coherent aside from M1. Manifest entry and test position match the pack.toml order. The ownership classification (`reference`, no render) is correct for a template file that uses `<placeholder>` rather than `{{variable}}` substitution. The create-if-absent / never-overwrite semantics are stated consistently in AGENTS.md, orchestrator.md, and the template header. The four-level severity scale (`low`/`medium`/`high`/`critical`), the outcome terms (`clean` / `new valid findings`), and the findings-file path convention (`docs/plans/<task>.reviews/`) match across the template, reviewer.md, triager.md, and the AGENTS.md prose. The artifact-classification vocabulary ("low-risk" / "risky / high-blast-radius") is consistent between the template and AGENTS.md.
