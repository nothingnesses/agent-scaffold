# Plan review: Q-66 / Q-67 fold (workflow-consistency, completeness, scope lens)

Artifact: the plan fold at `1ce5af9`, diff `c0feb59..1ce5af9`. Reviewer worktree at
`.claude/worktrees/q66-plan-review-consistency`. Read-only with respect to the product;
this findings file is the only write.

Three findings: one medium, two low. No high or critical.

---

## F1 (medium): Q-67 reinforces the "closed list" that lives in the GENERATED, byte-guarded `isolation_policy` fragment, but claims it "edits no source" and never cross-references the completed `driver-output-generation` / `agents-worktree-planner-scope` (Q-51) that owns it

- Files:
  - `docs/plans/agent-scaffold.steps/planner-folds-decisions.md:8` and `:10`
  - `docs/plans/agent-scaffold.plan.toml:1221` (step title) and the mirror at `docs/plans/agent-scaffold.md:1159`, `:1161`

The Q-67 step's second bullet says:

> Reinforce that the orchestrator's closed list of direct-on-main integration edits (a step's status flip, an increment declaration, a round record, and the ledger's resume anchor) EXCLUDES authoring new questions or steps ...

and the documentation-currency paragraph asserts:

> This step edits no prompt file and no source: it is guidance-only. It does not change the closed list's membership; it only makes the excluded case (authoring decided entries) explicit where the actor was previously unnamed.

That exact four-item "closed list" is not free-standing editable prose in `pack/AGENTS.md`. It
is the GENERATED `ISOLATION_POLICY_FRAGMENT`:

- `src/isolation_policy.rs:33` defines it: "... The only edits made directly on main are the
  orchestrator's own integration-level ones ...: flipping a step's status, declaring an
  increment, recording a round record, and moving the ledger's resume anchor."
- It renders into the slot `pack/AGENTS.md:91`, which is literally the token
  `{{isolation_policy}}` (verify: `sed -n '91p' pack/AGENTS.md` prints `{{isolation_policy}}`).
- The expanded text appears in the rendered `AGENTS.md:91` (verify:
  `grep -n 'declaring an increment, recording a round record' AGENTS.md`).
- It is byte-guarded and drift-guarded: `src/isolation_policy.rs:64` asserts the fragment
  content, `:78` asserts `COMMITTED_AGENTS.contains(ISOLATION_POLICY_FRAGMENT)`.

Consequences the fold does not address:

1. There is no editable `pack/AGENTS.md` prose to "reinforce" the closed list in; line 91 is a
   `{{isolation_policy}}` substitution token. To change or extend what the closed list says
   (for example to add the explicit "EXCLUDES authoring new questions or steps" clause) the
   implementer must edit the SOURCE `src/isolation_policy.rs`. That contradicts the step's own
   "edits no ... source: it is guidance-only" claim.
2. If the implementer instead HAND-RESTATES the closed list into ordinary `pack/AGENTS.md`
   prose to avoid touching source, that is exactly the "standalone hand-edit" and the drifting
   hand-copied duplicate the plan's own single-source direction forbids (Q-51:
   `docs/plans/agent-scaffold.md:924` "in the shared generated `isolation_policy` fragment ...
   rather than landing as a standalone hand-edit"; Q-60: "always restate and avoid pointers
   where possible, instead rendering duplicates from a single source to prevent drift",
   `docs/plans/agent-scaffold.md:161`). This trips plan Principle 8 ("Structured data first,
   project for humans", the one-source-of-truth thinking the step itself cites).
3. The fragment already draws the exclusion the step wants: it says the integration edits
   "author no reviewed product content and so stay the orchestrator's direct job rather than a
   spawned agent's". Authoring a `[[question]]` or `[[step]]` IS reviewed product content, so
   the fragment ALREADY implies it is spawned-writer (planner) work, not an orchestrator direct
   edit. The step does not say whether its Part-2 "reinforce" adds anything beyond the existing
   fragment or is redundant with it.
4. The overlap is with a step that is DECIDED and marked COMPLETE, not open:
   `driver-output-generation` (Q-51) is `complete` (`docs/plans/agent-scaffold.md:260`), and
   `agents-worktree-planner-scope` is `skipped`, "Absorbed into `driver-output-generation`
   (`Q-51`), whose first fix authors this planner-isolation clarification in the shared
   generated `isolation_policy` fragment" (`docs/plans/agent-scaffold.md:922-924`). The Q-67
   fold never references either, even though it reinforces precisely that fragment's content.
   Contrast the sibling Q-66 step, which DID include a "Shared machinery with step 87 (the
   synergy note)" cross-reference (`docs/plans/agent-scaffold.steps/reviewer-reproducible-evidence.md:14`);
   Q-67 has no analogous note despite a clearer, already-built overlap.

Note: the Q-67 diagnosis premises about the CURRENT AGENTS.md text are all accurate (see
"Verified clean" below), so this is not a wrong-premise finding; it is a scope/completeness
gap in the FIX half (Part 2, the closed-list reinforcement).

- Fix: in `planner-folds-decisions.md` (and the mirrored TOML title / rendered md), (a) add a
  cross-reference to the `isolation_policy` fragment / Q-51, the way the Q-66 step references
  step 87; (b) resolve the mechanism for Part 2: either it is a SOURCE edit to
  `src/isolation_policy.rs` (in which case drop "edits no ... source: it is guidance-only" and
  add the `.agents/*` and byte-guard/drift-guard regeneration to the currency list), or it
  references the existing fragment rather than restating it (in which case say so and confirm
  Part 1, the human-input-contract / Socratic-mode / checkpoint prose naming the planner, is
  the only actual pack/AGENTS.md prose edit); and (c) state whether Part 2 adds anything the
  fragment's existing "author no reviewed product content" clause does not already cover.

---

## F2 (low): Q-66 file set omits `pack/prompts/checks-reviewer.md`, a third reviewer-role prompt that authors findings

- File: `docs/plans/agent-scaffold.steps/reviewer-reproducible-evidence.md:9-12` (the "Files
  the implementer changes" list names `pack/prompts/reviewer.md`, `pack/prompts/triager.md`,
  `pack/AGENTS.md`, and the deployed copies of those three).

The decided rule is "every reviewer finding must carry reproducible evidence proportional to
its claim". There are THREE reviewer-role prompts, not two (verify:
`grep -rlniE 'you are (a|the).*reviewer|spawned as one reviewer' pack/prompts/` returns
`triager.md`, `checks-reviewer.md`, `reviewer.md`). `checks-reviewer.md:3` is "the
deterministic checks reviewer, spawned as one reviewer in the work-review phase (phase 4)
alongside the LLM reviewers", and it authors findings (`checks-reviewer.md:13-15`). The Q-66
step and its deployed-copies bullet name only `reviewer.md`, `triager.md`, and their
`.agents/` copies; `checks-reviewer.md` (and `.agents/prompts/checks-reviewer.md`) is not
mentioned.

This is LOW, not a contradiction: `checks-reviewer.md:13` already mandates the strongest
evidence tier ("the check name, the exact command, its exit code, and the offending
`file:line`"), so it already complies with (indeed exceeds) the tiered rule and would not be
left self-contradictory. But an acceptance-phase documentation-currency reviewer (the check the
plan itself runs, AGENTS.md phase 5) could reasonably ask why the third reviewer prompt was
skipped.

- Fix: add one clause to the Q-66 step noting `pack/prompts/checks-reviewer.md` is deliberately
  out of scope because its findings already carry command/exit-code/`file:line` evidence, so a
  reader (and the acceptance currency check) sees the omission is intentional rather than a miss.

---

## F3 (low): Q-66 sidecar's "triager dismisses any testable claim whose demonstration does not reproduce" does not note that a high/critical such dismissal still passes the backstop re-check

- Files: `docs/plans/agent-scaffold.steps/reviewer-reproducible-evidence.md:6` ("The TRIAGER
  reproduces whatever the reviewer provided and DISMISSES any testable claim whose
  demonstration does not reproduce.") against the existing backstop at AGENTS.md
  `pack/AGENTS.md:59` ("before a dismissed finding at or above the backstop severity ... counts
  towards a clean round, or settles a single reviewers-then-triager pass ... have a second,
  independent triager (or a human) confirm the dismissal").

The two rules compose without conflict: Q-66 adds a new dismissal GROUND (evidence does not
reproduce); the backstop gates high/critical dismissals regardless of ground, so a
high/critical finding whose demonstration fails to reproduce is still a high/critical dismissal
that the backstop re-checks before it can count clean. There is therefore no defect, and the
composition holds automatically. This is a readability nuance only: as written, "DISMISSES any
testable claim whose demonstration does not reproduce" reads as an immediate, unconditional
dismissal, and an implementer writing the `triager.md` edit could phrase it as a fast-path that
appears to shadow the backstop.

- Fix (optional): add a half-sentence to the Q-66 step (and carry it into the `triager.md`
  edit) that a high/critical non-reproducing dismissal still goes through the second-triager
  backstop re-check before it counts toward a clean round, so the new dismissal ground does not
  read as a backstop bypass.

---

## Verified clean (checks that passed, for the orchestrator's confidence)

- Scope: the diff `c0feb59..1ce5af9` touches only plan content (`docs/metrics/workflow.jsonl`,
  `docs/plans/agent-scaffold.md`, `.plan.toml`, `.questions/`, `.steps/`). No `pack/AGENTS.md`,
  no `pack/prompts/*`, no `src/*` edit (verify: `git diff --stat c0feb59 1ce5af9`). The planner
  authored plan content only; the implement-phase edits are correctly deferred.
- `next` is a valid status (`src/plan.rs:93` lists it; `src/plan/source.rs:184` maps it) and
  there is exactly ONE `status = "next"` step, no "the next step" collision (verify:
  `grep -n 'status = "next"' docs/plans/agent-scaffold.plan.toml` -> only line 1209). Having a
  `next` step alongside the `in progress` step 86 is reconciled by the step body ("built first,
  before resuming the paused code-value-audit-static step 86").
- Status-line accounting is correct: 3 not-started + 2 in-progress + 58 complete + 4 skipped +
  1 next + 3 optional + 18 deferred = 89 (`docs/plans/agent-scaffold.md:20`); +2 from the prior
  87 matches the two added steps (one `next`, one `not-started`).
- `validate --source` passes: "89 steps, 67 questions, valid" and "204 records, valid" (verify:
  `cargo run -q -- validate --source docs/plans/agent-scaffold.plan.toml`, exit 0).
- The Q-66 synergy note's `src/checks.rs:94-95` citation is accurate: line 94 is the doc
  comment "A mutation-testing run (reserved for the mutation module; skipped here)." and line 95
  is the `Mutation,` variant (verify: `grep -n 'Mutation' src/checks.rs` -> `95: Mutation,`).
  The note correctly separates the reviewer's by-hand demonstration in its own agent worktree
  from the tool-side `checks` machinery step 87 will automate, and does not over-claim
  (it explicitly says it does not build the mutation module or touch `src/checks.rs`).
- All four Q-67 premises about the CURRENT `pack/AGENTS.md` text are accurate:
  - request-interrupt path names the planner: `pack/AGENTS.md:39` "non-trivial and routes to
    the planner to fold into the plan".
  - human-input-contract is actor-less passive: `pack/AGENTS.md:41` "A resolved decision is
    recorded in the plan's Open Questions section and folded into the step it affects".
  - Socratic mode only says it reuses machinery: `pack/AGENTS.md:43` "reusing the intake and
    Open-Questions machinery".
  - checkpoint rule: `pack/AGENTS.md:71` "the orchestrator updates this queue".
- Q-66 completeness for the two named prompts: `pack/AGENTS.md` states reviewer/triager evidence
  duties at the Reviewers bullet (`:21`), the Triager bullet (`:22`), and the Findings files
  section (`:67`); the step's "Reviewers and Triager role bullets and/or the Findings files
  section" covers all three. The "Design explorations" section (`:65`) instructs EXPLORERS, not
  reviewers, and explorers do not produce findings, so it does not need the rule and is
  correctly out of scope.
