# Review: `workflow-invariants` (commit `04e26b1`) - correctness of the enforcement logic

Reviewer lens: correctness of the W3 / round-log enforcement. Read-only; ran build, tests, clippy, and `validate --workflow` against the repo's real `docs/metrics/workflow.jsonl`. Evidence is `file:line` in the worktree at `.claude/worktrees/workflow-invariants`.

Build state: `cargo test` = 108 passed, 0 failed. `cargo clippy --all-targets -- -D warnings` = clean. The claimed 108 + clean is accurate. The tests pass but do NOT prove the claims (see F1, F5): the fixtures model only the one round-record shape that happens to hide the central bug.

## Summary of findings

- F1 (critical): the consecutive-clean streak is grouped and checked PER ARTIFACT, but the log's `consecutive_clean` is a per-loop (per task/increment) running streak that accumulates across rounds naming different artifacts. Both the W3 streak check and the internal-consistency check use the wrong grouping key. Against the real log this emits ~33 false-positive W3 violations and 2 false-positive consistency violations and exits 1.
- F2 (high): the b1/b2 historical steps were never relabelled `grandfathered`/`trivial` in the Roadmap, so `validate --workflow` exits 1 on this repo's own data even setting F1 aside. The spec says the relabelling happens when this lands.
- F3 (low): `leading_slug` over-strips any task ending `-inc<alnum>` (a hypothetical slug like `foo-increment` collapses to `foo`); latent, no current slug triggers it, but the guard is purely lexical.
- F4 (low): the internal-consistency check accumulates the implied streak across a whole (task,artifact) history, so a legitimately re-opened loop that starts with a `clean` would be miscounted. Minor and interacts with F1.
- F5 (medium): the 12 new tests do not exercise the real multi-artifact-per-loop convergence shape, which is why F1 slipped. The `round-log-core` fixture uses a constant artifact per increment, matching only the minority of real steps.

## F1 (CRITICAL): the streak is grouped per-artifact, but the log's streak is per-loop

### The data model

`pack/instrument.md:5` defines `consecutive_clean` as "the streak after this round". In the real log a review loop for one task/increment runs several rounds, each naming a DIFFERENT `artifact` (what that round examined), and `consecutive_clean` is a single running counter across those rounds. Real records (`docs/metrics/workflow.jsonl`):

```
deliberation-mode  artifact "deliberation-mode change"        new_valid cc0 risky
deliberation-mode  artifact "deliberation-mode fixes"         clean     cc1 risky
deliberation-mode  artifact "deliberation-mode verification"  clean     cc2 risky
```

The streak climbs 0 -> 1 -> 2 across three distinct `artifact` values. The single `clean` record "deliberation-mode verification" logs `cc2`, which is only possible if the streak carried over from the prior "fixes" round: the counter is per LOOP (per task/increment), not per artifact. Same shape for `file-safety-rules` (change/fixes/T1 fix), `no-wrap-convention` (change/F1 fix/unchanged artifact), `instrument-flag` (change/fixes), and `state-schema-inc*`.

The design agrees with the per-loop model: `trivial-and-grandfather-A.md:46` computes "Streak per leading-slug = max `consecutive_clean` seen for that slug" (per slug, ignoring artifact) and lists `deliberation-mode` and `no-wrap-convention` as "2 converged (risky)", `state-schema`/`instrument-flag` as "1 converged". The design treats all of these as PASSING W3.

### The bug

W3 sub-groups each increment's records by artifact and requires EACH artifact label to independently reach the class streak:

- `src/workflow.rs:167-171`: `peak` is a `BTreeMap` keyed by `round.artifact`.
- `src/workflow.rs:172-184`: every artifact whose peak `< required` is reported.

For `deliberation-mode` the artifacts "change" (peak 0) and "fixes" (peak 1) each fall short of risky's 2, so the step is flagged three ways even though the loop reached `cc2`. Only the final round's artifact carries the converged value, so essentially every genuinely-converged multi-artifact step is false-flagged.

The internal-consistency check has the same defect:

- `src/workflow.rs:89-92`: groups by `(task, artifact)`.
- `src/workflow.rs:95-108`: recomputes the implied streak per (task,artifact) starting at 0.

For "deliberation-mode verification" (a lone `clean` record logged `cc2`) it recomputes `implied=1` and reports "records consecutive_clean 2 but its outcome sequence implies 1" - a false positive, because the real predecessor "fixes" round lives under a different artifact key.

### Observed impact (real repo data)

`cargo run -- validate --plan docs/plans/agent-scaffold.md --workflow` (exit 1) emits, among ~35 lines:

```
round log line 25: task `deliberation-mode` artifact `deliberation-mode verification` records consecutive_clean 2 but its outcome sequence implies 1
round log line 30: task `no-wrap-convention` artifact `no-wrap-convention unchanged artifact` records consecutive_clean 2 but its outcome sequence implies 1
Roadmap step `deliberation-mode` increment `deliberation-mode` artifact `deliberation-mode change` reached a consecutive-clean streak of 0 but its `risky` risk class needs 2
Roadmap step `deliberation-mode` increment `deliberation-mode` artifact `deliberation-mode fixes` reached a consecutive-clean streak of 1 but its `risky` risk class needs 2
Roadmap step `no-wrap-convention` ... `no-wrap-convention F1 fix` reached ... 1 ... `risky` ... needs 2
Roadmap step `state-schema` increment `state-schema-inc2` artifact `metrics validator` reached ... 0 ... needs 1
Roadmap step `instrument-flag` ... `instrument-flag change` reached ... 0 ... `low_risk` ... needs 1
```

`deliberation-mode`, `no-wrap-convention`, `state-schema`, `instrument-flag`, `triager-independence`, `file-safety-rules`, `agent-isolation`, `human-onboarding`, `gate-prompt-clarity`, `compaction-prep`, `human-review-queue`, `findings-files`, `ledger-template` are all in the design's "converged" set (`trivial-and-grandfather-A.md:54-75`) and MUST pass W3, yet all are flagged. Both lines 25 and 30 are VALID records under the per-loop model. These are not grandfather-gap cases: no amount of relabelling fixes them, because the check itself rejects data the design calls convergent.

Why the `round-log-core` reference case passes anyway: incA and incB each use ONE constant artifact per increment (`round-log-core-incA` / `round-log-core-incB`, `docs/metrics/workflow.jsonl:53-57`), so per-artifact peak coincidentally equals per-loop peak. `session-preflight` and `optional-modules-inc1` likewise use a constant artifact. The bug is invisible exactly for the single-artifact shape and fires for every multi-artifact loop.

### Fix direction

Group the streak by increment (full `task`) only, not `(task, artifact)`. For W3, take the peak `consecutive_clean` over ALL of the increment's records (matching the design's per-slug max) instead of sub-grouping by artifact (`src/workflow.rs:167-184`). For the consistency check, group by `task` only and walk the increment's rounds in file order (`src/workflow.rs:89-108`). This makes `deliberation-mode`/`no-wrap-convention` peak 2 (risky, pass), the b2 steps peak 0 (fail, correctly needing grandfather), and lines 25/30 consistent.

## F2 (HIGH): historical steps not relabelled, so the check ships red on this repo

The step detail says "The earliest steps that predate logging are relabelled `grandfathered` when this lands." The Roadmap still marks every historical step `complete`: `core-assets` (`docs/plans/agent-scaffold.md:123`), `convergence-accounting:135`, `round-log-core:161`, etc.; there is no `grandfathered` or `trivial` row in the Roadmap. The diff touches only `pack/plan-template.md`, `src/*`, not the Roadmap. Consequently the 11 b1 no-record steps and the 3 b2 short-streak steps are flagged (see the run above: `core-assets` ... `init-vcs` "has no round records"; `convergence-accounting`/`pack-rebuild-tracking`/`user-prompts-dir` "streak of 0"). Independently of F1, `validate --workflow` cannot go green on the project's own artifacts. If the relabelling is intended as a separate follow-up that should be stated; as delivered the feature fails its own detection on landing.

## F3 (LOW): `leading_slug` over-strips a slug ending in `-inc<alnum>`

`src/workflow.rs:52-60` strips any trailing `-inc` followed by an all-alphanumeric run. A real step slug ending that way (e.g. a hypothetical `foo-increment`, or `bar-inca`) collapses to its prefix, so its records would be attributed to the wrong step and the real step would hit the no-records catch. No current slug triggers it (`instrument-flag` has no `-inc`, safe; `optional-modules-inc1` strips correctly). `-incA-incB` strips only the last marker via `rfind` (`workflow.rs:53`). All latent; the strip is purely lexical with no slug allowlist. Boundary tests at `workflow.rs:235-249` cover the benign cases but not the over-strip case; low severity because no live slug is affected.

## F4 (LOW): consistency streak accumulates across re-opened loops

`round_log_consistency_problems` (`src/workflow.rs:95-108`) accumulates the implied streak over the entire history of a group with no loop boundary. If a task/artifact converged, then was legitimately re-reviewed later starting from a `clean`, the recompute would keep climbing and false-flag the re-open. Not exercised by current data; interacts with F1 (fixing the grouping does not by itself add loop boundaries). Minor.

## F5 (MEDIUM): the new tests do not cover the real convergence shape

The 12 tests are green but the W3/consistency fixtures use a CONSTANT artifact per increment: `per_increment_grouping_...` and `check_workflow_...` set `artifact` to `src/metrics.rs` for every incB round (`src/workflow.rs:288-297, 376-381`); the streak/consistency fixtures reuse a single artifact `a`/`AGENTS.md`. None reproduces the change -> fixes -> verification multi-artifact loop that most real steps use, which is precisely the shape F1 mishandles. A fixture with three distinct artifacts and `cc` 0/1/2 in one increment (a `risky` step converging at 2) would have failed and caught F1. Missing critical cases: a converged multi-artifact loop (the F1 case), and an orphan task whose leading-slug matches no Roadmap step (covered indirectly by iterating steps, but untested). The risk-class-inconsistency and pause-catch tests are genuine and not vacuous.

## Items checked and found correct

- Exemptions: `trivial`/`grandfathered`/`skipped` are skipped because W3 only iterates `status == "complete"` (`src/workflow.rs:133`); tests `a_trivial_step_is_exempt` etc. confirm. `grandfathered`/`trivial` added to `ROADMAP_STATUSES` (`src/plan.rs`) and documented in `pack/plan-template.md`, drift guard covers them (test `the_review_exempt_terminal_statuses_are_accepted`).
- The pause catch (complete + zero matching records) fires (`src/workflow.rs:138-144`), verified on real data (`pause`-style b1 steps flagged) and in `check_workflow_catches_the_pause_pattern...`.
- Per-increment risk-class consistency and grouping-by-full-task are correct (`src/workflow.rs:148-162`); the round-log-core two-risk-class case passes for the right reason (increments judged separately). This half of the "per increment" requirement is right; only the per-artifact sub-grouping inside the increment (F1) is wrong.
- `required_streak` (`src/metrics.rs`): `low_risk` 1, `risky` 2; comparison `reached < required` (`workflow.rs:173`) has no off-by-one (a single `clean` cc1 satisfies low_risk).
- CLI: `--workflow` has `requires = "plan"` (`src/main.rs`), reuses `--metrics` (default path), reads each file once, reports into the shared problem list with exit 1, and skips with a stderr note when a file is absent; help text is accurate. Missing-file handling matches the rest of `run_validate`.
- `parse_rounds` (`src/metrics.rs`) is a best-effort projection that skips malformed/other-type records rather than re-reporting them (that is `validate_log`'s job); ordering is file order; line numbers are 1-based. Correct.
- ASCII-only: no non-ASCII characters introduced in the diff.

## Bottom line

F1 is a genuine correctness defect in the core enforcement logic: the streak is checked against the wrong grouping key, so `validate --workflow` rejects data the design defines as convergent and cannot be made green on this repo. F2 compounds it (relabelling not done). Both must be fixed before this can enforce anything. The exemption logic, CLI wiring, per-increment risk-class handling, and parsing are correct.
