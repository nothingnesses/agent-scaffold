### `audit-user-prompt`: add the `audit.md` user prompt, carrying the three disciplines that made the 2026-08-13 audit measure anything (`Q-72`)

Decided (`Q-72`, human, 2026-08-13). Blocked on `ship-v0-0-2` and sequenced before `rename-to-agent-flow`: the release comes first because it is the delivery this prompt has no claim on, and the prompt comes before the rename because the rename touches 138 files and would bury it.

The 2026-08-13 audit was invoked ad hoc. Nothing in the pack lets a user run one, so the method it used exists only in that record. This step adds `pack/user-prompts/audit.md` beside the six prompts already there, so a user can run the same kind of audit on their own project.

WHAT THE PROMPT MUST CARRY. Three disciplines, and they are the whole point of the step. A prompt that asks an agent to reflect on how the work is going produces a flattering summary, which is worse than nothing because it looks like evidence:

1. FIX THE CRITERIA BEFORE MEASURING. The criteria and the failure condition are written down and committed before any measurement runs, so they cannot be chosen to fit the result. The audit record names this as "the single control that makes this record worth anything".
2. NAME THE FAILURE CONDITION IN ADVANCE SO IT CANNOT MOVE. State what result would mean the thing being audited has failed, in falsifiable terms, before looking. The 2026-08-13 audit's condition was a conjunction of three measurable claims, each with an innocent explanation on its own, which is why the conjunction was decidable.
3. GIVE THE MEASURING TO AGENTS THAT DO NOT BENEFIT FROM THE ANSWER. The party whose work is being measured does not do the measuring. The 2026-08-13 audit used nine agents, each isolated in its own detached worktree, read-only, each told to treat the project's own ledger as hypotheses rather than facts and to prefer primary sources where the two disagree.

A fourth practice is worth carrying because the audit itself demonstrated its value: the audit reports the claims it FALSIFIED that belonged to the auditing party, on the ground that an audit which only finds other people's errors is not an audit.

SCOPE. One new pack asset and the mechanics that make it drop:

- `pack/user-prompts/audit.md`, matching the existing prompts' shape: a short explanation of when to use it, then a horizontal rule, then the copy-and-paste block with bracketed fill-ins.
- A `[[asset]]` entry in `pack/pack.toml` (source `user-prompts/audit.md`, dest `.agents/user-prompts/audit.md`, `ownership = "reference"`, matching the other six).
- The built-in asset-list test at `src/manifest.rs:611-619`.
- The README scaffolded-layout listing. Note that listing is already one prompt short of the pack: `review.md` is missing from it. Adding the missing line is inside this step because the same list is being edited; nothing else in the README is.

ONE DESIGN POINT TO SETTLE AT ENTRY, not a new question. The other five human-invoked prompts are thin triggers that restate no workflow content (Principle 1, Prefer the cleaner long-term architecture over the smallest diff), and `review.md` and `explore.md` each trigger an entry mode defined in `pack/AGENTS.md`. There are four such modes today. So the audit prompt is either a fifth entry mode with a paragraph in `pack/AGENTS.md` and a thin trigger here, or a self-contained prompt that carries its three disciplines itself. The recommendation is SELF-CONTAINED: the three disciplines are an audit METHOD rather than workflow machinery, they name no role and add no phase, and putting them in `pack/AGENTS.md` grows the shipped guidance that the audit's recommendation 8 wants cut from 7,734 words to about 2,500. Decide it in the design pass and record which was chosen; if the entry-mode form is chosen, that is a `pack/AGENTS.md` edit and the regeneration set widens to `AGENTS.md` and `.agents/AGENTS.reference.md`.

ACCEPTANCE CRITERIA:

1. `scaffold` into an empty directory drops `.agents/user-prompts/audit.md` with the pack's content, and the asset-list test names it.
2. The prompt states all three disciplines explicitly, in a form a reader can act on, not as a summary of them.
3. The prompt restates no workflow content that `pack/AGENTS.md` already owns, or, if the entry-mode form was chosen, `pack/AGENTS.md` owns the mode and the prompt only triggers it. Check by reading both, not by word count.
4. The default scaffold is otherwise byte-identical: `cargo test` passes, and a scaffold with no module selected drops the same set of files plus this one.
5. The README layout lists every prompt the pack ships, `review.md` included.
