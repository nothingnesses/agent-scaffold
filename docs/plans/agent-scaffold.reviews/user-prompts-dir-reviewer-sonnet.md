# Review: user-prompts-dir (reviewer: sonnet)

Diff range: `9f52ef0..9c1a88d`. Files changed: `.agents/user-prompts/kickoff.md`
(new), `pack/user-prompts/kickoff.md` (new), `pack/pack.toml`, `src/manifest.rs`,
`README.md`.

---

## S1 - Roadmap not updated after implementation

Severity: medium

Location: `docs/plans/agent-scaffold.md`, line 123 (Roadmap table) and line 3
(top-level status summary).

Evidence: After the implementation commit (`9c1a88d`), the Roadmap table still
shows `user-prompts-dir` as `not started`:

```
| `human-onboarding`       | next        |
| `user-prompts-dir`       | not started |
```

The plan's opening status paragraph also still says "the immediate next step is
the human-interface cluster (`human-onboarding`, `user-prompts-dir`,
`compaction-prep`)" with no mention that `user-prompts-dir` is now implemented.
The Documentation Protocol states the Roadmap is the single source of truth for
status and that the implementer keeps it current. `git diff 9f52ef0 9c1a88d --
docs/plans/agent-scaffold.md` produces no output, confirming the plan file was
not touched.

Impact: a resuming agent reading the plan would believe `user-prompts-dir` has
not started, creating confusion about where to begin work. The plan is the
durable resume anchor, so stale status here directly undermines the purpose of
the Documentation Protocol.

---

## S2 - One validation criterion is unsatisfiable by this step alone

Severity: low

Location: `docs/plans/agent-scaffold.md`, lines 375-376 (the `user-prompts-dir`
validate block) and Roadmap line 122 (`human-onboarding` status `next`).

Evidence: The step specifies four validation criteria; the third is:

> "the 'Getting started' section points to `.agents/user-prompts/` rather than
> embedding the kickoff prompt"

This section does not exist. It is `human-onboarding`'s output (that step adds
the "Getting started, for the human" section to `pack/AGENTS.md`), and
`human-onboarding` is still `next` in the Roadmap. The Roadmap lists
`human-onboarding` before `user-prompts-dir`; the diff implements
`user-prompts-dir` first. The order is logically defensible (create the
directory before adding a pointer to it), but it means this validation criterion
cannot be signed off as part of this step.

The three other validation criteria (assets drop through the loader, the
asset-list test is updated and passes, the README names the new directory) are
all met by the diff.

---

## No findings of high or critical severity.

---

## Summary of the substantive content

The kickoff prompt is thin in the correct way. It states the task slot
(`[describe what you want done]`), an optional context slot, and three
directives: act as the orchestrator, read `AGENTS.md` first, read or start a
plan. It does not restate the review loop, convergence rule, the human-input
contract, the Open Questions queue, role definitions, or any other workflow
content owned by `AGENTS.md`. The meta-description above the separator is
accurate ("it deliberately does not restate the workflow"). A new human can
follow the copy-fill-paste instructions. Not too thin to be usable.

Scope is correct: `pack/AGENTS.md` is not touched (that is `human-onboarding`'s
territory), no compaction-prep prompt is added (that is `compaction-prep`'s
territory). The `pack.toml` comment and README clearly explain the
human-invoked-vs-role-prompt distinction. The `ownership = "reference"` setting
matches the other `.agents/prompts/` assets and is appropriate (the file is a
template to copy from, not to edit in place).

The `src/manifest.rs` asset-list test is correctly extended with
`".agents/user-prompts/kickoff.md"`.

Findings: 1 medium, 1 low.
