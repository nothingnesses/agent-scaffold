# state-schema increment 3, round 2 (confirming) review

Reviewer: opus. Scope: verify the six round-1 fixes (TRI-1..TRI-6) landed in `cae8da7..944facf`, that the hard-fail cases fail, the drift guard catches drift, the real plan still validates, and nothing new broke. Read-only; temporary drift/malformation edits were made in scratch and the repo and all reverted (tree clean, verified).

## Verdict: CLEAN. No new findings at any severity (no low, no medium, no high, no critical).

All six fixes are present and correct, all hard-fail cases exit non-zero with an accurate report and no panic, the drift guard catches drift in both directions and is anchored, the real plan still validates, and the suite/clippy/scaffold are green. This is a converging clean round for a LOW-risk artifact.

## Fix verification (each confirmed against the built binary, not just the diff)

Built plans in the scratchpad (not the repo) and ran `validate --plan <path>`:

- TRI-1(a) Roadmap with no delimiter row -> exit 1, "Roadmap section has no well-formed table (a header row then a `| --- | --- |` delimiter row)". Correct.
- TRI-1(b) garbled Roadmap data row -> exit 1, "Roadmap table row is malformed (expected a backticked slug and a status): ...". Correct.
- TRI-2 / TRI-1(c) `Q-5 (openfoo)` -> exit 1, "Open Questions item `Q-5` has an unknown status `openfoo`". Exact-match rejection confirmed; `question_status_ok` uses `QUEUE_EXACT_STATUSES.contains(&status)` for `open`/`superseded` and keeps `starts_with(QUEUE_FOLD_PREFIX)` for the parametric form (src/plan.rs:325-327). `supersededxyz` also rejected (unit test `terminal_queue_statuses_match_exactly`).
- TRI-1(d) `Q-5` with a broken status group (no parentheses) -> exit 1, "Open Questions item `Q-5` is missing a well-formed `(status)` group". Correct. `queue_structure_problems` mirrors `parse_questions`'s two drop conditions ('(' prefix present AND a ')' in the remainder), so any live `Q-<n>` the parser would drop is reported instead.
- TRI-3 / TRI-1(e) `decided -> folded into `nonexistent-slug``-> exit 1, "Open Questions item`Q-5`folds into`nonexistent-slug`, which is not a Roadmap step". Correct. The two other fold-into branches also verified: a bare prefix (empty target) is caught as an unknown status, and a fold target with no backticks is caught as "a fold-into status with no target slug".
- TRI-4 / TRI-1(f) `### `orphan-slug``detail with no Roadmap row -> exit 1, "Step Detail`orphan-slug` has no matching Roadmap row". Correct; the reverse cross-reference (src/plan.rs:373-377) complements the pre-existing forward one.
- TRI-5 drift guard `plan_template_documents_every_accepted_status` present (src/plan.rs:710-741), driven from `ROADMAP_STATUSES`, `ROADMAP_BLOCKED_PREFIX`, `QUEUE_EXACT_STATUSES`, `QUEUE_FOLD_PREFIX`; see the dedicated section below.
- TRI-6 README documents `validate` (default metrics log + `--plan` for the plan's Roadmap and Open Questions structure, exits non-zero on any violation) and `status` (best-effort projection, `--json`, never fails on missing/malformed input). Verified the `status`-never-fails claim: `status` on a malformed plan exits 0 and emits a partial projection.

No parser PANIC on any malformed input. Additional adversarial probes (multibyte/unicode cells and slugs, `|||` / `||` empty-cell rows, colon-only delimiter cells, a Roadmap section with prose but no pipe table, ragged rows with a missing status cell, an unbackticked slug row) all exit non-zero with a report or are correctly reported, and none panic. Byte-index slicing in `first_backtick` / `queue_structure_problems` is safe because every index is a backtick or `(`/`)` byte offset (all ASCII, on a char boundary).

## Critical regression check: PASS

`validate --plan docs/plans/agent-scaffold.md` still exits 0 ("36 steps, 24 open-questions items, valid"). The stricter parser did not over-reject the real plan.

## Drift guard: PASS, anchored in both directions

- Code -> template: renamed `ROADMAP_STATUSES` `complete` -> `complete-DRIFT`; `just test` failed with "Roadmap status `complete-DRIFT` accepted by the validator is not documented in pack/plan-template.md". Reverted; clean.
- Template -> code (the weak-substring concern that took the metrics guard three rounds): temporarily removed `skipped` from the template's `Statuses:` list; the guard failed naming `skipped`. Reverted; clean.
- Anchoring is real, not a weak substring: each anchored token (`not started,`, `in progress,`, `complete,`, `skipped,`, `next,`, `optional,`, `deferred,`, `blocked on <slug>`, `` `open` ``, `` `superseded` ``, `decided -> folded into`) occurs exactly once in `pack/plan-template.md`, so removing the corresponding status makes the anchor disappear and cannot be masked elsewhere. Roadmap statuses anchor on the trailing comma of the template's comma-list; queue statuses anchor on backticks; both catch a removed status.

## Coverage completeness

`validate_plan` now covers both cross-reference directions (Roadmap slug -> Step Detail, and Step Detail -> Roadmap row) and all three malformed-structure classes the triager named (missing/malformed delimiter row, unparseable data row, live `Q-<n>` with a broken status group), plus the fold-into target resolution. `roadmap_table_problems` and `queue_structure_problems` are aligned with `parse_roadmap` / `parse_questions` respectively (same cell/backtick/parenthesis conditions), so a row/item the best-effort parser would drop is reported rather than silently dropped, within the documented bounded single-table / first-backtick-id model. I found no remaining silent-drop path that contradicts the hard-fail design.

## Suite / lint / scaffold

- `just test`: 79 passed, 0 failed.
- `cargo clippy --all-targets -- -D warnings`: clean.
- `just scaffold-self`: generated pack assets byte-identical (no generated file modified). Note: the `nix fmt` step inside `scaffold-self` reformats one already-committed round-1 review file (`state-schema-3-reviewer-opus.md`) that was committed without prettier formatting; this is orchestrator housekeeping on a review artifact, unrelated to the increment-3 source or the scaffold output, and not a code finding. Reverted it to keep the tree clean.
- README section reads correctly and matches observed behaviour.

## Not raised (out of scope per the round brief)

Line length / wrapping; the deferred L1 duplicate-JSON-keys metrics item; ledger round-summary parsing. No new evidence to reopen any settled finding.
