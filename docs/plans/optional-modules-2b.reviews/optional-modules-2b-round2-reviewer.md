# Optional-modules sub-increment 2b (checks module CONTENT), round 2 (confirming reviewer)

Independent confirming review of branch `impl/inc2b-checks-content`, full diff `64f79db..54dc75f` (commits `260a222`, `05fac05`, `54dc75f`). Round-1 fixes live in `54dc75f`. Scope: checks module CONTENT only; the `checks` subcommand and pre-commit hook (2c) are out of scope. Every check below was run empirically in a throwaway worktree at `54dc75f` (now removed); the main repo was not modified.

## Verdict

Near-clean round. Every round-1 fix (1 through 6) landed and is empirically correct. One LOW wording-consistency residual from fix 2 (the "read-only checks-reviewer" label survives in three spots that the same fix corrected elsewhere). No medium/high/critical. Byte-identity holds, tests green (126), clippy clean, all new files ASCII-clean.

## Round-1 fixes: confirmed

1. Formatter check command (MEDIUM in R1). `pack/checks.toml:63` now reads `check = "nix fmt -- --fail-on-change"`. Empirically: `nix fmt -- --fail-on-change` exits 0 on a clean tree; the old `nix fmt -- --check` is rejected (exit 1, treefmt prints its usage). Correct.

2. Format-check accuracy wording (ACCURACY in R1). `pack/checks-guidance.md:10`, `pack/prompts/checks-reviewer.md` (format-check bullet), the `check`-field doc `pack/checks.toml:18-27`, and the format-row comment `pack/checks.toml:52-58` no longer claim the format check is universally non-mutating / needs no isolation. They now say: prefer a non-mutating dry-run where the formatter supports one (`rustfmt --check`, `prettier --check`), but treefmt v2 has no dry-run and `--fail-on-change` formats in place THEN fails, so the check mutates the tree and must run where changes are safe to discard. I verified this behavior empirically: appending a mis-formatted `fn` to a source file and running `nix fmt -- --fail-on-change` both (a) exited 1 and (b) reformatted the file in place (the appended block collapsed from 4 to 2 diff lines). The corrected wording matches observed treefmt v2 behavior. See LOW below for a residual.

3. ast-grep rule severity (LOW in R1). `pack/checks/ast-grep/rules/no-dbg-macro.yml:12` is now `severity: error`, with a comment (lines 6-9) noting the rule is `language: rust` and that `severity: error` makes a hit a non-zero exit. Empirically: `ast-grep scan --config sgconfig.yml` over a file containing `dbg!(x)` prints `error[no-dbg-macro]` and exits 1; over a clean file it exits 0.

4. Two commented example rows (LOW in R1). `pack/checks.toml:65-77` adds a commented `ascii-clean` ripgrep row (`sh -c '! rg --pcre2 -n "[^\x00-\x7F]" .'`) with rationale (non-ast-grep text/regex linter; ripgrep's find-is-exit-0 inverted with `!`), and `pack/checks.toml:79-84` adds a commented `ruff` row with `paths = ["**/*.py"]`. Empirically, with real ripgrep 15.1.0: the ascii-clean command exits 1 on a file containing a non-ASCII byte (é) and 0 on an ASCII-only file, as documented. Both rows are valid TOML when uncommented, and the ascii-clean `command` deserializes to exactly `sh -c '! rg --pcre2 -n "[^\x00-\x7F]" .'` (the TOML `\\x` escapes resolve to `\x`, which `rg --pcre2` interprets). The whole file (all six rows uncommented, including the reserved `test`/`mutation` rows) parses.

5/6. paths schema doc + prompt (LOW in R1). `pack/checks.toml:31-33` now documents `paths` as language-scoping globs and points at the `ruff` example. `pack/prompts/checks-reviewer.md` (paragraph 3) now tells the reviewer to scope its run to a check's `paths`. Present and correct.

## Structural / regression checks: pass

- BYTE-IDENTITY. Built the binary at `54dc75f` and at `64f79db`, scaffolded module-free (`scaffold --write --vcs none`) from each, and `diff -r` the outputs: byte-identical, no differences. No checks assets appear in the module-free output.
- `--module checks` drops exactly the four assets. Scaffolding `--module checks` adds exactly `.agents/checks.toml`, `.agents/checks/ast-grep/sgconfig.yml`, `.agents/checks/ast-grep/rules/no-dbg-macro.yml`, and `.agents/prompts/checks-reviewer.md` over the module-free set, and AGENTS.md gains the `## Deterministic checks` guidance. Ownership matches the manifest (three working, the reviewer prompt reference) per the `manifest.rs` test. An unknown `--module nope` errors and writes zero files.
- Nothing new runs by default. Both new example rows (`ascii-clean`, `ruff`) are commented.
- `just test`: 126 passed, 0 failed (includes the two new tests). `just clippy` (`cargo clippy --all-targets`): 0 warnings/errors after a forced recompile.
- ASCII-clean. All five new/changed pack files, plus `src/main.rs`, `src/manifest.rs`, and `pack/pack.toml`, contain no non-ASCII bytes.
- Multi-language / plug-in story is concrete: a non-ast-grep example (ascii-clean), a paths-scoped example (ruff), the paths doc, and the reviewer prompt's paths handling all present.
- The round-1 fix commit (`05fac05..54dc75f`) contains exactly the six fixes and introduces nothing else.

## Findings

### LOW: "read-only checks-reviewer" label survives fix 2 in three spots

Fix 2 removed the "read-only, so it needs no isolation" framing for the checks-reviewer from `checks-guidance.md` and the top of `checks-reviewer.md`, precisely because the format `check` (`nix fmt -- --fail-on-change`) mutates the tree. But the same "read-only" role label remains in:

- `pack/checks.toml:36` -- "Who runs what: the checks-reviewer (read-only, in the work-review phase) runs the lint `command`s and the format `check`s." This is internally inconsistent with the `check`-field doc two paragraphs above in the SAME file, which now says the format check "mutates the tree."
- `pack/pack.toml:13` -- the `[[module]]` description: "with a read-only checks-reviewer spawned in the work-review phase."
- `pack/pack.toml:137` -- asset comment: "The read-only checks-reviewer role prompt".

Why it is only LOW, and may be intentional: `AGENTS.md:25` defines "read-only" for a role in the OUTCOME sense (a role that "write[s] only their own findings files" and does not persist plan/code changes), and the corrected `checks-reviewer.md` explicitly tells the reviewer to discard the reformatting rather than keep it. Under that outcome-based definition the label is defensible. The tension is that the general isolation rule (`AGENTS.md:85`, `:98`, `orchestrator.md:11`) ties "read-only" to "need no isolation / no blast radius," which is exactly the claim fix 2 corrected for this reviewer's format check. A reader who maps the residual "read-only" label onto that rule reaches the "needs no isolation" conclusion the fix meant to retire. Recommend either dropping "read-only" from these three descriptors or qualifying it (for example "reports-only" / "read-only in outcome"), so fix 2 is applied consistently. This is NOT the settled isolation-MECHANISM question (deferred to 2c); it is a leftover wording inconsistency within fix 2 itself.

## Not re-raised (settled, no new evidence)

- Whether the format check needs a real isolation MECHANISM (2c design decision, deferred; only the WORDING was corrected here). The corrected wording is accurate; the LOW above is about label consistency, not the mechanism.
- CHANGELOG (orchestrator-owned, deferred to increment-2 close).
