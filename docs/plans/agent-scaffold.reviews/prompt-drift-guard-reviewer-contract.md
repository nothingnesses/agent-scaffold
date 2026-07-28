# `prompt-drift-guard` work review: reviewer (contract, scope, code quality)

Artifact: `git diff 852a8c4..8012e05`, a single-file change to `src/agents_md_drift.rs` (+121 / -18).
Brief: `docs/plans/agent-scaffold.steps/prompt-drift-guard.md`.
Lens: contract fidelity, scope discipline, code quality. False-negative hunting in the detection itself was another reviewer's lens and is not duplicated here.
Worktree: `.claude/worktrees/rev-pdg-contract`, detached at `8012e05`. All commands below were run there.

## Verdict

ONE finding, severity `low` (`CT-1`). No `medium`, `high`, or `critical` finding. All four contract requirements MET, scope discipline held, the `checks-reviewer.md` exclusion mechanism is sound, `cargo test` and `cargo clippy --all-targets -- -D warnings` both clean.

## Findings

### `CT-1` (low): the residual-gap note presents an incomplete list as the complete complement, omitting the 12 committed `docs/plans/TEMPLATE.*` assets

`src/agents_md_drift.rs:71-77`, the `PROMPT_DEST_PREFIX` doc comment, reads:

> Narrower than the full set of copied assets on purpose: the other copied assets the self-scaffold emits (`.agents/user-prompts/*`, `.agents/LEDGER.template.md`, `.agents/principles.toml`, `.agents/workflow.toml`) carry the same gap and are not covered here, which is a scope call rather than an oversight.

"the other copied assets the self-scaffold emits (LIST)" reads as the exhaustive complement of the guarded set. It is not. The self-scaffold also emits 12 manifest assets under `docs/plans/TEMPLATE` (10 content-bearing plus 2 `.gitkeep`), all committed, all copied verbatim, and none guarded by any test. They carry exactly the gap the brief describes: an edit to a `pack/plan-template.*` source that is never regenerated ships a stale committed plan template with every check green. (A 13th committed file, `docs/plans/TEMPLATE.md`, is the generated view rather than a manifest asset per `pack/pack.toml:36-37`, so it could never enter a derived set and is not counted here.)

This matters because the brief made the residual an explicit human decision (`prompt-drift-guard.md:21`: do not widen "without a human call ... note it in the step's report so the human can decide whether a follow-up is worth it"). The note the human would read understates the residual by the largest omitted group, the plan template this project's whole planning workflow is seeded from.

The brief itself carries the same omission (`prompt-drift-guard.md:21` lists the same four), so the implementer reproduced its source faithfully. Raised anyway because a shipped code comment that states a false completeness is a defect regardless of provenance, and the reviewer's job is not to assume the requester's framing is correct.

Evidence, all re-runnable from the worktree root:

1. The comment: `src/agents_md_drift.rs:71-77`.
2. The omitted assets are manifest assets, copied not rendered (no `render = true` on any of them): `pack/pack.toml:38-96`, twelve `[[asset]]` blocks with dests `docs/plans/TEMPLATE.plan.toml`, `TEMPLATE._status-narrative.md`, `TEMPLATE.motivations.md`, `TEMPLATE.principles-note.md`, `TEMPLATE.documentation-protocol.md`, `TEMPLATE.repo-layout.md`, `TEMPLATE.queue-intro.md`, `TEMPLATE.roadmap-intro.md`, `TEMPLATE.success-criteria.md`, `TEMPLATE.steps/example-step.md`, `TEMPLATE.steps/.gitkeep`, `TEMPLATE.questions/.gitkeep`. Pinned in the expected-asset list at `src/manifest.rs:591-602`.
3. They are committed: `git ls-files docs/plans/TEMPLATE*` returns all 12.
4. They are byte-identical to the raw render, i.e. in exactly the same "committed copy must equal the render" class as the guarded files:

```
cargo run --quiet -- scaffold --output-dir /tmp/rd --write --force --principles default --instrument
(cd /tmp/rd && find . -type f -not -path './.git/*' | sed 's|^\./||' | sort) \
  | while read f; do diff -q "/tmp/rd/$f" "$f" >/dev/null && echo "SAME $f" || echo "DIFFERS $f"; done
```

Every one of the 31 emitted files reports `SAME`, including all 12 `docs/plans/TEMPLATE.*`.

5. They are unguarded: `grep -rn "TEMPLATE\." --include=*.rs src/ | grep -v "testdata\|render-fixture"` returns only the manifest's expected-dest list (`src/manifest.rs:591-602`). No `include_str!` and no comparison against a render exists for any of them (cross-check: `grep -n "include_str!" src/*.rs src/plan/*.rs` shows the only pack-side embeds are `pack/principles.toml`, `pack/workflow.toml`, and `pack/instrument.md`, all of which pin the PACK SOURCE against Rust constants, never a deployed copy).

Strongest counter-argument, stated fully so the triager can weigh it. The `docs/plans/TEMPLATE.*` assets are `ownership = "working"` while all four the comment lists are `ownership = "reference"`, and `pack/pack.toml:32-37` describes them as "create-if-absent working files (a project curates its own plan), copied verbatim", so one could read the comment as meaning "the other copied TOOL-OWNED assets", under which its list IS complete. Why I still raise it: (a) the comment says "copied assets", with no ownership qualifier, and the set it contrasts against includes `AGENTS.md`, itself `ownership = "working"` (`pack/pack.toml:27-31`), so ownership demonstrably does not define the guarded class in this repo; (b) "a project curates its own plan" is about a CONSUMING project, whereas this guard exists for this repo's own dogfooding, and here all 12 are byte-identical to the render and `just scaffold-self` overwrites them with `--force`, so they are in the identical must-match-the-render class as the guarded files; (c) the practical consequence is unaffected either way, since a `pack/plan-template.*` edit without regeneration ships stale here and the human deciding on widening is not told. If the triager prefers the tool-owned reading, the fix reduces to adding the qualifier the comment currently lacks.

Suggested fix (comment only, no behaviour change): add `docs/plans/TEMPLATE.*` to the parenthetical, or reword to "for example" so the list is not read as exhaustive.

## Contract requirements, one by one

Requirement text is from `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:9-12`.

### 1. A pack edit without regeneration must fail `cargo test`, with a message naming the file and telling the reader to regenerate: MET

Demonstrated. Edited `pack/prompts/triager.md:3` ("You adjudicate review findings." -> "You adjudicate the review findings.") with the deployed copy left stale, then:

```
cargo test --bin agent-scaffold the_committed_role_prompts_match_a_fresh_render
```

```
test agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render ... FAILED
thread '...' panicked at src/agents_md_drift.rs:417:13:
assertion `left == right` failed: .agents/prompts/triager.md has drifted from a fresh render of the pack's prompts (ignoring prettier wrapping): either its `pack/prompts/` source was edited without regenerating, or the committed copy was hand edited. Edit the pack source, not the copy, then run `just scaffold-self`
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 366 filtered out
```

The message names the file (`.agents/prompts/triager.md`), names both possible causes, and gives the fix (`just scaffold-self`). Reverted with the Edit tool.

Note on how to run this: the module is declared in `src/main.rs:12`, not in a lib target, so the tests live in the bin test binary. `cargo test --lib <name>` matches nothing; use `cargo test --bin agent-scaffold <name>` or plain `cargo test`.

### 2. A hand edit to a committed copy must ALSO fail (two-way correspondence): MET

Demonstrated separately. Edited `.agents/prompts/open-questions-gate.md:9` ("the next steps." -> "the next step.") with the pack left alone:

```
assertion `left == right` failed: .agents/prompts/open-questions-gate.md has drifted from a fresh render of the pack's prompts (ignoring prettier wrapping): either its `pack/prompts/` source was edited without regenerating, or the committed copy was hand edited. Edit the pack source, not the copy, then run `just scaffold-self`
```

The check is structurally two-way (a fresh render compared against bytes read at test time), not a timestamp or staleness heuristic, so neither direction can pass. Reverted with the Edit tool.

### 3. An incidental prettier reflow must NOT fail: MET

Demonstrated. Replaced the same line of `.agents/prompts/open-questions-gate.md` with a soft-wrapped form ("Otherwise, state that none are open and proceed with\nthe next steps.", a 1-line to 2-line reflow, `git diff --stat` = 2 insertions / 1 deletion, no content change):

```
test agents_md_drift::tests::the_committed_role_prompts_match_a_fresh_render ... ok
```

Reverted with the Edit tool. The normalisation path used is the existing `normalize_wrapping` plus `assert_no_unprotected_construct` on BOTH sides (`src/agents_md_drift.rs:414-415`), exactly the reuse the brief required at `prompt-drift-guard.md:16-17`. The precondition assertion is live on the prompts, not skipped: all 7 committed prompts and their renders pass it today, so no prompt currently carries a nested list, indented code, or a multi-space inline span.

### 4. The failure must be demonstrable: MET

Re-demonstrated independently above (requirements 1 and 2), rather than taken from the implementer's report. Both directions produce a real failure with the actionable message.

## The decided implementation choice (derived, not enumerated)

DERIVED, as decided. `src/agents_md_drift.rs:393-396` renders the self-scaffold asset set once and filters on `asset.dest.starts_with(PROMPT_DEST_PREFIX)`; nothing enumerates prompt paths. Adding a prompt to `pack/prompts/` plus a `[[asset]]` block in `pack/pack.toml` puts it under the guard with no edit to `agents_md_drift.rs`. There are no new `include_str!` constants in the diff.

The vacuity fail-safe at `src/agents_md_drift.rs:401-404` (assert the filter matched at least one asset) is a real addition over the brief's letter and is the right instinct: a derived set that silently empties reads as coverage while providing none. It is inside the test being added, so it is not scope expansion.

Refactor behaviour-preservation for the two pre-existing files: `self_scaffold_asset` (`src/agents_md_drift.rs:97-103`) is now `self_scaffold_assets()` plus the identical `.into_iter().find(|a| a.dest == dest)` lookup, the identical `unwrap_or_else(|| panic!("the self-scaffold render includes an asset at {dest}"))`, and the identical `.contents`. The config moved verbatim: same `manifest::builtin()`, same `"default"` selection, same `pack::Detail::Summary`, same empty vars, same `true` for instrument, same empty module slice. Verified behaviourally, not just by reading: with `.agents/AGENTS.reference.md:3` mutated (dropped word),

```
test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... FAILED
assertion `left == right` failed: .agents/AGENTS.reference.md has drifted from a fresh pack render (ignoring prettier wrapping); run `just scaffold-self`
```

so the two original `include_str!` comparisons still trip after the refactor. Reverted with the Edit tool.

Independent check that the test's pinned config really matches the justfile's CLI invocation (`justfile:47`): rendering through the actual CLI with `--principles default --instrument` produces output byte-identical to all 31 committed copies (the `SAME` sweep in `CT-1` evidence 4). If the config had diverged, the committed files would not match.

## Scope discipline: HELD

- `git diff 852a8c4..8012e05 --stat` reports exactly one file changed, `src/agents_md_drift.rs`. Nothing else in the tree was touched: no pack edit, no plan edit, no justfile edit, no test outside this module.
- No widening. `PROMPT_DEST_PREFIX = ".agents/prompts/"` (`src/agents_md_drift.rs:77`) is the only filter. `.agents/user-prompts/*`, `.agents/LEDGER.template.md`, `.agents/principles.toml`, and `.agents/workflow.toml` are NOT matched by that prefix (`.agents/user-prompts/` does not start with `.agents/prompts/`) and are not guarded by the new test. Confirmed behaviourally by the passing run: the guard covers 7 destinations, and the committed `.agents/user-prompts/*` files remain unguarded.
- The residual IS noted in the code (`src/agents_md_drift.rs:71-77`) rather than silently widened, which is the "report, do not do" the brief asked for; `CT-1` is that the note's list is incomplete, not that the note is missing.
- No CHANGELOG entry, no doc edits, no unrelated refactor. The only incidental change is the `rustfmt` reflow of four pre-existing over-long lines in the same file, which is accepted and not raised.

## The `checks-reviewer.md` exclusion, and whether the mechanism is sound: HONOURED, and sound

Mechanism: implicit, not an explicit skip-list. `pack/pack.toml` tags the asset `module = "checks"`; `self_scaffold_assets` passes `&[]` for modules (`src/agents_md_drift.rs:92`), so the render never emits it and the derived filter never sees it. Documented at `src/agents_md_drift.rs:44-49`.

Verified directly rather than argued:

```
cargo run -- scaffold --output-dir /tmp/rd        --write --force --principles default --instrument
cargo run -- scaffold --output-dir /tmp/rd-checks --write --force --principles default --instrument --module checks
ls /tmp/rd/.agents/prompts/         -> 7 files, no checks-reviewer.md
ls /tmp/rd-checks/.agents/prompts/  -> 8 files, checks-reviewer.md present
```

and the repo commits no copy (`ls .agents/prompts/` returns the same 7; `git ls-files .agents/` has no `checks-reviewer.md`), so an enumerated guard expecting it would indeed fail on a correct tree, as the brief warned.

Soundness of the implicit form. The obvious objection to an implicit exclusion is that it can drift out of intent silently. It does not here, because the same pinned config feeds the pre-existing `AGENTS.md` comparison, and the `checks` module changes `AGENTS.md`:

```
diff -q /tmp/rd/AGENTS.md /tmp/rd-checks/AGENTS.md   ->  files differ
```

So if `justfile:47` ever gained `--module checks` without `self_scaffold_assets` being updated to match, `the_committed_scaffold_matches_a_fresh_render` fails loudly first; once the test config is corrected, `checks-reviewer.md` enters the derived set automatically and correctly, because it would then genuinely be a deployed committed prompt. The exclusion is therefore self-correcting rather than a latent skip-list, which is the better property. The residual hole is confined to a hypothetical future asset-dropping module that changes no guidance text, which does not exist today (`src/manifest.rs:695`, `builtin_isolation_module_renders_its_guidance_only_when_selected`, pins that the one guidance-only module drops zero assets).

## Code quality

- ATTRIBUTES: none added. `grep -n "#\[allow\|#\[expect" src/agents_md_drift.rs` returns nothing, and the diff adds no attribute of either kind, so the `expect`-over-`allow` convention does not arise. The only `expect(...)` occurrences in the diff are pre-existing `Result::expect` calls carried across the refactor unchanged.
- COMMENTS: WHY-focused and at the surrounding density. The module doc explains why derived beats enumerated and names the hermeticity trade-off taken (`:32-42`); `committed_asset` explains why `include_str!` cannot serve and why a missing file panics rather than skips (`:105-109`); the test's opening comment states what the gap was and why the check is two-way (`:381-392`); the vacuity assertion explains why an empty derived set is worse than no guard (`:398-400`). None restates the code.
- MESSAGES: actionable. The equality failure names the file, both causes, and the fix. The `committed_asset` panic names the path, the underlying IO error, and the fix. The vacuity assertion names the constant to repoint. All three tell a developer what to do next.
- DUPLICATION / one source of truth: the render config IS duplicated between `src/agents_md_drift.rs:87-94` and `justfile:47`. Assessment: real duplication, but PRE-EXISTING and unchanged by this diff (the same config and the same "pinned here to match the justfile recipe" comment are present at the base commit `852a8c4`), and it is self-guarding, since any config divergence that changes the guidance makes the `AGENTS.md` comparison fail (demonstrated above with `--module checks`). Single-sourcing it would mean exposing the self-scaffold config from library code and having both the CLI recipe and the test consume it, which is a real option but a separate refactor the brief did not scope. NOT raised as a finding: not introduced here, documented in place, and detectable when it drifts.
- NAMING: `self_scaffold_assets` vs `self_scaffold_asset` differ by one character. Mildly easy to misread, but the plural/singular pair is the conventional Rust naming for exactly this relationship and the doc comments disambiguate. Not a finding.
- DEAD CODE / ALLOCATION: no dead code (`self_scaffold_asset` still has two call sites at `:347-348`). `self_scaffold_assets()` re-renders the pack per call (3 calls across the two tests), same as the pre-change helper did per call; the full bin test binary runs 367 tests in 0.13s, so this is not worth a `OnceLock`.
- FAILURE DIAGNOSIS: considered and NOT raised. `assert_eq!` on whole files prints both normalized sides in full, which for `.agents/prompts/orchestrator.md` (13.4 KB) is a wall. Mitigating and decisive: the custom message is printed FIRST, before the two dumps (see the requirement-1 output above), the loop stops at the first mismatching file so only one wall appears, and this is exactly the established behaviour of the pre-existing `AGENTS.md` comparison over a 58 KB file, which the brief told the implementer to reuse rather than reinvent.
- ASCII: `grep -nP "[^\x00-\x7F]" src/agents_md_drift.rs` returns nothing.

## Does it break anything else

- `cargo test`: 367 + 5 + 1 + 3 + 1 + 2 = 379 passed, 0 failed, 0 ignored, across all six test binaries.
- `cargo clippy --all-targets -- -D warnings`: clean, `Finished dev profile`, no diagnostics.
- No other test depended on `self_scaffold_asset`. `grep -rn "self_scaffold_asset\|PROMPT_DEST_PREFIX\|committed_asset" --include=*.rs .` returns hits only inside `src/agents_md_drift.rs`; the helper is a private `#[cfg(test)]` fn in a module declared only at `src/main.rs:12`.

## Honesty of the reported result (CHANGELOG convention)

VERIFIED, claim reproduces. `git log --format='%h %s' -- CHANGELOG.md` returns 14 commits, all `feat:`, `fix:`, `refactor:`, `style:`, or `docs:` folds of shipped behaviour. No commit with a `test:` prefix has ever touched `CHANGELOG.md`. The drift guard's own predecessor commits (`cba4fcc`, `d743ae1`, `ef3b80d`, `53a367e`, all `test:`) appear in none of them. Omitting a CHANGELOG entry for this test-only change is the convention, not a lapse. The brief's own instruction to "check what `decision-receipt` and `waiver-model` did" points at `294cc3c` and `9476ca8`, both `feat:` commits that DID add CHANGELOG lines, which is consistent: they changed shipped behaviour and this does not.

## Considered and deliberately not raised

- `H4-3` (orphan prompt removed from the pack leaves an unguarded committed copy): known accepted residual, owner step 92. Its recorded description at `docs/plans/agent-scaffold.ledger.md:353` is ACCURATE as written: the derived filter cannot see a dest the render no longer emits, so the committed file is neither compared nor deleted, and the enumerated form would have panicked on the missing `include_str!` target. Not re-raised.
- `R3-CQ-1` / step `drift-guard-test-hook-hygiene`: settled as valid-but-non-blocking and already scheduled, so not raised as a finding. One observation for the triager, offered only because this change slightly raises its stakes: the recorded description (`docs/plans/agent-scaffold.steps/drift-guard-test-hook-hygiene.md:3,5`) says the impact is "suppressing a concurrent or subsequent real panic's backtrace" and "the only impact is backtrace visibility", but a no-op panic hook suppresses the whole `thread ... panicked at ...: <message>` line, message included. The new test's entire actionable diagnosis lives in that message. This does not change the verdict (still diagnostic-only, still cannot flip a pass/fail, still low, still needs a real race inside a sub-millisecond window), so no action is requested; it is noted in case the fix's priority is ever re-weighed.
- The step sidecar `docs/plans/agent-scaffold.steps/prompt-drift-guard.md:3` still opens "Not started." while `docs/plans/agent-scaffold.md:304` has the step `in progress`. That inconsistency was introduced by the loop-open commit `852a8c4` (which touched only `agent-scaffold.md` and `agent-scaffold.plan.toml`), so it predates the artifact under review and is the orchestrator's narrative to update at convergence, not the implementer's deliverable. Not a finding against this change; noted only so it is not lost.
- The module doc's phrase "module-gated in `src/manifest.rs`" (`src/agents_md_drift.rs:45`) is loose, since the `module = "checks"` tag is declared in `pack/pack.toml` and `src/manifest.rs` implements the filter and pins the behaviour in tests. A reader following the pointer lands on `src/manifest.rs:650-660` (`builtin_checks_module_adds_its_five_assets`, whose `absent` list names `.agents/prompts/checks-reviewer.md`), which does document the gating, so the pointer is useful and not misleading. Too thin to raise.

## Tree state

Clean. Every mutation above was reverted with the Edit tool (never `git checkout`). One incident to declare in full: an early `touch src/lib.rs` used to force a clippy rebuild CREATED a new empty untracked file (the crate has no lib target; `Cargo.toml` declares only the bin). It was caught by `git status`, confirmed untracked and 0 bytes via `git ls-files src/` and `wc -c`, and removed. It also explains why `cargo test --lib <name>` matched 0 tests earlier in this review. `git status --short` at the end of the review returns empty output, with only this findings file left to be added.
