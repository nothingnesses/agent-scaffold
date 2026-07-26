# Step 89 triage (Q-67, `planner-folds-decisions`) -- round 1

## Outcome

ROUND 1 IS CLEAN. One low finding (F1) from the consistency reviewer; the fidelity
reviewer reported zero. F1's evidence reproduces and its observation is accurate,
but it is VALID-BUT-ACCEPT-RESIDUAL for this step: it names a residual currency gap
in a file (`pack/prompts/orchestrator.md`) that Q-67 deliberately excluded, not a
defect in step 89's decided scope. Recommended resolution scope: (b) OUT-OF-SCOPE
FOLLOW-UP. No high or critical findings anywhere in round 1, so no backstop re-check
is owed. Accepting F1 as residual does not block convergence (orchestrator prompt,
"A valid finding may instead be resolved by consciously accepting its residual
risk and recording that; an accepted risk does not block convergence").

Triager worktree: `.claude/worktrees/step89-triage` at `aa771ce`. All evidence below
is reproduced from that worktree per the Q-66 tiered-reproducible-evidence rule.

## F1 verdict: VALID-BUT-ACCEPT-RESIDUAL

Finding (consistency reviewer, low, self-flagged out-of-scope): `pack/prompts/
orchestrator.md:33` describes durable decisions folding into the plan passively
("fold into it") and routes only the request-interrupt path to the planner
(`orchestrator.md:29`), so the prompt does not yet name planner-routing on the
decision-fold path that `pack/AGENTS.md:41` now establishes. Reviewer calls it a
currency GAP (silence on the actor), not a contradiction.

### Evidence reproduced (all confirmed verbatim)

- `pack/prompts/orchestrator.md:29` reads "route anything non-trivial to the planner
  to fold into the plan (revise the Roadmap steps and Success Criteria, resolve any
  new open questions), then re-enter plan review." So the prompt routes only the
  request-interrupt path to the planner. Confirmed.
- `pack/prompts/orchestrator.md:33` reads "The ledger is separate from the plan: do
  not put individual findings in the plan's Open Questions section; only durable
  decisions, the ones that change the plan, fold into it." Passive "fold into it",
  no actor named. Confirmed.
- `pack/AGENTS.md:41` now reads "... when that fold is non-trivial (authoring a
  `[[question]]` or a `[[step]]`), the orchestrator routes it to the planner to
  author, as on the request-interrupt path above, rather than editing the plan
  directly." So AGENTS.md now names planner-routing on the decision-fold path.
  Confirmed.
- orchestrator.md is SILENT, not contradicting: line 33 is passive and never asserts
  the orchestrator folds decisions itself. Its human-input-contract counterpart,
  `orchestrator.md:31`, already references "the human-input contract in `AGENTS.md`"
  and states "This covers every human-input point", so the prompt already inherits
  the AGENTS.md:41 rule by reference. Confirmed.

### Why VALID-BUT-ACCEPT-RESIDUAL and not VALID or INVALID

The observation is factually accurate (so not INVALID), but it is not a defect step
89 must repair to be complete (so not a bare VALID that forces a fix). Three
reproduced facts settle this:

1. Q-67's decided scope excluded the prompt. The Q-67 DECIDED text
   (`plan.toml`, Q-67 ask) states the pass "edits only the three actor-less
   `pack/AGENTS.md` prose points" and "The work this schedules edits `pack/AGENTS.md`
   only (guidance, no prompt or source change)". The step title was narrowed to
   Part-1-only in a dedicated commit `c0e880a` ("align planner-folds-decisions step
   title to Part-1-only scope"). The step sidecar repeats "all in `pack/AGENTS.md`"
   and "This step edits no prompt file and no source". `orchestrator.md` was never in
   scope; it was deliberately left out.

2. The prompt line's true AGENTS.md twin was itself deliberately left passive.
   `orchestrator.md:33` is the ledger-separation passage; its structural counterpart
   in AGENTS.md is not line 41 (the human-input contract) but line 63 (the
   "Preventing relitigation (the ledger)" paragraph), which still reads "only durable
   decisions, the ones that change the plan, fold into the plan's steps" -- passive,
   no actor. The step-89 diff (`git show aa771ce -- pack/AGENTS.md`) touched only the
   hunks at lines 41/43 and 71; line 63 was NOT edited. So the identical passive
   "fold into" phrasing survives inside AGENTS.md itself by design. orchestrator.md:33
   is therefore no more stale than AGENTS.md:63; step 89 introduced no new
   inconsistency in the prompt.

3. One source of truth is satisfied by reference, not violated. The
   planner-routing rule now lives in one authoritative place (AGENTS.md:41), and the
   prompt's human-input-contract paragraph (orchestrator.md:31) points to it. That is
   the intended prompt-to-AGENTS.md model. Restating the routing at orchestrator.md:33
   would duplicate a rule the prompt already references, cutting against one-source-of-
   truth rather than serving it.

The residual (the prompt does not name the actor at its own point of use, as AGENTS.md
now does at three of its points) is real but acceptable for step 89: nothing is WRONG
today, the gap is symmetric with a passage Q-67 deliberately left passive, and the
rule is reachable from the prompt by reference.

## Resolution scope: recommend (b) OUT-OF-SCOPE FOLLOW-UP

### Against (a) FIX NOW

Option (a) would add a planner-routing clause to `orchestrator.md` (plus a
`scaffold-self` regen of `.agents/prompts/orchestrator.md`) under step 89, arguing
from documentation-currency and one-source-of-truth. It fails on three grounds:

- No silent scope expansion (the AGENTS.md workflow guidance that Q-67's own
  rationale cites) forbids exactly this. Q-67 was narrowed to `pack/AGENTS.md` only in
  a dedicated title commit; folding an orchestrator.md edit into step 89 now would
  silently expand a decided, deliberately-bounded step. Doing so would violate the very
  boundary discipline the decision was made to enforce.
- Minimal by default (plan Principle 2): the decision explicitly chose the small,
  guidance-only pass and recorded the prompt-side change as out of it. The chosen
  alternative on record rejected only "capturing just the diagnosis" for the in-scope
  bug; the prompt edit was never part of the decided work.
- The currency argument overstates the gap. Because orchestrator.md:33 is silent
  (not contradicting), its twin AGENTS.md:63 is deliberately still passive, and the
  rule is referenced from orchestrator.md:31, there is no currency shortfall that step
  89 created and must close to be complete. (a) treats a pre-existing, symmetric
  residual as if step 89 had broken currency, which it did not.

### For (b) OUT-OF-SCOPE FOLLOW-UP

- No silent scope expansion (AGENTS.md guidance) and Structured data first, project
  for humans (plan Principle 8, its one-source-of-truth thinking): keep step 89 to its
  decided `pack/AGENTS.md`-only scope; enter any new prompt-side work through the plan
  as its own step rather than quietly widening this one.
- Prefer the cleaner long-term architecture over the smallest diff (plan Principle
  1): the follow-up IS worth recording, because the orchestrator drives from the
  prompt it reads, and naming the planner on the decision-fold path in the prompt (and
  in the AGENTS.md:63 ledger passage) would close the same actor ambiguity everywhere
  it lives. That is the cleaner long-term rule; it is simply a new, separately-scoped
  unit of work, not a completion owed by step 89.

Recommended follow-up scope, for the orchestrator/human to size and schedule: a new
deferred step covering BOTH `pack/prompts/orchestrator.md` (name planner-routing on
the decision-fold path; consider line 33 and/or a clause near line 29's request path)
AND the parallel `pack/AGENTS.md:63` ledger passage, since they are the same passive-
fold passage in two files. Regenerate the deployed copies (`.agents/prompts/
orchestrator.md`, `AGENTS.md`, `.agents/AGENTS.reference.md`) via `just scaffold-self`.
Grounding: keeps ONE authoritative statement (AGENTS.md:41) that the other points
name or reference, matching Q-67's own "name the actor at the point of use" reasoning.

## Does this need a human decision?

Split:

- The triager's immediate verdict does NOT need a human. Accepting a low, out-of-scope
  residual for step 89 (and declining to expand the step's scope) is within the
  triager's remit; the convergence rule expressly allows recording a valid finding as
  accepted residual, and the finding is low, not high/critical, so no backstop applies.
- Scheduling the follow-up DOES go to the human, via the orchestrator, per the
  human-input contract. Authoring a new `[[step]]` (or `[[question]]`) is reviewed
  product content and so the planner's job, routed by the orchestrator, entered through
  the plan -- which is precisely the rule Q-67 just codified. The orchestrator should
  raise it as an Open-Questions item at the next checkpoint (its scope, and whether to
  broaden it to AGENTS.md:63, being the decision the human owns). The triager records
  the recommendation; it does not schedule the step itself.

If the orchestrator/human instead prefers (a), it must run as a properly-scoped new
step, not be folded into step 89 silently.

## Round outcome and backstop

- Round 1 outcome: CLEAN. F1 is VALID-BUT-ACCEPT-RESIDUAL, resolved by accepting the
  residual and recording a follow-up (scope b); no new valid finding is fixed under
  step 89, so this is not a new-valid round. The consecutive-clean streak increments.
- Backstop: NOT required. F1 is low and is accepted-as-residual (not dismissed); both
  reviewers reported no high or critical finding. No second-triager re-check is owed.
