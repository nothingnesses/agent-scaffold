# Reviewer findings: `human-onboarding` (Q-9)

Reviewer: independent (Opus). Diff range: `cdcd6c8..27e6171` (single commit
`27e6171`). Files touched: `pack/AGENTS.md` (source), `AGENTS.md` and
`.agents/AGENTS.reference.md` (generated mirrors), `README.md` (mirror pointer).

## Verification summary (what checks out)

- Spec conformance. The new "Getting started, for the human" section in
  `pack/AGENTS.md` (lines 7-22) does all four things Q-9 requires: it points the
  human at `.agents/user-prompts/kickoff.md` to copy/fill/paste and does NOT embed
  the prompt (correctly reflects the `user-prompts-dir` revision, plan lines
  308/375); it distinguishes user-prompts ("prompts you invoke by hand") from role
  prompts ("the role prompts the orchestrator hands to the agents it spawns, which
  you do not paste yourself"); and it states the ongoing decision duty tied to the
  plan's Open Questions / human-decision queue raised at each checkpoint.
- Principle 1 (one source of truth). The README change (lines 60-63) is a pointer,
  not a restatement: it says to "see the 'Getting started, for the human' section
  of the scaffolded `AGENTS.md`" and gives only a one-clause gloss of what lives
  there. No workflow content is duplicated. The kickoff prompt text is not
  duplicated anywhere; the section references the file only.
- Coherence with the rest of `pack/AGENTS.md`. The new section names the plan's
  "Open Questions" section, matching the existing ledger paragraph's phrasing
  ("Never put individual findings in the plan's Open Questions section", line 202)
  and the plan template's section (`pack/plan-template.md` line 35, "Open
  Questions, Decisions, Issues and Blockers"). The push-to-human framing is
  consistent with the Human-requests/intake section (lines 100-120), which it
  complements rather than duplicates (that section is the human pushing requests
  in; this one is the orchestrator pushing decisions out).
- Generated mirrors in sync. `AGENTS.md` and `.agents/AGENTS.reference.md` are
  byte-identical to `pack/AGENTS.md` except for the rendered `{{principles}}` block
  (expected). The "Getting started" section is identical across all three.
- ASCII-clean. `grep -P '[^\x00-\x7F]'` over all four changed files returns
  nothing. No em-dashes, emoji, or unicode symbols.
- No scope creep. The diff touches only the four expected files; no unrelated
  changes.

## R1 (low) - Newcomer-facing section uses undefined workflow jargon

Location: `pack/AGENTS.md` lines 16-22 (and the two generated mirrors).

Evidence: The section is addressed to a human "New to this workflow?" (line 9) yet
uses the terms "orchestrator" (lines 16, 20) and "checkpoint" (line 20) without any
gloss. In top-to-bottom reading order these terms first appear here, before the
Workflow section that defines "orchestrator" (line 40) and before any definition of
"checkpoint" (which is used as established vocabulary throughout but never defined
for a first-time reader). The Q-9 validation criterion is that "a human with no
prior context could start a task and knows their standing decision duty" (plan line
310); a reader who has never seen the workflow meets undefined jargon in the exact
section meant to onboard them. Impact is small: the human's actual duty (watch the
Open Questions section, decide when a decision is raised) is stated in plain terms
regardless, so the undefined terms do not block the task. Hence low, not medium. A
one-line gloss (for example, "the orchestrator, the agent that drives the
workflow") would close the gap.

## Severity roll-up

- critical: none.
- high: none.
- medium: none.
- low: R1.
