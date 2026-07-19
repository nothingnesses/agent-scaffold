# Inc 6 doc/prompt-currency plan edit: triage verdicts

Triager: independent, read-only, separate from both the producer (planner) and the orchestrator. Adjudicates the two findings in `inc6-docsync-plan-reviewer-opus.md`. Every claim was verified against the actual files (`pack/prompts/implementer.md`, `pack/prompts/orchestrator.md`, `pack/AGENTS.md`, `pack/instrument.md`) and against the plan artifacts (`docs/plans/agent-scaffold.steps/structured-skeleton.md` Inc 6 bullet, `docs/plans/agent-scaffold.success-criteria.md`).

## R-1: currency scope covers plan AUTHORING but misses stale plan STATUS-MAINTENANCE wording

Verdict: VALID.
Severity: medium (reviewer's rating confirmed).

Reasoning:

The three stale locations are real and reproduce exactly:

- `pack/prompts/implementer.md:5`: "Keep the plan's status current as you go: per the plan's Documentation Protocol, set the step's Status cell in the Roadmap table, which is the single source of truth for status, and change the status there only, not in prose elsewhere." This is an explicit instruction to a scaffolded implementer to hand-edit the Roadmap status table.
- `pack/prompts/orchestrator.md:31`: "Status and decisions are owned by the plan (the Roadmap is the single source of truth for step status; decisions live in the Open Questions queue)".
- `pack/AGENTS.md:57` (Tracking progress): "the status table described in the plan's Documentation Protocol and the single source of truth for step status; the implementer keeps it current."

Under the flow Inc 6 ships (`[meta].primary = "toml"`, which the Inc 6 template sets), the single source of truth for step status is `[[step]].status` in the `.plan.toml`; the Roadmap table in the generated `<task>.md` is a do-not-edit projection that `render` overwrites and `render --check` guards. The `implementer.md` wording is therefore not merely imprecise: shipped as-is in a TOML-primary scaffold it directs the implementer to hand-edit the generated file, the exact anti-pattern this currency pass exists to remove; the edit would be clobbered on the next `render` and would fail `render --check`. That is a genuinely stale instruction that would mislead a scaffolded implementer.

The current Inc 6 scope does miss it. Scope item (1) names `orchestrator.md` and `planner.md` explicitly plus a catch-all "(and any other `pack/prompts/*.md` referencing the plan format or `TEMPLATE.md`)", but the described transformation is authoring-only: "move from 'author a Markdown plan from `docs/plans/TEMPLATE.md`' to the TOML+render+sidecar authoring AND editing flow". `implementer.md` does not author a plan and has no `TEMPLATE.md` reference; its defect is a status-maintenance instruction, so the item-(1) transformation does not cleanly apply to it. Scope item (4) limits the `pack/AGENTS.md` edit to the phase-2 authoring sentence only, so the Tracking-progress status-SSOT claim at line 57 is outside the enumeration. The step-level ACCEPTANCE tests only for "no stale `TEMPLATE.md` or Markdown-only-plan authoring references left in `pack/prompts/`"; `implementer.md`'s status-editing instruction is neither, so the step's own acceptance passes while `implementer.md` (and `orchestrator.md:31`, and the AGENTS.md line-57 claim) stay stale and the shipped pack stays internally incoherent.

The reviewer's conceded mitigation is real but insufficient at the level the implementer works from. The catch-all hedge and the umbrella Success Criterion (`docs/plans/agent-scaffold.success-criteria.md:27`, "a freshly scaffolded project's `pack/prompts/*` ... never hand-editing it") do reach the defect at the umbrella-criterion altitude, and a final acceptance pass against the Success Criteria could catch it. But an implementer building Inc 6 works from the step bullet and its acceptance, not the Success Criteria, and "referencing the plan format" reads naturally as authoring-only. Relying on the umbrella criterion to backstop a concretely-enumerated step is exactly the incoherence Q-47 was chosen to prevent ("a pack that is never internally inconsistent"). The most operationally dangerous stale instruction should be named in the concrete scope, not left to the catch-all.

Severity medium is correct. It actively misdirects a scaffolded implementer and produces an internally-incoherent shipped pack, which is the specific harm Q-47 targets, so it is above low. It is not high/critical: `render --check` (fail-CI) is a hard backstop that would catch the clobbered edit rather than let it corrupt state silently, and the blast radius is prompt/doc coherence in a fresh scaffold, not data loss, security, or an irreversible code defect.

Concrete fix:

1. Add to Inc 6 scope a status-maintenance currency item (either extend item (1) or add a new item) that names explicitly:
   - `pack/prompts/implementer.md`'s "set the step's Status cell in the Roadmap table, which is the single source of truth for status" instruction, changed to: the source of truth for status is `[[step]].status` in the `.plan.toml`; edit it there and re-render; never hand-edit the generated Roadmap table in `<task>.md`.
   - `pack/prompts/orchestrator.md:31`'s "the Roadmap is the single source of truth for step status" claim, reconciled the same way (the TOML `[[step]].status` is the source; the Roadmap is a projection).
2. Decide the `pack/AGENTS.md` Tracking-progress (line 57) status-SSOT claim explicitly: either name it in scope alongside item (4), or consciously defer it alongside `Q-48`, since it is a canonical-guidance altitude question of the same kind as the phase-2 wording already parked under Q-48. Do not leave it silently unaddressed.
3. Add a step-level acceptance check: the shipped prompts (and `pack/AGENTS.md`, if brought into scope) do not instruct hand-editing the generated Roadmap status table; status maintenance is described as editing `[[step]].status` in the `.plan.toml` and re-rendering.

## R-2: the `instrument.md` decision-requires-`task` gap is overstated

Verdict: VALID.
Severity: low (reviewer's rating confirmed).

Reasoning:

Inc 6 scope item (5) asserts "`pack/instrument.md` documents that a `type:"decision"` record requires the `task` field (a known doc gap, the `task` field is required on every metrics record but `instrument.md` does not say so for the decision receipt)". The premise is imprecise. `pack/instrument.md:3` preamble already states, for all records: "Every record also carries `task` (a string naming the plan/ledger task, so records can be grouped per task) ...". So `task` IS already documented as required for the decision record via the general rule. No per-type bullet (`round`, `escalation`, `waiver`, `intake`, `baseline`, or `decision`) restates `task` locally; they all rely on the preamble. The claim that `instrument.md` "does not say so for the decision receipt" is therefore wrong: it says so for every record, decision included.

This is a soft, non-blocking imprecision in the plan's scope premise, not a defect in a shipped file, so low is correct. Singling out only the `decision` bullet to restate `task` would make it inconsistent with every other per-type bullet, and there is a defensible self-contained-doc reading (Principle 20).

Concrete fix:

Correct the item-(5) premise (and the matching clause in Success Criterion line 27, "`instrument.md` documents that a `type:"decision"` record requires the `task` field") to reflect that `task` is already documented as required for all records via the preamble. Then make the intent explicit as one of:
- treat this as already-covered and drop item (5) (and drop the corresponding Success-Criterion clause, or reframe it as "confirms" rather than "adds"); or
- if a self-contained restatement is genuinely wanted (Principle 20), restate `task` on the `decision` bullet AND do so consistently across the per-type bullets, not only on the decision receipt, and reword the premise to say so.

Do not ship item (5) with its current "does not say so" wording, since it would send the implementer to add a reminder that is already present in the preamble.
