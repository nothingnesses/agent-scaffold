# Plan-fold review (correctness and faithfulness) - opus

Reviewing commit `1f73f68` (`git diff 827ea15..1f73f68 -- docs/plans/agent-scaffold.md`) against the ledger's LOCKED DECISIONS (`docs/plans/agent-scaffold.ledger.md` RESUME STATE) and the ten design-notes files.

## Verdict

The fold is accurate and complete on the substance. Every locked decision (Q-26, Q-27, Q-28, Q-30, Q-31, Q-33, Q-35, Q-36) is present and faithful; I found no dropped or over-stated element, no drift on the load-bearing specifics, and no contradiction between a step detail and its locked decision. Mechanical checks all pass:

- `validate --plan` exits 0 (46 steps, 36 open-questions items, valid), so every new Roadmap slug has a matching `### ` detail block and vice versa. The four new slugs `reviewer-diversity`, `review-mode`, `test-driven`, `mutation` each have a Roadmap row and a detail block.
- Stale `ledger-parse` / `src/ledger.rs` references remain only as the skipped-step block or are explicitly framed as "skipped; there is no `src/ledger.rs`" (Q-28 item, workflow-invariants and state-queries details). None survive as a live dependency.
- Queue "decided -> folded into <step>" targets all resolve to real Roadmap slugs (Q-26->optional-modules, Q-27->workflow-invariants, Q-28->state-queries, Q-30->test-driven, Q-31->mutation, Q-33->optional-modules, Q-35->review-mode, Q-36->reviewer-diversity + optional-modules).
- Dependency ordering is consistent: `test-driven` and `mutation` sit after `optional-modules` in the table, and `mutation` after `test-driven`; both details state the dependency-forced order explicitly.
- ASCII-clean (no non-ASCII introduced).

Spot-verified accurate against the locked decisions: the two `plan.rs` statuses `trivial`/`grandfathered`; the alphanumeric `-inc<x>` strip and per-increment streak/consistency musts; the three b2 short-streak steps and eleven b1 zero-record steps; the `review` Phase enum + one-clause reviewer/triager generalization + single-pass + committed kept report + one-round-ledger-deleted-at-close; test-driven requires-checks + separate `test-author` role + two-gate-profile + frozen-tests tripwire + kind="test"; mutation once-per-step/diff-scoped/risk-scaled/reuse-checks-reviewer/route-to-test-artifact/degrade-to-skip; Q-33/Q-36 adopt-now diversity rule + deferred spawn-map + increment-3-stays-pointer; the checks.toml schema + `{{modules}}` slot + checks-reviewer + `agent-scaffold checks [--staged]` + `--with-precommit-hook`. All three Success Criteria additions are accurate.

The findings below are all LOW: clarity/completeness nits, not substance drift.

## Findings

### F-1: Status resume-anchor "remaining planned work" enumeration not updated for the four new steps

- Severity: low
- Evidence: `docs/plans/agent-scaffold.md` line 3 (Status), the sentence the fold edited: "The remaining planned work is `optional-modules` then `workflow-calibration` (the whole-codebase acceptance review once noted here as a LATER job is now SUBSUMED by `review-mode`, `Q-35`)".
- What is wrong: the fold added four not-started Roadmap steps (`reviewer-diversity`, `review-mode`, `test-driven`, `mutation`) and touched this exact sentence, but the "remaining planned work" enumeration still lists only `optional-modules` then `workflow-calibration`. It omits the four new steps and the already-present process cluster (`workflow-invariants`, `state-queries`). The Status line is the resume anchor, so the summary of remaining work is now understated. (The Roadmap is the declared source of truth for order/status and does carry all of them, which caps the impact.)
- Fix: extend the enumeration to mention the newly-folded steps, e.g. note that `workflow-invariants`/`state-queries` (process cluster) and the new `reviewer-diversity`/`review-mode`/`test-driven`/`mutation` are also planned/not-started, or point the reader to the Roadmap for the full not-started set rather than naming a partial list.

### F-2: Increment-2 detail bullets render as peers of the increment bullets, not as sub-points

- Severity: low
- Evidence: `docs/plans/agent-scaffold.md` `### optional-modules` step, the increment (2) rewrite (diff hunk at old line ~511). The increment (2) bullet ends "... The module owns the whole schema:" and is then followed by four top-level `-` bullets (`.agents/checks.toml`, `{{modules}}` slot, `checks-reviewer`, hook path) and a `Rationale:` paragraph, all at column 0, before "- Increment (3)".
- What is wrong: those four bullets and the Rationale paragraph sit at the same indentation as the `- Increment (1/2/3)` bullets, so structurally they read as siblings of the increments rather than as children of increment (2). Increment (1) and increment (3) are each a single self-contained bullet, so the pattern is now inconsistent and a reader could misattribute the four schema bullets. It renders readably and `validate --plan` does not care, so impact is cosmetic.
- Fix: indent the four schema bullets (and optionally the Rationale) under the increment (2) bullet, or restate them as an explicitly-scoped "Increment (2) settled layout:" sub-list, so they nest under increment (2) like the other increments' content.

### F-3: W3's per-step-slug "risk_class must be consistent" phrasing sits in tension with the per-increment must in the same block

- Severity: low
- Evidence: `docs/plans/agent-scaffold.md` `### workflow-invariants`. W3 (line ~626): "filter `round` records whose `task` leading-slug (before an optional `-inc<x>` suffix) equals the step slug; if any exist, their `risk_class` must be consistent and the consecutive-clean streak (per artifact) must reach the required count". Then two paragraphs later the BUILD-TIME MUSTS: "the streak / risk_class-consistency checks must run PER INCREMENT, not per step slug (increment A converged `low_risk` at streak 1 while increment B converged `risky` at streak 2, so a single-step aggregate would see an inconsistent `risk_class` and a broken streak)".
- What is wrong: taken literally, W3 groups all records by the stripped step slug and requires `risk_class` consistency across that set; for `round-log-core` (incA `low_risk` + incB `risky`) that set is inconsistent, so W3-as-stated would false-flag it. The following paragraph is the correction (compute per increment), so the block is internally self-correcting and faithfully reproduces both halves of the locked decision, but a builder reading only the W3 summary sentence could implement the false-flagging step-level version. This is a clarity nit, not drift (the locked decision itself carries both requirements and leaves the reconciliation to build time).
- Fix: fold the per-increment qualifier into the W3 sentence itself (e.g. "... group the filtered records BY INCREMENT, and within each increment the `risk_class` must be consistent and the streak reach the required count"), so W3 does not read as a step-level consistency check that the later paragraph then contradicts.

## Nothing else

No other correctness or faithfulness issues found. The locked decisions are captured completely and accurately.
