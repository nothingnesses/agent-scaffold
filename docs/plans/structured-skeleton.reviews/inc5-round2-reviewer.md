# structured-skeleton Inc 5 - round 2 (final confirming) - independent reviewer

Increment `707532a..cbc454c` (branch `impl/structured-skeleton-inc5`, 4 commits: prep `5b45506`, fix `9c247e9`, cutover `6700c44`, migrate-removal `cbc454c`). Fresh independent reviewer; did NOT author the code. Read via `git show`/`git diff` from the main repo and ran read-only in the worktree at `.claude/worktrees/structured-skeleton-inc5` (HEAD `cbc454c`).

Round 1 was clean from two reviewers (`inc5-reviewer-integrity.md`, `inc5-reviewer-cutover.md`) plus the prep review. This round is a genuine adversarial full-increment re-verification from a different starting point (my own probes and data reconstruction), not a re-read of theirs.

## VERDICT: CLEAN

No critical / high / medium / low findings after a genuine adversarial pass. Every acceptance clause in the Inc 5 contract holds on the live migrated data. The two consecutive clean rounds condition is met (round 1 clean, round 2 clean): this converges the increment. Real-data enforcement is intact and NON-VACUOUS (I independently re-proved W3 and W5 bite). Worktree left clean at `cbc454c`.

## What I verified (independent evidence)

### 1. Cross-substrate real-data enforcement: PASS (non-vacuous)

`validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow --source docs/plans/agent-scaffold.plan.toml`:

```
docs/metrics/workflow.jsonl: 111 records, valid
docs/plans/agent-scaffold.plan.toml: 51 steps, 46 questions, valid
docs/plans/agent-scaffold.md: generated projection of a TOML-primary source; skipping the Markdown plan validator
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

Exit 0.

Non-vacuity probes (run on scratch copies of the committed TOML under the scratchpad, never against the worktree):

- W3 pause.md catch INTACT. Deleted the `core-assets-w1` `[[step.waiver]]` block -> validate exits 1: "Roadmap step `core-assets` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it if it predates logging or its review was skipped". An undeclared complete-with-no-evidence step still fails; the skip logic did not neuter W3.
- W5 cross-substrate join INTACT and unit-scoped. Re-pointed `waiver-model-w1`'s `evidence` from `waiver-model` to `nonexistent-escalation-probe` -> validate exits 1: "TOML waiver `waiver-model-w1`: `record-backed` waiver cites evidence `nonexistent-escalation-probe` but no `type:"escalation"` record with `human_decision` `decision` is scoped to this waiver's unit". The record-backed tier cannot be laundered by pointing at a nonexistent escalation.

Both accepted-at-escalation record-backed waivers pass across the substrate split on the real data (the headline risk): `optional-modules-w1` (evidence `optional-modules-inc2cii`) and `waiver-model-w1` (evidence `waiver-model`) both join their RETAINED JSONL `type:"escalation" human_decision:"decision"` records; `validate --source` reports no W5 problem. Baseline probe on an unmodified scratch copy of the TOML: workflow invariants hold, exit 0 (so the probe harness itself is not false-greening).

### 2. JSONL prune (128 -> 111): PASS, deletions-only, byte-exact

- `diff <(git show 707532a:...jsonl | grep -vE '"type":"(waiver|baseline)"') <(git show cbc454c:...jsonl)` -> IDENTICAL. The new file equals the old file with EXACTLY the typed lines filtered out; every retained event is byte-unchanged and in original relative order.
- Removed-line type census: exactly 16 `"type":"waiver"` + 1 `"type":"baseline"`. Added lines (excluding the `+++` header): 0. Line counts 128 -> 111 (delta 17). No `round`/`escalation`/`decision`/`intake`/`dismissal` line was removed, reordered, or edited.
- Faithful re-home into the TOML: 16 `[[step.waiver]]` entries, `[meta].w4_baseline = "Q-44"`, the 2 record-backed waivers carry `evidence` pointers matching the pruned records; both cited escalations survive the prune.

### 3. Projection integrity: PASS

- `render --check docs/plans/agent-scaffold.plan.toml`: "up to date", exit 0. Same with `--strict`: "up to date", exit 0. The committed `.md` is byte-identical to a fresh render; no hand-edit slipped in.
- `nix fmt`: "formatted 3 files (0 changed)"; `git status` clean afterward; `render --check` still "up to date" after fmt. `flake.nix` `settings.global.excludes` lists `docs/plans/agent-scaffold.md` (line 49) alongside the render fixtures, so the generated `.md` cannot be reflowed. The formatter cannot break the projection.
- `validate --source`: 111 records valid, 51 steps, 46 questions valid, exit 0.
- `status --source`: 51 steps (38 complete, 4 deferred, 1 in progress, 3 not started, 3 optional, 2 skipped); 46 open-questions items; 111 records. Exit 0.
- `status --json --source`: `"records": 111`, exit 0.

### 4. Data-loss in the generated `.md`: PASS, no prose lost

I ran an independent line-level check: every non-blank, non-header line of `git show 707532a:docs/plans/agent-scaffold.md` grep-searched (fixed-string) in the committed `.md`. 65 lines did not grep-match verbatim; I classified ALL 65 and confirmed the content survives:

- Project Principles 1-8: the pre-cutover `n. name: text` lines re-render as `n. name - text` (separator canonicalised). The text is preserved (distinctive Principle 8 phrases "project for humans" and "prioritise the best long-term design" both present in the new `.md`).
- Roadmap table: the pre-cutover 2-col `| Step | Status |` (51 rows + 2 header/rule lines = 53 lines) re-renders as a 3-col `| Step | Status | Notes |` table. All 51 step slugs appear as table rows in the new `.md` (grep count 51); nothing dropped, only the Notes column added.
- Queue items Q-19 / Q-36 / Q-39 / Q-43: the leading status parenthetical was retyped (`folded into X and Y` -> single typed `folded_into`; `(superseded)` -> `(superseded by ...)`), so the full line does not grep verbatim, but each ask body is present ("pickup counterpart to", "generalise reviewer/triager diversity beyond one harness", "treefmt v2 (this project", "second permanent hand-authored home for the option-labels" all found). Secondary fold targets survive in the ask prose (settled in round 1).

No line falls outside these three canonicalization categories. Zero actual prose loss, consistent with round 1.

### 5. Option-3 skip (`9c247e9`): PASS, scoped correctly

Read `src/main.rs` `run_validate` (lines 715-830). `plan_is_projection = source_plan.as_ref().is_some_and(plan::PlanToml::is_toml_primary)` is true ONLY when `--source` parses AND `[meta].primary == "toml"` (`src/plan/source.rs:370-372`). `validate_plan(--plan)` is skipped exclusively in that branch; the summary line names the `.md` as a generated projection. Every other branch (no `--source`, Markdown-primary, or unparseable source, since `is_some_and` is false on `None`) runs `validate_plan` unchanged. `plan_contents` is still returned `Some(contents)` in both branches, so the downstream `--workflow` match is unaffected. The `--workflow` block recomputes `toml_primary` from the same `is_toml_primary` predicate (consistent). The integration test `tests/validate_toml_primary_skips_markdown_plan.rs` pins both directions (TOML-primary skips; Markdown-mode still fails on the same fixture), making the skip non-vacuous. The gate that protects the projection (`render --check --strict`) is declared in `.agents/checks.toml` and verified green above.

### 6. Migrate removal: PASS, clean, no dead code

- `src/plan/migrate.rs`: absent. `src/main.rs`, `src/plan.rs`: no `migrate` wiring (verified in round 1; re-confirmed no `migrate` symbol references remain). The only `migrate` string in `src/` is the doc-comment in `src/plan/source.rs:107` describing `w4_baseline` ("migrated with pre-existing decisions"), which is retained descriptive content, not dead code.
- `cargo test --all-targets`: 293 + 1 + 3 + 1 = 298 passed, 0 failed. 298 = 301 - 3 matches exactly the 3 removed migrate unit tests; no other coverage dropped.
- `cargo clippy --all-targets -- -D warnings`: clean, exit 0.

### 7. Checks wiring / flake exclude: PASS

`.agents/checks.toml` declares one `render-check` lint running `agent-scaffold render --check ... --strict`, with a comment that this is a narrow hand-curated file (not full self-adoption). Well-formed; `--strict` fails hard on divergence. The "+ CI" clause resolves to the checks module since no `.github/workflows` exists (SETTLED). `flake.nix` exclude confirmed above.

## Reversibility

Not re-run this round (round 1's cutover reviewer executed `git revert 6700c44`, confirmed the Markdown path restores to green at 128 records, then `git reset --hard HEAD~1` back to `cbc454c` clean). The contract's revert property is structurally evident: `6700c44` is a single commit touching 5 files (`.agents/checks.toml`, `docs/metrics/workflow.jsonl`, `docs/plans/agent-scaffold.md`, `docs/plans/agent-scaffold.plan.toml`, `flake.nix`), so a `git revert` of it restores the hand-authored `.md`, the pruned JSONL lines, and `primary = "markdown"` atomically. I did not repeat the revert to avoid unnecessary worktree churn; round 1's evidence stands and I found no reason to doubt it.

## Settled items NOT re-raised (per instructions, no new evidence found)

The "+ CI" -> `.agents/checks.toml` resolution; `render --check` needing `agent-scaffold` on PATH; the inline-ask / empty question sidecars / no `## Question Details` editorial choice; multi-fold secondary targets kept in ask prose; sparse increments; and the option-3 `validate_plan` skip design. I looked at each for NEW evidence of a defect and found none.

## Worktree state

`cd .claude/worktrees/structured-skeleton-inc5 && git status --short` -> empty; HEAD `cbc454c`. I edited only scratchpad copies of the TOML (probe fixtures under the session scratchpad), never files in the worktree or main repo. No revert experiment was run this round, so no reset was needed. Worktree is clean at `cbc454c`.

## Bottom line

CLEAN. Zero findings at any severity after a genuine independent adversarial pass. Real-data enforcement holds and is non-vacuous (W3 pause.md catch and W5 record-backed cross-substrate join both independently re-proved to bite; both accepted-at-escalation waivers pass). The JSONL prune is deletions-only and byte-exact; no prose lost in the generated `.md`. All source-primary read paths green; projection pinned and formatter-stable; migrate removal clean with 298 tests green and clippy clean. This is the second consecutive clean round, which converges Inc 5.
