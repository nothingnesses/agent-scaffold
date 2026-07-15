# instrument-flag round 2, reviewer findings

Role: reviewer (round-2 verification). Artifact: the `instrument-flag` step fixes in commit `68da587` (fix range `d458373..68da587`). Verified against `AGENTS.md` Principles 1-7 and the plan. Severity scale: low / medium / high / critical.

Verdict: CLEAN VERIFICATION. Both fixes (V-1, V-2) landed and are correct. No new valid findings at any severity. No critical, no high, no medium, no low.

## V-1 (was medium): render normalises to one trailing newline

Verified as correct. The fix is `src/manifest.rs:224`, `format!("{}\n", out.trim_end())` in `render()`, applied to rendered (`render = true`) assets only; verbatim assets bypass `render()` (`src/manifest.rs:244`).

(a) NO-OP for committed generated files. Ran `direnv exec . just scaffold-self` from a clean tree at HEAD `68da587`. Result: 15 assets refreshed, `nix fmt` reported "formatted 15 files (0 changed)", `git diff --stat -- AGENTS.md .agents/AGENTS.reference.md docs/plans/TEMPLATE.md` was EMPTY, and full `git status --short` was clean. No committed generated file drifted.

(b) RAW OFF output is byte-identical without a formatter. Built and ran the binary directly (no `nix fmt`) into scratch dirs. OFF (`--principles default`, no `--instrument`): `AGENTS.md` ends `...gs.\n` (od-confirmed final bytes `g s . \n`), exactly one trailing newline with no trailing blank line, and contains zero occurrences of "Instrumentation". The raw OFF file is byte-identical to the committed (formatter-processed) root `AGENTS.md` (`diff` of the OFF bytes against `AGENTS.md` reported identical, sizes both 25233 bytes), confirming the raw binary output no longer depends on a downstream formatter for byte-stability. ON (`--instrument`): ends `...ly.\n` (final bytes `l y . \n`), one trailing newline, and includes the "Instrumentation (metrics logging)" section and the `docs/metrics/workflow.jsonl` path.

(c) Tests pin the invariant and pass. `direnv exec . just test`: 50 passed, 0 failed (matches the expected 50), including the new `manifest::tests::render_normalises_to_a_single_trailing_newline`, the updated `manifest::tests::render_substitutes_known_and_leaves_unknown` (now asserts the substituted body gains one newline), and `tests::rendered_agents_ends_with_a_single_trailing_newline` in `src/main.rs` (asserts OFF and ON both `ends_with('\n')` and `!ends_with("\n\n")`, exercising `build_assets` with `instrument` false and true; satisfies Principle 11). `direnv exec . cargo clippy --all-targets -- -D warnings` is clean.

The new tests assert the correct invariant (exactly one trailing newline: `ends_with('\n') && !ends_with("\n\n")`). Pre-existing tests that pass a template without a trailing newline (`directory_source_loads_through_the_same_path`, the var fixtures) still hold because their sources already end in `\n`, so `trim_end()` + `\n` is a no-op for them.

## V-2 (was medium): README documents `{{instrument}}` as reserved

Verified as correct and consistent with the code. README (`README.md:213`) now states both `{{principles}}` and `{{instrument}}` are built-in and reserved, that `{{instrument}}` is filled from the pack's optional `instrument.md` render fragment when `--instrument` is set and empty otherwise, and that the fragment (like `principles.toml`) is read directly and inlined, not dropped as its own asset.

Cross-checked against the code:

- Both reserved: `RESERVED_VARS = &["principles", "instrument"]` (`src/manifest.rs:76`); the two reserved-variable rejection paths are covered by `reserved_variable_is_rejected` and `reserved_instrument_variable_is_rejected`.
- Fill rule: `src/main.rs:204-206`, `if instrument { source.read("instrument.md").unwrap_or_default() } else { String::new() }`, matches "filled from `instrument.md` when set, empty otherwise".
- "Read directly, not dropped as its own asset": confirmed `instrument.md` has no `[[asset]]` entry in `pack/pack.toml`; it is read via `source.read`, exactly as `principles.toml` is.

## "Did the fixes introduce anything new?"

Checked the three concerns raised:

- Is always appending a trailing newline correct for all rendered assets? Yes. Only `AGENTS.md` and `.agents/AGENTS.reference.md` are `render = true` (verified in `pack/pack.toml`); both are Markdown docs for which a single terminating newline is the correct POSIX-text convention. As a general default for any pack's rendered asset it is also correct, and byte-exact output remains available via `render = false` (verbatim), which the updated doc comment now states explicitly. No new issue.
- Does trimming trailing whitespace risk removing meaningful content? No. `trim_end()` only affects the very end of the fully substituted string; for these guidance docs no trailing whitespace is meaningful (a Markdown hard-break on the final line would be inert). It cannot touch interior content.
- Any test asserting the wrong thing? No. See V-1(c).

No critical findings. No high findings. No medium findings. No low findings.

## Settled findings (not re-raised)

Per the ledger and the round-2 charter, V-3 (accepted residual risk), V-4 (deferred to `state-schema`), and V-5 (ruled invalid) are settled; no new evidence overturns any of them, so none is re-raised.
