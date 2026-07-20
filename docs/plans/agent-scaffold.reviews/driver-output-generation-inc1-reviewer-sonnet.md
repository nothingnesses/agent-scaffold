# Review: driver-output-generation Inc 1 (Reviewer 2 - Sonnet, orchestrator preamble / scaffold consistency / scope / style)

Branch: `impl/dog-inc1`. Diff base: `main` (`041c581`). HEAD: `9a8ea5a`. Worktree: `.claude/worktrees/dog-inc1`.

Two commits under review:
- `9a10702` feat: generate the writer-isolation policy fragment into AGENTS.md
- `9a8ea5a` docs: add a required writer-spawn preamble to the orchestrator prompt

---

## D1s-1: LOW - Parenthetical in preamble edges toward policy restatement

`pack/prompts/orchestrator.md:13`

The preamble sentence reads: "for the policy itself, including which roles count as writers (a spawned planner or exploration writer is a writer, distinct from your own integration-level edits on main) and the authoring-versus-integration line, it references the Writer isolation rule in `AGENTS.md` rather than restating it."

The parenthetical "(a spawned planner or exploration writer is a writer, distinct from your own integration-level edits on main)" is paraphrased policy content, not a per-spawn fact. The isolation_policy fragment already states "A spawned planner is a writer, and so is any design or exploration writer that authors content" and "distinct from the orchestrator's own integration-level edits on main" (`src/isolation_policy.rs:36`). The parenthetical is framed as an illustration of what the policy covers rather than a full restatement, and it accurately reflects the fragment's content, so there is no current drift. However, it adds a second prose location where this classification is stated. If the fragment is edited but `pack/prompts/orchestrator.md` is not (it is verbatim-copied, not rendered, so no drift-guard test covers it), the parenthetical can silently diverge from the canonical fragment.

The parenthetical can be trimmed to "including which roles count as writers and the authoring-versus-integration line" without losing any load-bearing content, because AGENTS.md carries the full specification. This is the same restraint the Q-54 rule applied: it does not summarize what "viable options" or "recommendation" mean in the parenthetical; it names the slots and points at `AGENTS.md`.

Severity: low. The preamble's core purpose (per-spawn facts + reference) is sound; the parenthetical is illustrative rather than authoritative, and the fragment itself is the canonical source. But it is a second location for this classification and not guarded.

---

## D1s-2: LOW - Main.rs nix fmt reflows absent from stash (process note)

`src/main.rs` (committed diff), `stash@{0}` "nix fmt reflow (dog-inc1)"

The review task states the implementer "REVERTED three unrelated nix fmt reflow hunks inside `src/main.rs` (in `run_resume`/`run_next`)" and that the reflows are "stashed (not committed)." The committed diff of `src/main.rs` is clean: it contains only the `mod isolation_policy;` declaration, the `build_assets` doc-comment update, and the `builtin.insert("isolation_policy" ...)` call. No reflow hunks are present in the commit. However, stash@{0} ("nix fmt reflow (dog-inc1)") contains `src/next.rs` (58 lines, the next.rs nix fmt reflow) and `src/plan/source.rs`, but does NOT contain `src/main.rs`.

The global project instructions require `git stash push -m "..."` rather than `git checkout`/`git restore` to preserve discarded working-tree changes. The absence of main.rs from the stash means the main.rs reflows were either (a) never applied to the tree (avoided via partial staging with `git add -p`), in which case nothing was discarded, or (b) discarded via `git restore`/`git checkout` without stashing. Option (a) is fine; option (b) is a minor process discipline violation (the reflows are unrecoverable if needed later).

The committed result is correct either way. This is a process note only.

Severity: low. The committed diff is clean. The concern is whether the discarded reflows are retrievable, not whether the change is correct.

---

## On the preamble judgment call (required for this lens)

**Claim (a): Is the "not rendered" assertion true?**

YES. Confirmed from `pack/pack.toml`. The asset entry for `prompts/orchestrator.md` -> `.agents/prompts/orchestrator.md` carries `ownership = "reference"` and NO `render = true` field. Per `src/manifest.rs:316-334` (`render` function) and the module doc (`src/manifest.rs:1-9`): only assets with `render = true` pass through `{{var}}` substitution; others are copied verbatim. A `{{isolation_policy}}` slot in `pack/prompts/orchestrator.md` would be copied as a literal string into the deployed prompt, not substituted. The implementer's claim is factually correct.

Compare: `pack/AGENTS.md` (both asset rows) carries `render = true` and receives `{{isolation_policy}}` substitution from `build_assets` (`src/main.rs:267-270`). `pack/prompts/orchestrator.md` does not and cannot without a pack.toml change.

**Claim (b): Does the preamble restate policy or only state per-spawn facts + reference?**

Mostly correct, with the low-severity concern in D1s-1 above. The structural content (role, writer status, isolation tier, worktree path) is genuinely per-spawn originated content (category 1 per the design doc's taxonomy). The policy reference is a pointer, not a restatement. The parenthetical "(a spawned planner or exploration writer is a writer, distinct from your own integration-level edits on main)" is a partial policy echo that slightly oversteps the per-spawn-facts scope, but is framed illustratively and does not constitute a full second source.

**Claim (c): Is it a genuine required emitted block, not just prose?**

YES. The preamble matches the Q-54 pattern exactly: it states the action ("emit a short writer-spawn preamble at that point of action"), specifies what to emit (four named per-spawn facts), and declares the absence a defect ("treat its absence as a defect, not a matter of taste"). The Q-54 precedent (`pack/prompts/orchestrator.md:31`, commit `99c5c84`) uses identical structure: "emit a structured block at that gate... Presenting a gate without [its required content] is a defect; its absence is a visible lapse, not a matter of taste." The preamble follows the established pattern.

**Claim (d): Is referencing vs. embedding the right call?**

YES. The design spec explicitly decides this at D-c (`docs/plans/driver-output-generation.design.md:137-144`): "For the orchestrator-prompt writer-spawn preamble (bundle fix 3), reuse the isolation fragment IF the preamble is to be a restated checklist; otherwise keep the existing pointer." The preamble is NOT a restated checklist; its content is per-spawn facts. The isolation policy is referenced, not restated. The design spec's conclusion is "generate where a driver state is the point of action and the rule is stable; point where the consumer loads the content in full anyway." The orchestrator loads its own prompt in full (category 4 per the design doc), so keeping a pointer is the design-sanctioned choice. The implementation follows D-c correctly.

**Overall verdict on the preamble judgment call: SOUND.** The three premises (verbatim-copy confirmed, per-spawn-facts content confirmed, Q-54 pattern matched) and the design-doc decision D-c all support the reference-vs-embed choice. The one weak spot is the parenthetical in D1s-1, which slightly overshoots the "per-spawn facts + reference" scope, but it is low severity and does not invalidate the judgment.

---

## Checks by area

**1. Scaffold / dogfood consistency**

- `pack/prompts/orchestrator.md` and `.agents/prompts/orchestrator.md` are byte-identical (diff produces no output). Clean.
- `AGENTS.md` and `.agents/AGENTS.reference.md` both contain `ISOLATION_POLICY_FRAGMENT` verbatim (grep count: 1 in each). Clean.
- The drift-guard test `isolation_policy::tests::the_committed_scaffold_carries_the_isolation_policy_fragment` passes and covers both files. Clean.
- The `the_fragment_states_the_writer_classification` test pins the fragment's classification content independent of scaffold output. Clean.

**2. Scope discipline (Principle 8)**

- `src/next.rs`: not changed in any committed diff between `main` and HEAD. The `spawns_writer` bug fix (D-d) and the always-on isolation reminder in next (bundle fix 2) are correctly deferred to increment 2 per the design doc's staging section (`docs/plans/driver-output-generation.design.md:101-113`).
- `docs/plans/`, `docs/metrics/`, `docs/plans/agent-scaffold.ledger.md`: not changed in any committed diff. Plan.toml, ledger, metrics untouched. Clean.
- `src/main.rs` committed diff: three changes only - `mod isolation_policy;` declaration at the module list; one-line doc comment update naming `{{isolation_policy}}`; `builtin.insert("isolation_policy" ...)` call in `build_assets`. No reflow hunks, no changes to `run_resume`, `run_next`, or any other function.

**3. Regression**

- `just test`: 343 tests pass (336 unit + 7 integration). No failures.
- `just clippy`: clean, no warnings.
- `just build`: clean.

**4. Style**

- No em-dashes, en-dashes, or unicode symbols in new code or docs (grep -P "[--]" finds none in the changed files).
- No `#[allow(` in `src/isolation_policy.rs`; the one `#[expect(dead_code, ...)]` in `src/manifest.rs:80` is pre-existing, not part of this diff.
- No hard-wrapped prose in the new `pack/prompts/orchestrator.md` paragraph (this lens does not raise line-length concerns per the review brief).
- No emoji or unicode bullets in any changed file.

---

## Summary

Two findings, both low severity. D1s-1 is substantive (a parenthetical that restates partial policy content, creating a second location not covered by any drift guard); D1s-2 is a process note (main.rs reflows absent from the stash, committed diff is clean). No scope creep, no regression, no style violations. The preamble judgment call is sound: the verbatim-copy claim is confirmed, the per-spawn-facts content is correct, the Q-54 pattern is matched, and design decision D-c sanctions the reference-vs-embed choice.
