# Triage: triager-on-findings round 1

Triager: independent, read-only except this verdict file. Independent of both the
producer and the orchestrator. Scope: adjudicate the findings raised by round 1 on
`plan/triager-on-findings`, commit `423bf9b`, base main `cfeec01`. Two reviewers ran:
reviewer A (consistency/completeness) raised one medium finding F1; reviewer B
(capture/integrity/mechanical) raised zero findings. I adjudicate on evidence and
severity; I do not re-review afresh for new issues and I do not fix.

## F1: Review-reports paragraph names the triager's findings file as an input to every review-mode report, contradicting the new zero-findings clean-report path

Reviewer A location: `pack/AGENTS.md` line 69 (mirrored byte-identically in `AGENTS.md`
line 69 and `.agents/AGENTS.reference.md` line 69). Reviewer A severity: medium.

Verdict: VALID. Final severity: medium (confirmed, unchanged from the reviewer's rating).
Blocking: yes, this requires a fix and another round (NEW-VALID).

### Claims verified in the worktree

Both paragraphs read directly from the shipped commit via
`git show 423bf9b:pack/AGENTS.md`.

- Review-entry-mode paragraph (line 47) establishes the zero-findings carve-out.
  Quoted: it staffs "reviewers, and a separate triager when the reviewers raise one or
  more findings; a zero-findings review produces a clean report with no triage step".
  So the change itself states that a zero-findings review has no triage step, and
  therefore (per the Findings-files rule at line 67, where "the triager's is
  `<step>-triage.md`") no triager findings file is produced on a clean review.
- Review-reports paragraph (line 69) asserts the triager's findings file as an
  unconditional input. Quoted: "A review-entry-mode run (the review mode above)
  terminates in a single committed report ..., which the orchestrator synthesises from
  the reviewers' and the triager's findings files". The clause "the reviewers' and the
  triager's findings files" names a triager findings file as an input to every
  review-mode report, with no conditional.
- The contradiction is real and internal to the same review entry mode. For a
  zero-findings review, line 47 says no triager runs and no triager findings file
  exists, yet line 69 describes the report as synthesised from that non-existent file.
  Line 69 is exactly the leftover always-run implication this change set out to remove:
  the change added conditional phrasing everywhere else that mentions the triager
  (phases 3, 4, 5, the convergence-decision line, the ledger round-record line, and the
  orchestrator prompt), but line 69 was not updated to match.

### Why medium (confirmed, not raised)

- Practical impact is limited, which is why this is medium and not high. On a clean
  (zero-findings) review the report body is trivially empty: there are no triager-valid
  findings, no dismissed findings, and no kickoff tasks, so an orchestrator following
  the rest of the guidance would still write a correct clean report regardless of the
  mis-stated input clause. No gate or test keys on this wording (it is guidance prose,
  and reviewer B confirmed `render --check` up to date and `cargo test` green).
- It is a self-contradiction in shipped workflow guidance, not a cosmetic nit, so it is
  above a low. The acceptance/review path is precisely the path this change introduced
  the carve-out for, and line 69 is a dangling always-run triager dependency on the
  clean run. Medium is the correct rating: real defect, contained blast radius.

### Why blocking (NEW-VALID, not an accepted residual)

Per the round's own standard, a self-contradiction in the shipped workflow guidance is
a real defect that should be fixed rather than accepted as a non-blocking residual,
unless the wording already reads acceptably. It does not read acceptably here: line 69
states an unconditional input ("the reviewers' and the triager's findings files") that
is false for exactly the zero-findings case the sibling paragraph (line 47) just
carved out, so the two paragraphs disagree about whether a triager findings file exists
on a clean review. A reader tracing the clean-review path hits a direct contradiction.
This is fix-worthy, so the round is NEW-VALID and another round is owed. A minimal fix,
parallel to the conditional phrasing the change already uses elsewhere, would qualify
the input clause, for example "from the reviewers' findings files and, when a triager
ran, its findings file", applied byte-identically to `pack/AGENTS.md` line 69 and both
generated mirrors (`AGENTS.md`, `.agents/AGENTS.reference.md`) with a re-render so
`render --check` stays clean. (I do not apply the fix; I only state it.)

### Backstop

F1 is medium, below the high/critical backstop threshold, so the dismissed-high
re-check does not apply. F1 is valid in any case, so the backstop would not have been
triggered regardless.

## Reviewer B: zero findings

Confirmed reasonable. Reviewer B's stated integrity checks are appropriate for its lens
(capture fidelity, receipt integrity, plan/sidecar integrity, scope, and mechanical
validator/test results) and its evidence is specific and internally consistent:
Q-63 question and step captured with correct ids/status/provenance; sidecars present
(step populated, Q-63 a 0-byte placeholder matching the Q-61/Q-62 convention); the
`docs/metrics/workflow.jsonl` receipt is a pure single-line append (173 -> 174) with
`chosen` a member of `options`; changed-file set exactly the expected ten files with no
`src/` or ledger touched; and `pack/prompts/triager.md` correctly untouched. I did not
re-run the validators; reviewer B reported `validate`, `validate --workflow`,
`render --check`, and `cargo test` all clean, consistent with reviewer A's independent
run of the same commands. Nothing in reviewer B's scope overlaps F1 (a guidance-prose
consistency issue, which is reviewer A's lens), so B raising zero findings in its lens
is not in tension with F1. No reason to add a finding on B's behalf, and re-reviewing
afresh is out of my scope.

## Summary

- F1: VALID, medium (confirmed), blocking. `pack/AGENTS.md` line 69 (and both mirrors)
  names the triager's findings file as an unconditional input to every review-mode
  report, contradicting the line-47 carve-out that a zero-findings review has no triage
  step and hence no triager findings file. Fix-worthy self-contradiction; minimal fix is
  a conditional "when a triager ran" qualifier on the input clause, re-rendered.
- Reviewer B zero findings: confirmed reasonable for its integrity/mechanical lens.

Outcome: NEW-VALID (F1 is a valid finding requiring a fix and another round).
