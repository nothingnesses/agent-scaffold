# structured-skeleton Inc 5 PREP - independent reviewer

Commit under review: `5b45506` on base `707532a`, branch `impl/structured-skeleton-inc5`.
Role: independent reviewer (did not author the code). Read via `git show`/`git diff`; ran read-only in the worktree.

## VERDICT: SAFE TO CUT OVER (no blocking defects)

Zero data loss confirmed by independent reconstruction. The shadow-render fidelity diff against the live `docs/plans/agent-scaffold.md` contains ZERO unexpected diffs: every difference is explained by an expected render-canonicalisation category. Live path is byte-identical to base. All gates green. The one thing to fix before cutover is not a correctness defect, it is a scope/hygiene choice (the `migrate` subcommand shipped publicly); see F1.

Evidence counts:

- Step Details: 320 non-blank lines live, 320 rendered, sorted-line diff EMPTY -> pure reorder, zero prose lost/altered.
- Open-Questions queue: 46 items live, 46 rendered; after stripping the status parenthetical and the derived `Receipt:` pointer, the sorted ask bodies are IDENTICAL (empty diff).
- Front/tail prose sidecars (Motivations, Documentation Protocol, Repository Layout, Success Criteria): each VERBATIM against the live section including its heading.
- Status narrative sidecar (`_status-narrative.md`): VERBATIM equal to the live preamble minus the H1 title line (both the Status paragraph and the intro paragraph preserved).
- Principles: alphanumeric character stream IDENTICAL (1793 chars each) -> zero word loss; only the `. `/`: ` separator and numbering are canonicalised.
- Regeneration: re-running `migrate` reproduces the committed `.plan.toml` content-identically (the only diff is TOML array pretty-printing applied by the formatter) and all 105 sidecars BYTE-IDENTICAL -> no hand-transcription; the structure comes from the shipped parsers.
- Unexpected fidelity diffs: 0.

## 1. Data-loss audit (the critical one): PASS

Method (independent, not the implementer's tests):

- Extracted the `## Step Details` body from the live plan and from a scratch render, dropped blank lines, sorted, diffed: 0 differences. Any dropped, truncated, reworded, or summarised line would appear on exactly one side; none did. The reorder is render emitting steps in Roadmap `order` rather than the live document's curated grouping order; content is preserved. The umbrella preamble ("Durability, recovery, isolation, and user prompts ...") renders immediately before its attached step (`file-safety-rules`), so the attach-to-next-step logic is correct.
- Extracted every `- \`Q-...\`` queue line from both; 46 == 46; stripped the leading status parenthetical and any trailing `Receipt: \`Q-n\``; sorted; diffed the ask bodies: 0 differences. Every question's prose is verbatim.
- Compared each front/tail sidecar to its live section (heading included): verbatim for all four meta prose blocks and Success Criteria.
- Compared the status-narrative sidecar to the live preamble (H1 removed, ends trimmed): verbatim.
- Compared the principles alnum stream: identical (only separators/numbering canonicalised).
- Distinctive-sentence spot checks (found in a sidecar, verbatim): "an implementer ran repo-wide `just fmt` then `git checkout` on the ledger it did not own" (`file-safety-rules`), "a later compaction flattened \"human-decision gate\" into \"agent-facing role prompt\"" (`gate-prompt-clarity`), "the pickup counterpart to `compaction-prep`'s flush" (Q-19 ask), "the 16 historical JSONL `waiver` lines + the 1 `baseline` line are PRUNED" (Q-46 ask).

Nothing is missing, truncated, summarised, or reworded. No blocking data-loss defect.

## 2. Fidelity diff: every diff is an EXPECTED category (0 unexpected)

Shadow-rendered the committed `.plan.toml` to a scratch path (render writes over `<task>.md` in place; captured the output and restored the live file, which is byte-identical to base afterwards). `diff live rendered` = 338 diff lines, all classified:

- Do-not-edit banner added at top. Expected.
- Derived Status line ("51 steps (...); 1 open questions; 16 waivers (...)") replacing the hand-authored Status narrative, which now lives verbatim in `_status-narrative.md`. Expected. The "16 waivers" count is correct (16 JSONL waiver records -> 16 `[[step.waiver]]` tables; a 17th raw grep hit was the literal `[[step.waiver]]` inside the Q-46 ask prose at line 910, not a table).
- `## Project Principles` relocated to render's fixed section order and regenerated as `n. name - text` (separator canonicalised from `. `/`: `). Full text preserved (alnum stream identical). Expected.
- `## Roadmap Status Vocabulary` section added (generated from code constants). Expected.
- `## Documentation Protocol` prose moved to `documentation-protocol.md` (verbatim) with the generated vocabulary now under `## Roadmap Status Vocabulary`. Expected.
- Roadmap intro ("Steps in implementation order ...") relocated as part of the section reorder. Expected.
- Roadmap table regenerated 2-col -> 3-col with a `Notes` column carrying the derived `waived: ...` annotations. Expected.
- Open-Questions queue re-emitted in Q-id order (live is curated order). Presentational reorder; asks proven verbatim. Expected.
- Q-19 / Q-36 multi-fold status collapsed to the single typed `folded_into` slug; Q-39 status drops the trailing `2c` increment qualifier. The secondary fold targets and `2c` survive in the ask prose (verified). Expected (see F2).
- Q-43 `(superseded)` -> typed `(superseded by \`Q-44\`)`. Expected.
- Q-45 / Q-46 gain a derived `Receipt: \`Q-n\`` pointer. Expected.
- Step Details reordered to Roadmap order (the `file-safety-rules`/`agent-isolation`/`user-prompts-dir`/`gate-prompt-clarity`/`compaction-prep` cluster, `ledger-template`, and `reviewer-diversity` relocate as whole blocks). Content byte-identical (sorted diff empty). Expected.

No diff falls outside these categories. No unexpected content change or loss.

## 3. Tooling and gates: PASS

- `cargo test --all-targets`: green (296 + 1 + 3 passed, 0 failed). The migrate module's 3 tests are non-vacuous: `synthetic_migration_validates_and_renders` asserts `validate_source` is empty, the emitted plan parses, `primary == markdown`, the blocked row became a typed `blocked_by`, waivers re-homed with generated ids, the increment risk class inferred, the decided item carries its receipt, then renders the migration in a scratch dir and asserts 13 verbatim prose fragments survive and umbrella prose precedes its step; `step_details_reconstruct_verbatim` asserts document-order sidecar concatenation reproduces the body; `a_superseded_item_without_a_supplied_id_is_a_problem` asserts the missing-superseding-id error path.
- `cargo clippy --all-targets -- -D warnings`: clean.
- `validate --source docs/plans/agent-scaffold.plan.toml`: passes (51 steps, 46 questions, valid; metrics 128 records valid).
- Parser reuse: `migrate` calls the shipped `parse_roadmap`, `parse_questions`, `detail_slugs`, `parse_waivers`, `parse_baseline`, `parse_rounds`, `parse_decisions` for all structured/log data; regeneration is byte-identical to the committed artifacts, confirming no hand-transcribed structure.

## 4. Live-path invariance: PASS

- `docs/plans/agent-scaffold.md` and `docs/metrics/workflow.jsonl` are byte-identical to base `707532a` (`git diff 707532a 5b45506 -- <path>` empty for both).
- `validate --metrics ... --plan docs/plans/agent-scaffold.md --workflow`: green (51 steps, 46 open-questions items, workflow invariants hold).
- Committed `.plan.toml` carries `[meta].primary = "markdown"`, so `render`/`status`/`validate --workflow` keep reading the Markdown plan. Additive, no cutover.

## 5. Flagged editorial decisions

- F3 (queue asks kept inline, empty question sidecars + no `## Question Details`): ACCEPTABLE for cutover. No content is lost, only not split; the full ask is inline in the queue and proven verbatim, and the 46 question sidecars are 0 bytes so render emits no `## Question Details` entries (the rendered output has zero occurrences of "Question Details"). The queue is readable. The short-ask/long-body split is deferred editorial work, correctly out of scope for a lossless prep.
- F2 (multi-fold Q-19/Q-36 and Q-39 `2c`): PRESERVED, ACCEPTABLE. The typed `folded_into` holds only the primary slug, so the derived status shows a single target, but the secondary target and the `2c` qualifier survive verbatim in the ask prose ("... per `user-prompts-dir`", "... and `optional-modules` increment 3", "... worked out in 2c ..."). No pointer is lost. Optional future improvement: a multi-target `folded_into` would let the derived status name every target, but it is not required for a lossless cutover.
- Increments declared only where a waiver references them (5 `[[step.increment]]`, all on waiver-bearing steps): ACCEPTABLE for prep. `validate --source` passes and W3/W4/W5 read the live Markdown path under `primary = "markdown"`, so the sparse increment list does not affect any live gate now. Note for the cutover step (not this prep): once `primary` flips to `toml`, confirm whether the increment list needs to be complete for the intended `status`/W3 semantics; today it is intentionally minimal (only what a waiver cites).
- migrate subcommand shipped publicly (F1 below): see findings.

## Findings

- F1 - LOW (non-blocking, hygiene) - `src/main.rs` `Command::Migrate` / `src/plan/migrate.rs`. The one-time repo-migration tool is exposed as a public, un-hidden CLI subcommand that every scaffolded downstream project inherits. It is safe (additive, refuses to clobber without `--force`, writes `primary = "markdown"`), so this is not a correctness defect and does not block cutover. Suggested direction: after the cutover lands, either remove `migrate` or mark it hidden (`#[command(hide = true)]`) since it is a one-shot maintenance command for this repo, not a workflow feature end users need. Decide with the human whether it stays as a documented escape hatch.

- F2 - INFO (not a defect) - multi-fold status annotations (`folded into X and Y`, `... 2c`) collapse to the single typed `folded_into` in the derived status; the secondary targets are preserved in the ask prose. Recorded for the human's awareness; no action required for a lossless cutover.

Operational note for future reviewers (not a finding against this commit): `agent-scaffold render <plan>.plan.toml` writes over the sibling `<plan>.md` IN PLACE (its Inc-3 designed behaviour), not stdout. To shadow-render for a fidelity diff, render then immediately capture and `git checkout -- <plan>.md`, or render a copy of the source in a scratch directory. The live `.md` was restored and verified byte-identical to base after this review.

## Bottom line

Zero data loss (reconstruction diff empty for Step Details, queue asks, and every prose sidecar). Zero unexpected fidelity diffs. All gates green. Live path untouched. The prep is safe to cut over; F1 is a post-cutover hygiene choice for the human, not a blocker.
