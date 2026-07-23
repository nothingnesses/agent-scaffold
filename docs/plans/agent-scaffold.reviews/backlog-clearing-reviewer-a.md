# Fidelity review: backlog-clearing (reviewer A)

Lens: fidelity, completeness, and code-accuracy of the five deferred
documentation-polish items against their step sidecars and (for item 3) the code.

Range reviewed: base main c8308ef .. aec4144, branch plan/backlog-clearing.

## Verdict

No valid findings. All five items are done, faithful to their sidecars, in scope,
and (item 3) accurate against the code.

## What I checked

### Item 1: document-receipt-task-convention

Sidecar decision was to DOCUMENT the convention (task carries the `folded_into`
slug, descriptive not load-bearing, W4 keys on `q_id`). The added sentence on the
`type: "decision"` bullet reads: "By convention the shared `task` field on a
decision receipt carries the question's `folded_into` slug ..., but nothing joins
on it (W4 keys on `q_id`), so it is a descriptive convention rather than a
load-bearing key." This is accurate and does not overclaim any join on `task`.
Confirmed `task` is a shared field on every record (pack/instrument.md intro line 5:
"Every record also carries `task`"). Confirmed `folded_into` is a real question
field (src/plan/source.rs:317). The added sentence is synced across pack/instrument.md,
AGENTS.md, and .agents/AGENTS.reference.md.

### Item 2: formatter-reflow-wording-polish

Sidecar asked to also name the sibling clause (incidental reformatting left to the
orchestrator) in the Prose-formatting reconciliation. The change adds "and leaves
incidental reformatting to the orchestrator" to "... which still holds that a writer
does not proactively run a repo-wide formatter". This matches the actual
"Format only your own files" file-safety rule wording (pack/AGENTS.md:79:
"... and leaves incidental reformatting to the orchestrator."). Faithful; synced
across the three AGENTS copies. The "writer" subject is pre-existing phrasing in the
reconciliation clause, not introduced by this change, so no subject-mismatch issue.

### Item 3 (scrutinised most): reconcile-baseline-doc-drift

Verified against the code that both substrates are described accurately and that the
implementer correctly described BOTH rather than claiming the JSONL record was pruned:

(a) TOML-primary flow reads `[meta].w4_baseline`. Confirmed: is_toml_primary tests
    `meta.primary == Primary::Toml` (src/plan/source.rs:412); main.rs routes such a
    plan to check_workflow_toml (src/main.rs:904-916); check_workflow_toml sources the
    baseline via baseline_from_toml (src/workflow.rs:191); baseline_from_toml reads
    `plan.meta.w4_baseline` (src/workflow.rs:275-286).
(b) Markdown-sourced flow reads the JSONL `type:"baseline"` record. Confirmed:
    check_workflow sources the baseline via metrics::parse_baseline(log_contents)
    (src/workflow.rs:164); parse_baseline reads records whose `type` is `baseline`
    (src/metrics.rs:785-811, gate at 797).
(c) The `type:"baseline"` parser was NOT removed. Confirmed: parse_baseline exists
    (src/metrics.rs:785) and the Baseline projection exists (src/metrics.rs:764).
(d) `[meta].primary` is a real field (src/plan/source.rs:111-112) and `[meta].w4_baseline`
    is a real field (src/plan/source.rs:108-109), so the doc's `[meta].primary = "toml"`
    and `[meta].w4_baseline` references are correct.

The reworded prose ("For the plan-TOML flow (`[meta].primary = "toml"`) the live cutoff
is `[meta].w4_baseline` ..., which W4 reads directly; this JSONL record is the legacy
form of the same cutoff that the TOML field superseded per Q-46, and its parser still
backs the Markdown-sourced flow") is accurate and complete: it does not overstate (it
does not claim the JSONL parser was removed), does not understate (it names the live
TOML cutoff), and does not misdescribe either substrate. The decision bullet's parallel
edit ("the declared W4 baseline cutoff (`[meta].w4_baseline` in the plan TOML for the
TOML-sourced flow, or a `type: "baseline"` record for the Markdown-sourced flow)") is
likewise accurate for both flows, including the "when no baseline is declared" case
(baseline_from_toml returns empty when w4_baseline absent; parse_baseline returns empty
when no baseline record; W4 then requires a receipt for every decided item in both).
"superseded per Q-46" matches the code's own provenance comments ("the Inc 4 (Q-46)
source swap", src/workflow.rs:171; "the Inc 4 gate", src/plan/source.rs:407).

### Item 4: soften-writer-agent-framing

Sidecar named the File-safety intro in AGENTS.md and pack/isolation-guidance.md lines
3, 30, 37, and said to leave role-named duties untouched. The change softens exactly:
the file-safety intro ("Every writer agent's ..." -> "Every spawned agent's ...";
"running writers under isolation" -> "running an agent under isolation") in the three
AGENTS copies, and pack/isolation-guidance.md lines 3, 30, 37 (three hunks:
"writer agents under container isolation" -> "spawned agents ...", "Run a writer under
isolation ... writer agent ... spawn the writer" -> "Run a spawned agent ...
spawned agent ... spawn the agent", "for writer agents ... whether a writer will run"
-> "for spawned agents ... whether a spawned agent will run"). No over-reach: the
"Clean tree before a writer" bullet and the "Writer isolation (capability-tiered)"
section name are untouched, matching the sidecar's intent. No under-reach: after the
edits, lines 3/30/37 carry no residual "writer"-only isolation framing. The assembled
AGENTS.md does not embed the isolation-guidance section (grep found none of its text),
so there is no unsoftened stale copy to reconcile; its only remaining "writer" is the
"Clean tree before a writer" duty at AGENTS.md:77, which is legitimately role-specific.

### Item 5: acceptance-doc-currency-phrasing-polish

Sidecar asked to reword phase 5 so the reviewer-verifies clause is the acceptance
action and the implementer-updates clause is the shortfall follow-up. The old text
("The implementer updates the docs and prompts the change made stale, and the reviewer
verifies that currency as part of the acceptance review.") became "The reviewer verifies
that currency as part of the acceptance review; any staleness it finds is a shortfall
that routes back to implementation, where the implementer updates the docs and prompts
the change made stale, the same path the shortfalls below take." This keeps the Q-50
meaning (implementer updates, reviewer verifies), makes the reviewer's verification the
read-only acceptance action, and reframes the implementer's update as the follow-up a
shortfall triggers rather than an action the acceptance pass performs. Synced across the
three AGENTS copies.

## Scope

The diff touches only pack/AGENTS.md, pack/instrument.md, pack/isolation-guidance.md,
and the two assembled copies AGENTS.md and .agents/AGENTS.reference.md. Each item's
edits land in the correct pack piece and its assembled copies; the tri-file sync is
consistent, not over-reach. No edits stray beyond the five sidecars' named targets.

I did not raise line-length or prose-wrapping (out of scope per the project's
no-hard-wrap convention).
