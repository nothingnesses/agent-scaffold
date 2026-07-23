### `principle-by-name-projection`: project per-phase principles BY NAME, dropping fragile numeric citations (`Q-64`)

Generalize the driver's escalate-only `projected_principle_reminder` to a phase->principle-names map, so each phase projects its applicable principles BY NAME rather than by numeric citation. Numeric citations are the fragile form that produced the driver principle-numbering bug (cured earlier by DELETING the fragile citation, not by a generator); projecting the principle NAMES from the single `[[principle]]` source removes that whole drift class at its root (Q-64-delivery-fsm D3; the checkable-versus-promptable partition).

This is a proven-duplication single-source: the principle names live once in the plan's `[[principle]]` data, and both the AGENTS.md projection and the runtime reminder read that one source, so the two copies cannot drift. It serves Make illegal states unrepresentable (a stale number cannot mis-cite a renumbered principle because there are no numbers to go stale) and Structured data first, project for humans.

Endorsed-now core item under `generated-projection` (Q-64, Option A refined). Build now.
