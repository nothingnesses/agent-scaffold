# Pause prompt

Paste this to the agent when you want to stop work on an in-progress task for now and pick it up later, whatever the reason (you are stepping away, ending the session, or about to compact). It triggers the checkpoint procedure defined in `AGENTS.md` so the task's state survives the break; it does not restate that procedure. Resume later with the resume prompt.

---

I am pausing this task. Run the checkpoint from the "Checkpoint and resuming after context loss" section of `AGENTS.md` so the work can be resumed later: bring the plan and the ledger up to date, commit any pending work so the tree is clean, and record where the task stands and what comes next. Tell me when the tree is clean and it is safe to stop.
