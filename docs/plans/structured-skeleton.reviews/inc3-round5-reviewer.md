# Inc 3 round 5 (final confirming) review - render engine, independent full-increment pass

Reviewer lens: independent FINAL confirming round (round 5) on the render engine. Role separation holds: I did NOT write this code. This is the risky increment; round 4 was clean, so a clean round here converges it (two consecutive clean rounds). I am a fresh set of eyes: rather than re-verify a single fix, I re-read and re-attacked the WHOLE increment for anything the prior four rounds missed.

Full increment under review: `9afe567..ffb0eca` (HEAD `ffb0eca`). Nothing changed since round 4 (no new commits), so this is a genuine independent read of the entire increment.

Build/test state (run READ-ONLY in the worktree `structured-skeleton-inc3`, HEAD `ffb0eca`):

- `cargo test --all-targets` -> 286 passing (282 unit + 1 `checks_staged_hook_env` + 3 `scaffold_precommit_hook`), 0 failed. Matches the expected count.
- `cargo clippy --all-targets -- -D warnings` -> clean, zero warnings.
- `cargo run -- render --check src/plan/testdata/render-fixture.plan.toml` -> `up to date`.
- `docs/plans/agent-scaffold.md` (the live plan) last touched at `9afe567`, the increment base; no commit in `9afe567..ffb0eca` touches it. Confirmed untouched.

## Verdict

CLEAN. No findings of any severity after a genuine full-increment adversarial pass. The increment satisfies its contract bullet (`docs/plans/agent-scaffold.md:686`) and its stated acceptance criteria. The two prior residuals stand as previously settled: L1 (symlink escape) is the orchestrator's consciously-accepted residual with no new evidence, and F2 (mid-section sidecar placement) is deferred to Inc 5. I re-examined both and neither warrants re-raising.

---

## What I verified end to end

### Render projection correctness

- Determinism / ordering. Steps sort by `order` then `slug` (`render.rs:176`), questions by `question_id_index` (`render.rs:183`), principles by their own `n` (`render.rs:398`), waiver reasons by `WaiverReason::ALL` (`render.rs:379`), step-status buckets by `StepStatus::ALL` (`render.rs:353`). All orderings are over fixed keys or fixed constant arrays; no map iteration or hash order reaches the output. `assemble` builds a `Vec<String>` joined with `"\n\n"` plus one trailing `\n`. Two renders are byte-identical (`render_is_deterministic_and_matches_the_golden` asserts `first == second`).
- The numeric question sort has no silent lexical fallback. I checked the one place that could hide a determinism gap: `question_id_index` returns `Option<u64>`, and a `None` key would sort ahead of others. But `validate_source` (`source.rs:457`) rejects any question id that does not parse as `Q-<n>`, and `render_plan` runs `validate_source` FIRST and returns on any problem (`render.rs:145-148`), so by the time the sort runs every id is a real `Q-<n>` and every key is `Some`. A `u64`-overflowing id (`Q-<huge>`) fails `parse::<u64>()`, so it is `None` at validate and rejected there; it never reaches the sort. No panic, no lexical fallback. The C5 test pins `Q-1 < Q-9 < Q-10` (a lexical sort would misorder) and the equal-`order` slug tiebreak (`epsilon` before `zeta`, both order 5, `zeta` declared first).
- Derived Status line (`status_line`, `render.rs:351-392`). The open-question count is `matches!(status, Open | Exploring)` (the C1-decided semantics; `decided`/`superseded` excluded). The step-status distribution counts over `StepStatus::ALL` and drops zero buckets. Waivers are collected across all steps and broken down by `WaiverReason::ALL`, with the total from `waivers.len()`. Counting is order-independent, so using unsorted `plan.steps` here (vs the sorted `step_blobs` for the Roadmap) is fine. The golden pins the whole line verbatim ("7 steps (...); 3 open questions; 2 waivers (1 predates-logging, 1 accepted-at-escalation).").
- Status-vocabulary fragment (`vocabulary_section`, `render.rs:410-419`). Generated from `ROADMAP_STATUSES` + `ROADMAP_BLOCKED_PREFIX` + `QuestionStatus::ALL` (the code constants, `plan.rs:91-101`), under the `## Roadmap Status Vocabulary` heading (renamed off the live plan's `## Documentation Protocol`, the F3 fix). This is the same constant `validate` enforces, so the vocabulary cannot drift (B3).
- Roadmap table + inline waiver Notes (`roadmap_section`, `notes_cell`, `waiver_note`). Each row is `| slug | label | notes |`; Notes render `blocked_by` as `blocked on <slug>` then inline waiver descriptors joined with `; `. The defensive `(Increment, None)` / `(Step, _)` arms are unreachable in a valid source (validate pairs unit and increment) and render without panicking. The golden pins a blocked+waived row and an increment-unit waiver row.
- Question Details / Step Details (`render.rs:519-548`). Both return `Option<String>` and emit NOTHING (no bare heading) when no item has non-empty trimmed body (the N1 fix). The queue itself is strictly one line per item (the F1 relocation), so a body blank line cannot fragment the Markdown list. `question_status_display` shows the fold target / superseding id / receipt appropriately.
- Principle numbering. Rendered from each entry's own `n`, sorted by `n`; the golden shows the intended non-consecutive `1.` and `8.`, confirming it is not a running counter.

### `render --check` byte compare

- `check_render` (`render.rs:256-279`) re-renders in memory and byte-compares against the committed `<task>.md`. Hand-edit detection (`a_one_byte_hand_edit...`), stale-render-after-source and after-TOML detection (`a_stale_render_after_a_source_edit...`, `..._toml_edit...`), and absent vs present-but-unreadable are all covered: only `io::ErrorKind::NotFound` reports `committed_exists: false`; any other read error (invalid UTF-8, permission) is its own `Err(... present but unreadable ...)` (the C3 fix), tested by `a_present_but_unreadable_committed_file_is_an_error_not_absent`.
- Exit-code wiring (`main.rs:478-556`, `run_render`). `Match` -> prints "up to date", exit 0. `Mismatch` without `--strict` -> stderr WARNING, exit 0 (warn-local). `Mismatch` with `--strict` -> stderr error, `exit(1)` (fail-CI). A render failure under `--check` -> problems to stderr, `exit(1)` (no golden to check a broken source against). `--strict` carries `#[arg(long, requires = "check")]`, so `render --strict` without `--check` is a clap usage error. The absent-file branch words a distinct "does not exist; run ..." message.
- `difference_summary` (`render.rs:202-240`). First differing line reported with both sides truncated; when no line differs, a line-count difference is worded from counts, and a pure trailing-whitespace/newline delta is worded from a BYTE compare, not the degenerate "N vs N" (the C2 fix), pinned by `a_trailing_newline_only_mismatch...`.

### Write atomicity

- `write_rendered` (`render.rs:556-583`) writes to a same-directory temp `.{file}.{pid}.tmp`, then `fs::rename` into place. The write+rename is a single `and_then` chain; on ANY error branch the shared `if write_then_rename.is_err() { remove_file(temp) }` cleans up, so neither a write failure (ENOSPC / kill mid-write) nor a rename failure leaks a temp (the L2 fix), pinned by `a_failed_atomic_write_leaves_no_temp_file_behind`. The pre-existing `<task>.md` is never opened until the rename, so every failure leaves it byte-intact. Write-nothing-on-failure holds at the `render_plan` level: schema/xref/missing-sidecar all fail in memory before the CLI ever calls `write_rendered` (`a_missing_sidecar...`, `an_unresolved_cross_reference...`, `a_schema_invalid_toml...` all assert no `<task>.md` is written). `render_writes_exactly_one_file...` snapshots every source before the render and asserts each is byte-identical after and that the ONLY created file is `<task>.md` (no temp sibling).

### Input / path handling

- `task_name` derives `<task>` by stripping `.plan.toml`; an empty stem or a non-`.plan.toml` name is a reported usage error (`a_non_plan_toml_path_is_a_usage_error`), not a guess or panic.
- Sidecar refs. `is_safe_sidecar_ref` (`source.rs:393-400`) rejects any absolute ref and any ref with a `..` (ParentDir) component, allowing only `Normal` / `CurDir` components. It is enforced in `validate_source` (`source.rs:590-603`) for both `front` and `tail`, so render, `render --check`, and `validate --source` all refuse the same thing (the R1 fix). Empty-string and `.`-only refs are lexically safe and resolve to a hard "missing or unreadable sidecar" error, not a traversal.
- No panic on malformed-but-schema-valid input: all `write!`s target a `String` (infallible), `truncate` uses a char iterator (unicode-boundary safe), `escape_cell` neutralizes `|`, `\n`, `\r`, and `\r\n` (the R2 fix). Free text reaching a table cell is only a waiver note, and it is escaped.

### Drift guards

- `StepStatus` and `QuestionStatus` are generated by the `enum_field!` macro, and each carries `const _: () = assert!(X::ALL.len() == X::VARIANTS.len());`. Adding a variant grows `VARIANTS` (macro-generated) but not the hand-written `ALL`, so the const assert fails to compile until `ALL` is extended (the L3b fix; round 4 verified the serde spellings are byte-identical to the prior `rename_all`). `WaiverReason::ALL` carries the same cross-check.

### Test integrity (Principle 11) and golden faithfulness

- Tests are non-vacuous: each asserts a specific property (a pinned Status line, a numeric-order inequality, a present-vs-absent distinction, a no-temp-leak scan, a no-clobber snapshot compare), and the negative-path tests assert both the failure AND that nothing was written.
- The golden `render-fixture.md` is faithful engine output, not a hand/prettier edit: `render_is_deterministic_and_matches_the_golden` asserts `render_plan(...) == include_str!(golden)` and it is green, and `render --check` on the fixture reports `up to date`. The fixture exercises all seven `StepStatus` values, all four `QuestionStatus` values, both waiver units/reasons, an equal-`order` slug tiebreak, and `Q-1/Q-9/Q-10` numeric ordering.

## Re-attacks that found nothing

- Non-determinism hunt: no hash/map iteration reaches output; the status-line counts over unsorted `plan.steps` are order-independent; waiver breakdown is over a fixed constant array.
- Numeric-sort fallback: closed by the validate boundary (every id is `Q-<n>` before the sort).
- Empty / degenerate sidecar refs, empty task stem, bare-filename paths (no parent): all resolve to a clear error or the current directory, no panic, no traversal.
- `write_rendered` on a bare filename (no parent): temp lands in cwd, rename within cwd, no leak.
- Section assembly with dropped optional sections: joined with `"\n\n"`; an absent section is simply not in the vector, so blank-line structure stays well-formed (golden byte-matches).
- L1 (symlink sidecar ref): re-examined under the CURRENT trusted in-repo threat model. The check is still lexical and a symlink still escapes, but that is only reachable via an in-repo committed artifact today; no new evidence raises it above low. Left as the orchestrator's accepted residual for the git-url-fetch increment. Not re-raised.
- F2 (mid-section sidecar placement): deferred to Inc 5 by decision; no new evidence. Not re-raised.

## Notes

- Test count: 286 (282 + 1 + 3), matching rounds 3 and 4. No tests added or removed since round 4.
- Clippy clean under `-D warnings`.
- The increment touches only render/schema/CLI code and testdata (23 files, `git diff --stat 9afe567 ffb0eca`); the live `docs/plans/agent-scaffold.md` is not among them.
- This is the second consecutive clean round (round 4 CLEAN, round 5 CLEAN), so the increment converges.
