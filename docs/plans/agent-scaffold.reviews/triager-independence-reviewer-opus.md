# Review: triager-independence (diff 0c95ab9..bd483e1)

Reviewer: claude-opus-4-8 (independent). Lens: internal consistency and correctness.

Note: this reviewer ran in a read-only harness and could not write the file itself;
the orchestrator transcribed these findings verbatim from the reviewer's report
(findings-files mechanism is not yet built in the pack).

Verification of sync: `pack/prompts/orchestrator.md` and `pack/prompts/triager.md`
are byte-identical to their `.agents/prompts/` copies. Root `AGENTS.md` and
`.agents/AGENTS.reference.md` differ from `pack/AGENTS.md` only by the expected
`{{principles}}` template expansion (lines 1-187 identical). Generated files are in
sync. All three specified sub-changes are present.

## R1 - MEDIUM: AGENTS.md no-subagent fallback still collapses the triager

Location: `pack/AGENTS.md` lines 13-14 (mirrored in root `AGENTS.md` /
`.agents/AGENTS.reference.md`).

The fallback sentence reads: "Where sub-agents are unavailable, one agent plays the
roles in sequence but still writes down each role's output, so the separation holds
on paper." This literally includes the triager among "the roles" that one agent
plays. The newly added exception two sentences later ("The triager is the one
exception to collapsing: it is always a separate agent, never merged into the
producer or the orchestrator") directly contradicts the base sentence: if one agent
plays all the roles in sequence, the triager IS merged into that one agent.

The parallel sentence in `orchestrator.md` (line 11) WAS tightened to "perform the
other roles yourself in sequence" - grammatically excluding the triager from the
fallback clause. `pack/AGENTS.md` was not given the same treatment ("the roles"). So
the canonical source file both contains an internal tension and now diverges from
its own orchestrator prompt on the exact invariant this step hardens. Violates
Principle 1 (coherence, one source of truth) and undercuts Principle 5 (make illegal
states unrepresentable) by leaving the collapse-the-triager reading textually
available in the top-level rule.

## R2 - LOW: AGENTS.md omits the "(or a human)" escape the prompts rely on

Location: `pack/AGENTS.md` collapse exception sentence and the Triager role bullet.

Both `orchestrator.md` ("always a separate agent (or a human)") and `triager.md`
("a second triager (or a human)") state that in the no-subagent/single-agent case
the triager independence is satisfied by a human. `pack/AGENTS.md` states only
"always a separate agent" in both the collapse-exception sentence and the Triager
role bullet, with no human fallback. Combined with "Where sub-agents are
unavailable," AGENTS.md thus asserts a requirement unsatisfiable precisely in the
harness it describes, whereas the two prompts describe how to satisfy it. Minor
cross-file disagreement on HOW triager independence is achieved (Principle 1).

## Items checked, no finding
- Convergence backstop: "second, independent triager (or a human)" - consistent.
- Orchestrator role bullet: "does not plan, implement, review, or triage itself" - consistent.
- Acceptance phase: spawns a triager separately - consistent.
- README.md: no collapse/independence rule to update; diagram already shows a distinct triager node.
