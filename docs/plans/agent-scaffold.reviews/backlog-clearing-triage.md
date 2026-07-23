# Triage verdicts: backlog-clearing round 1

Branch `plan/backlog-clearing`, commit `aec4144`, base main `c8308ef`. Five
deferred documentation-polish backlog items, documentation-only, low_risk.

Inputs adjudicated:
- `backlog-clearing-reviewer-a.md` (fidelity / completeness / code-accuracy): zero findings.
- `backlog-clearing-reviewer-b.md` (mechanical / consistency / prose): one LOW finding (L1).

## Reviewer A: zero findings

Confirmed reasonable. Reviewer A's stated checks cover the fidelity lens
concretely: each of the five items is matched to its sidecar intent, the
tri-file sync (pack source plus the two assembled copies) is verified, and
scope is confined to the five expected files. Item 3 (the item most likely to
carry a code-accuracy risk) was checked against named source lines for both
substrates: the TOML-primary flow reading `[meta].w4_baseline`
(src/plan/source.rs:412, src/main.rs:904-916, src/workflow.rs:191 and 275-286)
and the Markdown-sourced flow reading the JSONL `type:"baseline"` record
(src/workflow.rs:164, src/metrics.rs:785-811), plus confirmation the
`type:"baseline"` parser was not removed (src/metrics.rs:764 and 785) and that
`[meta].primary` and `[meta].w4_baseline` are real fields. Reviewer B
independently corroborated the same field and provenance facts
(src/plan/source.rs:109 and 112, plan Q-46 at line 1277) and reported all drift
guards and validation green. I do not need to re-run these checks; the
zero-findings verdict stands.

## L1: two names for the same flow in adjacent instrument.md hunks

Restatement: in the `type:"decision"` bullet the TOML-primary flow is called
"the TOML-sourced flow"; in the immediately following `type:"baseline"` bullet
the same flow is called "the plan-TOML flow (`[meta].primary = "toml"`)". The
Markdown-sourced flow is named identically ("the Markdown-sourced flow") in
both bullets. Reviewer B rated this low, plain descriptive prose, both clear,
safe to dismiss.

Verdict: DISMISSED (not a defect requiring a fix).

Reasoning: the inconsistency is real but does not rise to a correctness or
consistency defect. Both labels are descriptive prose, not defined terms, and
the disambiguating context is strong:
- The two bullets share a parallel two-flow contrast; in each bullet the
  TOML flow is set against the identically named "Markdown-sourced flow", so a
  reader parallels "plan-TOML flow" to "TOML-sourced flow" by structure.
- The `type:"baseline"` occurrence carries the explicit qualifier
  `[meta].primary = "toml"` in parentheses, which pins it to the TOML-primary
  flow and forecloses any reading that it is a third, distinct flow.
Confusion risk that these are two different flows is therefore very low. This
is a cosmetic naming nit in prose, not a blocking issue. A one-word
unification (making both read "the TOML-sourced flow", or both "the plan-TOML
flow") would be a marginal readability gain but is not required for
convergence; the orchestrator may fold it into a follow-up if desired. It does
not require another round.

Blocking: no (non-blocking; DISMISSED as cosmetic).

Backstop: no HIGH or CRITICAL finding was dismissed, so the dismissed-high
re-check backstop does not apply.

## Round outcome

CLEAN
