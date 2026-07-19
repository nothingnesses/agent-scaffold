<!-- reviewer: consistency (contract adherence, CLI design, docs, test fidelity) -->
<!-- commit: 36a872f on base 3f29a81 -->
<!-- scope: W3/W4/W5 + status at the TOML source (Q-46) -->

# Inc 4 Review: Contract Adherence, CLI Design, Docs, Test Fidelity

All 293 tests pass. Two findings and one minor test-comment note follow.

## Contract check

The Inc 4 bullet in `docs/plans/agent-scaffold.md` (tag `structured-skeleton-inc4`) and the Q-46 decision require:

- W3/W4/W5 + `status` read steps/questions/waivers from `parse_toml` and the baseline from `[meta].w4_baseline` when `[meta].primary == "toml"`. Yes.
- W4 decided-gate becomes `status == "decided"` reading `folded_into`. Implemented via `question_views()` projecting decided questions to `QUEUE_FOLD_PREFIX + "`" + slug + "`"`, which W4's existing `starts_with` guard matches without forking. Yes.
- Cutoff from `[meta].w4_baseline`. Implemented via `baseline_from_toml`. Yes.
- W5 reads `[[step.waiver]]` and joins each record-backed waiver's `evidence` to the JSONL escalation. Implemented via `waivers_from_toml` + unchanged `w5_problems`. Yes.
- `[meta].primary == "markdown"` fallback leaves the live repo unaffected. Gated by `is_toml_primary()` in both `run_validate` and `run_status`. Yes.
- One implementation for both substrates (Principle 16). `run_checks` is the single function both `check_workflow` and `check_workflow_toml` funnel into; no forked check logic. Yes.
- No scope creep (Principle 8). The `run_checks` refactor is Principle 16, not overbuilding; `report_workflow` de-duplicates identical formatting in two match arms. Nothing over-built.

Synthesis section 4 says waivers and the baseline stay JSONL-sourced; the Q-46 decision (folded into the plan bullet) overrides that recommendation and is what the implementation correctly follows.

## Fixture fidelity

The two accepted-at-escalation TOML fixtures are faithful to the real `workflow.jsonl` records.

**optional-modules-inc2cii** (real: lines 78-83 in `docs/metrics/workflow.jsonl`). Real shape: four `new_valid` rounds (consecutive_clean=0 each), then the escalation (`human_decision:"decision"`), then one `clean` round (consecutive_clean=1). Fixture: one `new_valid` + one `clean` (peak=1) + escalation. Essential properties match: risky class, peak streak 1 (< required 2), escalation with `human_decision:decision`. Order of escalation vs. clean round is reversed in the fixture but does not affect W3 (peak over all records) or W5 (any-match). The simplified round count does not introduce false pass or false fail.

**waiver-model self-waiver** (real: lines 106-112). Real shape: two `new_valid` rounds, one `clean` (peak=1), one `new_valid`, one `clean` (peak=1 again), escalation (`human_decision:"decision"`), then the JSONL waiver record. Fixture: one `new_valid` + one `clean` (peak=1) + escalation. Essential properties match: risky class, peak streak 1 (< required 2), escalation present. The self-referential increment id `"waiver-model"` (no `-inc` suffix) correctly passes W5's scope check because `leading_slug("waiver-model") == "waiver-model" == step.slug`. Faithful.

## Findings

---

### S1 - Medium - `src/workflow.rs:420-483`

**W5 error messages say "round log line N" for TOML waivers.**

`w5_problems` formats every violation as `"round log line {}: ..."` where `{}` is `waiver.line`. For JSONL waivers `waiver.line` is the 1-based line number in the round log; for TOML waivers `waiver.line` carries the 1-based document-order position set by `waivers_from_toml` (a counter incremented per nested `[[step.waiver]]` entry, including dropped ones). The `waivers_from_toml` doc comment acknowledges this ("there is no JSONL line for a TOML waiver, so it is a stable disambiguator for the shared W5 message rather than a real log line"), but the message itself is not fixed.

A user of a TOML-primary plan who gets a W5 violation will see, for example:

```
round log line 1: record-backed waiver cites evidence `x` but no `type:"escalation"` record ...
```

"Round log line 1" implies a JSONL record at line 1, but the waiver is in the TOML file. The user will look at `docs/metrics/workflow.jsonl` line 1 and find an unrelated record. This is the only user-visible error path for TOML-sourced checks that names a misleading file and line.

**Suggested direction.** The `Waiver` struct already carries `line: usize`. Add a `label: String` field (set to `format!("log line {}", index + 1)` in `parse_waivers` and `format!("TOML waiver {}", position)` in `waivers_from_toml`), then replace the four `format!("round log line {}: ...")` prefixes in `w5_problems` with `format!("{}: ...", waiver.label)`. This keeps Principle 16 (one `w5_problems`) and produces unambiguous messages for both substrates. Alternatively, include the waiver `id` field (already in the TOML schema) in the `Waiver` struct for the TOML path and emit `"TOML waiver `<id>`"` instead.

---

### S2 - Low - `src/main.rs:379-382`, `src/main.rs:383-387`, `src/main.rs:660-671`

**Three doc strings are stale after the Inc 4 source-swap.**

(a) The `--workflow` clap help reads: "Needs both the plan (via --plan) and the metrics log (via --metrics, which defaults). Requires --plan." This is now incomplete: when `--source` is TOML-primary, `--workflow` reads from `--source` instead and `--plan` is ignored (though still syntactically required by the `requires = "plan"` constraint, which is deferred to Inc 5/6). A new user reading the help has no way to discover the TOML path.

(b) The `--source` help text in `ValidateArgs` reads: "to validate (its schema and internal cross-references). When omitted, no source is validated." It does not say that a TOML-primary `--source` ALSO gates `--workflow` (redirecting the check away from `--plan` and the JSONL waiver/baseline records).

(c) The `run_validate` function-level doc comment says: "With `--workflow` (which requires `--plan`), the plan status is cross-referenced against the round log." This does not mention the TOML-primary arm.

The `requires = "plan"` clap attribute is acknowledged as a deferred wart (Inc 5/6) and is not a finding on its own; the behavioural code is correct. The doc strings are the gap.

**Suggested direction.** Append to the `--workflow` help: "When `--source` is TOML-primary, reads steps/waivers/baseline from the TOML instead; `--plan` is still required by the current CLI constraint (to be relaxed in Inc 5)." Append to `--source` (ValidateArgs): "Also drives `--workflow` when `[meta].primary == \"toml\"`, replacing `--plan` as the check's plan-side source." Update the `run_validate` function-level doc to mention the TOML arm.

---

### S3 - Low - `src/workflow.rs:1561` (test comment)

**`check_workflow_toml_w5_rejects_a_mis_tiered_waiver` comment says "isolating the W5 rejection" but two W5 checks fire.**

The fixture has a step waiver with `reason = "predates-logging"` and `evidence_tier = "record-backed"` and `evidence = "x"`. The waiver passes `waivers_from_toml`'s presence filter (RecordBacked + Some("x")). In `w5_problems` two checks fire for it: (1) the evidence join fails (no escalation with task="x" in the empty log), emitting a "record-backed waiver cites evidence `x` but no escalation..." message; (2) the reason-tier pairing check fires, emitting "reason `predates-logging` must not carry evidence tier `record-backed`". The test assertion (`any(|p| p.contains("reason `predates-logging`must not carry evidence tier`record-backed`"))`) is correct and passes, but the comment says "isolating the W5 rejection" as though only one W5 violation fires.

**Suggested direction.** Adjust the comment to note that two W5 problems fire (missing escalation AND reason-tier mismatch) and the assertion covers the reason-tier one; or restructure the fixture so only the pairing problem fires (give the evidence pointer a real escalation record so the join check passes, leaving only the tier mismatch).

---

## Ruled out

- **Duplicate check logic / Principle 16 drift.** `run_checks` is the single implementation. No forked W3/W4/W5 paths.
- **`waivers_from_toml` vs `parse_waivers` parallelism.** The increment-presence and evidence-presence filters are identical in structure; the only intentional difference is `step` sourced from the enclosing TOML step rather than a JSONL field.
- **`baseline_from_toml` safe direction.** An absent or non-`Q-<n>` `w4_baseline` yields no baseline, which requires a receipt for every decided item. This matches the contract and mirrors the JSONL path when no `type:"baseline"` record is present.
- **`question_views()` empty `folded_into`.** When `folded_into = None`, the projected status is `"decided -> folded into \`\`"`, which still starts with `QUEUE_FOLD_PREFIX`so W4 correctly treats it as a decided item.`validate_source`(Inc 1) should reject a decided question with no`folded_into`, so this branch is defensive.
- **`toml_source` helper used once.** `run_status` uses `toml_source`; `run_validate` has its own inline parsing (it must also run `validate_source`, so it cannot share the helper directly). Minor structural asymmetry, not a Principle 16 violation given the different duties.
- **Test coverage adequacy.** Pause.md catch, clean convergence, both cross-substrate record-backed waivers, W5 mis-tier, W5 wrong escalation, W4 baseline cutoff, W4 with-receipt, W4 no-baseline, malformed-drop, step-unit cover: all exercised.
- **`primary = "markdown"` gate.** `is_toml_primary()` is a one-liner tested in `source.rs`; the fallback path uses the unchanged `check_workflow` / Markdown plan. The live repo (still Markdown-primary) is unaffected by this increment.
- **The `requires = "plan"` clap wart.** Explicitly deferred to Inc 5/6 in the contract. Acceptable for Inc 4 because no real TOML-primary project exists yet. The fallback (pass `--plan` to a non-existent path so `plan_contents` is None, then the TOML arm fires) is workable during the migration window.
