# Triage verdicts: `ledger-template` step (round 1)

Triager: opus (independent of the producer and the orchestrator). Change under review: `691fcbc..062d8cf`. Artifact: `pack/LEDGER.template.md` (new), with `pack/pack.toml` / `src/manifest.rs` wiring and reference sentences in `pack/AGENTS.md` and `pack/prompts/orchestrator.md`. Step detail: `docs/plans/agent-scaffold.md` "### `ledger-template`" (`Q-2`, `Q-3`). Principle citations use the plan's own Project Principles 1-7 (`docs/plans/agent-scaffold.md:14`): Principle 1 (cleaner long-term architecture, internal coherence, maintainability, which subsumes one-source-of-truth here), Principle 2 (minimal by default), Principle 5 (make illegal states unrepresentable). Reviewer citations to the shipped-AGENTS.md numbering ("Principle 16" = one source of truth, "Principle 13" = illegal states) are normalised to these.

Sources read directly (Q-14): `ledger-template-reviewer-opus.md` (correctness lens: L1, L2), `ledger-template-reviewer-sonnet.md` (consistency lens: M1, L1-L4), plus the template, both edited docs, the step detail, and this project's own live ledger (`docs/plans/agent-scaffold.ledger.md`) as evidence of intended usage.

Reviewer findings map to four distinct issues after dedup:

- V1 = sonnet M1 (no findings-file-path column vs the "reference by path" note).
- V2 = opus L1 + sonnet L1 + sonnet L2 (one theme: AGENTS.md/orchestrator.md prose restates a partial, drifting finding-row schema).
- V3 = opus L2 (the "Consecutive clean" column stores a derivable value).
- V4 = sonnet L3 (example row "new valid" vs defined term "new valid findings").
- V5 = sonnet L4 (RESUME STATE block not named in the checkpoint procedures).

No finding is about line length or wrapping. No finding is high or critical, so no backstop re-check is triggered.

---

## V1 (sonnet M1): Findings table has no findings-file-path column, but the note and orchestrator.md say to reference a finding by its path

Verdict: VALID. Severity: LOW (corrected down from the reviewer's medium).

Reasoning: the template's Findings-section note (`pack/LEDGER.template.md:19`) and `pack/prompts/orchestrator.md:7` both say to reference a finding by its `.reviews/` file path rather than copying its text, yet the Findings table (`:21-22`, columns ID / Round / Severity / Triager verdict / Reasoning / Action) has no path column. The note sits directly above that table, so a reader can reasonably expect a per-row place to record the path and find none. That is a real internal awkwardness.

It is LOW, not medium, because it breaks nothing operational and the path is in fact captured elsewhere by design: the Round records section (`:26`) explicitly says to record "which reviewers and which separate triager ran (with their findings-file paths)". This project's own ledger is the decisive evidence: its Findings table (`docs/plans/agent-scaffold.ledger.md:114-115`) is a bare header, and every findings-file path is recorded in the round-records narrative (e.g. `:181`, `:202`, `:209`, `:305`). So the intended pattern, path in the narrative, not a table column, already works in practice; the defect is only that the note is placed so as to imply a per-row column. Impact if unfixed is a mild ambiguity, hence low.

Recommended fix: do NOT add a "Findings file" column. A per-row path column would duplicate what the Round records narrative already holds (cutting against Principle 1) and add a column the project's own usage shows is unused (cutting against Principle 2, minimal). Instead, relocate or reword the "Reference each agent's findings file by its path ... rather than copying its text" clause so it is clearly satisfied by the Round records section (which already names findings-file paths), leaving the Findings table as the compact within-round relitigation record keyed by id. This matches how the live ledger actually did it.

## V2 (opus L1 + sonnet L1 + sonnet L2): AGENTS.md and orchestrator.md prose restate a partial, drifting finding-row schema

Verdict: VALID (one finding; opus L1, sonnet L1, sonnet L2 are the same theme). Severity: LOW.

Reasoning: `Q-2` (step detail, `docs/plans/agent-scaffold.md:358`) makes the template "the single source of the ledger format, referenced (not restated)" from `pack/AGENTS.md` and `pack/prompts/orchestrator.md`. Both files still restate the finding-row schema: "one row per finding: the round it was raised in, the triager's verdict, the reasoning, and the action taken (fixed in `<commit>`, or dismissed because `<reason>`)" (`pack/AGENTS.md:55`, `pack/prompts/orchestrator.md:7`). Measured against the template's schema (`:19-22`), that restatement is a strict-subset, drifting copy: it omits the Severity and ID columns (sonnet L1) and lists two of the three actions, dropping `accepted residual risk` (sonnet L2). This is exactly the divergence `Q-2` and Principle 1 (one authoritative source, derive the rest) wanted to avoid: a reader who consults only AGENTS.md gets a schema that disagrees with the designated single source.

It is LOW because the restatement predates this step, does not contradict the template (it is a subset, not a conflict), the added pointer sentence in AGENTS.md ("The ledger's format is pinned by the scaffolded template `.agents/LEDGER.template.md` ...") does establish the template as authoritative, and the operationally significant field sonnet flags, Severity, is not actually lost: the convergence and backstop prose in both files already turns on a dismissed finding's severity independently of this row description, and the fix restores it via the template anyway. So the impact is doc-drift risk, not a broken mechanism.

On the question posed (acceptable behavioural summary, or defer to the template): the prose should keep a short behavioural description of what the ledger records but stop enumerating the column list and action vocabulary, deferring those to the template as the schema source. Recommended fix: in both `pack/AGENTS.md:55` and `pack/prompts/orchestrator.md:7`, trim "one row per finding: the round ... the action taken (fixed in `<commit>`, or dismissed because `<reason>`)" to a behavioural phrase that names the template as the schema source (e.g. "one row per finding, per the schema, the columns and the verdict/action vocabulary, pinned by `.agents/LEDGER.template.md`"). AGENTS.md already carries the pointer sentence; orchestrator.md already carries the copy-from-template instruction, so the enumeration can simply be dropped in favour of those. This discharges `Q-2`'s "referenced (not restated)" and fixes all three sub-findings at once.

## V3 (opus L2): the "Consecutive clean" column stores a value the note says is derivable

Verdict: VALID. Severity: LOW.

Reasoning: the Round-summaries note (`pack/LEDGER.template.md:11`) says "The total-round count and the consecutive-clean streak are countable from this table" (i.e. derivable from the Outcome column), yet the table (`:13`) also carries a stored `Consecutive clean` column holding that streak. Storing a value declared derivable admits a drift state where the stored streak disagrees with the Outcome column (Principle 1, one source of truth; and a mild Principle 5, illegal-states, concern). It does not break recomputability, the Outcome column remains the source, and a running convenience count is normal in a hand-maintained ledger, so it is low.

Recommended fix (weighing Principle 2, minimal, on keep-vs-drop): keep the column but add a one-clause caveat that the Outcome column is authoritative and `Consecutive clean` is a derived convenience view. This satisfies Principle 1 (a labelled derived view is fine) while preserving the ergonomic value: the streak-versus-required-clean-count is the single number the convergence decision turns on each round, and this project's own ledger keeps the column (`docs/plans/agent-scaffold.ledger.md:27`), so the convenience is real. Dropping the column entirely (strictly more minimal) is an acceptable alternative if the implementer prefers it, but the labelled-convenience option is my recommendation.

## V4 (sonnet L3): example row uses "new valid" while the instruction defines "new valid findings"

Verdict: VALID. Severity: LOW.

Reasoning: the note (`pack/LEDGER.template.md:11`) defines "Outcome is exactly one of `clean` or `new valid findings`", but the example row (`:15`) shows `<clean \| new valid>`, dropping "findings". The rest of the corpus (`pack/AGENTS.md`, `pack/prompts/orchestrator.md`) uses the full term "new valid findings" consistently, and an orchestrator copying the example rather than the definition could record the shorthand. A small internal inconsistency within one document (Principle 1). Low.

Recommended fix: change the example row's `<clean \| new valid>` to `<clean \| new valid findings>` so the example matches the defined term.

## V5 (sonnet L4): RESUME STATE block not named in the checkpoint procedures

Verdict: INVALID.

Reasoning: the finding is that `pack/prompts/orchestrator.md:23` ("update the plan's Open Questions queue and push its open items ...") and `pack/AGENTS.md:79-84` ("the orchestrator flushes the plan, the ledger, and the plan's Open Questions queue to current") do not name the template's RESUME STATE block, so the trigger to update it is implicit. But the trigger is already covered: RESUME STATE is a subsection of the ledger, and both checkpoint procedures instruct flushing the ledger to current, which subsumes flushing its RESUME STATE block; the template itself names the block and points back to "the checkpoint procedure in `AGENTS.md`" (`:30`). Naming the ledger's internal subsections in AGENTS.md/orchestrator.md would restate the template's own structure in the very files `Q-2` says should reference (not restate) it, cutting against Principle 1. The mechanism is adequately connected as-is, so there is no defect. Even read charitably the impact would be negligible (low), well below the high/critical bar that would require a second-triager re-check.

---

## Round outcome

Round 1 produced 4 valid findings (V1, V2, V3, V4), all LOW severity, and 1 invalid (V5). No high or critical finding was raised or dismissed, so no backstop re-check is required.

Because round 1 has new valid findings, its outcome is NEW VALID FINDINGS, not clean; the consecutive-clean streak is 0. The artifact is classified LOW-risk (additive template asset plus test and doc references, well-contained, tested, reversible), so it needs one clean round to converge. Next: the implementer applies the four fixes above (all low, all in the template plus the two reference sentences), then a fresh round 2 runs; if that round is clean the artifact converges.
