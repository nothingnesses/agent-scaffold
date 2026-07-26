# Step 89 review (Q-67, `planner-folds-decisions`) -- reviewer: FIDELITY and SOUNDNESS

Lens: does the change correctly and coherently name the PLANNER as the folder of
every non-trivial decided decision, at the human-input-contract, Socratic-mode,
and checkpoint points in `pack/AGENTS.md`, cross-referencing (not restating) the
generated isolation-policy fragment?

Worktree: `/home/jessea/Documents/projects/agent-scaffold/.claude/worktrees/step89-review-a` at `aa771ce`.
Combined diff reviewed: `git diff f1abf65 aa771ce` (title commit `c0e880a` + prose
commit `aa771ce`, both in range: `git log --oneline f1abf65..aa771ce`).

## Verdict: ZERO findings.

The change is faithful to the decided rule and internally sound. Every adversarial
check below reproduced clean. No low/medium/high/critical findings.

## Adversarial checks run (all cleared, with evidence)

### 1. All THREE points edited and each names the planner as author of the non-trivial fold. CLEAR.
`git diff f1abf65 aa771ce -- pack/AGENTS.md` shows exactly three changed
paragraphs (6 changed content lines: `... | grep -E '^[-+]' | grep -v '^[-+][-+]' | wc -l` = 6).

- Human-input contract, `pack/AGENTS.md:41`: added
  "when that fold is non-trivial (authoring a `[[question]]` or a `[[step]]`), the
  orchestrator routes it to the planner to author, as on the request-interrupt path
  above, rather than editing the plan directly." Names planner as author, orchestrator
  as router. Faithful.
- Socratic mode, `pack/AGENTS.md:43`: added
  "its non-trivial fold routed to the planner to author as above rather than edited in
  directly." Names planner as author. Faithful.
- Checkpoint, `pack/AGENTS.md:71`: added
  "Here \"updates this queue\" means raising and pushing the open items, not authoring a
  decided decision's `[[question]]` or `[[step]]` fold, which is the planner's job
  (routed as above) ...". Names planner as author of the fold, orchestrator as queue-pusher.
  Faithful.

### 2. Cross-reference to the request-interrupt path is real and accurate, not a dangling "as above". CLEAR.
The request-interrupt path exists ABOVE all three edits at `pack/AGENTS.md:39` and
says the planner folds:
`grep -n "routes to the planner to fold into the plan" pack/AGENTS.md` -> line 39:
"... is non-trivial and routes to the planner to fold into the plan ...".
Line 41's "as on the request-interrupt path above" and line 43's "as above" both point
upward to a passage that does name the planner as folder; the referent is correct in
direction and in content. Line 43's "as above" is loose (either line 39 or line 41
satisfies it), but both referents name the planner, so no incorrect meaning results.

### 3. Checkpoint clarification is correct and cross-references (does not hand-copy) the closed list. CLEAR.
Line 71 says "updates this queue" means "raising and pushing the open items, not
authoring a decided decision's `[[question]]` or `[[step]]` fold" -- the positive-side
statement the ask requires. It refers to "the generated isolation-policy fragment below"
(the `{{isolation_policy}}` slot is rendered at `pack/AGENTS.md:91`, i.e. BELOW line 71,
so "below" is directionally correct) and characterises it as listing "the orchestrator's
closed set of direct-on-main edits, which author no reviewed product content" WITHOUT
enumerating the four items. The source fragment (`src/isolation_policy.rs:33`) lists
"flipping a step's status, declaring an increment, recording a round record, and moving
the ledger's resume anchor"; none of those four strings appears in the added line-71
prose. No hand-copy of the closed list. The inference "a `[[question]]` or `[[step]]`
(reviewed product content) is not among them" is sound against the fragment's own
"author no reviewed product content" rationale. The parenthetical "(reviewed product
content)" correctly disambiguates authoring a step's body from the orchestrator's
status-flip on an existing step, so there is no conflation with the closed list's
"flipping a step's status".

### 4. No contradiction with the rest of AGENTS.md. CLEAR.
- Orchestrator role, `pack/AGENTS.md:19`: "It does not plan, implement, review, or
  triage itself." Routing the fold to the planner reinforces this; no contradiction.
- Phase-2 planner scope, `pack/AGENTS.md:30`: `[[question]]`/`[[step]]` entries live in
  "the `<task>.plan.toml` skeleton" which "The planner edits". Naming the planner as the
  author of those entries is consistent, not contradictory.
- Writer-review rule (`pack/AGENTS.md:91` region: "A writer authors the reviewed product,
  so its output goes through the review and convergence loop before it is accepted")
  already implies the planner-authored fold is reviewed, so the prose correctly does not
  restate review re-entry (consistent with the "reuses the intake and Open-Questions
  machinery" framing at lines 43 and elsewhere).

### 5. Title alignment (title, ask, sidecar all Part-1-only). CLEAR.
- Plan title `docs/plans/agent-scaffold.plan.toml:1225`: "name the PLANNER as the folder
  of every non-trivial decided decision at the human-input-contract / Socratic-mode /
  checkpoint points in `pack/AGENTS.md` (`Q-67`)". The prior "and reinforce that the
  orchestrator's closed list ... EXCLUDES authoring new questions or steps" clause is
  gone (removed in `c0e880a`; confirmed by the diff hunk on that line).
- Q-67 ask (`docs/plans/agent-scaffold.plan.toml:1680`): DECIDED text scopes the pass to
  "the three actor-less `pack/AGENTS.md` prose points" and states "this pass restates none
  of that list".
- Sidecar title (`docs/plans/agent-scaffold.steps/planner-folds-decisions.md:1`): "name
  the planner as the folder of every non-trivial decided decision (`Q-67`)".
  All three agree on Part-1-only scope; no leftover closed-list clause.

### 6. Guidance is hand-prose only; the generated region and source are untouched. CLEAR.
`git show --stat aa771ce` = 3 files (`pack/AGENTS.md`, `AGENTS.md`,
`.agents/AGENTS.reference.md`), 3 lines each; no change to `src/isolation_policy.rs` or
the `{{isolation_policy}}` slot. Render fidelity of the two deployed copies confirmed by
`cargo test the_committed_scaffold` -> 5 passed, 0 failed (includes
`agents_md_drift::the_committed_scaffold_matches_a_fresh_render` and
`isolation_policy::the_committed_scaffold_carries_the_isolation_policy_fragment`), so the
committed `AGENTS.md` and `.agents/AGENTS.reference.md` are a fresh render of the edited
pack and the isolation fragment is byte-identical. The ~28 uncommitted formatter reflows
are correctly out of the commit (working tree clean at `aa771ce`: `git status`).

### 7. Passive "is recorded ... and folded" retained at line 41 -- checked, faithful. CLEAR.
The step targets the actor-less passive at the contract point. The passive clause is
retained but the appended "(authoring a `[[question]]` or a `[[step]]`) ... the
orchestrator routes it to the planner to author" maps the recording act (a `[[question]]`
in the Open Questions section) and the step-fold act (a `[[step]]`) to the planner for the
non-trivial case, which is the scoped case (title: "every non-trivial decided decision").
The actor for the in-scope case is named; no fidelity gap.
