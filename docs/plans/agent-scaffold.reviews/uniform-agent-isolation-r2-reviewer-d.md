# Reviewer D findings (round 2): uniform-agent-isolation (Q-61), diff `1ce9de3..856ddf1`

Lens: fresh full-artifact adversarial + mechanical, on the revised artifact at `856ddf1`. Round-1's two high findings (F1 pack carve-out, F2 explorer writer-vs-read-only) are confirmed fixed below; they are not re-raised. I read the whole uniform-isolation rule end to end (roles anchor, File safety, Writer isolation, the who-isolates single-source paragraph, Worktree-lifecycle/batch-merge, Findings files, Design explorations, Preflight) plus the three prompts and the `src/` guards.

## Round-1 fixes confirmed (not findings)

- F1 (pack/isolation-guidance.md carve-out): the retired sentence "Read-only agents ... need no isolation and run without a container or a worktree of their own" is gone. Line 37 now ends "Every spawned agent, including the reviewers, the triager, and the explorers, runs under the resolved isolation tier ..., and the orchestrator merges their outputs onto main." The flat contradiction is resolved.
- F2 (explorer classification): `AGENTS.md:65` now reads "Explorers author advisory design notes, not the plan or the code, so they are read-only with respect to the reviewed product, like the reviewers and the triager; like every spawned agent they still run under the file-safety and isolation rules below." The Q-61 receipt (`plan.toml` + plan Open-Questions), the step-detail "Why it exists" (`docs/plans/agent-scaffold.md`, sidecar), and the isolation fragment now all classify explorers as read-only-that-isolate, consistently. Direction (a) from round-1 triage was applied everywhere; the self-contradiction and false receipt are gone.

## Coherence sweep: coherent (no high/medium/critical)

- Batch-merge + reviewers-then-triager ordering is unambiguous and identical across the three places that state it: `AGENTS.md:93` ("the parallel reviewers all finish, the orchestrator batch-merges their findings onto main, then spawns the triager in its own worktree to read the merged findings, then merges the triager's verdict onto main"); `.agents/prompts/orchestrator.md:11` (same sequence); `.agents/prompts/triager.md:3` ("Read the reviewers' findings files directly ..., which the orchestrator has merged onto main from the reviewers' worktrees"). "Batch-merge only after they ALL finish, never as each one completes" appears verbatim in both `AGENTS.md:93` and `orchestrator.md:11`; the single-non-parallel-agent carve appears in both. No drift.
- Product-authority framing holds together: `AGENTS.md:25` (roles anchor, reviewers/triager read-only w.r.t. plan and code), `:83`/`:89`/`:91` (uniform isolation + authority-not-isolation distinction), `:65` (explorers), and the `ISOLATION_POLICY_FRAGMENT` all agree.
- Emitted tool instructions are faithful to the AGENTS.md prose: `orchestrator.md:11` carries the who-isolates + batch-merge rule and references the `AGENTS.md` Writer isolation rule for policy; `reviewer.md:11` and `triager.md:9` both say "inside your own worktree: you run isolated like every spawned agent, and the orchestrator merges your findings/verdicts onto main." No stale "reviewers need no isolation" text survives in any prompt.
- `src/next.rs` `spawns_writer` NOTE and the pinning test `the_reviewer_states_carry_no_isolation_reminder` are honest about Q-62 (open): they pin current writer-only reminder scope and cross-reference the still-open decision, rather than silently diverging from the uniform rule. Coherent with Q-62 in the plan.

## Finding 1: File-safety baseline is still framed writer-only, but the now-universal isolation rule falls back to it for read-only agents

- Severity: low.
- Evidence:
  - `AGENTS.md:75` (File safety, UNCHANGED by this diff): "Every writer agent's damage must stay a visible, committed-or-recoverable diff ... This is the always-on baseline; running writers under isolation builds on it rather than replacing it (see Writer isolation below)." The intro names only writers.
  - The now-universal isolation rule's tier-3 fallback (`AGENTS.md:83`) is "The file-safety discipline above, as the fallback when the harness offers no isolation," and `:89` calls isolation "the structural upgrade over the file-safety baseline." So a read-only agent (reviewer/triager/explorer) that resolves to tier 3 falls back to a baseline whose intro is scoped to "writer agent" damage, while `AGENTS.md:25` defines "'Writer agent' ... means a spawned writer role" (i.e. not a reviewer/triager/explorer).
  - The individual file-safety bullets are a mix: "Recover on interrupt" (`AGENTS.md`, "On any agent kill or interrupt, the orchestrator ...") and "Commit before delete" ("a findings file ...") are already general and do cover a tier-3 read-only agent's only write; but "Clean tree before a writer" is writer-scoped.
- Why it matters: the change universalized who isolates without updating the baseline that isolation "builds on" to acknowledge read-only agents. The operative protection for a tier-3 reviewer (recover-on-interrupt, commit-before-delete) is present in the general bullets, so there is no behavioural hole or illegal state; this is a framing/coherence gap, which is why it is low, not medium. A reader resolving a read-only agent to the file-safety fallback finds a section that, in its header and intro, talks only about writers.
- Suggested fix (discretionary): generalise the `AGENTS.md:75` intro from "Every writer agent's damage" / "running writers under isolation builds on it" to "every spawned agent," matching the Writer isolation rule it now underpins; or add one clause noting read-only agents fall back to and build on the same baseline. Out-of-scope alternative: leave as-is since the general bullets already cover the reviewer's findings-file write.

## Finding 2: `pack/isolation-guidance.md:37` still frames tier preference and the preflight statement as writer-only, one clause before the newly-added universal sentence

- Severity: low. (Adjacent to round-1 F1; flagged transparently as a residual of that fix, not a fresh independent contradiction. New evidence: round-1 triage's required fix was "rewrite line 37 AND soften the writer-only framing at 30/37"; only the last sentence was rewritten.)
- Evidence: `pack/isolation-guidance.md:37`: "`ab spawn --local` is the un-sandboxed alternative ...; prefer the `--git` sandbox for writer agents. The orchestrator states the resolved isolation tier at preflight, so you know before a run whether a writer will run in a container, a worktree, or under the file-safety fallback. ... Every spawned agent, including the reviewers, the triager, and the explorers, runs under the resolved isolation tier ..." The first two sentences still single out "writer agents" / "a writer," then the last sentence generalises to every spawned agent, within one paragraph.
- Why it matters: under uniform isolation the `--git` sandbox should be preferred for every spawned agent, not only writers, and the preflight tier statement covers every agent. The added final sentence carries the correct universal rule and dissolves the hard contradiction round-1 F1 was about; the residual is that the how-to advice ("prefer the `--git` sandbox for writer agents") and the preflight paraphrase ("whether a writer will run in a container") were left writer-scoped, so a reader wiring reviewer/triager/explorer spawns could infer `--git` is a writer-only preference. Low because the module is explicitly the container-wiring module (heading "Writer isolation via agent-box and agent-images," the `ab new`/`ab spawn` commands are writer mechanics) and the closing sentence states the universal rule; the mismatch is soft, not a flat contradiction.
- Suggested fix (discretionary): change "prefer the `--git` sandbox for writer agents" to "prefer the `--git` sandbox for every spawned agent" and "whether a writer will run" to "whether an agent will run," completing the round-1 fix's parenthetical.

## Not raised (checked, coherent, or settled in round 1)

- The "keeps review from having to review its own reviews" rationale (`AGENTS.md:89`) vs the optional light review pass for an exploration (`AGENTS.md:65`): checked, not a contradiction. Line 65's review pass checks the design notes' quality; the notes are never accepted AS product (plan/code), which is the property `:89` asserts of read-only agents. An exploration is original advice, not itself a review, so it is not in the review-reviewing-reviews recursion the clause guards against. Reviewer A raised the adjacent point in round 1; direction (a) was chosen knowingly. No new evidence to overturn.
- Roles anchor `AGENTS.md:25` omits explorers: pre-existing, deemed acceptable in round 1 (explorers are a mode-specific role introduced in Design explorations, not one of the four core spawned roles). Unchanged; not re-raised.
- Historical/design records (`docs/plans/agent-scaffold.md` driver-output-generation narrative, `driver-output-generation.design.md`): frozen records, out of scope, ruled low in round 1. Untouched by this diff. Not re-raised.

## Severity summary

- critical: none.
- high: none.
- medium: none.
- low: 2 (F1 writer-scoped file-safety baseline framing; F2 residual writer-only framing in pack isolation-guidance line 37).

## Mechanical checks (run in worktree `ui-review-d` at `856ddf1`)

### `cargo run -- validate --source docs/plans/agent-scaffold.plan.toml`
```
docs/metrics/workflow.jsonl: 159 records, valid
docs/plans/agent-scaffold.plan.toml: 69 steps, 62 questions, valid
```

### `cargo run -- validate --workflow --source docs/plans/agent-scaffold.plan.toml`
```
docs/metrics/workflow.jsonl: 159 records, valid
docs/plans/agent-scaffold.plan.toml: 69 steps, 62 questions, valid
docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold
```

### `cargo run -- render --check docs/plans/agent-scaffold.plan.toml`
```
docs/plans/agent-scaffold.plan.toml: up to date
```

### `cargo test`
```
test result: ok. 342 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s
```
Plus the four integration binaries (1, 3, 1, 2 tests) all passed. The isolation-policy byte-guard `the_committed_scaffold_carries_the_isolation_policy_fragment` (committed `AGENTS.md` and `.agents/AGENTS.reference.md` both contain the exact `ISOLATION_POLICY_FRAGMENT`) and the content-pin `the_fragment_states_the_uniform_isolation_rule` both pass. No guard was weakened: the content-pin holds three assertions ("every spawned agent runs in the strongest isolation", "even a findings or an exploration file is a write", "the orchestrator's own integration-level ones"), all present in the fragment.

### Re-scaffold determinism
`cargo run -- scaffold --output-dir . --write --force --principles default --instrument` (no `nix fmt`) reported "Wrote to . (30 changed, 0 left untouched)" and left the tree clean: `git status --porcelain` shows no tracked modifications and no untracked files. The committed `AGENTS.md` / `.agents/` match scaffold output.
