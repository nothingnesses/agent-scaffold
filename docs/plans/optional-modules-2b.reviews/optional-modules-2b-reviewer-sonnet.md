# Review: optional-modules increment 2b (checks module content)

Reviewer: independent content/docs reviewer (claude-sonnet-4-6) Diff range: 64f79db..05fac05 Branch: impl/inc2b-checks-content Date: 2026-07-17

Files examined: `pack/checks.toml`, `pack/checks-guidance.md`, `pack/prompts/checks-reviewer.md`, `pack/checks/ast-grep/rules/no-dbg-macro.yml`, `pack/checks/ast-grep/sgconfig.yml`, `pack/pack.toml`, `src/main.rs`, `src/manifest.rs`. Context read: `docs/plans/agent-scaffold.md` (lines 512-522), `docs/plans/checks-module-config.explorations/checks-config-A.md`, `docs/plans/checks-module-config.explorations/checks-config-B.md`, `pack/AGENTS.md` (findings-file naming convention, `{{modules}}` slot placement).

Scope: content quality, self-containedness, and the multi-language plug-in story. The `checks` subcommand and pre-commit hook are 2c scope; their absence is not a finding.

---

## Findings

### F1 - medium

`pack/checks.toml` (header comment, lines 31-34): the plug-in / multi-language story is stated in prose only, with no example row for a non-ast-grep tool.

The header says "Plug in any other linter or formatter by adding a `[[check]]` row; nothing here is privileged." That sentence is accurate but thin. The exploration (checks-config-A.md section 6 / TOML sketch lines 76-83) included a commented clippy example:

```
# -- Plug in ANY other linter or formatter identically: add a [[check]] row.
#    A language with no ast-grep grammar plugs in ruff / eslint / clippy the
#    same way ast-grep is wired above. --
# [[check]]
# name = "clippy"
# kind = "lint"
# command = "cargo clippy --all-targets -- -D warnings"
```

That block was dropped. The shipped file gives the test and mutation kinds as commented template rows, but gives NO commented template for a third-party language-specific linter (ruff, eslint, clippy). A user with a Python or TypeScript codebase reads "add a row" and must synthesize the correct shape from the schema table alone, rather than copying and editing an example. For a polyglot user, this is the most important example to ship: it demonstrates that adding ruff (`kind = "lint"`, `command = "ruff check ."`) or eslint is no different from the seeded ast-grep row, and that multi-language support is just multiple rows. The exploration treated this as load-bearing; the shipped version states it in one clause and moves on.

Suggested fix: add a commented block after the formatter entry (or inside the "Who runs what" paragraph) with one or two example linter rows (e.g., clippy and ruff) clearly marked as "replace or add rows for your language's tool."

---

### F2 - medium

`pack/checks-guidance.md` (line 3) / `pack/checks.toml` header: the guidance names "ASCII-clean" as one of the hardcoded conventions being generalized, but no shipped file documents that ASCII-clean enforcement requires a separate regex/grep [[check]] row (not an ast-grep rule), and nothing explains why the seeded example rule is `no-dbg-macro` (Rust-specific) rather than anything related to ASCII.

The guidance says: "It generalises the hardcoded verification convention (clippy / `nix fmt` / ASCII-clean) into a declared list of commands."

A reader following this reference will look at the seeded checks.toml and the seeded ast-grep rule expecting to find the module's answer to ASCII-clean. They find a Rust-specific `dbg!` rule instead, with no note explaining:

- why the seeded rule is not ASCII-related,
- that non-ASCII detection is not soundly expressible in ast-grep (the reason the implementer chose no-dbg-macro),
- or what the user would need to do to get ASCII-clean coverage (add a `[[check]]` row running `grep -r '[^[:ascii:]]'` or `rg '[^\x00-\x7F]'`, not an ast-grep rule).

The gap is especially visible because the guidance explicitly raises the ASCII-clean expectation. A user who reads "generalises... ASCII-clean" and then sees only a dbg! rule will reasonably assume ast-grep covers ASCII-clean via the seeded rule, run the scan on a non-Rust or mixed project, and get no findings - not because the code is clean, but because the rule is Rust-only and unrelated to ASCII.

Suggested fix: either (a) add a short note to the ast-grep block in checks.toml explaining that ASCII-clean requires a separate `[[check]]` row (e.g., `rg '[^\x00-\x7F]'`) since non-ASCII matching is not practical in ast-grep, or (b) remove "ASCII-clean" from the guidance's parenthetical list and let the user discover that use case on their own. Option (a) is preferable because it actively completes the extension story the guidance invites.

---

### F3 - low

`pack/checks/ast-grep/rules/no-dbg-macro.yml` (lines 1-10): the header comment does not flag that this is a Rust-only rule.

The file says "Seeded example ast-grep rule demonstrating the rule format, so `ast-grep scan` runs out of the box. It is yours to edit or replace." The `language: rust` field is visible, but the prose comment says nothing about it. A user working in Python, TypeScript, or a mixed project who runs `ast-grep scan` and sees zero findings has no way to tell from the file comment whether the tool is misconfigured or simply has no Rust source to match. One sentence noting that this rule targets Rust and that non-Rust or multi-language projects should replace or supplement it would resolve the ambiguity immediately.

The checks.toml header does say "Replace this row with your language's linter... if ast-grep does not cover your language" - but that addresses the [[check]] row, not the rule file, and a user reading the YAML directly would not see it.

---

### F4 - low

`pack/checks.toml` (schema table, `paths` field, line 26): the `paths` field's use case for language-scoping is described in one line ("(optional) globs scoping the check") but never demonstrated or called out as the mechanism for polyglot repos.

The test/mutation commented rows do include `paths = ["tests/", "src/**/*.rs"]`, so a reader can infer the syntax. But the connection between `paths` and language-scoping - using `paths = ["**/*.py"]` to restrict a ruff check to Python files, or `paths = ["**/*.ts"]` to restrict an eslint check to TypeScript - is never mentioned. Given that F1 (missing third-party linter example) is already a finding, a parenthetical example in the `paths` schema entry would make both issues easier to address together: an example row like `ruff` with `paths = ["**/*.py"]` demonstrates both the plug-in story and the scoping mechanism in one place.

---

### F5 - low

`pack/prompts/checks-reviewer.md` (line 9, the lint check instruction): the prompt says "run its `command`" with no mention of the `paths` field. In the 2b increment with only the ast-grep row (which handles path scoping via its own sgconfig.yml), this is not a live problem. But once a user adds a paths-scoped row (e.g., `ruff check .` scoped to `paths = ["**/*.py"]`), the LLM reviewer reading this prompt would have no instruction to incorporate the `paths` field into the command it runs. The `paths` field is documented in checks.toml's schema table, which the reviewer is told to read, so a careful agent might use it anyway - but an explicit sentence like "If a check declares `paths` globs, scope the command to those paths when running it" would make the expected behavior unambiguous.

---

## Non-findings worth recording

**Rule-swap deviation (no-dbg-macro vs. no-ascii-escape):** the choice of no-dbg-macro over no-ascii-escape is a valid implementation decision (ast-grep cannot soundly match arbitrary byte values), but its documentation consequences are captured in F2 and F3 above. The rule itself is correct for what it claims to do.

**checks-reviewer self-containedness:** the role prompt is clear and self-contained for its scope. The read-only contract, the lint/format split, the degraded-finding fallback for a missing `check` command, and the findings-file contract are all stated. AGENTS.md is cited for the naming convention (correct single-sourcing). The prompt does not re-state the schema (correctly defers to checks.toml). The lint/format/test/mutation run policy is unambiguous.

**sgconfig.yml path resolution:** the `ruleDirs: - rules` entry resolves relative to the file (`ast-grep` behavior), which matches the command `ast-grep scan --config .agents/checks/ast-grep/sgconfig.yml` run from repo root. The comment in sgconfig.yml explains this. Correct.

**`<step>-checks.md` naming:** checks-guidance.md assigns the findings-file name `<step>-checks.md`, which is distinct from the `<step>-<role>-<disambiguator>.md` pattern in AGENTS.md. Because there is exactly one checks-reviewer, the disambiguator is absent; this is consistent with there being no collision risk and is acceptable, though slightly irregular.

**checks-guidance.md three-point structure:** the guidance correctly explains (1) formatter auto-apply at the implementer's verify step, (2) checks-reviewer spawned read-only in the work-review phase, and (3) triager adjudicating deterministic findings on the same scale as LLM findings. Single-source deference to checks.toml for the schema is correct. The guidance does not re-state the schema (Principle 1 honoured).

**CHANGELOG:** no CHANGELOG entry is warranted for 2b alone. The plan (agent-scaffold.md line 512) records a single changelog entry at the close of all of increment 2 (covering 2a/2b/2c together). No finding.

**ASCII and unicode audit:** no em-dashes, en-dashes, unicode arrows, math symbols, box-drawing characters, or emoji found in any shipped file. All clean.

**Commented test/mutation rows as schema documentation:** these are present and correct. They establish the full schema (including `paths`, `budget`, `threshold`) in a concrete example the reader can copy. The one gap is the absence of a linter-plug-in example row (F1), not the reserved-kind rows themselves.
