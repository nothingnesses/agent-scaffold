# Triage verdicts: `findings-files` (Q-14), round 1

Change under review: `4ebe8a3..a8a75f5` (`git diff 4ebe8a3..a8a75f5 -- pack/`). Step detail: `docs/plans/agent-scaffold.md`, "### `findings-files`". Reviewers' findings read directly:

- `docs/plans/agent-scaffold.reviews/findings-files-reviewer-opus.md` (correctness lens: F1, F2, F3).
- `docs/plans/agent-scaffold.reviews/findings-files-reviewer-sonnet.md` (consistency lens: one medium, one low).

Principle citations are the plan's own Project Principles 1-7 (plan lines 18-24). The reviewers' "Principle 16 (one source of truth)" and "AGENTS.md Principle 16" citations are normalised to plan Principle 1, into which the plan folds one-source-of-truth (see plan lines 313, 409). Plan Principle 5 is "make illegal states unrepresentable". Line length / wrapping is not treated as a finding (none was raised).

Overlap handled: opus F2 and sonnet MEDIUM are the same theme (reviewer filename convention does not guarantee uniqueness, and two documents give divergent formulations); they are adjudicated once as Verdict 1.

---

## Verdict 1 (opus F2 + sonnet MEDIUM, deduplicated) - VALID, severity medium

Finding: the reviewer filename convention does not guarantee the uniqueness the parallel-safety claim depends on, and it is stated two different ways. AGENTS.md's "Findings files" subsection gives `<step>-<role>.md`; `reviewer.md` gives `<step>-<your-role-or-model>.md`; the plan's own design (line 366) uses `<step>-<agent>.md`. Three formulations for one slot, none guaranteeing a distinct path per concurrent writer.

Reasoning (valid): the feature's stated reason to exist is parallel-safety, "parallel writers never contend for one file" (AGENTS.md subsection; plan line 366, "so parallel writers never contend for one file (parallel-safe)"). That guarantee holds only if each concurrent writer resolves to a distinct filename. Neither shipped convention delivers it:

- `<step>-<role>.md` collides for two reviewers sharing a role (the common case: the plan prefers several reviewers, and two given the same lens both resolve to `<step>-reviewer.md`).
- `<step>-<your-role-or-model>.md` is internally ambiguous ("role-or-model" reads as pick-one, not combine), and even combined it collides when two reviewers share both role and model. A convention that admits colliding paths recreates the exact contention the feature is built to prevent. This is a live inconsistency across the canonical `AGENTS.md` and the prompt an agent actually follows (`reviewer.md`), plus the plan text, so it also fails plan Principle 1 (internal coherence; one authoritative source for the filename format, not three divergent ones). And it fails plan Principle 5: the scheme leaves the illegal state (two writers, one path) representable instead of encoding it out.

Severity medium (opus rated low, sonnet medium; I confirm medium, correcting opus). It is above a nit because it holes the feature's core guarantee and is a genuine three-way documentation divergence in the canonical guidance, not a cosmetic slip. It is below high because a real-world collision is uncommon (distinct lenses or models usually differ) and the failure mode is one review file overwriting another within a round, recoverable and not data-, security-, or money-sensitive.

Recommended fix: pick ONE authoritative filename convention that guarantees uniqueness, state it once in the AGENTS.md "Findings files" subsection, and have `reviewer.md`, `triager.md`, `orchestrator.md`, and the plan's line-366 example point to it rather than restate a divergent form (plan Principle 1). Use `<step>-<role>-<disambiguator>.md`, where the disambiguator is an index (or model name) the orchestrator assigns to each reviewer when it spawns the round. Prefer an orchestrator-assigned index over a self-chosen role/model token: a self-chosen token can still collide (two same-role, same-model reviewers), whereas a per-spawn index cannot, which makes the colliding-path state unrepresentable rather than merely unlikely (plan Principle 5). Align the plan's `<step>-<agent>.md` wording to the chosen form so the three sources agree.

## Verdict 2 (opus F1) - VALID, severity low

Finding: the convergence backstop's second, independent re-check triager has no distinct filename and would collide with `<step>-triage.md`.

Reasoning (valid): the backstop (AGENTS.md line 51) requires a second independent triager to re-check a dismissed high/critical finding before it counts toward a clean round. Following `triager.md`, that second triager writes to `docs/plans/<task>.reviews/<step>-triage.md`, the same path as the first triager, overwriting the original verdicts. The re-check verdict has no distinct home. Same root cause as Verdict 1 (the triage side of the naming scheme lacks a disambiguator for a second same-role writer), so it fails the same plan Principles 1 and 5.

Severity low (confirming opus). The backstop fires only for a dismissed high-or-above finding, which is rare, and it is a completeness gap rather than a defect in the common path.

Recommended fix: give the re-check triager a distinct name under the same authoritative convention, for example `<step>-triage-recheck.md` (or `<step>-triage-2.md`), stated once in the AGENTS.md subsection and referenced from `triager.md`. Fold this into Verdict 1's single-convention fix so triage filenames follow the same uniqueness rule as reviewer filenames (the triager is `<step>-triage.md`, its re-check `<step>-triage-recheck.md`).

## Verdict 3 (opus F3) - VALID, severity low

Finding: the triager prompt is not told to create the reviews directory, whereas the reviewer prompt is.

Reasoning (valid, weakest of the four): `reviewer.md` says "(create the directory if it does not exist)"; `triager.md`'s new write instruction does not. In the normal flow reviewers always write first (the triager reads their files), so the directory exists by the time the triager writes, which mitigates this to low. It is still a real self-containment gap: the triager prompt silently depends on an ordering guarantee it does not state, and that guarantee does not cover every path (a re-check triager, or a triage-only re-run). Cheap to close and makes the two write instructions symmetric.

Severity low (confirming opus). Ordinary control flow guarantees the directory in the common case; the residue is robustness, not a live break.

Recommended fix: add the same "(create the directory if it does not exist)" parenthetical to the triager's write instruction in `triager.md`, matching `reviewer.md`.

## Verdict 4 (sonnet LOW) - VALID, severity low

Finding: the Triager role bullet in AGENTS.md was not given the "(see Findings files below)" pointer that the Reviewers bullet got; asymmetry.

Reasoning (valid): in this diff the reviewer role bullet gained "(see Findings files below)"; the triager bullet ("returning a verdict for each to its own file") did not, though the triager writes a findings file governed by the same subsection. A reader scanning role bullets for output conventions is pointed to the convention for the reviewer but not the triager. A symmetric convention stated asymmetrically fails plan Principle 1 (internal coherence).

Severity low (confirming sonnet). Navigational/cosmetic; no behavioural effect.

Recommended fix: add "(see Findings files below)" to the triager role bullet, matching the reviewer bullet.

---

## Round outcome

NOT clean. Four valid findings this round: one medium (Verdict 1) and three low (Verdicts 2, 3, 4). No high/critical dismissals, so the convergence backstop re-check does not apply to this round.

The artifact is classified LOW-risk (one consecutive clean round required to converge, per the ledger). Because this round produced valid findings, the streak does not start here. Next: the planner/implementer addresses the valid verdicts (the primary fix is the single authoritative uniqueness-guaranteeing filename convention of Verdict 1, which absorbs Verdict 2's triage-filename case; plus the two one-line edits of Verdicts 3 and 4), then a fresh round is spawned on the revised change. Convergence needs one clean round after the fixes land.
