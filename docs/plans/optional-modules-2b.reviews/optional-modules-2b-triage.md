# optional-modules 2b (checks module content) triage

Triager: independent adjudicator (opus). Diff range `64f79db..05fac05`, branch `impl/inc2b-checks-content`. Inputs: reviewer findings `optional-modules-2b-reviewer-opus.md` (1 medium + 3 low) and `optional-modules-2b-reviewer-sonnet.md` (2 medium + 3 low). Verification: all runtime claims re-checked against `05fac05` files via `git show`; the `nix fmt` flags run directly against the (unchanged) flake in the main tree (treefmt v2.5.0).

Deduplication: sonnet F2 and opus L4 are the SAME issue (ASCII-clean named in the intro but not represented in the seed) and are merged into issue E. All other findings are distinct. No finding is INVALID; severities are largely low. One consolidated implementer fix round resolves the batch.

Orchestrator note carried through: the human has DECIDED to add a commented ripgrep (`rg '[^\x00-\x7F]'`) ASCII-clean example `[[check]]` row to the seeded `checks.toml` in the upcoming fix round. That row is a second, non-ast-grep tool AND encodes the ASCII recipe, so it is the planned resolution for D (F1) and E (F2 / opus L4). I assess residue below.

---

## Issue A - `nix fmt -- --check` is an invalid check-mode flag (opus M1)

- Verdict: VALID.
- Severity: medium (impact if unfixed: on this repo and any treefmt-v2 project the seeded format check ALWAYS exits non-zero regardless of tree state, so the checks-reviewer raises a spurious format finding every work-review round; the project ships its own convention broken).
- Independent confirmation: `nix fmt -- --version` -> `treefmt v2.5.0`. `nix fmt -- --check` -> exit 1 with the usage dump (unknown flag), i.e. treefmt v2 has no `--check`. `nix fmt -- --fail-on-change` -> exit 1 only when a file actually changed (correct check-mode semantics), exit 0 on a clean tree. `nix fmt -- --ci` -> exit 0 on a clean tree. So opus's diagnosis and fix are correct.
- Fix: change `pack/checks.toml` `check = "nix fmt -- --check"` to `check = "nix fmt -- --fail-on-change"` (opus's recommendation; `--ci`, which is `--no-cache --fail-on-change`, is also acceptable and slightly more robust against a stale cache). `command = "nix fmt"` is correct and unchanged.
- Owner: implementer.
- Round needed: yes (folds into the planned fix round).
- Not critical because: `checks.toml` is a create-if-absent user-owned working file whose own comment tells users to swap in their formatter's check flag, the core output stays byte-identical, and the module is off by default. But it is the project's own stated convention shipped broken, so a user (or this repo dogfooding the module) who enables it unedited gets a false gate. Medium stands.

### Issue A caveat - treefmt writes in place even in check mode (surfaced by A's fix; NOT in either findings file)

- Verdict: VALID observation, raised for routing.
- Severity: low-to-medium.
- Detail: treefmt v2 has NO non-mutating dry-run. `--fail-on-change` (and `--ci`) format the files ON DISK and THEN return non-zero if anything changed. The checks-reviewer prompt (`checks-reviewer.md`, the `kind = "format"` bullet) and the guidance (`checks-guidance.md`) both describe the format check as verifying formatting "without applying" / "in check / dry-run mode ... without applying," and the guidance leans on that to justify "read-only, so it needs no isolation." For treefmt that claim is not literally true: on an unformatted tree the format check WRITES the formatted result as a side effect of review. Impact is contained (formatting is idempotent, the implementer would apply it anyway, and the reviewer's tree changes are normally discarded), so this is not a functional break, but the stated "non-mutating" / "no isolation needed" rationale is inaccurate for any in-place formatter.
- Recommendation: the implementer should, at minimum, correct the "without applying" wording (state that the format check may rewrite in place for formatters like treefmt that lack a true dry-run, and that this is why the reviewer's tree is treated as disposable). Whether the format check actually needs isolation after all is a design question better owned by the orchestrator and probably belongs with the 2c subcommand/hook work, not blocking 2b.
- Owner: implementer (doc-wording fix now) + orchestrator (isolation-design question, deferable to 2c).
- Round needed: the wording fix rides the planned round; the design question is an Open Question, not a blocker.

---

## Issue B - seeded ast-grep rule is `severity: warning`, so it exits 0 on a hit (opus L2)

- Verdict: VALID.
- Severity: low.
- Confirmed: `no-dbg-macro.yml:7` is `severity: warning`; ast-grep exits 0 when it reports a violation at `warning` and exits 1 at `error`. Not a 2b functional break: the checks-reviewer parses tool OUTPUT ("turn each reported violation into a finding"), so the `dbg!` hit is still caught.
- Is this 2b, 2c, or a note? It is a genuine latent gap that becomes LIVE in 2c: the plan (increment 2, the hook path) says the `agent-scaffold checks` subcommand "runs the lint / format-check commands" and the pre-commit hook fails "on any violation" - both are exit-code consumers. A seeded example rule that exits 0 on its own violation will silently NOT gate under 2c. Because the rule ships NOW as the out-of-box example and `severity: error` costs nothing, breaks no byte-identity (module off by default), and makes the example both self-consistent (a reported finding then carries a non-zero exit, matching the prompt's "report its exit code" evidence line) and forward-compatible with 2c, I recommend fixing it NOW rather than deferring.
- Fix: set the seeded rule to `severity: error` (or, if the implementer prefers to defer, add a one-line note that lint findings surface via output rather than exit code and defer the change to 2c; fixing now is cleaner).
- Owner: implementer.
- Round needed: rides the planned round.

---

## Issue C - guidance's example findings-file name `<step>-checks.md` omits the disambiguator (opus L3; sonnet non-finding)

- Verdict: VALID but borderline/cosmetic.
- Severity: low.
- Confirmed: `pack/AGENTS.md` (and the plan's Findings-files section) fix the reviewer convention as `<step>-<role>-<disambiguator>.md`; `checks-guidance.md:` assigns `<step>-checks.md`. With role=`checks` and a single deterministic reviewer there is no collision, so the disambiguator is legitimately empty; sonnet correctly judged this "acceptable ... slightly irregular." Harmless in practice: the orchestrator assigns the exact path and the prompt defers to "the findings-file path the orchestrator assigned you."
- Fix (optional, doc only): either write it as `<step>-checks-<disambiguator>.md` or add a half-clause noting the single deterministic reviewer needs no disambiguator. Do NOT treat as a defect that must block.
- Owner: implementer (doc).
- Round needed: fold into the planned round if touching the guidance; not worth a dedicated round.

---

## Issue D - no copyable non-ast-grep / language-linter example row (sonnet F1)

- Verdict: VALID.
- Severity: medium as raised (soft medium; see below). Impact if unfixed: a polyglot user must synthesize the `[[check]]` shape for ruff/eslint from the schema table alone rather than copy an example, undercutting the increment's headline multi-language plug-in story.
- Mitigation already present: the `checks.toml` header prose DOES name "ruff / eslint / clippy / etc." and says to add a row "in the same shape," and the commented test/mutation rows show the row syntax. So the gap is specifically the absence of a COPYABLE non-ast-grep example row, not a total silence.
- Effect of the planned rg row: it delivers a concrete, copyable NON-ast-grep row, which removes the core "no non-ast-grep example exists" complaint. RESIDUE: the rg row is a grep-family tool, not a language-specific linter, so a polyglot user still does not see the "adding ruff is no different from ast-grep" demonstration that F1 (and the dropped exploration block) specifically wanted. Recommend closing the residue cheaply by making the added example (or a second commented row) a language linter, e.g. a commented `ruff` row with `kind = "lint"`, `command = "ruff check ."`, `paths = ["**/*.py"]` - which ALSO closes issue G (F4) in one place.
- Owner: implementer.
- Round needed: the planned round; verify the rg row lands and consider adding the ruff/paths row.

---

## Issue E - guidance names "ASCII-clean" as a generalised convention the seed does not represent or explain (sonnet F2 + opus L4, MERGED)

- Verdict: VALID.
- Severity: medium (sonnet F2). Impact if unfixed: `checks-guidance.md:3` advertises the module as generalising "clippy / `nix fmt` / ASCII-clean," but the seed contains only a Rust-specific `dbg!` ast-grep rule; a reader who enables the module expecting the ASCII-clean answer finds a `dbg!` rule and may wrongly assume ast-grep covers ASCII-clean (it does not, and cannot soundly match arbitrary non-ASCII bytes), then run it on a mixed project and get a false all-clear. opus rated the intro-list half of this low; the mismatch is real either way.
- Effect of the planned rg row: this is exactly sonnet's preferred fix (option a): a commented `rg '[^\x00-\x7F]'` row gives ASCII-clean a concrete referent AND shows it is a grep check, not an ast-grep rule. This FULLY resolves E PROVIDED the row (or an adjacent line) carries the short "why this is a grep check, not an ast-grep rule" note - i.e. that non-ASCII matching is not practical in ast-grep, which is the reason the seeded rule is `no-dbg-macro`. RESIDUE if that note is omitted: the reader gets the example but not the reason the seeded ast-grep rule is unrelated to the advertised ASCII-clean convention. Ensure the note ships.
- Owner: implementer.
- Round needed: the planned round; verify the rg row plus the one-line rationale.

---

## Issue F - seeded ast-grep rule not flagged as Rust-only in its comment (sonnet F3)

- Verdict: VALID.
- Severity: low.
- Detail: `no-dbg-macro.yml` prose comment describes the rule format but never says it targets Rust; `language: rust` is present as a field but a non-Rust/mixed-project user who runs `ast-grep scan` and sees zero findings cannot tell "misconfigured" from "no Rust source to match." The `checks.toml` "replace this row with your language's linter" note addresses the `[[check]]` row, not the rule file, and a user reading the YAML directly does not see it. One sentence in the rule comment ("this rule targets Rust; non-Rust or multi-language projects should replace or supplement it") resolves it. This is within the increment's deliberate multi-language-extensibility quality goal, so worth doing.
- Owner: implementer.
- Round needed: fold into the planned round.

---

## Issue G - `paths` field's language-scoping use is undocumented (sonnet F4)

- Verdict: VALID.
- Severity: low.
- Detail: `checks.toml` documents `paths` as "(optional) globs scoping the check" and the commented rows show the syntax, but never connects `paths` to polyglot scoping (`paths = ["**/*.py"]` to restrict ruff to Python, `["**/*.ts"]` for eslint). Best closed together with D: a single commented `ruff`-with-`paths` example row demonstrates both the plug-in story (D) and the scoping mechanism (G) in one place.
- Owner: implementer.
- Round needed: fold into the planned round (ideally as one row shared with D).

---

## Issue H - checks-reviewer prompt is silent on the `paths` field (sonnet F5)

- Verdict: VALID.
- Severity: low.
- Detail: `checks-reviewer.md` lint bullet says "run its `command`" with no mention of `paths`. Not a live problem in 2b (the only lint row is ast-grep, which self-scopes via its own `sgconfig.yml`). It becomes relevant the moment a `paths`-scoped row exists - and the planned fix round may add exactly that (D/G's ruff example), which raises H's pertinence. The `paths` field IS in the `checks.toml` schema the reviewer is told to read, so a careful agent could honour it, but one explicit sentence ("if a check declares `paths` globs, scope the command to those paths when running it") removes the ambiguity.
- Owner: implementer.
- Round needed: fold into the planned round (especially if a paths-scoped example row is added).

---

## Non-findings confirmed (no verdict required)

- CHANGELOG: neither reviewer raised it as a finding; both correctly recorded that increment 2 takes a single deferred CHANGELOG entry at increment-2 close (orchestrator-owned). Nothing to adjudicate. No implementer round.
- Byte-identity, asset ownership (config/sgconfig/rule = Working, checks-reviewer = Reference, count +4), ast-grep runnability, `ruleDirs` resolution, the rule-swap rationale, clippy `-D warnings` clean, and 126 tests passing are all confirmed by opus end-to-end and by sonnet's content read. No CRITICAL and no HIGH findings; I concur.

---

## Disposition summary

- One consolidated IMPLEMENTER fix round resolves the batch and spawns one more review round on the revised artifact:
  - A: `check = "nix fmt -- --fail-on-change"` (medium; the one materially-broken behaviour).
  - A caveat: correct the "without applying" wording for in-place formatters; the isolation-design question is deferable to 2c/orchestrator.
  - B: `severity: error` on the seeded rule (low; recommended now for 2c forward-compatibility, defer permissible).
  - D + G + E: land the planned commented `rg` ASCII row WITH its "grep check, not ast-grep rule" note, and ideally a commented `ruff`+`paths` language-linter row (closes D's polyglot residue and G together).
  - C, F, H: one-line doc/comment additions.
- No finding is INVALID; nothing is HIGH or CRITICAL. The batch is low-risk documentation/seed polish plus the single medium format-flag fix.
