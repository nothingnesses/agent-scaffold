# Compaction-prep prompt

Paste this to the agent before you compact the context or otherwise end the
session, so the task's state survives the loss. It triggers the checkpoint
procedure defined in `AGENTS.md`; it does not restate that procedure.

---

I am about to compact the context. Before I do, run the pre-compaction checkpoint
from the "Checkpoint and resuming after context loss" section of `AGENTS.md`, so
the task can resume after the context is lost. Tell me when the tree is clean and
it is safe to compact.
