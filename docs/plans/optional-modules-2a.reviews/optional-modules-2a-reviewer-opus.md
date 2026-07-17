# Reviewer findings: optional-modules sub-increment 2a (`{{modules}}` slot, `guidance`, `requires`)

Reviewer lens: correctness, edge cases, adversarial fixture fuzzing. Change under review: commit `de70ad0` on `impl/inc2a-modules-slot`, diff range `b565103..de70ad0`. Verified in a throwaway detached worktree at `de70ad0`: `just test` (123 passed), `just clippy` (clean, `-D warnings`), plus binaries built at both `b565103` and `de70ad0` and driven with real and adversarial fixture packs via `--template`.

## Verdict

No `critical` or `high` findings. The machinery is correct on every adversarial path I could construct: `requires` cycles and self-reference terminate, a diamond enables the shared dependency once, guidance order is `[[module]]`-declaration-deterministic regardless of `--module` argument order or requires-discovery order, the enabled set is genuinely single-sourced through `expand_modules` for both `load` filtering and `module_guidance`, the reserved `modules` var is rejected both when declared and when set via `--var`, and the BYTE-IDENTITY guarantee holds. Findings below are `medium`/`low`: test gaps and one contextless error message.

## Byte-identity: verified empirically (no finding)

Built the binary at `b565103` and at `de70ad0`, ran `scaffold --write` for the module-free built-in pack into separate trees, and diffed. `AGENTS.md` is byte-identical and the whole 18-file tree is identical (`diff -r` reports no differences). The tail change in `pack/AGENTS.md` (`{{instrument}}\n\n{{modules}}\n`) collapses correctly: with `principles` non-empty and both `instrument` and `modules` empty, `render`'s `trim_end` + single `\n` (`src/manifest.rs:288`) removes the trailing empty blocks. The unit test `modules_slot_renders_empty_for_the_module_free_builtin` (`src/main.rs:958`) exercises the real built-in render tail and its assertions (`!contains("{{modules}}")`, ends with `\n`, not `\n\n`) are load-bearing. Confirmed the claim; nothing to fix.

## Adversarial cases exercised (all pass)

- Self-require (`A requires ["A"]`): terminates, guidance emitted once. The `enabled.insert` guard plus the `if !enabled.contains(req)` pre-check in `expand_modules` (`src/manifest.rs:400-410`) prevent re-push.
- Diamond (`A requires [B,C]`, `B` and `C` each `requires [D]`): all four enabled, `D` emitted exactly once, guidance in declaration order `A,B,C,D`. Because `module_guidance` iterates `manifest.module` (declaration order) and gates on `enabled.contains` (`src/manifest.rs:355-361` region of the new fn), dedup and ordering are structural, not incidental.
- Argument-order independence: `--module C --module A` yields the same declaration-order output as `--module A`. Deterministic.
- Cycle (`a<->b`): the existing `a_requires_cycle_terminates` test would HANG rather than fail if the fixed point regressed, so it is a real termination guard. Confirmed.
- Reserved `modules`: rejected end-to-end via `--var modules=x` on the CLI, and via a pack `[[var]] name = "modules"` in the unit test. `RESERVED_VARS` updated at `src/manifest.rs:112` (now `principles`, `instrument`, `modules`).

## Findings

### 1. [medium] Missing-guidance-file behavior is untested (Principle 11 gap)

The prompt asks explicitly: a module whose `guidance` names a file the pack does not ship, error or silent empty? Answer verified empirically: it ERRORS and writes nothing (`module_guidance` -> `source.read(guidance)?` at `src/manifest.rs:365-367` propagates the `io::Error` as `LoadError::Io`). I built a fixture with `guidance = "does-not-exist.md"`, selected the module, and the scaffold aborted with nothing written. This is the right behavior (fail loud on a pack-authoring typo, unlike a silently-absent `instrument.md`), but there is NO test covering it. Every other error path in this change has a dedicated test; this one is missing, and it is the single likeliest real pack-authoring mistake for the new field. Add a fixture test asserting the missing-guidance-file load fails and drops nothing.

Note the deliberate asymmetry with `instrument`: `build_assets` reads instrument with `source.read("instrument.md").unwrap_or_default()` (`src/main.rs:218`, silent empty when absent) but guidance with `?` (hard error). I think the asymmetry is defensible (guidance is an explicitly named field; instrument's absence is the normal "pack ships no instrument" case), but it is undocumented as a deliberate choice and worth a one-line comment so a future reader does not "fix" the inconsistency by swallowing the guidance error.

### 2. [low] Missing-guidance-file error names neither the file nor the module

When the guidance file is absent for a `Directory` pack, the user sees only `error: No such file or directory (os error 2)` with no filename, no module name, and no hint it is about guidance. Root cause is pre-existing: `PackSource::Directory::read` returns the bare `fs::read_to_string` error (`src/manifest.rs:255`), whereas the `Embedded` arm wraps with `format!("pack file not found: {rel}")` (`src/manifest.rs:253`). 2a makes this newly relevant because `guidance` is a filename a pack author types by hand. Not a 2a regression, but a poor experience for exactly the mistake this field invites. Consider having `Directory::read` add the `rel` path to its error, or having `module_guidance` map the read error to a variant that names the module and file.

### 3. [low] `instrument`-off with `modules` present leaves an un-normalized internal blank-line run

With the built-in tail layout `{{instrument}}\n\n{{modules}}`, when `--instrument` is off but a module contributes guidance, the empty instrument slot leaves three blank lines before the guidance (verified: `HEAD\n\n\n\nA...`). `render` normalizes only TRAILING whitespace (`format!("{}\n", out.trim_end())`, `src/manifest.rs:288`), not internal runs, so this is not cleaned up. It has ZERO impact now (the built-in pack declares no modules, so `{{modules}}` is always empty and byte-identity holds, as confirmed above). It becomes a cosmetic defect in 2b, when a real module with guidance is added to a pack whose template stacks these two empty-able slots. Flagging so it is not a surprise then; a fix would live with 2b (e.g. collapse internal blank runs, or place the slots so an empty middle slot does not orphan blank lines).

### 4. [low] Test gaps beyond finding 1 (Principle 11)

The tests are honest about what they exercise (I checked each claim against the code path; none over-claim). But several behaviors this change introduces are only verified by me empirically, not pinned by a test:

- No DIAMOND test (shared dependency enabled once / guidance emitted once). The cycle test proves termination but not the dedup path.
- No SELF-require test.
- No test that guidance order is independent of `--module` argument order.
- No test for a module enabled (via `requires`) that declares NO `guidance` contributing nothing while a sibling does; `requires_auto_enables...` happens to give both modules guidance.

All four pass empirically; adding them would harden the determinism and dedup guarantees against future refactors of `expand_modules`.

## Design question raised by the implementer

Dedicated `LoadError::UndeclaredModuleRequire { module, requires }` vs folding into `UndeclaredModuleTag`: keep the dedicated variant. `UndeclaredModuleTag` carries `kind: "asset" | "var"` and an `entry` identifier and its message reads `"{kind} \`{entry}\` is tagged with module \`{module}\`..."`. A `requires`reference is module->module, not a tag from an asset/var; folding it in would force either a wrong`"tagged with"`message or a`kind: "module"`hack with an`entry` that is really the requiring module. The dedicated variant's message (`"module \`X\` requires \`ghost\`, which no [[module]] declares"`) is accurate and the split is cheap. This matches the change's own precedent of precise, entry-named errors.

## Non-defect note

`build_assets` computes `module_guidance` and then calls `load`, and each independently re-reads `pack.toml` and re-runs `expand_modules` (`src/main.rs:224` then `226`). Redundant parse + expansion, but NOT a correctness risk: both receive the identical `modules` slice and the same file, so the single-source-of-truth claim for the enabled SET holds (they cannot disagree short of a TOCTOU edit of `pack.toml` mid-load). Left as a note, not a finding; a future tidy could compute the enabled set once and pass it down.
