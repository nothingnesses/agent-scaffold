# Drift-guard round 3 review: code-quality / mechanical lens

Scope: the consolidated fix to `src/agents_md_drift.rs` (git diff e7f18e1 HEAD),
which rewrote `assert_no_unprotected_construct` to track `in_fence` and assert
each non-fence line equals its canonical whitespace form, rewrote the
doc-comment, and added `precondition_rejects` (catch_unwind + silenced panic
hook) plus two regression tests.

## Evidence lines (all run under the project Nix toolchain)

- `cargo test`: `test result: ok. 346 passed; 0 failed` (lib), plus integration
  suites `1`, `3`, `1`, `2` passed, `0 failed` across the board.
- Drift module specifically: `cargo test agents_md_drift` ->
  `test result: ok. 4 passed; 0 failed; 0 ignored; ...; 342 filtered out`
  (`the_committed_scaffold_matches_a_fresh_render`,
  `normalization_tolerates_wrapping_but_not_content_change`,
  `precondition_rejects_non_space_whitespace_and_round_one_cases`,
  `precondition_exempts_fenced_indented_lines_but_not_bare_ones`).
- `cargo clippy --all-targets`: `Finished dev profile ... in 4.73s`, no warnings.
- `cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml`:
  `docs/plans/agent-scaffold.plan.toml: 84 steps, 65 questions, valid`.
- `... validate --source ... --workflow`:
  `docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold`.
- `cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml`:
  `docs/plans/agent-scaffold.plan.toml: up to date`.

The three validator/render targets are unaffected by this diff, as expected.

## Findings

### R3-CQ-1 (low / nit): global panic-hook swap in `precondition_rejects` is not serialized across parallel tests

- Location: `src/agents_md_drift.rs:381-388` (`precondition_rejects`), called
  from the two tests at `:390` and `:416`.
- Problem: the panic hook is a process-wide global. The sequence
  `take_hook()` -> `set_hook(noop)` -> `catch_unwind(...)` -> `set_hook(previous)`
  is not atomic, and `cargo test` runs the binary's tests in parallel by
  default. The two tests that call `precondition_rejects` can interleave their
  take/set pairs: e.g. test A takes the default hook and installs the no-op,
  then test B's `take_hook()` returns A's no-op as its "previous", so a restore
  can leave the no-op hook installed. A concurrent or subsequent genuinely
  failing test would then have its panic message/backtrace suppressed.
- Why it is only a nit, not a flake: each call captures acceptance/rejection via
  `catch_unwind(...).is_err()`, which is independent of the hook. The hook only
  governs whether a backtrace is printed, so no assertion outcome depends on the
  race; it cannot flip pass/fail of these tests or of others. The impact is
  purely diagnostic (a real failure elsewhere might print no location) and only
  during the brief window these two tests run.
- Fix (optional): serialize the swap behind a shared `std::sync::Mutex` guard
  held across the take/catch/restore, or drop the hook swap entirely and accept
  the backtrace noise on the two intended panics, or set a no-op hook once for
  the whole test binary. Given only two callers and no correctness impact, this
  can be left as-is; recording it for completeness.

## Items checked and clean

- Rewritten helper (`:99-124`): readable; fence toggling at `:110-117` uses the
  exact same rule as `normalize_wrapping` (`line.trim_start()` then
  `starts_with("```")` / `starts_with("~~~")`), so the two agree on which lines
  are verbatim; fence-marker lines and in-fence lines are `continue`d (correctly
  exempt, since `normalize_wrapping` emits them verbatim). The assert message
  (`:119-122`) names the file (`{name}`), the 1-based line (`number = index + 1`,
  `:107`), the offending line (`{line:?}`), the canonical form (`{canonical:?}`),
  and the required hardening. No opaque `unwrap`.
- Predicate correctness (`:118`): `line.split_whitespace().collect::<Vec<_>>().join(" ")`
  correctly subsumes the round-1 leading-whitespace and double-space checks and
  additionally trips on non-space whitespace (tab, NBSP, form feed); the two new
  regression tests confirm tab and NBSP rejection, the round-1 cases, and the
  fenced/bare indentation split.
- Doc-comment (`:72-98`): matches the new predicate precisely, states the exact
  canonical-form expression, that fenced code is exempt (with the fence rule),
  and lists the unprotected constructs. No stale references to the removed
  `starts_with(' ')` / `contains("  ")` checks.
- House rules on the added lines: hard tabs only (no space-indented added lines);
  no em-dash / en-dash / double-hyphen-as-dash; no emoji; no non-ASCII bytes in
  the file (the NBSP fixture is written as the ASCII escape `\u{00a0}`, source
  stays ASCII); no `#[allow]` added (so the expect-over-allow rule is N/A).
- No regression: existing tests and module structure unchanged; the two prior
  drift tests still pass, and the whole suite is green.

## Verdict

One low-severity / nit finding (R3-CQ-1, diagnostic-only, cannot flake results).
No correctness, build, lint, or house-rule defects in the diff.
