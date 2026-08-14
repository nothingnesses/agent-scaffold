# Audit prompt

Copy this, fill in the bracketed parts, and paste it to the agent when you want a measured verdict on something you believe about your own project, rather than a summary of how the work is going. Use it when the question is empirical (is this process paying for itself, when did this change, is this claim true) and the honest answer might be one you would rather not have. Unlike the other prompts here, this one is self-contained: it carries an audit method of its own rather than triggering anything defined in `AGENTS.md`, and it adds no role and no phase to the workflow. The method is the whole point, because an agent asked to reflect on how a project is going produces a flattering summary, which is worse than nothing, since it looks like evidence.

---

I want an audit, not a summary, and I would rather have a result that goes against me than one that agrees with me.

Question: [state what you want tested, and the claim or belief you currently hold about it].

Scope: [what the audit may measure over, for example the repository history, a date range, a subsystem, a log file, or a dataset].

Work read-only against the material under audit. The only thing you write is the audit's own record.

Run it in three parts, in this order. Do not begin part 2 until part 1 is committed.

Part 1, fix the criteria before you measure, and commit them. Write a pre-registration file and commit it before any measurement runs, so that no definition can be chosen later to fit a result. It states:

- Each measure: exactly what is counted, how it is computed, and which primary source it comes from. Fix every classification rule here, before you have seen how the items distribute across it.
- The failure condition, in falsifiable terms, specific enough that it could actually occur. "The process is not working" is not one. "Measure A stays below X while measure B does not fall across rounds" is one. A conjunction of two or three measures is usually the decidable form, because each one alone has an innocent explanation.
- For each hypothesis, what result would falsify it, not only what would confirm it.
- What you commit not to do. At least: not to revise this file after seeing data, and not to treat the project's own narrative record as evidence of what happened.

Write it from structure alone (the directory layout, the field names in the log, the record and commit counts), before you read any of the project's accounts of itself, so that the existing story cannot influence the criteria. If you later need a measure the pre-registration did not name, add it, and say in the report that you added it and what you would have concluded without it.

Part 2, give the measuring to agents that do not benefit from the answer. Spawn [how many] independent agents, one question each, each in its own isolated copy and read-only against the material under audit. Do not tell any of them which answer would be convenient, and do not let the party whose work is under audit do the measuring. Brief each one:

- Treat the project's own prose, its ledger, plans, status notes, and earlier write-ups, as hypotheses to test rather than facts to repeat. Prefer the primary sources, the history, the logs, the data, and the code itself, wherever the two disagree.
- Give the exact command or query behind every number, so that I can re-run it.
- Before dating any change with a measure, check when the field or the record that measure reads was first written. A measure that is structurally zero before its schema existed shows a step change on that date whatever the behaviour did, so its early era is a schema gap and not a finding.
- Do not let the activity under audit supply its own denominator. Check that the material you measure over excludes the output that activity itself produces. If it does not, the ratio is near-tautological and will confirm whatever it was pointed at.
- Test a dated claim under several binnings, and against dropping the single largest contributor. If the date moves, report a gradual change rather than a turn.
- Name the items wherever they can be named, rather than reporting a count.

Where several agents agree, check whether they measured overlapping quantities on the same series before reading that agreement as corroboration. On a series that only ever climbs, independent agents converge on similar dates whether or not anything turned.

Part 3, report against the pre-registration, including where it went against you. Give the verdict in the pre-registered terms, whether that confirms my claim or falsifies it. Then, in its own section, list the claims this audit falsified that belong to the auditing side: the project's own recorded claims, and any of your own hypotheses or discarded first measures that the evidence did not support. An audit that only finds other people's errors is not an audit. Where you corrected a measure of your own part-way through, say what you would have concluded had you stopped at the first version. Finish with what this evidence does not settle, and the measurement that would settle it.

[Optional: which models or harnesses to use for the independent agents, how long to spend, and where to keep the audit record.]
