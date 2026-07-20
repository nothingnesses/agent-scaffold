# Round 3 review: task-entry-regrounding-inc2

Reviewer: independent (fresh sampling, claude-sonnet-4-6)
Artifact HEAD: unchanged since Round 2 (main 5f73cc9, branch tip 4b498a4)
Result: CLEAN - no new findings.

## What I probed

### 1. Provenance schema and back-compat

`Provenance` has `deny_unknown_fields` and three `#[serde(default, skip_serializing_if = "Vec::is_empty")]` lists. `Step.provenance` is `Option<Provenance>` with `skip_serializing_if = "Option::is_none"`. Verified:

- A step with no provenance deserialises to `None` and re-serialises with no `provenance` key (test: `a_step_without_provenance_round_trips_with_no_provenance_key`).
- A fully-populated step (provenance + increment + waiver) round-trips byte-identical through `toml::to_string` -> `parse_toml` (test: `a_populated_provenance_parses_and_round_trips_alongside_increments_and_waivers`).
- A mistyped inner key (`decisons`) fails at parse (test: `a_mistyped_provenance_inner_key_fails_to_parse`). No silent drop.

### 2. D1 validation correctness and test non-vacuity

Walked each validation branch manually and confirmed behaviour on the edge inputs the prompt listed:

- Duplicate decisions (e.g., `["Q-1", "Q-1"]`): both resolve without error. No duplicate check. Consistent with `blocked_by`, which also has no dedup check. Not a defect.
- Commit exactly 7 chars (`abc1234`): accepted (`(7..=40).contains(&7)` = true). Exercised by the positive test.
- Commit exactly 40 chars (`0123456789abcdef0123456789abcdef01234567`): accepted. Exercised by the positive test.
- Mixed-case commit (e.g., `Abc1234`): `A` is not in `b'a'..=b'f'`, rejected correctly. No explicit test, but the code path is covered by the non-hex test (`xyz1234`), which also targets the same byte-range check.
- `0x`-prefixed commit (e.g., `0x12345a`): `x` not in `a-f`, rejected correctly.
- Findings path with trailing slash (`notes/a.md/`): Rust's `Path` drops the trailing slash before component iteration, so it passes `is_safe_sidecar_ref`. Not a path-traversal risk.
- `.` component (`./more.md`): `CurDir` is explicitly allowed; tested in the positive case.
- Provenance with two lists populated and one empty: only the all-empty case is rejected (D5); partial population is valid, and render emits only non-empty sub-lists. Confirmed correct.

Tests are non-vacuous. The negative tests (`a_bad_provenance_commit_shape_is_flagged`, `an_unsafe_provenance_findings_path_is_flagged`, `a_dangling_provenance_decision_is_flagged`, `a_malformed_provenance_decision_id_is_flagged`, `an_empty_provenance_block_is_flagged`) each check a specific error message substring; removing the corresponding validation branch would cause the test to fail. The render test (`provenance_note_renders_only_the_present_sub_lists_in_order`) would fail if the empty-list guards were removed (output would include stray `findings ;` and `commits ;` labels).

### 3. validate_source purity

`validate_source(contents: &str) -> Vec<String>` operates purely over the parsed struct. No `std::fs`, no `std::process`, no shell calls anywhere in the function body or its callees (`is_commit_shaped`, `is_safe_sidecar_ref`, `is_kebab_case_token`, `question_id_index`). Confirmed pure.

### 4. Render

- `provenance_note` emits only present sub-lists in the fixed decisions -> findings -> commits order. The `notes_cell` appends it last (after blocked-on markers and waiver descriptors), so blocking/waiver state stays leftmost.
- The result of `notes_cell` is passed through `escape_cell` at the `roadmap_section` call site (line 467), so any `|` in findings paths is escaped to `\|` before it reaches the Markdown table.
- `render --check` on `docs/plans/agent-scaffold.plan.toml` exits 0 ("up to date").
- Exemplars: `task-entry-regrounding` renders `why: decisions Q-53` and `human-input-gate-reinforce` renders `why: decisions Q-54`. Both Q-53 and Q-54 are real `[[question]]` entries in the live plan; both are `decided` with valid `folded_into` targets.

### 5. Scope

`src/workflow.rs`, `src/next.rs`, `src/metrics.rs`: no diff against main. No W-check added. No `[[step.increment]]` for inc2 declared in the live plan. No step status change: `task-entry-regrounding` stays `in-progress`, `human-input-gate-reinforce` was already `complete` on main.

### 6. Round 1 fix soundness

- Corrected `Step` doc comment and the round-trip test docstring: both accurately describe that placement is a readability choice, not a constraint, and that the toml serialiser handles ordering on its own. No false claims remain.
- Removed output commit from `task-entry-regrounding` exemplar: `decisions = ["Q-53"]` only. The render of that step is `why: decisions Q-53`, which is correct and tested by the committed golden.

### 7. Style and regression

- `just test`: 341 tests pass (334 unit + 7 integration), 0 failed.
- `just clippy`: no warnings.
- No em-dashes, en-dashes, double-hyphens as dashes, or non-ASCII characters in the diff.
- No new `#[allow]` attributes in the touched files (`src/plan/source.rs`, `src/plan/render.rs`).
