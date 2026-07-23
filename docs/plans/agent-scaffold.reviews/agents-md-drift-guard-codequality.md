# Drift-guard review: CODE-QUALITY / INTEGRATION / MECHANICAL lens

Scope: `git diff main HEAD` in worktree `dg-rev-codequality`. Adds `src/agents_md_drift.rs`
(283 lines, a `#[cfg(test)] mod tests`) and one `mod agents_md_drift;` line to `src/main.rs`.
Commit under review: `cba4fcc test: add whole-file AGENTS.md drift guard (normalized in-test, Q-64)`.

## Evidence (build / test / lint / validators)

- `cargo test`: `test result: ok. 344 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
  (plus the integration-test binaries, all `ok`). Both new tests run and pass:
  - `test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... ok`
  - `test agents_md_drift::tests::normalization_tolerates_wrapping_but_not_content_change ... ok`
- `cargo clippy --all-targets`: `Finished dev profile` with NO warnings emitted. The
  `clippy::byte_char_slices` lint the implementer reported fixing does not recur (the one
  byte-string literal, `*b"-*_"` at src/agents_md_drift.rs:107, is clean).
- `cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml`:
  `docs/plans/agent-scaffold.plan.toml: 84 steps, 65 questions, valid`.
- `cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml --workflow`:
  `docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold`.
- `cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml`:
  `docs/plans/agent-scaffold.plan.toml: up to date`.

All five commands pass and the plan-path outputs are unaffected by the change.

## Mechanical / house-rule checks

- Indentation: hard TABS throughout (confirmed src/agents_md_drift.rs:33-35 and body);
  `grep -nP '^    '` for space-indent returns nothing.
- Non-ASCII: `grep -nP '[^\x00-\x7F]'` over the new file returns nothing. No em-dashes,
  en-dashes, `--`-as-dash, emoji, or unicode arrows/math symbols. ASCII `->`, `>=`, `!=`
  usage only.
- Suppressions: no `#[allow(...)]` and no `#[expect(...)]` in the added code (none needed;
  clippy is clean without any).
- Integration: `mod agents_md_drift;` sits at src/main.rs:12, alphabetically first and
  consistent with the sibling `mod` block (checks, isolation_policy, manifest, metrics, ...);
  line 16 `#[macro_use]` on metrics is pre-existing and untouched.
- Consistency with sibling byte-guards: `isolation_policy.rs` and `workflow_spec.rs` carry
  production code plus an inline `#[cfg(test)] mod tests`. The new file is test-only and uses
  the same inline `#[cfg(test)] mod tests` idiom, so it does not needlessly diverge. Because
  it guards the WHOLE render (not one fragment) it correctly lives in its own module rather
  than inside a fragment source.

## Correctness / test-quality spot checks

- Empty-input safety: `normalize_wrapping("")` -> `input.lines()` yields nothing, `pending`
  stays `None`, returns `""`. No panic. `is_hard_start` is only ever called on a non-empty
  `collapsed` string (the `trimmed.is_empty()` branch returns first), and is additionally
  safe on empty input via `bytes.first()` returning `None`. No off-by-one: ordered-list
  detection indexes `bytes.get(digits)` / `bytes.get(digits + 1)` with bounds-checked `.get`.
- Fence handling: `~~~` and ``` ``` ``` toggles use `trim_start` so indented/trailing-space
  fences still match; in-fence lines pass verbatim, so whitespace changes inside code are
  caught, not masked.
- Error handling: `self_scaffold_asset` uses descriptive `.expect(...)` / `unwrap_or_else(||
  panic!("... {dest}"))` messages; a config/render break points at the exact failing step
  rather than panicking opaquely.
- Second test asserts BOTH directions and is not trivially passing: `assert_eq` for an
  intra-paragraph soft wrap and for collapsed spaces/blank-line runs (tolerated), and
  `assert_ne` for a dropped word, a dropped list item, and a merged block boundary (caught).
  Confirmed the merged-block case genuinely differs post-normalization (heading joins the
  following paragraph line), so the `assert_ne` is load-bearing, not vacuous.
- Failure messages on both tests point to `just scaffold-self`, which is the actionable fix.

## Findings

### F1 (Info / non-blocking) - src/main.rs:12

Problem: `mod agents_md_drift;` is declared un-gated even though the file's only content is a
`#[cfg(test)] mod tests`. In a release build this compiles to an empty module (zero code, so
it "adds nothing to the release build" as intended), but the test-only nature is implicit.

Suggested fix: optional. Gating the declaration as `#[cfg(test)] mod agents_md_drift;` would
make the test-only intent explicit and drop the empty module from non-test builds. Counter-
argument (why the current form is defensible and I am NOT recommending a change): no `mod`
declaration in `src/main.rs` is currently cfg-gated, and the inline `#[cfg(test)] mod tests`
idiom matches the sibling guards exactly; gating only this one would itself be a small
divergence. Leave as-is unless the orchestrator wants an explicit test-only marker. No
correctness or lint impact either way.

## Verdict

No blocking findings. One Info-level, non-blocking nit (F1). The guard builds, both new tests
pass, clippy is clean, and the plan validators/render-check are unaffected. Indentation,
ASCII, suppression, and module-placement house rules all hold.
