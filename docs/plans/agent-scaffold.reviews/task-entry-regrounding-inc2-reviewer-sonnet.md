# Review: task-entry-regrounding-inc2 (Reviewer 2, Sonnet)

Lens: render + scope discipline + exemplars + determinism.
Diff: main `e48cb75`..`fd42f21` (commits `4feb84d` schema, `565e421` render, `fd42f21` exemplars).

## Findings

### I2s-1 (low) - Exemplar commit is a step deliverable, not a prior justification

`docs/plans/agent-scaffold.plan.toml`, lines adding `commits = ["1e1d26f"]` to `task-entry-regrounding`.

The commit `1e1d26f` ("docs: add task-entry re-grounding discipline to pack guidance (Q-53 part a)") is an implementation deliverable of the `task-entry-regrounding` step (it is inc1's output), not a commit that predates or justifies the step's existence. The `provenance.commits` field is documented as "commits that justify this step" (`src/plan/source.rs:257`) and the render prefix is "why: ...", which a reader would interpret as "reasons why this step was created." The cited commit is what the step produced, not why it was created; that role belongs to Q-53 (already in `decisions`).

The commit is real, reachable from main (confirmed via `git merge-base --is-ancestor 1e1d26f main`), and directly related to the step's work, so this does not reach the medium threshold ("non-existent/irrelevant commit"). The build plan's intent for the optional `commits` entry was to "exercise all three lists on the live plan" (build plan line 127), and the entry does that. The issue is that the exemplar slightly misrepresents the field's semantic to future authors who use it as a template.

Impact: future authors may copy the pattern of citing implementation commits rather than prior justification commits, weakening the "why this step exists" value of the field over time.

---

## Items checked clean (no findings)

**Render ordering / placement (D3):** `notes_cell` in `src/plan/render.rs:478-490` pushes blocked-on markers, then waiver descriptors, then the provenance fragment. The fixture golden confirms the order: the `alpha` row shows waiver note first, then `why: ...` last. Correct.

**Only present sub-lists shown:** `provenance_note` at `render.rs:499-511` conditionally pushes each of the three sub-lists, in decisions -> findings -> commits order, with an inner `"; "` separator. The unit test `provenance_note_renders_only_the_present_sub_lists_in_order` (render.rs:1106) covers a decisions-only case and a findings+commits-no-decisions case; the fixture exercises all three. Correct.

**Escape:** `escape_cell` is called on the full `notes_cell` return value (`render.rs:467`), so any `|` or newline in a provenance value (e.g., a findings path) is neutralised before it enters the Markdown table. The function covers `\r\n`, lone `\n`, lone `\r`, and `|` (`render.rs:538-540`). Correct.

**Determinism:** No wall-clock calls. `Vec` source order is stable (already deterministic). `render_is_deterministic_and_matches_the_golden` confirms byte-stability across two renders. `render --check` is green on the live plan (confirmed: `docs/plans/agent-scaffold.plan.toml: up to date`). Correct.

**Back-compat (render layer):** The fixture golden diff (`src/plan/testdata/render-fixture.md`) shows only the `alpha` row changed; all other rows are byte-identical to main. Steps without provenance produce no change to their Notes cell. `render --check` passes on the live plan. Correct.

**Golden test non-vacuous:** `the_rendered_document_carries_every_generated_fragment` (render.rs:689) asserts the specific fragment text: `"why: decisions Q-2; findings render-fixture.findings/alpha.md; commits abc1234"`. Non-vacuous and exercises the all-three-lists case. Correct.

**Exemplar Q-IDs are real decided questions:** Q-53 and Q-54 are present in `docs/plans/agent-scaffold.plan.toml` as `status = "decided"` questions with matching `folded_into` targets (`task-entry-regrounding` and `human-input-gate-reinforce` respectively). Both questions pass `validate_source` cleanly. Correct.

**Commit `1e1d26f` is real and reachable from main:** `git merge-base --is-ancestor 1e1d26f main` returns exit 0. The subject ("docs: add task-entry re-grounding discipline to pack guidance (Q-53 part a)") is directly related to the `task-entry-regrounding` step. Real commit, correct step. (See I2s-1 for the output-vs-justification concern.)

**No other steps populated (scope discipline on exemplars):** Exactly 2 `[step.provenance]` blocks appear in the live plan TOML (`grep -c` returns 2). No mass back-population occurred. Correct.

**Live rendered output shows the fragment:** `docs/plans/agent-scaffold.md` diff confirms `task-entry-regrounding` row shows `why: decisions Q-53; commits 1e1d26f` and `human-input-gate-reinforce` row shows `why: decisions Q-54`. Both fragments are present and correctly formed. Correct.

**Scope (no W-check, no next.rs / workflow.rs / metrics.rs changes):** `git diff main..HEAD -- src/workflow.rs src/next.rs src/metrics.rs` produces empty output. No W-check was added. Confirmed.

**Step statuses and inc2 increment NOT added (orchestrator close actions):** `git diff main..HEAD -- docs/plans/agent-scaffold.plan.toml | grep '^[+-].*status'` is empty. No `task-entry-regrounding-inc2` increment entry appears in the TOML diff. The status of both steps is unchanged. Correct.

**Style:** Clippy clean. All 341 tests pass across all test targets. No em-dashes, en-dashes, unicode symbols, or emoji in new or changed code lines (confirmed by grep). No `#[allow]` annotations added. Correct.

**Schema tests (source.rs):** All required tests from the build plan test plan are present: round-trip with provenance alongside increments and waivers (line 1272), `deny_unknown_fields` typo detection (line 1306), back-compat no-provenance-key assertion (line 1319), dangling decision (line 1339), malformed decision id (line 1351), bad commit shape (line 1363), unsafe findings path (line 1382), empty provenance block (line 1400), clean positive case (line 1412). Correct.
