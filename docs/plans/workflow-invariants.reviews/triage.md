# Triage: `workflow-invariants` (commit `04e26b1`)

Triager role: independent of both the implementer and the orchestrator. Each finding adjudicated on its merits against the code in the worktree (`.claude/worktrees/workflow-invariants`), the `workflow-invariants` spec in `docs/plans/agent-scaffold.md`, the round schema in `pack/instrument.md`, the design record `docs/plans/workflow-invariants.explorations/trivial-and-grandfather-A.md`, and the real log `docs/metrics/workflow.jsonl` (observed via `cargo run -- validate --workflow --plan docs/plans/agent-scaffold.md`, exit 1, 37 problem lines).

Build state confirmed as reported by both reviewers: the code builds, `cargo test` is green, clippy is clean. The green tests do not prove the logic (see T1/T5).

## Ground-truth for T1 (the critical finding)

`pack/instrument.md:5` defines `consecutive_clean` as "the streak after this round" with no per-artifact qualifier. The real log settles it empirically: a single review loop for one task runs several rounds each naming a DIFFERENT `artifact`, and `consecutive_clean` is one running counter across them:

```
line 23  deliberation-mode  artifact "deliberation-mode change"        new_valid cc0 risky
line 24  deliberation-mode  artifact "deliberation-mode fixes"         clean     cc1 risky
line 25  deliberation-mode  artifact "deliberation-mode verification"  clean     cc2 risky
```

The streak climbs 0 -> 1 -> 2 across three distinct artifacts. `cc2` on a lone `clean` record ("verification") is only possible if the counter carried over from the prior artifact's round: the streak is per LOOP (per task/increment), not per artifact. Same shape for `no-wrap-convention` (lines 28-30), `file-safety-rules` (11-13), and the other pre-`round-log-core` steps.

The design doc is decisive on intent. Its prose says "per-artifact" (A.md:3, 38), but the analysis it actually performed and every conclusion it drew use per-slug max: A.md:46-48 "Streak per leading-slug = max `consecutive_clean` seen for that slug", and the convergence table (A.md:50-75) lists `deliberation-mode` and `no-wrap-convention` as "2 converged (risky)" and expects them to PASS W3. That only holds under a per-loop max. The whole grandfather-boundary argument depends on it: the design says the ONLY real complete steps that fail are the 11 b1 (no records) and 3 b2 (short-streak) steps. Per-artifact grouping instead fails ~13 additional multi-artifact steps the design explicitly calls converged, which no relabelling can fix. The spec's parenthetical "(per artifact)" (agent-scaffold.md W3 paragraph) is the source of the implementer's literal reading and is itself inconsistent with the schema, the real log, and the design's own convergence table.

Observed impact (real run, exit 1): consistency false positives at lines 25 and 30 ("records consecutive_clean 2 but its outcome sequence implies 1"), plus W3 false positives on `deliberation-mode`, `no-wrap-convention`, `file-safety-rules`, `agent-isolation`, `human-onboarding`, `gate-prompt-clarity`, `compaction-prep`, `human-review-queue`, `findings-files`, `ledger-template`, `state-schema` (multiple increments) and more, all of which the design defines as converged. The single-artifact-per-increment steps (`round-log-core`, `state-schema-inc1`, `metrics-fields`, etc.) coincidentally pass because per-artifact peak equals per-loop peak there, which is exactly why the 12 tests (all single-artifact) missed it.

---

## Findings

### T1 (opus F1) - VALID, CRITICAL. Owner: implementer. BLOCKING.

The W3 streak check (`src/workflow.rs:167-184`) sub-groups each increment's records by `artifact` and requires EACH artifact to independently reach the class streak; the internal-consistency check (`src/workflow.rs:89-108`) groups by `(task, artifact)` and recomputes the implied streak per artifact from 0. Both use the wrong grouping key: the log's `consecutive_clean` is a per-loop (per task/increment) running streak spanning multiple artifacts. Confirmed against the schema, the real log, and the design's per-slug-max convergence table. The check rejects data the design defines as convergent and cannot go green on the repo's own valid log.

Fix (both must change together):

- W3 (`workflow.rs:167-184`): drop the per-artifact `BTreeMap`; take the peak `consecutive_clean` over ALL of the increment's records and require that single peak to reach `required`. Keep the outer grouping by full `task` (increment) and the per-increment `risk_class`-consistency check unchanged, so the `round-log-core` two-risk-class case still passes.
- Consistency (`workflow.rs:89-108`): group by `round.task` (full increment task) only, not `(task, artifact)`; walk each increment's rounds in file order recomputing the implied streak. Increments stay separate (each `-incA`/`-incB` is a distinct task string), which is correct because the counter resets at increment boundaries.
- Also correct the now-wrong prose that encodes the defective model: `metrics.rs:344` ("its convergence streak is per-artifact"), the module doc `workflow.rs:19-22`, and the inline comments at `workflow.rs:85-88, 124-126, 163-166`.

Verified the fix does not break the per-increment requirement: grouping by full `task` for both risk-class consistency and streak keeps `round-log-core-incA` (low_risk, peak 1) and `-incB` (risky, peak 2) judged separately, and the existing test `per_increment_grouping_passes_a_step_that_converged_across_two_risk_classes` still passes. Under the fix, `deliberation-mode`/`no-wrap-convention` peak 2 (risky, pass), the b2 steps peak 0 (fail, correctly needing grandfather), and lines 25/30 become consistent.

### T2 (opus F2) - VALID, HIGH. Owner: ORCHESTRATOR (post-merge). Does NOT block the code review's convergence.

`validate --workflow` exits 1 on the repo's own data even setting T1 aside, because the 11 b1 no-record steps (`core-assets` .. `init-vcs`) and the 3 b2 short-streak steps (`convergence-accounting`, `pack-rebuild-tracking`, `user-prompts-dir`) are still marked `complete` in the Roadmap, not `grandfathered`. This is real and release-gating, but it is NOT an implementer code defect: the code correctly flags those steps (they genuinely lack convergent records), and the spec (Decision B) chose the `grandfathered` STATUS approach, relabelling each historical step in `docs/plans/agent-scaffold.md`. The implementer was explicitly instructed not to edit that file; the relabel is the orchestrator's planned post-merge integration action ("The earliest steps that predate logging are relabelled `grandfathered` when this lands").

Who does what: implementer does nothing for T2 (the `grandfathered`/`trivial` status machinery is already built correctly in `plan.rs` + `pack/plan-template.md`). Orchestrator, post-merge, relabels the 11 b1 and 3 b2 steps `grandfathered` in the Roadmap. Note the check only goes fully green once BOTH T1 (implementer) is fixed AND T2 (orchestrator) relabel is done; neither alone suffices.

### T3 (opus F3 + sonnet M2) - VALID, LOW. Owner: implementer. Non-blocking (defer-with-comment acceptable).

`leading_slug` (`workflow.rs:52-60`) uses `rfind("-inc")` and strips when the suffix is all-alphanumeric, so a slug literally ending `-inc<alnum>` (e.g. `foo-incidental` -> `foo`, or a Roadmap pair `increment`/`increment-tracker`) is misrouted. Latent: no current slug triggers it, and the alphanumeric requirement is genuinely needed (`round-log-core` uses `-incA`/`-incB`). The purely-lexical strip cannot self-distinguish a real increment marker from a slug that happens to end that way. Acceptable to keep as a documented latent risk (the doc comment already describes the lexical behaviour); if hardening, gate the strip on the remainder matching a known Roadmap slug (the allowlist approach), which removes the ambiguity at the cost of passing the step slugs into `leading_slug`. Implementer's call; not a merge blocker.

### T4 (opus F4) - VALID, LOW. Owner: implementer. Non-blocking.

The consistency recompute accumulates the implied streak over a group's whole history with no loop boundary, so a legitimately re-opened loop that starts directly with a `clean` (rather than a `new_valid`) would be miscounted. Latent: real re-opens start with `new_valid` (which resets to 0), so current data never hits it, and T1's fix (group by full task, walk file order) does not by itself add re-open boundaries. Minor; fix opportunistically alongside T1 or defer with a note.

### T5 (opus F5 + sonnet L6) - VALID, MEDIUM. Owner: implementer. Coupled to T1 (add with the fix).

The 12 tests all use a constant artifact per increment (`workflow.rs:288-297, 376-381` and the `a`/`AGENTS.md` fixtures), so none exercises the change -> fixes -> verification multi-artifact loop that most real steps use; that is why T1 slipped. Add a test with a `risky` increment of three DISTINCT artifacts logging `cc` 0/1/2 in one loop that must PASS (this fails today and catches T1), plus one where a multi-artifact loop peaks below the required count and fails exactly once. Note: sonnet L6's specific "one artifact passes, one fails within the increment" shape becomes obsolete under T1's fix (per-artifact reporting is removed); replace it with the whole-loop pass and whole-loop-short-fail cases above.

### T6 (sonnet M1) - VALID, MEDIUM (doc). Owner: implementer. Should accompany the fix.

`README.md:183-195` documents `validate` and `--plan` but never mentions `--workflow`; a reader cannot discover it. Extend the description at line 187 and add a usage example parallel to the `--plan` example at line 194. (`--help` at `main.rs:279` is already accurate.)

### T7 (sonnet L1) - VALID, LOW (doc). Owner: implementer.

`CHANGELOG.md` `[Unreleased]` does not mention the new `trivial`/`grandfathered` statuses, the `--workflow` flag, or `src/workflow.rs`. Add an entry for these user-visible additions.

### T8 (sonnet L2) - VALID, LOW. Owner: implementer.

`RiskClass::label` (`metrics.rs:128-133`) re-spells `"low_risk"`/`"risky"`, duplicating the `enum_field!` on-disk strings; the `risky` branch is exercised by a problem-message test but `low_risk` is not. Add a test that fails a `low_risk` step and asserts the message contains `low_risk`, closing both the coverage gap and the silent-divergence risk. Small; deriving `label` from `VARIANTS` would remove the duplication but is optional over-engineering.

### T9 (sonnet L3) - VALID, LOW. Owner: implementer. Resolution: keep peak, document.

W3 checks the PEAK `consecutive_clean`, not the terminal value. This matches the design's stated computation (A.md:46 "max `consecutive_clean` seen"), so peak is the INTENDED semantics, not an error; peak equals terminal in a correctly-run loop (the loop stops at convergence). The latent gap is a step that converges, re-opens, and is not re-converged: peak would still pass it. Acceptable as designed; add a one-line comment at `workflow.rs:166-171` noting the deliberate peak-not-terminal choice. Not a behaviour change, not blocking.

### T10 (sonnet L4) - VALID, LOW (doc). Owner: implementer.

The module comment (`workflow.rs:17-18`) calls `skipped` "review-exempt", conflating a dropped step with a review-exempted one. The code is correct (W3 checks only `complete`). Reword to: "W3 checks only `complete` steps; all others (`trivial`, `grandfathered`, `skipped`, and the in-flight statuses) are not checked."

### T11 (sonnet L5) - VALID, LOW (test). Owner: implementer.

The exempt tests cover `trivial`/`grandfathered`/`skipped` but not an in-flight status. Add a test that an `in progress` (or `not started`) step WITH matching rounds in the log is not checked by W3, pinning the `!= "complete"` guard against a future status-list refactor.

---

## Items confirmed correct (no action)

- Exemption logic: W3 iterates only `status == "complete"` (`workflow.rs:133`); `trivial`/`grandfathered` added to `ROADMAP_STATUSES` (`plan.rs:59-69`) and drift-guard-covered.
- The `pause.md` catch (complete + zero matching records) fires correctly, verified on real data and in tests.
- CLI wiring: `--workflow` has `requires = "plan"`, reuses `--metrics`, reports into the shared problem list with exit 1, skips absent files with a stderr note; help text (`main.rs:279`) is accurate.
- `parse_rounds`/`validate_log` split: `parse_rounds` is an intentional best-effort projection that delegates enum parsing to the same methods; no second schema definition.
- Per-increment risk-class consistency and grouping-by-full-task (`workflow.rs:148-162`) are correct; only the per-artifact sub-grouping inside the increment (T1) is wrong.
- `required_streak` (low_risk 1, risky 2) and the `reached < required` comparison have no off-by-one.
- ASCII-only; no new `#[allow]`; no `unwrap`/`panic` on untrusted input.

---

## Rollup

| ID | Finding | Verdict | Severity | Owner | Blocks merge |
| --- | --- | --- | --- | --- | --- |
| T1 | Streak grouped per-artifact, not per-loop | VALID | Critical | Implementer | Yes |
| T2 | b1/b2 steps not relabelled `grandfathered` | VALID | High | Orchestrator (post-merge) | No (post-merge) |
| T3 | `leading_slug` over-strips `-inc<alnum>` slugs | VALID | Low | Implementer | No |
| T4 | Consistency streak crosses re-opened loops | VALID | Low | Implementer | No |
| T5 | No multi-artifact-loop test (missed T1) | VALID | Medium | Implementer | Yes (with T1) |
| T6 | README omits `--workflow` | VALID | Medium | Implementer | No |
| T7 | CHANGELOG not updated | VALID | Low | Implementer | No |
| T8 | `RiskClass::label` duplication, `low_risk` untested | VALID | Low | Implementer | No |
| T9 | W3 uses peak not terminal | VALID | Low | Implementer | No |
| T10 | Module comment mislabels `skipped` | VALID | Low | Implementer | No |
| T11 | No in-flight-status exemption test | VALID | Low | Implementer | No |

Total valid: 11 of 11. Owner split: 10 implementer fixes (T1, T3-T11), 1 orchestrator post-merge action (T2).

No finding was dismissed. The one critical finding (T1) is confirmed against real data; the one high finding (T2) is confirmed and assigned to the orchestrator (not dismissed, not an implementer defect). Merge-blocking: T1 + T5 (implementer). Full green additionally requires T2 (orchestrator relabel) after merge.
