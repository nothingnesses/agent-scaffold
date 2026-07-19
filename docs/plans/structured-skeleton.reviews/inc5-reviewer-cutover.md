# Inc 5 reviewer: cutover mechanics, reversibility, formatter stability, migrate removal, design

Increment `707532a..cbc454c` (branch `impl/structured-skeleton-inc5`, 4 commits: prep `5b45506`, fix `9c247e9`, cutover `6700c44`, migrate-removal `cbc454c`). Reviewed read-only from the worktree at `.claude/worktrees/structured-skeleton-inc5` (HEAD `cbc454c`) and via `git show`/`git diff` from the main repo.

Verdict: CLEAN. No high, critical, medium, or low findings. All five verification areas pass. One informational observation (OBS-1) is noted; it is not a defect.

Worktree is at `cbc454c`, clean (confirmed at the end of the revert experiment). Evidence follows per verification item.

---

## 1. Generated .md == render output: PASS

```
cargo run -- render --check docs/plans/agent-scaffold.plan.toml
docs/plans/agent-scaffold.plan.toml: up to date
```

Exit code 0. The committed `docs/plans/agent-scaffold.md` is byte-identical to a fresh render of `agent-scaffold.plan.toml` plus its Markdown sidecars. No hand-edit slipped in.

---

## 2. Formatter stability: PASS

`nix fmt` in the worktree reports:

```
traversed 251 files
emitted 224 files for processing
formatted 3 files (0 changed) in 82ms
```

Zero changes. `git status` after is clean (no working-tree modifications). `render --check` re-run after confirms still up to date.

How the stability is enforced:

- `docs/plans/agent-scaffold.md` is listed in `flake.nix` `settings.global.excludes` (alongside the render fixtures), so prettier cannot reflow it. (`flake.nix` lines 47-50, added in `6700c44`.)
- The step sidecars (`agent-scaffold.steps/*.md`), the meta sidecars (`.motivations.md`, `._status-narrative.md`, etc.), and the front/tail list in `[meta.sidecars]` are `.md` files that prettier DOES touch, but they are already prettier-stable (no trailing-whitespace or prose-wrap change was needed), evidenced by the 0-changed result.
- The `agent-scaffold.plan.toml` is handled by taplo, which also made no changes.
- The 46 question sidecar files (`agent-scaffold.questions/Q-*.md`) are all 0 bytes; prettier is a no-op on empty files.

Because `nix fmt` makes no changes, the sidecars and the TOML are formatter-stable, and `render --check` survives `nix fmt` unchanged.

---

## 3. Reversibility: PASS

Procedure executed in the worktree:

```
git revert --no-edit 6700c44
```

Result: `e15c5f8` "Revert feat: cut this repo's plan over to the TOML source (structured-skeleton Inc 5)", 5 files changed (the 5 files that `6700c44` touched: `.agents/checks.toml` deleted, `docs/metrics/workflow.jsonl` restored to 128 lines, `docs/plans/agent-scaffold.md` restored to the hand-authored version, `docs/plans/agent-scaffold.plan.toml` flipped back to `primary = "markdown"`, `flake.nix` reverted to remove the new exclude entry).

Acceptance check:

```
cargo run -- validate --metrics docs/metrics/workflow.jsonl --plan docs/plans/agent-scaffold.md --workflow
docs/metrics/workflow.jsonl: 128 records, valid
docs/plans/agent-scaffold.md: 51 steps, 46 open-questions items, valid
docs/plans/agent-scaffold.md vs docs/metrics/workflow.jsonl: workflow invariants hold
```

Exit 0. The Markdown path is fully restored: `primary = "markdown"`, 128-line JSONL (with the 17 pruned records re-present), hand-authored `.md` passes `validate_plan` and the workflow invariants.

Cleanup:

```
git reset --hard HEAD~1
```

Result: `HEAD is now at cbc454c`. `git status`: `nothing to commit, working tree clean`.

The worktree is at `cbc454c`, clean.

---

## 4. Migrate removal: PASS

Checks performed:

- `src/plan/migrate.rs`: absent (`ls src/plan/` shows only `render.rs`, `source.rs`, `testdata`).
- `src/plan.rs`: `migrate` `mod` and re-export removed (grep returns nothing).
- `src/main.rs`: `MigrateArgs`, `run_migrate`, and all `migrate`-subcommand wiring absent (grep returns nothing).
- `cargo run -- --help`: `migrate` does not appear in the subcommand list.
- `cargo clippy --all-targets -- -D warnings`: exit 0, no warnings.
- `cargo test --all-targets` summary:
  - lib/unit tests binary: 293 passed
  - `checks_staged_hook_env` integration test: 1 passed
  - `scaffold_precommit_hook` integration test: 3 passed
  - `validate_toml_primary_skips_markdown_plan` integration test: 1 passed
  - Total: 298 passed, 0 failed

298 = 301 - 3 matches the expected count (3 migrate unit tests removed from `src/plan/migrate.rs`, no other coverage dropped). The only remaining reference to "migrated" in the source is a doc comment in `src/plan/source.rs` line 107 describing the `w4_baseline` field ("Absent for a fresh project"), which is correctly retained content, not dead code.

---

## 5. Checks wiring: PASS

`.agents/checks.toml`:

```toml
[[check]]
name = "render-check"
kind = "lint"
command = "agent-scaffold render --check docs/plans/agent-scaffold.plan.toml --strict"
```

The entry is well-formed for the checks module schema:
- `name`: unique (only entry in the file).
- `kind = "lint"`: correct. `render --check` is read-only (writes nothing), so `lint` is the right kind. A `format` kind would invoke a `check` sub-command, not appropriate here.
- `command`: the `--strict` flag causes `render --check` to exit non-zero on any mismatch, which is the required behavior for a CI gate.

CI half: no `.github/` directory exists in the repo. The CI half of the "wire into `.agents/checks.toml` + CI" contract is legitimately not wired. The checks.toml comment (lines 10-13) explicitly acknowledges this: "This is a narrow, hand-curated checks file, not full checks-module self-adoption (that dogfood, an active ripgrep ASCII row and the rest, is tracked separately); it declares only the render gate for now."

---

## 6. Design soundness: PASS

**Skipping `validate_plan` in TOML-primary mode.** The fix in `9c247e9` is coherent. In TOML-primary mode, `docs/plans/agent-scaffold.md` is a generated projection: its correctness is already guaranteed by `validate --source` (TOML schema + cross-refs) and by `render --check` (the committed `.md` equals a fresh render). Running the hand-authored-Markdown validator against the projection would be category-confused and would false-fail on rendered forms the Markdown vocabulary cannot express (e.g., `(superseded by Q-x)` is a rendered status the standalone validator rejects). The skip is scoped precisely: `plan_is_projection` is true only when the `--source` parses as TOML-primary (`[meta].primary = "toml"`); in Markdown mode (no `--source`, Markdown-primary, or unparseable source) `validate_plan` runs unchanged. The integration test `toml_primary_skips_the_markdown_plan_validator_but_markdown_mode_still_fails` pins both directions against a fixture that would fail the Markdown validator.

**Migrated repo coherence.** All three source-primary read paths are green:

- `validate --source docs/plans/agent-scaffold.plan.toml`: 111 records valid, 51 steps, 46 questions valid. Exit 0.
- `status --source docs/plans/agent-scaffold.plan.toml`: 51 steps (38 complete, 4 deferred, 1 in-progress, 3 not-started, 3 optional, 2 skipped), 46 open-questions items, 111 records. Exit 0.
- `status --json --source docs/plans/agent-scaffold.plan.toml`: JSON projection emitted, `"records": 111`. Exit 0.
- `validate --source docs/plans/agent-scaffold.plan.toml --plan docs/plans/agent-scaffold.md --workflow`: source valid, `.md` noted as "generated projection of a TOML-primary source; skipping the Markdown plan validator", workflow invariants hold. Exit 0.

Note: `--plan` is still required when `--workflow` is given, even in TOML-primary mode. The help text and a code comment ("the relaxation for a TOML-only project is deferred") document this as a known deferred simplification. For this repo it is non-blocking because the generated `.md` always exists.

**W5 record-backed waivers across the substrate split.** The two record-backed waivers (`optional-modules-inc2cii`, `evidence_tier = "record-backed"`) and (`waiver-model`, `evidence_tier = "record-backed"`) have their `evidence` fields joining against retained JSONL escalation records:

```
{"type":"escalation","task":"optional-modules-inc2cii","human_decision":"decision",...}
{"type":"escalation","task":"waiver-model","human_decision":"decision",...}
```

Both escalation records are present in the pruned JSONL. W5's cross-substrate join therefore holds. `validate --source` passes cleanly (exit 0, no W5 problems reported).

**Decision receipts Q-45 and Q-46.** Both are present in the JSONL as well-formed `type:"decision"` records with `chosen` in `options`. Both are past the `w4_baseline = "Q-44"` threshold, so W4 enforces them and the dogfood is live. `validate --source` confirms clean (W4 passes for both).

**Editorial decisions.** The 46 questions all use inline `ask = "..."` fields; no question requires a question sidecar for its primary prose. The 46 question sidecar files (`agent-scaffold.questions/Q-*.md`) are all 0 bytes (placeholder slots), which the render handles by omitting the "Question Details" section entirely (`question_details_section` returns `None` when all sidecars are empty, confirmed in `src/plan/render.rs` lines 537-548). The rendered `.md` contains no bare "Question Details" heading. Multi-fold secondary targets are preserved in each folded question's inline `ask` prose (e.g., Q-40 folded into `waiver-model` retains the full deliberation record). The `structured-skeleton` step's increment list is sparse (the `increment = []` field is empty at the step level; each of the six Inc builds are tracked via the reviews directory and commit history, not as TOML increment entries), which matches the agreed sparse-increments editorial decision.

---

## Observations (not findings)

**OBS-1: `render-check` command name requires `agent-scaffold` on PATH at `checks` run time.**

The `checks` module executes commands via `sh -c` in an isolated worktree. The `render-check` command uses `agent-scaffold` by bare name, so the binary must be on PATH when `cargo run -- checks` (or the installed binary `agent-scaffold checks`) is invoked. This is a known property of how the checks module works for any command (all commands need their tools on PATH), and the binary is available on PATH in the Nix dev environment this repo uses. Not a defect; noted for awareness.

---

## Worktree state confirmation

After all experiments, the worktree is at `cbc454c` (HEAD `cbc454c refactor: remove the one-time migrate subcommand after the cutover`), working tree clean. The revert commit `e15c5f8` was dropped by `git reset --hard HEAD~1` before writing this report.
