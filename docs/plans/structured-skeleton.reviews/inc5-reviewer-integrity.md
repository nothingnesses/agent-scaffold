# Inc 5 reviewer: data-integrity + enforcement lens

Increment `707532a..cbc454c` (branch `impl/structured-skeleton-inc5`, 4 commits: prep `5b45506`, fix `9c247e9`, cutover `6700c44`, migrate-removal `cbc454c`). Reviewed read-only from the worktree at `.claude/worktrees/structured-skeleton-inc5` (HEAD `cbc454c`) and via `git show`/`git diff` from the main repo.

Verdict: CLEAN. No high/critical/medium/low findings. The JSONL prune is byte-exact deletions-only; the two accepted-at-escalation record-backed waivers both still pass across the substrate split; no prose was lost. `cargo test --all-targets` is 298 passed / 0 failed; `cargo clippy --all-targets -- -D warnings` is clean (exit 0). One low-severity informational observation (OBS-1) that is not a defect.

Evidence follows per verification item.

## 1. The JSONL prune (128 -> 111 lines): PASS, byte-exact, deletions-only

`git diff 707532a cbc454c -- docs/metrics/workflow.jsonl`:

- Single hunk header `@@ -83,33 +83,16 @@` (33 old context lines -> 16), i.e. 17 fewer lines, one contiguous region. No other region of the file is touched.
- Removed lines: 17 total. Added lines (excluding the `+++` file header): 0. Confirmed by `grep -E '^\+' | grep -v '^+++'` returning nothing.
- Removed-by-type: exactly `16 "type":"waiver"` + `1 "type":"baseline"`. No `round`/`escalation`/`decision`/`intake`/`dismissal_recheck` line appears with a `-` prefix.
- The 16 waivers removed are: 11 step `predates-logging` (core-assets, file-dropper, idempotency-safety, selection-ui, mode-enum, tag-selection, available-filter, pack-manifest, external-packs, pack-owned-principles, init-vcs), 3 increment `review-skipped` (convergence-accounting, pack-rebuild-tracking, user-prompts-dir), and the 2 increment `accepted-at-escalation` record-backed (optional-modules-inc2cii, waiver-model). The 1 baseline removed is `decision-receipt questions_through Q-44`.

Order + byte-fidelity of retained events (the strong check):

```
diff <(git show 707532a:...workflow.jsonl | grep -vE '"type":"(waiver|baseline)"') \
     <(git show cbc454c:...workflow.jsonl)
-> IDENTICAL
```

The new file equals the old file with exactly the 17 typed lines filtered out and nothing else: every retained event is byte-unchanged and in its original relative order. No round/escalation/decision line was dropped, reordered, or edited. New-file type census: 107 round + 2 escalation + 2 decision, 0 waiver, 0 baseline. Old-file census: 107 round + 2 escalation + 2 decision + 16 waiver + 1 baseline. The delta is exactly the 17 pruned typed lines.

Faithful re-home into the TOML (`docs/plans/agent-scaffold.plan.toml`):

- 16 `[[step.waiver]]` entries (grep count = 16), `[meta].w4_baseline = "Q-44"` (matches the pruned baseline's `questions_through`).
- The 3 `reason`/`evidence_tier` families match the pruned records: 11 `predates-logging`/`self-declared` (step-unit), 3 `review-skipped`/`self-declared` (increment-unit, each carrying its `increment = "..."`), 2 `accepted-at-escalation`/`record-backed` with `evidence` pointers `optional-modules-inc2cii` and `waiver-model` respectively (matching the pruned records' `evidence` field verbatim). Evidence and escalation pointers are intact.

## 2. Enforcement on the real migrated data: PASS

```
cargo run -- validate --metrics docs/metrics/workflow.jsonl \
  --plan docs/plans/agent-scaffold.md --workflow \
  --source docs/plans/agent-scaffold.plan.toml
```
prints, exit 0:
```
docs/metrics/workflow.jsonl: 111 records, valid
docs/plans/agent-scaffold.plan.toml: 51 steps, 46 questions, valid
docs/plans/agent-scaffold.md: generated projection of a TOML-primary source; skipping the Markdown plan validator
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

Why it holds:

- W3: complete steps converge via real rounds or a covering TOML waiver. The `predates-logging` and `review-skipped` complete steps have no rounds but are now covered by their `[[step.waiver]]` entries; the actively-reviewed steps (decision-receipt, waiver-model, etc.) have their round records retained in the JSONL. The pause.md catch still fires on a complete step with neither rounds nor a covering waiver (proved non-vacuously below).
- W4: baseline is read from `[meta].w4_baseline = "Q-44"`; decided items past it carry receipts in the retained JSONL decisions.
- W5: the 16 TOML waivers' `evidence` (for the 2 record-backed ones) join cross-substrate to the RETAINED JSONL escalations. Both cited escalations survive the prune: `{"type":"escalation","task":"optional-modules-inc2cii",...,"human_decision":"decision"}` and `{"type":"escalation","task":"waiver-model",...,"human_decision":"decision"}` are both present in the 111-line file.

The two accepted-at-escalation record-backed waivers specifically pass across the substrate split (the headline risk):

- `optional-modules-w1` (unit increment, `increment = "optional-modules-inc2cii"`, `evidence = "optional-modules-inc2cii"`) joins the retained `optional-modules-inc2cii` escalation. Holds.
- `waiver-model-w1` (unit increment, `increment = "waiver-model"`, `evidence = "waiver-model"`) joins the retained `waiver-model` escalation (the self-waiver). Holds.

Non-vacuity / un-launderability probes (run on scratch copies of the TOML, read-only against the repo):

- Delete the `core-assets-w1` waiver block -> validate exits 1 with: "Roadmap step `core-assets` is `complete` but has no round records and no covering waiver; log its review rounds, or record a `type:"waiver"` for it ...". W3 still bites.
- Re-point `waiver-model-w1`'s `evidence` to `nonexistent-escalation` -> validate exits 1 with: "TOML waiver `waiver-model-w1`: `record-backed` waiver cites evidence `nonexistent-escalation` but no `type:"escalation"` record with `human_decision` `decision` is scoped to this waiver's unit". W5's cross-substrate join is unit-scoped and cannot be laundered by pointing at a nonexistent (or wrong-unit) escalation.

## 3. The Option-3 fix (`9c247e9`, `run_validate`): PASS, both directions pinned non-vacuously

`src/main.rs`: `--source` is now parsed before the `--plan` validation block, and `plan_is_projection = source_plan.as_ref().is_some_and(plan::PlanToml::is_toml_primary)`. `validate_plan(--plan)` is skipped ONLY when `plan_is_projection` is true (source present AND `[meta].primary == "toml"`, per `src/plan/source.rs:370`). In every other branch (no `--source`, a Markdown-primary source, or an unparseable source, since `is_some_and` is false when `source_plan` is `None`) the original block runs unchanged: read the md, `validate_plan`, and either push the "N steps, M open-questions items, valid" summary or the per-problem messages. `plan_contents` is still returned as `Some(contents)` in both branches, so nothing downstream regresses.

`tests/validate_toml_primary_skips_markdown_plan.rs` pins both directions, non-vacuously:
- TOML-primary run (`--source` toml + `--plan` md) on a `--plan` md whose queue item uses the rendered form `(superseded by `Q-1`)` (which the standalone Markdown validator rejects) -> asserts exit 0, "workflow invariants hold", and "skipping the Markdown plan validator".
- Markdown-mode run (no `--source`) on the SAME md -> asserts exit 1 and stderr containing "unknown status" and "Q-2".
The Markdown-mode assertion is what makes the skip non-vacuous: it proves the md genuinely fails the standalone validator, so the TOML-primary pass is a real skip, not a vacuously-clean md. Test passes.

Nothing of value is lost by skipping `validate_plan` in TOML mode: `validate --source` validates the TOML schema and cross-refs, and `render --check docs/plans/agent-scaffold.plan.toml --strict` pins the committed md to a fresh render (verified: prints "docs/plans/agent-scaffold.plan.toml: up to date", exit 0). The gate is declared in `.agents/checks.toml` (`[[check]] name = "render-check" kind = "lint"`). The commit message's stated rationale (the projection writes `(superseded by `Q-x`)`, a form the Markdown queue-status vocabulary lacks) matches the test fixture and the observed behavior.

## 4. Data-loss in the committed generated `.md`: PASS, no prose lost

The committed `docs/plans/agent-scaffold.md` is `render` output (render --check up to date). Data-loss checks against the pre-cutover plan `git show 707532a:docs/plans/agent-scaffold.md`:

- Every non-empty, non-header prose LINE of the pre-cutover Step Details region (lines 187..741, 264 lines) appears verbatim in the committed md: missing = 0.
- Every non-empty, non-header prose LINE of the ENTIRE pre-cutover md (Motivations, Principles, Documentation Protocol, Repository Layout, Open Questions bodies, Roadmap, all Step Details, Success Criteria, and the ~1,200-word Status narrative) appears verbatim in the committed md: missing = 0.
- Distinctive-sentence spot-check across scattered Step Details and the Status narrative (12 hand-picked unique fragments, e.g. "the loop caught and fixed a silent false-pass in W4's boundary", "a compaction flattened the purpose of the planner's gate prompts", "This document plans a tool that scaffolds the agent workflow", "Verification convention:"): all present.

The Status narrative round-trips into `docs/plans/agent-scaffold._status-narrative.md` and reappears verbatim in the committed md, consistent with the Inc 5 acceptance ("spliced back verbatim ... round-trip with no diff"). No prose was missing, truncated, or reworded. (The committed md is longer in raw word count than the pre-cutover md, 39216 vs 38986, consistent with the expected additive diffs: banner + derived Status line; nothing was removed.)

## Observation (not a finding)

- OBS-1 (low / informational): the Inc 5 plan bullet says "wire `render --check` into `.agents/checks.toml` + CI". The `.agents/checks.toml` half is done (the `render-check` lint row), and `flake.nix` excludes `docs/plans/agent-scaffold.md` and the render fixtures from the formatter so `render --check` cannot be broken by a reflow. There is no `.github/workflows` directory in the repo (there was none before this increment either), so the "+ CI" clause resolves to the checks-module / pre-commit-hook mechanism (`agent-scaffold checks` + `scaffold --with-precommit-hook`) rather than a hosted CI workflow. This is consistent with the project's harness-agnostic, no-runtime-binary-dependency design and is not a data-integrity or enforcement defect; noting it only so the triager can confirm "CI" was meant as the checks module, not a missing GitHub Actions file.

## Housekeeping confirmed

- The one-time `migrate` subcommand is removed from `src/` (only an unrelated doc-comment mention of "migrated with pre-existing decisions" remains in `src/plan/source.rs`); consistent with commit `cbc454c`.
- `cargo test --all-targets`: 298 passed, 0 failed. `cargo clippy --all-targets -- -D warnings`: clean.
