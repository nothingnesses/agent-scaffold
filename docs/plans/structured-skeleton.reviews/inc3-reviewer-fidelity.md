# Inc 3 review - DESIGN FIDELITY, PRINCIPLES, DOCS

Reviewer lens: design fidelity of the render engine against the synthesis and project principles. Change under review: commit `17a328e` on base `9afe567`. Tests run green (271 passed via `cargo test --all-targets` in the worktree). Sources read: `src/plan/render.rs` (783 lines), `src/main.rs` (`run_render` and `RenderArgs`), `src/plan/testdata/*`, `docs/plans/structured-skeleton.explorations/synthesis.md`, `docs/plans/agent-scaffold.md` (the live plan for Inc 5 fidelity comparison), `src/plan.rs`, `src/plan/source.rs`.

Summary: two medium, three low. No critical or high defect found. The OPAQUE SPLICING, STRICT FAILURE, BANNER, and B3 CLOSURE properties are all implemented correctly and verified (see "What I verified"). The mediums are design-fidelity gaps against explicit synthesis decisions that will enlarge the Inc 5 fidelity diff beyond the synthesis's stated expected diffs. The lows are a vestigial fixture value, a test claim that exceeds its assertions, and a missing note about `--strict` in the banner.

---

## F1 (medium) - question bodies are inlined in the queue section, not relocated to a separate `## Question Details` section

`src/plan/render.rs:390-406`, `questions_section()`

The synthesis section 3(f) has an explicit "ADOPT" decision: "Relocate question bodies to a `## Question Details` section mirroring `## Step Details` (A). One-line reasoning: makes the queue a scannable one-line-per-item list and is the natural consequence of the one-line `ask`." The render engine is the Inc 3 deliverable and is where this output-format decision should land.

The current implementation inlines each question's body sidecar immediately after its one-line list item, separated by a blank line:

```
- `Q-1` (open) An open ask still awaiting a decision.

The Q-1 body. An opaque question sidecar spliced verbatim after the one-line queue
item; render never parses it.
- `Q-2` (decided -> folded into `alpha`) A decided ask folded into alpha.
```

The blank line after `Q-1`'s item breaks the Markdown list: most parsers treat a subsequent paragraph as ending the list, so `Q-2` starts a new list. The queue is not scannable when bodies exist, directly opposite the stated reason for the ADOPT decision.

This also affects the Inc 5 fidelity diff. The synthesis's stated expected diffs for Inc 5 are "banner, derived Status line, question-body relocation." "Question-body relocation" implies bodies move TO a separate section, not that they stay inline after each item. The current implementation writes bodies inline regardless, which is a different structural outcome than the synthesis designed.

Suggested direction: add a `question_details_section()` function mirroring `step_details_section()`, splicing each question's body sidecar there (under a `## Question Details` heading) rather than inline in the queue list. The queue then becomes a clean one-line-per-item summary (or empty body entries produce no section at all, keeping the output minimal when questions have no body prose).

---

## F2 (medium) - fixed section order cannot accommodate Repository Layout in its live-plan position

`src/plan/render.rs:275-303`, `assemble()`

The `assemble` function has a fixed section order:

```
banner -> title -> status -> [front sidecars...] -> Principles -> Vocabulary -> Questions -> Roadmap -> Step Details -> [tail]
```

Front sidecars appear BEFORE the generated Principles and Vocabulary sections. There is no mechanism to place a sidecar between two generated sections.

The live plan's section order is:

```
# agent-scaffold plan
Status: ...
## Motivations
## Project Principles
## Documentation Protocol
## Repository Layout and Current Architecture
## Open Questions...
## Roadmap
## Step Details
## Success Criteria
```

`## Repository Layout and Current Architecture` sits AFTER `## Documentation Protocol` (the generated Vocabulary section), not before `## Project Principles`. During the Inc 5 migration, Repository Layout would have to become either a front sidecar (placing it before Principles in the output, wrong order) or a second tail sidecar (but there is only one tail, and Success Criteria already occupies it).

The synthesis's stated expected diffs for Inc 5 are "banner, derived Status line, question-body relocation." Section reordering of Repository Layout is not listed as an expected diff, yet the fixed ordering forces it. This is a gap: either the expected-diffs list is incomplete, or the render order needs a way to interleave sidecars with generated sections.

Suggested direction: document the reordering explicitly as an additional Inc 5 expected diff, OR introduce a mechanism for sidecars that appear between generated sections (for example, a `[meta].sidecars.mid` list placed between Vocabulary and Questions). The fixture is a good place to exercise this case by reordering its own sidecars to match the live-plan section order.

---

## F3 (medium) - `## Documentation Protocol` heading repurposed for the vocabulary fragment, colliding with the live plan's section of the same name

`src/plan/render.rs:376-387`, `vocabulary_section()`

The `vocabulary_section()` function generates a section headed `## Documentation Protocol` that contains only the 2-line status vocabulary fragment (Roadmap statuses, queue statuses).

The live plan's `## Documentation Protocol` section (lines 29-71 of `docs/plans/agent-scaffold.md`) contains approximately 6 paragraphs of formatting rules: how headings, Roadmap tables, and queue items work; the slug naming convention; and the push-at-checkpoint rule. These are hand-authored formatting conventions specific to the Markdown-sourced plan.

After the Inc 5 migration, these formatting rules have no place in the rendered output. If they move to a front sidecar (the only available prose mechanism), they appear BEFORE Principles, not in their live-plan position between Principles and Repository Layout. If they are dropped, a section the live plan's agents reference by name disappears. Either way the change is structural and not listed in the Inc 5 expected diffs.

The heading collision is also confusing to a reader: the generated `## Documentation Protocol` section looks like a subset of the live plan's `## Documentation Protocol` (it covers the vocabulary part but not the formatting-rules part), not a replacement.

Suggested direction: rename the generated section (for example `## Status Vocabulary` or `## Generated Status Vocabulary`) so it does not claim the same heading as the live plan's formatting-rules section. Then the formatting rules can be preserved in a front sidecar under a heading like `## Documentation Protocol` without colliding with the generated fragment. Add this to the Inc 5 expected-diffs list regardless, since the live plan's full section content will not be reproducible from the TOML alone.

---

## F4 (low) - `render_sha256` fixture placeholder is an all-zeros value that resembles a real hash

`src/plan/testdata/render-fixture.plan.toml`, `[meta]` block

The fixture TOML carries:

```toml
render_sha256 = "0000000000000000000000000000000000000000000000000000000000000000"
```

The module doc correctly notes the field is vestigial for this increment (design A chose a byte-compare against the committed `.md`; `[meta].render_sha256` was deferred). The deferral is documented and acceptable for Inc 3. The concern is narrower: the all-zeros value looks like a real but zeroed hash rather than a placeholder, and the `schema accepts it silently. A future reader of the fixture who has not read the module doc may assume it is computed and meaningful.

The field is `Option<String>` in the schema, so it can be omitted without changing parse behavior. The fixture should either omit it (since it is not exercised by any render test) or replace the value with a string that clearly signals placeholder status (for example `"vestigial-see-module-doc"`), so the fixture does not imply the field is active.

Suggested direction: drop `render_sha256` from the fixture TOML. The field remains in the schema for the later increment that reconciles it; the fixture need not carry an unused field.

---

## F5 (low) - `render_writes_exactly_one_file` test comment claims no sidecar or TOML changed but does not assert it

`src/plan/render.rs:590-609`, `render_writes_exactly_one_file_and_check_is_green_after`

The test writes the rendered bytes to the output path and then asserts:

```rust
// Exactly one file was produced, `<task>.md`, and no sidecar or the TOML changed.
assert_eq!(out_path, dir.join("render-fixture.md"));
assert!(out_path.exists());
```

The comment claims "no sidecar or the TOML changed," but the assertions only check that the output path is `<task>.md` and that it exists. They do not check that no other file in `dir` was created or modified. The OPAQUE SPLICING / NO ROUND-TRIP guarantee (synthesis section 1, "The render engine") requires that render never writes a sidecar or the TOML; this is a load-bearing invariant for the increment and the comment names it correctly, but the test does not enforce it.

In practice the code is correct (the CLI writes only `rendered_path`, the engine returns a `String`), so this is a documentation/coverage gap rather than a latent bug. But given the review brief calls the no-clobber property a principal guarantee, it should be pinned by an assertion rather than only a comment.

Suggested direction: after the write, verify that the source files are unchanged (for example, check file modification timestamps or re-read and compare their contents against known values) and that no new file was created beyond `<task>.md`. A simple approach: record the mtimes of the TOML and one sidecar before the render, then assert they have not changed after it.

---

## What I verified (no defect found)

**OPAQUE SPLICING / NO ROUND-TRIP.** `render_plan` returns `Result<String, Vec<String>>` and writes nothing; all filesystem writes in the module are inside test helpers or in `check_render` (read-only: `fs::read_to_string` only). The CLI's `run_render` writes exactly to `rendered_path`, which is `<base>/<task>.md` derived from the `.plan.toml` file name. `rendered_path` cannot alias a sidecar under the naming convention (`<task>.steps/<slug>.md`, `<task>.questions/<id>.md`, `<task>.<something>.md` prose sidecars all carry a segment not present in `<task>.md`). Sidecar content is passed as an opaque `String` to `assemble` and is never parsed back into structure.

**STRICT FAILURE.** `validate_source` runs BEFORE `parse_toml` and before any sidecar read (`render.rs:144-147`). The sidecar loading collects ALL missing sidecars then returns `Err` before calling `assemble` (`render.rs:188-190`). `assemble` is only reached when the source is clean and all sidecars are present; it does not write. Integration tests confirm: a missing sidecar, an unresolved cross-reference, and a schema-invalid TOML all return `Err` with no `<task>.md` written.

**BANNER.** The generated banner heads the file with a do-not-hand-edit directive and names the task-relative sources: `{task}.plan.toml`, `{task}.steps/`, `{task}.questions/`, and the `[meta].sidecars` prose. No absolute paths. The test `the_rendered_document_carries_every_generated_fragment` pins "do not hand-edit" and the TOML source name.

**CHECK SEVERITY (WARN-LOCAL, FAIL-CI).** `run_render` with `--check` and a mismatch: calls `eprintln!("warning: ...")` and returns `Ok(())` (exit 0). With `--check --strict` and a mismatch: calls `eprintln!("error: ...")` and `std::process::exit(1)` (exit 1). A render failure under `--check` always exits 1 regardless of `--strict`. This matches synthesis decision 3(d).

**B3 CLOSURE.** `vocabulary_section()` builds the Roadmap vocabulary from `ROADMAP_STATUSES` and `ROADMAP_BLOCKED_PREFIX` (code constants, `plan.rs:90-100`), not a hand-written list. `QuestionStatus::ALL` feeds the queue vocabulary. The drift guard test `every_step_status_label_is_an_accepted_roadmap_status` (render.rs tests) asserts that every `StepStatus::ALL` entry's `label()` is a member of `ROADMAP_STATUSES`, closing B3 in the relevant direction: what render writes is always a member of the vocabulary the generated section lists.

**`render_sha256` DECISION.** Design A (byte-compare against the committed `.md`) was implemented correctly. The `[meta].render_sha256` field is vestigial, documented as such in the module doc, and does not affect the byte-compare logic. Acceptable for this increment; see F4 for the fixture placeholder concern.

**PRINCIPLES SECTION.** `principles_section()` sorts by each principle's own `n` field (not a running counter) and formats as a numbered list. In the fixture the principles use non-consecutive numbers (1 and 8), and the golden confirms `1. ... - ...` and `8. ... - ...` are both rendered correctly.

**DOCUMENTATION.** `render.rs` is thoroughly documented. The module doc explains the no-round-trip guarantee, the strict-failure contract, the byte-compare mechanism, the vestigial field, and references to the synthesis and the project principles. Each exported function and each private helper carries a doc comment explaining its contract. The design choices (front-matter-as-opaque-block, splice-boundary trim, table-cell escaping) are all called out. A fresh reader can follow the projection from the module doc alone.
