# Reviewer 1 (schema + validation) findings: task-entry-regrounding-inc2 (Part B)

Diff reviewed: `main` (e48cb75) .. `impl/ter-inc2` (fd42f21). Focus: the `Provenance`
sub-struct on `Step` and its `validate_source` rules in `src/plan/source.rs`.

Verdict: mostly sound. Validation rules (decisions resolve, commits/findings shape-only,
D5 empty-block rejection), purity, back-compat, and struct/style conventions all check out.
Two findings below: one medium (the ordering guard is not what it claims), one low.

Build/test state: `just test` 334 passed 0 failed; `just clippy` clean; no non-ASCII in the
diff; no `std::fs` / `Command` / `canonicalize` added (purity holds).

## I2-1 (medium) The round-trip test does not pin the field-ordering constraint it claims, and the ordering rationale in the comments is empirically false

Files:
- `src/plan/source.rs:126-133` (Step doc comment ordering note)
- `src/plan/source.rs:151-158` (provenance field comment)
- `src/plan/source.rs:1271-1300` (test `a_populated_provenance_parses_and_round_trips_alongside_increments_and_waivers`)

What is wrong: the comments assert that `provenance` must be declared before the
`increment`/`waiver` arrays-of-tables because "a sub-table emitted AFTER `[[step.increment]]`
would bind to the wrong table," and the round-trip test is presented as pinning that
constraint. I tested this empirically against the exact toml crate in the lockfile
(toml 0.8.23) by building a struct whose `provenance` field is declared AFTER the
`increment` and `waiver` arrays-of-tables, serializing the same populated step
(decisions + findings + commits, plus an increment and a waiver), and re-parsing. Result:
toml emits

```
[[step]]
slug = "a"
[[step.increment]]
...
[[step.waiver]]
...
[step.provenance]
decisions = [...]
```

and this re-parses to an equal value (`equal=true`). `[step.provenance]` is a fully
qualified header path that always binds to the last `[[step]]`, not to the preceding
increment, so it round-trips correctly regardless of declaration order. The ordering
matters only for a scalar/inline field placed after a table field (that genuinely breaks
toml serialization); `provenance` is a sub-table whose inner lists are inline arrays, so it
can sit before or after the arrays-of-tables without any round-trip difference.

Consequence: the test would still pass if `provenance` were moved after `increments`/`waivers`,
so it does not guard the ordering it says it guards (Principle 11, test honesty: the
docstring claims to "pin the ordering constraint" but the assertion holds under both orders).
The Step/field comments state a toml behavior that does not occur.

Why it matters: no current correctness or SSOT-corruption risk. The field is placed in a
valid position and toml handles either order. The defect is that a future maintainer reading
the emphatic "would bind to the wrong table" comment will trust a false invariant, and will
trust a test that does not actually catch a reordering. If the real (scalar-after-table)
ordering hazard is what the increment wanted to guard, the current test does not exercise it.
Recommend either correcting the comments to state the real constraint (inline/scalar fields
must precede table and array-of-table fields; a sub-table's position among tables is
round-trip-neutral) and dropping the ordering claim from the test docstring, or, if an
ordering guard is wanted, add a test that fails on a scalar declared after a table.

## I2-2 (low) An empty-string findings ref is accepted, so `findings = [""]` slips past the D5 non-empty intent

File: `src/plan/source.rs:656-663` (findings loop) with `is_safe_sidecar_ref` at
`src/plan/source.rs:489-495`.

What is wrong: `is_safe_sidecar_ref("")` returns `true` (empty path is not absolute and has
no components, so `components().all(...)` is vacuously true; verified empirically). A
`[step.provenance]` with `findings = [""]` therefore validates clean: the block is non-empty
(so the D5 all-empty-block check at :634-642 does not fire), and the empty ref passes the
shape check. An empty findings pointer carries no meaning, which is the kind of illegal state
D5/Principle 13 aims to exclude. For contrast, an empty `decisions` entry is caught
(`question_id_index("")` is `None` -> "not a `Q-<n>` id") and an empty `commits` entry is
caught (`is_commit_shaped("")` is false, len 0 not in 7..=40), so `findings` is the only list
that admits an empty string.

Why it matters: low. It matches the existing sidecar precedent exactly (the same function has
the same gap for `[meta].sidecars`), so the increment is consistent with its stated precedent
rather than introducing a new inconsistency, and an empty findings ref is inert (render would
try to splice an empty path, not escape the directory). Flagging for completeness; a one-line
`!finding.is_empty()` guard, or reusing whatever the sidecar path decides, would close it.

## Items probed and found clean

- Field shape/order: `provenance: Option<Provenance>` is declared after `blocked_by`/`folds`
  and before `increments`/`waivers` (`src/plan/source.rs:151-158`), a valid position.
  `#[serde(default, skip_serializing_if = "Option::is_none")]` gives the None-serializes-to-nothing
  back-compat; `a_step_without_provenance_round_trips_with_no_provenance_key` genuinely pins it
  (asserts the serialized string contains no `provenance` key). Back-compat is real.
- `deny_unknown_fields` on `Provenance` (`src/plan/source.rs:242`): the mistyped-key test
  (`decisons`) genuinely fails to parse; would pass if the attribute were removed.
- Decisions rule: shape-then-resolve mirrors `folded_into`/`superseded_by`
  (`question_id_index` None -> malformed; else `!question_ids.contains` -> names no question).
  Both negative tests (`Q-99` dangling, `nope` malformed) fail on distinct messages and are
  real. Resolution is exact-string against the question set, consistent with existing precedent.
- Commits rule (`is_commit_shaped`, `src/plan/source.rs:504-506`): boundaries verified
  empirically: 6 rejected, 7 accepted, 40 accepted, 41 rejected, uppercase rejected, empty
  rejected. Lowercase-only via the explicit `b'a'..=b'f'` range (not `is_ascii_hexdigit`,
  which would wrongly admit A-F). Not git-resolved.
- Findings rule reuses the existing `is_safe_sidecar_ref`: mid `..` rejected, leading `..`
  rejected, absolute rejected, `./` allowed; not existence-checked. Negative tests real.
- D5 empty-block: `an_empty_provenance_block_is_flagged` is genuine.
- Purity: no `std::fs`, `Command`, or `canonicalize` added; `validate_source` remains a pure
  function over the string. Confirmed by grep over `src/plan/source.rs`.
- Style/conventions: `Provenance` follows the `Increment`/`Waiver`/`Sidecars` nested-struct
  pattern, uses `#[serde(default, skip_serializing_if = "Vec::is_empty")]` on each list,
  error-message style matches the surrounding checks. No em-dashes, unicode, or emoji in the
  diff. No `#[allow]` added.
