# Compaction-prep prompt

Paste this to the agent before you compact the context or otherwise end the
session, so the task's state survives the loss. It triggers the checkpoint
procedure defined in `AGENTS.md`; it does not restate that procedure.

---

I am about to compact the context. Before I do, run the pre-compaction checkpoint
from `AGENTS.md`: flush the plan, the ledger, and the plan's Open Questions queue
to current, verify the plan's Status line (the resume anchor) is accurate, and
commit everything, so the task can resume after the context is lost. Tell me when
the tree is clean and it is safe to compact.
