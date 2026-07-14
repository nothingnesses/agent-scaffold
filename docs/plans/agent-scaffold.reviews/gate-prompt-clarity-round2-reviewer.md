# Round 2 review: `gate-prompt-clarity` (Q-20)

Independent reviewer, round 2. Diff range: `fc30bb3..593b71d`. Source of truth is
`pack/`; `.agents/prompts/` and root `README.md` are generated targets.

Fixes verified; no new findings.

## Round-1 fix verification

R1 (blank line before "Otherwise"): VERIFIED FIXED. `pack/prompts/open-questions-gate.md`
now has the "If so ..." branch (lines 8-11) separated from the "Otherwise, state that
none are open ..." branch (line 13) by a blank line (line 12). The branch-specific
sentence "Prefer the approaches most consistent with the project's principles." ends the
if-branch paragraph; "Otherwise" starts its own paragraph. The two-way gate decision reads
cleanly again.

R2 ("surface" as a verb): VERIFIED FIXED. `grep -ni surface` over both gate prompts
returns nothing. Both routing sentences now read as intended:
- `clarifying-questions.md:14-16`: "... return your questions to the orchestrator, which
  relays them to the human and returns the answers."
- `open-questions-gate.md:15-17`: "... return the open items to the orchestrator, which
  relays them to the human and returns the decisions."
Routing is still correct: the sub-agent -> orchestrator -> human path and the human as
decider are both stated, and the conditional clause ("If you are a sub-agent without a
direct channel to the human") leaves the single-agent case untouched.

S1 (orchestrator gate-relay duty): CONFIRMED INTENTIONALLY DEFERRED. `orchestrator.md`
does not appear in the diff (`git diff --name-only` has no orchestrator entry), so no
edit was made here. The scope note is present and correct in `docs/plans/agent-scaffold.md`
under the `deliberation-mode` step (diff lines 100-101): it names S1, states that the gate
prompts now assert an orchestrator relay duty not restated in `orchestrator.md`, and
instructs `deliberation-mode` to add a sentence making that duty explicit. It is also
recorded in the ledger completion entry. Not re-raised as a defect.

## Overall checks

- Human named as decider in both prompts: `clarifying-questions.md:14` "The human is the
  decider here."; `open-questions-gate.md:15` "The human decides which options to take."
- Both prompts retain their gate function (ask/present before proceeding, convert
  assumptions to questions, record open items with approaches/trade-offs/recommendation,
  the "otherwise proceed" branch). Nothing material dropped by the reflow.
- No residual first-person routing ("ask me" / "for me to review" / "we confirm"): grep
  clean.
- README label correct: `README.md:37` "role prompts and the planner's decision gates";
  `:40` "gate: agent asks, the human answers, before starting"; `:41` "gate: agent
  presents options, the human chooses". These match the prompt bodies.
- Mirrors in sync: `pack/prompts/*` and `.agents/prompts/*` are byte-identical for both
  files (`diff` clean).
- ASCII-clean: `grep -P '[^\x00-\x7F]'` over both prompts, both mirrors, and `README.md`
  returns nothing.
- No new defect introduced by the round-1 fixes.

## Non-blocking observation (not a finding)

`README.md:69` prose still says ".agents/prompts/ ... carries one prompt per role". Since
the two gate prompts are planner decision gates rather than per-role prompts (as the newly
corrected layout label at line 37 now distinguishes), this prose is slightly looser than
the fixed label. It is pre-existing text outside this diff range and was not introduced or
touched by the round-1 fixes, so it is out of scope for this step's verification and I am
not raising it as a finding. Recording it only so it is not lost; a future README pass or
`deliberation-mode` could align it.
