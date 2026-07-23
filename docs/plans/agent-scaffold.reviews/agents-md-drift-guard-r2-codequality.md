# Drift-guard round 2 review: code-quality / mechanical lens

Scope: the blocking fix that added `assert_no_unprotected_construct(name, content)`,
called on both committed files and both fresh renders, and rewrote the
`normalize_wrapping` safety doc-comment to state a precondition and list the
unprotected constructs. Reviewed `git diff cba4fcc HEAD -- src/agents_md_drift.rs`
against the full file `src/agents_md_drift.rs`.

## Evidence (build / test / lint / validators)

- `cargo test`: `test result: ok. 344 passed; 0 failed; 0 ignored` (bin unittests),
  plus all integration suites green (`checks_staged_hook_env` 1, `scaffold_precommit_hook` 3,
  `validate_toml_primary_skips_markdown_plan` 1, `validate_workflow_toml_source_needs_no_plan` 2).
- Guard + normalization unit tests, run by name:
  - `test agents_md_drift::tests::the_committed_scaffold_matches_a_fresh_render ... ok`
    (`test result: ok. 1 passed; 0 failed; ...; 343 filtered out`).
  - `test agents_md_drift::tests::normalization_tolerates_wrapping_but_not_content_change ... ok`
    (`test result: ok. 1 passed; 0 failed; ...; 343 filtered out`).
- `cargo clippy --all-targets`: `Finished dev profile ... in 4.65s`, no warnings.
- `cargo run --quiet -- validate --source docs/plans/agent-scaffold.plan.toml`:
  `docs/metrics/workflow.jsonl: 182 records, valid` /
  `docs/plans/agent-scaffold.plan.toml: 84 steps, 65 questions, valid`.
- `cargo run --quiet -- validate --workflow --source docs/plans/agent-scaffold.plan.toml`:
  `docs/plans/agent-scaffold.plan.toml vs docs/metrics/workflow.jsonl: workflow invariants hold`.
- `cargo run --quiet -- render --check docs/plans/agent-scaffold.plan.toml`:
  `docs/plans/agent-scaffold.plan.toml: up to date`.

(Command-form notes: the crate is a binary, so `--lib` is rejected; `validate --source`
takes a flag not a positional; `validate --workflow` requires `--source`; `render --check`
takes the plan as a positional. Corrected forms were used and all pass.)

## Findings

### F1 (Low, non-blocking): assert scans fenced-code lines that normalize_wrapping actually protects

- Location: `src/agents_md_drift.rs:86-101` (`assert_no_unprotected_construct`), read
  against `src/agents_md_drift.rs:217-231` (fence handling in `normalize_wrapping`) and
  the doc-comment at `src/agents_md_drift.rs:167-169, 198-208`.
- Problem: `assert_no_unprotected_construct` iterates `content.lines()` with no fenced-code
  tracking and rejects any line with leading space/tab or a two-space run, everywhere,
  including inside a fenced code block. But `normalize_wrapping` passes fenced lines through
  VERBATIM (`src/agents_md_drift.rs:222-231`), so indentation and multi-space runs INSIDE a
  fence are preserved, not discarded, and therefore cannot mask drift. The assert's panic
  message asserts the opposite of that reality: it says the construct is one
  "that normalize_wrapping does NOT protect ... treat indented code verbatim", yet fenced
  code is already treated verbatim. Consequence is one-directional and fail-safe (it can only
  cause a future FALSE failure, never a masked drift): the day the guidance gains a fenced
  code block containing an indented or multi-space line (common in AGENTS-style docs: shell
  or JSON snippets), both the committed file and the fresh render would trip the assert with
  a message telling the author to harden a path that already handles their case. The guarded
  files are entirely flat today (grep: zero fenced fences, zero indented lines in `AGENTS.md`
  and `.agents/AGENTS.reference.md`), so this is latent, not active.
- Fix: mirror `normalize_wrapping`'s fence state in the assert, toggling an `in_fence` flag on
  a ```` ``` ````/`~~~` delimiter and `continue`-ing on fenced lines so verbatim-protected
  content is not rejected; or, if the over-rejection is intended as an extra "review any new
  fence" tripwire, say so in the doc-comment and drop the "normalize_wrapping does NOT protect"
  wording from the message for that case. Not blocking: the direction is fail-safe and no such
  content exists today.

## Checks that passed clean

- Helper shape (`src/agents_md_drift.rs:86-101`): clear name; panic messages name the file
  (`{name}`), line (`{number}`), construct class, and the exact hardening required; no opaque
  `unwrap`; no allocation on the success path (`starts_with`/`contains` only; `format!` runs
  only on failure).
- Reused fresh-render locals: `rendered_agents` / `rendered_reference` are computed once
  (`src/agents_md_drift.rs:276-277`) and passed to both the asserts and the `assert_eq!`
  checks (`src/agents_md_drift.rs:285-298`), replacing the previous inline double render.
- Doc-comment accuracy (`src/agents_md_drift.rs:171-208`): the stated precondition (no line
  with leading indentation; no run of two or more spaces) matches exactly what the assertion
  enforces (`!starts_with(' '|'\t')`, `!contains("  ")`), and the three listed unprotected
  constructs match the real transform gaps (`trim_start`/`trim` strip leading indentation;
  `split_whitespace().join(" ")` collapses inter-word runs). The one imprecision is F1.
- House rules on the diff: hard TABS throughout the added code (grep `^ +\S` in the added
  region: no matches; added `fn`/`let`/call lines confirmed tab-led); no `#[allow]`/`#[expect]`
  added; no non-ASCII bytes anywhere in the file (grep `[^\x00-\x7F]`: none); no em/en-dashes
  or double-hyphen-as-dash (the `--` hits are all CLI flag references like `--var`, `--module`).
- No regression: existing tests and module structure unchanged; 344 + integration all pass.
