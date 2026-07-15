# Reviewer findings: `ledger-template` step (correctness / completeness lens)

Reviewer: opus. Lens: CORRECTNESS and COMPLETENESS. Range reviewed: `691fcbc..062d8cf`. Judged against `AGENTS.md` Principles 1-22 and the step detail in `docs/plans/agent-scaffold.md` ("### `ledger-template`").

## Summary verdict

The change is correct and largely complete. The template pins the round-outcome schema `convergence-accounting` fixed, agrees with the AGENTS.md convergence / tracking / relitigation / findings-files sections, the asset wiring is correct and complete, `just test` passes (46/46), the asset drops, the doc references are mutually consistent, and there is no path by which scaffold-self can clobber a live ledger. Two low observations below; nothing higher.

- critical: none.
- high: none.
- medium: none.
- low: 2.

## Verified (no defect)

- Round-outcome schema matches `convergence-accounting` (`Q-1`, approach (b)). `pack/LEDGER.template.md:11` states the outcome is "exactly one of `clean` or `new valid findings`", the two-way partition; the total-round count and consecutive-clean streak are "countable from this table"; "both reset to zero when the loop moves to a new artifact or step"; and "There is no cross-round finding identity: a round is scored by its outcome, not by tracking a finding across rounds." The Findings note (`:19`) reinforces this: "The id is a within-round label for reference ... not a cross-round identity." This is exactly the schema the step requires (clean/new-valid per-round outcome, running total-round count, no cross-round finding ids) and agrees with `AGENTS.md:53` (Tracking progress: both counts per-artifact, reset on artifact/step change).
- Four-level severity, verdict, and action vocabulary agree with AGENTS.md as a whole. Template `:19` severity = `low`/`medium`/`high`/`critical` (matches `AGENTS.md:51`); verdict = `valid`/`invalid`; action = `fixed in <commit>` / `dismissed because <reason>` / `accepted residual risk`. The third action is grounded in `AGENTS.md:49` ("resolved by consciously accepting its residual risk and recording that"), so the template is consistent with the full document.
- Findings-files reference-by-path rule agrees. Template `:19` ("Reference each agent's findings file by its path under `docs/plans/<task>.reviews/` rather than copying its text") matches the AGENTS.md "Findings files" section (`:57`).
- Artifact classification captured. Template `:7` records low-risk (one clean round) vs risky/high-blast-radius (two clean rounds), classified once when the loop opens, matching `AGENTS.md:48`.
- RESUME STATE / checkpoint procedure consistent. Template `:28-30` names the RESUME STATE block, points to "the checkpoint procedure in `AGENTS.md`", and instructs flush-and-commit before compaction, consistent with the AGENTS.md "Checkpoint and resuming after context loss" section (`:79-84`). It complements the plan's Status line (the resume anchor) rather than contradicting it.
- Asset wiring correct and complete. `pack/pack.toml` gains `source = "LEDGER.template.md"`, `dest = ".agents/LEDGER.template.md"`, `ownership = "reference"` (correct: a template must be refreshed on rerun, and `rerun_refreshes_reference_but_skips_working` confirms reference assets refresh). The asset sits between `.agents/principles.toml` and `.agents/user-prompts/kickoff.md` in both `pack.toml` and the `builtin_manifest_lists_the_expected_assets` test (`src/manifest.rs:291`); the test order matches the manifest order. Asset drops: `.agents/LEDGER.template.md` is present and byte-identical to `pack/LEDGER.template.md`. `just test` passes 46/46.
- No live-ledger clobber. The reference asset is `.agents/LEDGER.template.md`; the per-task ledger `docs/plans/<task>.ledger.md` is NOT a manifest asset (confirmed: no `ledger.md` dest in `pack.toml`; only working assets are `AGENTS.md` and `docs/plans/TEMPLATE.md`). scaffold-self therefore refreshes only the template, never the live ledger. The create-if-absent copy in `pack/prompts/orchestrator.md` and the AGENTS.md sentence agree word-for-word on intent ("if that file does not already exist", "never overwriting a live ledger"), so `Q-3` (per-task copy is a create-if-absent working file) is implemented correctly and the two decisions coexist without conflict.
- Single-source-of-format intent honored in substance. The template carries the full ledger format; `pack/AGENTS.md` and `pack/prompts/orchestrator.md` add a pointer to it rather than re-deriving the schema.

## Findings

### L1 (low): AGENTS.md ledger paragraph still restates the per-finding row schema the template is meant to be the single source of

- Location: `pack/AGENTS.md` "Preventing relitigation (the ledger)" paragraph vs `pack/LEDGER.template.md:19-22`.
- Problem: `Q-2` (step detail, `docs/plans/agent-scaffold.md:358`) says the template is "the single source of the ledger format, referenced (not restated) from `pack/AGENTS.md` and `pack/prompts/orchestrator.md`." The added sentence does add the pointer ("The ledger's format is pinned by the scaffolded template `.agents/LEDGER.template.md` ..."), but the same paragraph still restates the finding-row schema: "one row per finding: the round it was raised in, the triager's verdict, the reasoning, and the action taken (fixed in `<commit>`, or dismissed because `<reason>`)." That restatement is now a partial, drifting copy of the template's Findings schema: it omits the Severity and ID columns and lists only two actions where the template lists three (it drops `accepted residual risk`). So a reader who consults only AGENTS.md gets a schema that disagrees with the designated single source. This is the exact drift `Q-2`/Principle 16 (one source of truth) wanted to avoid.
- Principle: 16 (one source of truth); consistency with `Q-2`.
- Note on severity: low, not higher, because the restatement predates this step, does not contradict the template (it is a strict subset), and the pointer sentence does establish the template as authoritative. A tightening (trim the AGENTS.md restatement to the ledger's purpose and defer the column/action list to the template) would fully discharge `Q-2`'s "referenced (not restated)".

### L2 (low): the "Consecutive clean" column materializes a value the schema also says is derivable, a minor redundancy

- Location: `pack/LEDGER.template.md:11-15` (Round summaries note and table header).
- Problem: the note says "The total-round count and the consecutive-clean streak are countable from this table" (i.e. derivable from the Outcome column, the source of truth), yet the table also carries a stored `Consecutive clean` column holding that streak. Storing a value that is also declared derivable admits a drift state where the stored streak disagrees with what the Outcome column implies (Principle 16, and a mild Principle 13 illegal-state concern). `convergence-accounting` emphasized that "a re-spawned orchestrator could recompute the escalation decision from the ledger alone"; the Outcome column already satisfies that, so the stored streak is a convenience view, not a source.
- Principle: 16 (one source of truth); 13 (make illegal states unrepresentable), mildly.
- Note on severity: low. The column does not break recomputability (Outcome remains the source) and a running convenience count is common in a hand-maintained ledger; it is a redundancy to be aware of, not a correctness break. If kept, a one-clause note that Outcome is authoritative and the column is a derived convenience would remove the ambiguity.

## Completeness

Nothing decided in the step detail is missing. The template is the single source of the ledger format (round-outcome schema, finding schema, artifact classification, resume state), referenced from AGENTS.md and orchestrator.md rather than restated (modulo L1); the manifest entry, the test entry, and the doc references are all present; the backstop re-check, though not a dedicated column, is covered by the free-text "Round records" narrative section (which the step did not require as structured fields). No scope was expanded beyond the ledger format this step owns.
