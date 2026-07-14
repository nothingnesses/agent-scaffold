# Resume prompt

Paste this to a fresh agent after a compaction or a lost session to continue an
in-progress task, as opposed to the kickoff prompt, which starts a new one. It
triggers the resume procedure defined in `AGENTS.md`; it does not restate it.

---

Continue the in-progress task. Reconstruct your state per the checkpoint and
resume section of `AGENTS.md`: read `AGENTS.md`, then the plan under `docs/plans/`
(its Status line is the resume anchor), then the review ledger beside it. Act as
the orchestrator and continue from where the plan and the ledger say the work left
off; do not start over.
