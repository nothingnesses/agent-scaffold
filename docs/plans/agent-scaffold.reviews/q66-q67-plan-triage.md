# Triage: Q-66 / Q-67 plan fold (round 1)

Triager: independent adjudicator, read-only w.r.t. the product; this file is the only write.
Artifact: plan fold at `1ce5af9` (steps `reviewer-reproducible-evidence` order 88 / Q-66,
`planner-folds-decisions` order 89 / Q-67), two decision receipts, re-rendered plan.
Worktree: `.claude/worktrees/q66-plan-triage`.

## Round outcome

NEW VALID FINDINGS. 4 findings adjudicated: 1 medium valid (F1), 3 low valid (F2, F3, F-fid).
No high or critical finding in the set (both reviewer files state this and it reproduces:
F1 medium, F2/F3/F-fid low), so NO backstop re-check is required. One finding (F1) needs a
substantive planner revision before implementation; I judge its mechanism a clear planner call,
NOT a human-escalation fork (reasoning under F1). The other three are cheap step-text additions.

Rule applied to this triage (the change under review): every finding below was re-checked against
its cited `file:line` / command before ruling. Nothing is upheld that did not reproduce.

## F1 (medium, consistency) -> VALID (severity upheld: medium)

Claim: Q-67's Part 2 tells the implementer to "reinforce" the orchestrator's closed list of
direct-on-main edits, but that closed list is the GENERATED `ISOLATION_POLICY_FRAGMENT`, not
hand-editable pack prose, so Part 2 either needs a SOURCE edit (contradicting the step's "no
source: guidance-only" claim) or a hand-restatement that drifts from the single source (against
Principle 8); and the fold never cross-references the completed `driver-output-generation` (Q-51)
/ skipped `agents-worktree-planner-scope` that own the fragment.

Reproduced (all premises hold):
- `planner-folds-decisions.md:8`: "Reinforce that the orchestrator's closed list of direct-on-main
  integration edits (a step's status flip, an increment declaration, a round record, and the
  ledger's resume anchor) EXCLUDES authoring new questions or steps ...". Confirmed verbatim.
- `planner-folds-decisions.md:10`: "This step edits no prompt file and no source: it is
  guidance-only. It does not change the closed list's membership ...". Confirmed verbatim.
- `src/isolation_policy.rs:33`: `ISOLATION_POLICY_FRAGMENT` is a generated `const &str` whose
  tail is exactly the four-item closed list "flipping a step's status, declaring an increment,
  recording a round record, and moving the ledger's resume anchor." Confirmed. It also already
  carries the rationale "which author no reviewed product content and so stay the orchestrator's
  direct job rather than a spawned agent's".
- `pack/AGENTS.md:91` = the literal token `{{isolation_policy}}` (sed confirms). So the closed
  list at that slot is NOT free-standing editable prose; it is a substitution.
- Byte-guard / drift-guard: `src/isolation_policy.rs:48` (`the_fragment_states_...`) and `:78`
  (`COMMITTED_AGENTS.contains(ISOLATION_POLICY_FRAGMENT)`) pin the fragment. Confirmed.
- Overlap targets: `docs/plans/agent-scaffold.md:260` `driver-output-generation` is `complete`;
  `:259`/`:922-924` `agents-worktree-planner-scope` is `skipped`, absorbed into
  `driver-output-generation` (Q-51), "whose first fix authors this planner-isolation
  clarification in the shared generated `isolation_policy` fragment ... rather than landing as a
  standalone hand-edit." The Q-67 step references NEITHER. Confirmed. Contrast the sibling Q-66
  step's step-87 synergy cross-reference (`reviewer-reproducible-evidence.md:14`). Confirmed.

Adjudication: the finding is a real scope/completeness gap in the FIX half. The step's own
caveat forecloses the worst reading (it asserts no source edit and no membership change), but it
does NOT say through WHAT prose Part 2's "reinforce" lands, and it omits the cross-reference the
plan's own single-source direction makes expected here. Left unresolved, an implementer could
plausibly (a) edit `src/isolation_policy.rs` to add the "EXCLUDES ..." clause (contradicting the
step's "no source" claim and pulling in unlisted `.agents/*` + drift-guard regeneration), or (b)
hand-restate the closed list into ordinary `pack/AGENTS.md` prose, which is precisely the
"standalone hand-edit" `docs/plans/agent-scaffold.md:924` already rejects and a Principle-8 drift.
That is a not-hard-to-hit implementer error with a single-source violation as the downside, which
warrants medium. Severity upheld.

Resolution recommendation: OPTION (a). Reduce/drop Part 2's closed-list "reinforce" and keep only
Part 1 (name the planner at the actor-less human-input-contract / Socratic-mode / checkpoint prose
points, which ARE hand-editable `pack/AGENTS.md` prose at lines 41, 43, 71), plus add a
cross-reference to the `isolation_policy` fragment / Q-51 `driver-output-generation` the way Q-66
references step 87, and state that Part 2 adds nothing the fragment's existing "author no reviewed
product content" clause does not already cover.

Grounding by the plan's Project Principles, by name:
- Principle 8 ("Structured data first, project for humans", one-source-of-truth): the closed list
  already lives ONCE in the generated fragment, and that fragment ALREADY implies the exclusion,
  since authoring a `[[question]]` or `[[step]]` IS reviewed product content and the fragment
  reserves the orchestrator's direct edits to those that "author no reviewed product content."
  Restating or editing the list duplicates a single source. `docs/plans/agent-scaffold.md:924`
  is direct precedent: this exact planner-isolation clarification was deliberately routed into the
  generated fragment, "rather than landing as a standalone hand-edit."
- Principle 2 ("Minimal by default"): the minimal correct edit is Part 1 (name the planner at the
  three actor-less points) plus the checkpoint clarification of what "the orchestrator updates
  this queue" excludes; adding a redundant exclusion clause to the fragment (option b) is
  over-scoped because the fragment's rationale already covers it.
- Principle 1 ("Prefer the cleaner long-term architecture over the smallest diff"): relying on the
  single generated fragment's existing rationale rather than a duplicated hand clause is the
  cleaner long-term rule.
Option (b) (scope an explicit source edit) is rejected as over-scoped: it adds content the fragment
already implies and expands the step past "guidance-only" for no gain. Option (c) (leave as-is) is
rejected: it leaves the mechanism ambiguous and the cross-reference missing.

Human decision needed? NO. This is a clear planner call, not a genuine fork. The decided rule
(Q-67 receipt: "NAME THE PLANNER ... at each point of use" and "reinforce that the ... closed list
... EXCLUDES authoring new questions or steps") is satisfied by option (a) without re-deciding
anything the human decided: "reinforce" is met by the fragment's existing rationale plus the
checkpoint prose clarification. The three Principles above point the same way, and the plan already
set the precedent at `:924`. The planner should revise the step per (a) in the next round; no
escalation.

## F2 (low, consistency) -> VALID (low)

Claim: the Q-66 file set omits `pack/prompts/checks-reviewer.md`, a third reviewer-role prompt
that authors findings.

Reproduced:
- `ls pack/prompts/` lists `checks-reviewer.md` (plus reviewer.md, triager.md, and non-reviewer
  prompts). Confirmed it exists.
- `grep -rlniE 'you are (a|the).*reviewer|spawned as one reviewer' pack/prompts/` returns exactly
  three: `checks-reviewer.md`, `reviewer.md`, `triager.md`. Confirmed there are three
  reviewer-role prompts.
- `checks-reviewer.md:3`: "You are the deterministic checks reviewer, spawned as one reviewer in
  the work-review phase (phase 4) alongside the LLM reviewers." It authors findings and already
  mandates the strongest evidence tier: "Report each finding with ... the check name, the exact
  command, its exit code, and the offending `file:line`". Confirmed.
- `reviewer-reproducible-evidence.md:9-12` names only `reviewer.md`, `triager.md`, `pack/AGENTS.md`,
  and the `.agents/` copies of reviewer/triager; `checks-reviewer.md` is not mentioned. Confirmed.

Adjudication: valid, low. Not a contradiction: checks-reviewer.md already requires
command + exit-code + `file:line` on every finding, i.e. it already complies with (indeed exceeds)
the Q-66 tiered rule, so no edit to it is needed and it would not be left self-contradictory. But
the omission reads as an unexplained gap; a phase-5 documentation-currency reviewer could ask why
the third reviewer prompt was skipped.

Fix: add one clause to the Q-66 step noting `pack/prompts/checks-reviewer.md` (and its `.agents/`
copy) is deliberately out of scope because its findings already carry command / exit-code /
`file:line` evidence, so the omission is visibly intentional rather than a miss.

## F3 (low, consistency) -> VALID (low; fix is optional polish)

Claim: the Q-66 sidecar's "triager DISMISSES any testable claim whose demonstration does not
reproduce" does not note that a high/critical such dismissal still passes the existing backstop
re-check (`pack/AGENTS.md:59`).

Reproduced:
- The TRIAGER sentence is at `reviewer-reproducible-evidence.md:5` (the reviewer cited `:6`; the
  quoted text is on line 5, an off-by-one cite, immaterial to substance): "The TRIAGER reproduces
  whatever the reviewer provided and DISMISSES any testable claim whose demonstration does not
  reproduce." Confirmed.
- `pack/AGENTS.md:59` is the backstop paragraph: "before a dismissed finding at or above the
  backstop severity ... counts towards a clean round, or settles a single reviewers-then-triager
  pass ... have a second, independent triager (or a human) confirm the dismissal." Confirmed at
  line 59.

Adjudication: valid, low, and there is NO defect in the plan itself: the two rules compose
correctly and automatically. Q-66 adds a new dismissal GROUND (evidence does not reproduce); the
backstop gates high/critical dismissals regardless of ground, so a high/critical non-reproducing
dismissal is still a high/critical dismissal the backstop re-checks. The concern is only that the
sentence, read alone, looks like an unconditional fast-path, and a downstream implementer writing
the `triager.md` edit could phrase it as a backstop bypass. This is a readability/completeness
nuance, not a correctness gap. The reviewer themselves marked the fix optional; I concur.

Fix (optional): add a half-sentence to the Q-66 step (carried into the `triager.md` edit) that a
high/critical non-reproducing dismissal still goes through the second-triager backstop re-check
before it counts toward a clean round, so the new ground does not read as a bypass.

## F-fid (low, fidelity) -> VALID (low)

Claim: the Q-66 `ask` and sidecar label the chosen option "(Option B)", but the receipt lists
"Tiered reproducible evidence" in position A (first).

Reproduced:
- `docs/plans/agent-scaffold.plan.toml:1669` (Q-66 `ask`): "DECIDED (Socratic, human, 2026-07-26):
  TIERED REPRODUCIBLE EVIDENCE (Option B)." Confirmed.
- `reviewer-reproducible-evidence.md:5`: "The rule (Option B, tiered reproducible evidence)."
  Confirmed.
- Q-66 receipt (`grep -F '"q_id":"Q-66"' docs/metrics/workflow.jsonl`):
  `"options":["Tiered reproducible evidence","Runnable test for every finding","Triager-enforced,
  reviewer SHOULD","Keep current process"]`, `"chosen":"Tiered reproducible evidence"`. Confirmed.
  Mapping array positions to letters, the chosen option is at position A; "Option B" would be
  "Runnable test for every finding".

Adjudication: valid, low, cosmetic. The prose parenthetical "(Option B)" contradicts the only
durable structured record (the receipt `options` array), whose ordering puts the chosen option
first (A). No functional impact: W4 only checks `chosen` is a member of `options`, and validate
passes. As the fidelity reviewer notes, "(Option B)" may reference the original Socratic
presentation order shown to the human, which is not recorded structurally, so it is not provably
wrong about history, only inconsistent with the record a reader would reconstruct.

Fix: drop the "(Option B)" parenthetical (it references an ordering the receipt does not preserve)
in both the Q-66 `ask` (`plan.toml:1669`) and the sidecar (`reviewer-reproducible-evidence.md:5`),
or renumber it to "(Option A)" to match the receipt array position. Dropping is simplest.

## Backstop / escalation

No high or critical finding in the round (F1 medium; F2, F3, F-fid low). NO second-triager
backstop re-check is required. No finding requires human escalation: F1's mechanism is a clear
planner call under Principles 8, 2, and 1 (option a), not a genuine source-vs-prose fork.
