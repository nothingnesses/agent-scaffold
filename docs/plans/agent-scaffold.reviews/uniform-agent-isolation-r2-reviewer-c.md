# Reviewer C findings (round 2, fix-verification): uniform-agent-isolation (Q-61), fix diff `600e6a3..856ddf1`

Lens: verify round-1 fixes F1 (retired carve-out in `pack/isolation-guidance.md`) and F2 (explorers reclassified read-only) are correct AND complete across the shipped/live tree, and introduced no NEW contradiction. Verified independently against the artifact at `856ddf1`, not trusting the writer's report. Round-1 findings themselves and the frozen historical records the triager ruled out of scope are not re-raised.

## Verdict: no high or critical findings. F1 and F2 are correct and complete in the shipped/live tree; no new contradiction introduced. One low observation and one out-of-scope historical note below.

## F1 verification (retired carve-out in `pack/isolation-guidance.md`): FIXED and complete

- `pack/isolation-guidance.md:37` last sentence now reads: "Every spawned agent, including the reviewers, the triager, and the explorers, runs under the resolved isolation tier (container, else worktree, else the file-safety fallback), and the orchestrator merges their outputs onto main." This is the uniform rule and matches `pack/AGENTS.md:83,89,91`. The old "Read-only agents (reviewers, the triager, explorers) need no isolation ... without a container or a worktree of their own" is gone.
- Residual-sweep for the retired carve-out across the SHIPPED tree (`pack/`, `src/`, `.agents/`, incl. `.agents/prompts/` and `pack/prompts/`): grep for "need no isolation | need no worktree | need none | needing no worktree | without a (container|worktree) of (their|its) own" returns ZERO hits in the shipped tree. The only remaining hits are in `docs/plans/` historical step records (see out-of-scope note below), not in any emitted asset.
- Coherence with the rest of the file: the closing sentence coheres with the file's own intro (line 3, which explicitly defers the who-isolates policy to the "Writer isolation (capability-tiered)" and "Worktree lifecycle and merge-back" rules and supplies "only the setup that makes the container tier available"). No contradiction.

## F2 verification (explorers classified read-only everywhere): FIXED and complete, coherent

- `pack/AGENTS.md:65` (and its twins `AGENTS.md:65`, `.agents/AGENTS.reference.md:65`) now: "Explorers author advisory design notes, not the plan or the code, so they are read-only with respect to the reviewed product, like the reviewers and the triager; like every spawned agent they still run under the file-safety and isolation rules below." The former "Explorers are writer agents, so the file-safety and writer-isolation rules below apply to them" is gone.
- Residual-sweep: grep for "explorers? are writer | exploration writer | design or exploration writer | explorers? ... writer agent" across `pack/`, `src/`, `.agents/`, and the live plan sources returns ZERO hits. No file classifies an explorer as a writer anywhere.
- Consistency across every shipped/live locus that classifies explorers, all now read-only:
  - `pack/AGENTS.md` / `AGENTS.md` / `.agents/AGENTS.reference.md` lines 65, 83, 89, 91, 93, 104: uniformly "read-only reviewers, triager, and explorers alike" and "a read-only agent (a reviewer, the triager, or an explorer)".
  - `pack/prompts/orchestrator.md:11,15` and `.agents/prompts/orchestrator.md:11,15`: "the writers ... and the read-only reviewers, triager, and explorers alike". Consistent.
  - `src/isolation_policy.rs` `ISOLATION_POLICY_FRAGMENT` (and the `///` doc at line 23, the test comment at line 50, the assert message at line 61): "read-only reviewers, triager, and explorers alike". Byte-matches `AGENTS.md:91`. Consistent.
- Naming/collision clause: the writer correctly changed "so writers never collide" -> "so they never collide" in all three copies of the Design-explorations paragraph, cohering with explorers no longer being called writers.
- Q-61 decision receipt (durable record) now agrees with the shipped text: `docs/plans/agent-scaffold.plan.toml:1291` (`ask`) and its projection `docs/plans/agent-scaffold.md:154` now read "explorers are read-only with respect to the reviewed product, since they author only advisory design notes and not the plan or the code, but they still isolate like every spawned agent, so they are no longer listed among read-only agents needing no worktree." This is the reverse of the false round-1 receipt ("explorers are writers that isolate") and now matches the artifact. The round-1 false-receipt integrity defect is resolved.
- Step-detail "Why it exists" now agrees: `docs/plans/agent-scaffold.md:901` and `docs/plans/agent-scaffold.steps/uniform-agent-isolation.md:5` read "the Design explorations section had called them writer agents while the worktree-lifecycle rule listed them among read-only agents needing no worktree. The uniform rule removes the contradiction ... explorers are read-only with respect to the reviewed product ... consistent with the reviewers and the triager." Correct.
- Coherence with the Design-explorations section as a whole: "prefer several independent explorers ... whose proposals the orchestrator then synthesises" and the optional light review pass ("one reviewer, one triager, no convergence loop") both cohere with read-only classification (an explorer's notes inform but are never accepted as product, exactly like reviewer findings, which are themselves synthesised/triaged). No new self-contradiction: no surviving locus calls an explorer read-only while another implies it authors reviewed product.

## Low observation (not a convergence blocker; not a contradiction)

- Severity: low.
- `pack/isolation-guidance.md:3,30,37` retain writer-centric example framing that the round-1 triage's fix direction had suggested softening ("(and soften the writer-only framing at 30/37)"): line 3 "the orchestrator runs writer agents under container isolation"; line 30 "Run a writer under isolation ... spawn the writer against it"; line 37 "prefer the `--git` sandbox for writer agents" and "whether a writer will run in a container, a worktree, or under the file-safety fallback". The fix rewrote only the carve-out sentence, not these.
- Why it is NOT a contradiction: these are the concrete agent-box/agent-images container-tier setup examples, which legitimately use a writer as the running example, and the same paragraph's closing sentence now explicitly generalizes ("Every spawned agent, including the reviewers, the triager, and the explorers, runs under the resolved isolation tier"). The file states nowhere that read-only agents do not isolate. Read whole, the file coheres with the uniform rule. This is minor phrasing incompleteness relative to the triage's parenthetical suggestion, left to the triager's discretion; I do not consider it a blocker, and the substantive Finding-1 contradiction (the "need no isolation" carve-out) is fully resolved.

## Out-of-scope historical note (not a finding)

- `docs/plans/agent-scaffold.md:419` and `docs/plans/agent-scaffold.steps/agent-isolation.md:5` still read "Read-only agents (reviewers reading, the triager) need no isolation (Principle 2 ...)". This is the frozen step-detail narrative of the `agent-isolation` step, which the Roadmap (`docs/plans/agent-scaffold.md:178`) marks `complete`. It is the same class of completed-step historical record the triager ruled out of scope in round-1 Finding 3 (editing it would falsify the history of what that step decided). It does not mention explorers and is not the explorer contradiction. Recorded here only because this specific pair was not enumerated in Finding 3; it carries the identical out-of-scope verdict and is not a convergence blocker.

## Severity summary

- critical: none.
- high: none.
- medium: none.
- low: 1 (residual writer-centric example framing in `pack/isolation-guidance.md:3,30,37`; not a contradiction, discretionary).
