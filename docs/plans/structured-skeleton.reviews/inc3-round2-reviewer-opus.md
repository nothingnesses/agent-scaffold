# Inc 3 round 2 (confirming) review - CORRECTNESS + ROBUSTNESS of the fixes

Reviewer lens: correctness and robustness of the round-1 fixes for the render engine (commit `69b21c4`, fix range `17a328e..69b21c4`; full increment `9afe567..69b21c4`). Role separation: I did not write this code. This is a risky increment, so I re-attacked the two highest-stakes fixes (R1 path-safety, R3 atomic write) with hostile inputs in a scratch temp dir, verified every other fix landed and is non-vacuous, and hunted for defects the fixes introduced.

Build/test state (run READ-ONLY in the worktree): `cargo test --all-targets` -> 284 passing (280 unit + 1 `checks_staged_hook_env` + 3 `scaffold_precommit_hook`), matching the implementer's claim. `cargo clippy --all-targets -- -D warnings` clean.

## Verdict

Not fully clean. Three LOW findings, all introduced or left standing by the fixes; no medium, high, or critical. The two highest-stakes fixes (R1, R3) achieve their stated acceptance criteria, but each leaves a narrow residual gap the fix's own comment implies is closed:

- R1: absolute and `..`-bearing refs are now rejected on all three entry paths (verified empirically). RESIDUAL: a symlink sidecar ref still escapes (the check is lexical), which re-opens the exact exfiltration vector that set R1's severity, but only under the future git-url-fetch (untrusted-source) path. Low today (trusted in-repo source).
- R3: the write is a same-directory temp + `fs::rename`, and a pre-existing `<task>.md` is left intact on failure (verified). RESIDUAL: the temp file leaks on the write-failure path (the exact disk-full scenario R3 targets), because the `?` on the temp write returns before the cleanup that only the rename-failure branch performs. Low.

Everything else (C1, C2, C3, C4-behavior, R2, plus the F1/F3 output changes) is correct and tested non-vacuously.

---

## L1 (low) - `is_safe_sidecar_ref` is purely lexical, so a symlink sidecar ref still escapes the plan dir

`src/plan/source.rs:365-372` (`is_safe_sidecar_ref`).

R1 re-attack, empirically, against a crafted plan in a scratch dir (binary built from the worktree, sources never edited):

- `front = ["../secret/passwd.txt"]` (leading `..`) -> REJECTED.
- `front = ["sub/../../secret/passwd.txt"]` (embedded `..`) -> REJECTED.
- `front = ["a/.."]` (trailing `..`) -> REJECTED.
- `tail = "/abs/.../abs.txt"` (absolute) -> REJECTED.
- All three entry paths covered: `render_plan` runs `validate_source` before any sidecar read (`render.rs:144-147`, front/tail join at `:171-173`), `check_render` calls `render_plan` first (`render.rs:257`), and `validate --source` calls `validate_source` directly. So render / `render --check` / `validate --source` refuse the same thing. Confirmed the rule catches `..` in any component position and rejects absolute refs. Tests (`a_traversal_sidecar_front_ref_is_flagged`, `an_absolute_sidecar_tail_ref_is_flagged`, `a_nested_traversal_sidecar_ref_is_flagged`, `a_task_relative_sidecar_ref_validates_clean`) are non-vacuous and cover both front and tail.

RESIDUAL gap: the check is a lexical component test, not a canonicalization. A ref that is a single normal segment but is a SYMLINK pointing outside the plan dir passes the check, and render follows it. Reproduced: `link.md` (a symlink in the plan dir -> `../secret/passwd.txt`) rendered exit 0 and spliced `TOPSECRET-CONTENTS` verbatim into `<task>.md`.

Why it still matters: R1's severity was set at medium specifically because "the planned git-url-fetch path would let an untrusted `.plan.toml` exfiltrate arbitrary file contents." A symlink in an untrusted, fetched plan dir bypasses this fix and re-enables exactly that exfiltration. Today the source (and any symlink beside it) is an in-repo committed artifact, so this is not exploitable beyond existing repo trust; hence low now. The triager explicitly chose the component-reject over canonicalization ("does not depend on the files existing yet"), so this is a known tradeoff, not a regression. Flagging it so the orchestrator tracks it as an open item for the git-url-fetch increment.

Suggested direction: before git-url-fetch lands, either canonicalize `base.join(ref)` and assert the result stays within a canonicalized `base` (catches symlinks; needs the file to exist), or refuse a sidecar path whose final component is a symlink. Not necessarily fix-now under the trusted-source model, but the fix's doc comment ("read a file OUTSIDE the plan directory ... Rejecting both at the boundary") reads as if the escape is fully closed, and it is not for symlinks.

---

## L2 (low) - the atomic write leaks its temp file on the write-failure path (the disk-full case R3 targets)

`src/plan/render.rs:552-562` (`write_rendered`).

R3 re-attack. Verified the good parts empirically and by reading:

- Temp file is a SAME-directory sibling (`out_path.parent().join(".{file_name}.{pid}.tmp")`), so `fs::rename` stays on one filesystem and is atomic, not a cross-device copy. Confirmed the success path leaves no temp behind.
- A pre-existing `<task>.md` is left intact on any failure: the real file is never opened until the atomic `rename`, and a rename failure both leaves the old file in place and removes the temp. The write-nothing-on-validation-failure contract still holds end to end (`render_plan` returns `Err` before the CLI ever calls `write_rendered`).
- Concurrent renders do not collide: the temp name carries `file_name` + `pid`.

RESIDUAL gap: `fs::write(&temp_path, rendered)?` at `render.rs:559` propagates on error via `?` WITHOUT removing the temp file. `fs::write` is `create` + `write_all`; a `write_all` that fails partway (ENOSPC / disk full, an I/O error, a kill between create and completion) leaves a partially-written `.<task>.md.<pid>.tmp` dotfile behind. Only the rename-failure branch cleans up. So on the exact interrupted-write scenario R3 was raised about, the real `<task>.md` is correctly preserved (the primary goal) but the temp leaks, contrary to the fix's own comment ("Do not leave the temp sibling behind if the rename fails") and the round-1 verify item "the temp file does not leak on the error path."

Impact: low (a leaked hidden dotfile on an I/O failure; no corruption of the real output, which is the important invariant and it holds).

Suggested direction: wrap the temp write so a failure removes the partial temp before returning, e.g. `if let Err(e) = fs::write(&temp_path, rendered) { let _ = fs::remove_file(&temp_path); return Err(e); }`, matching the rename branch.

---

## L3 (low) - the C4 fix's `WaiverReason::ALL` doc comment overclaims compile-time exhaustiveness that does not exist

`src/metrics.rs:194-201` (`WaiverReason::ALL`).

The C4 behavior fix is correct today: the render breakdown now iterates `WaiverReason::ALL` (`render.rs:361`) instead of a local literal, so it shares one source with any other consumer, and the golden ("2 waivers (1 predates-logging, 1 accepted-at-escalation)") exercises it. Matches the triager's ask ("a `WaiverReason::ALL` constant beside the other `ALL` arrays").

Defect the fix introduced: the new doc comment claims the iteration "is compiler-checked to include a future variant rather than silently dropping it from a hand-written literal array ... A new variant is a compile-time inclusion." That is false. `ALL` is itself a hand-written array literal with an explicit length: `const ALL: [WaiverReason; 3] = [ ... three variants ... ]`. Adding a fourth `WaiverReason` variant does NOT break this (the array still has three valid initializers matching `[_; 3]`); the exhaustive `label()` match forces the author to give the new variant a label, but nothing forces it into `ALL`. So a fourth variant would be silently dropped from `ALL` and thus from the render breakdown, reproducing C4 exactly. The sibling `StepStatus::ALL` (`[_; 7]`) and `QuestionStatus::ALL` (`[_; 4]`) share the same non-enforcement, and no test cross-checks `ALL.len()` against the macro-generated `VARIANTS` length. The relocation is a genuine DRY improvement (one place to maintain), but it does not foreclose the drift, and the comment says it does.

Impact: low (correct today; a misleading comment plus an unenforced invariant that could give a future maintainer false assurance). Suggested direction: make the claim true with a compile-time cross-check beside the constant, e.g. `const _: () = assert!(WaiverReason::ALL.len() == WaiverReason::VARIANTS.len());` (both are const), which would actually turn a missing variant into a compile error and could be applied to the sibling `ALL` arrays too; OR soften the comment to say it is a single hand-maintained source of truth (DRY), not a compiler-checked one.

---

## Fixes verified correct and non-vacuously tested (no finding)

- C1 (open-question count): now `matches!(status, Open | Exploring)` (`render.rs:359-361`), the orchestrator-recommended semantics. `the_status_line_counts_open_and_exploring_but_not_ decided_or_superseded` pins count 2 over a 4-question plan (open, exploring, decided, superseded), and the golden's "3 open questions" (Q-1 open, Q-9 exploring, Q-10 open) pins it end to end. Non-vacuous.
- C2 (trailing-newline diff summary): the no-differing-line branch now splits on `committed_lines != expected_lines` and, when equal, words a byte-based "differ only in trailing whitespace or a trailing newline (committed N bytes, a fresh render M bytes)" message (`render.rs:221-239`). Confirmed the fallback fires only for a pure trailing-newline delta (any last-line whitespace delta is caught earlier by the zip). Test `a_trailing_newline_only_mismatch_reports_a_trailing_newline_difference` asserts the message is worded from bytes and is not the degenerate "N line(s) ... N". Non-vacuous.
- C3 (unreadable vs absent): only `ErrorKind::NotFound` reports `committed_exists: false`; any other read error returns `Err(vec![... "committed file present but unreadable" ...])` (`render.rs:266-278`). `run_render` already handles `check_render`'s `Err` arm (exit 1), so no new unhandled path. `a_present_but_unreadable_committed_file_is_an_error_not_absent` writes invalid UTF-8 and asserts the error. Non-vacuous.
- R2 (`\r` / `\r\n` in cells): `escape_cell` now `.replace('|', "\\|").replace("\r\n", " ").replace(['\n', '\r'], " ")` (`render.rs:505`), order-correct (a `\r\n` collapses to one space). `escape_cell_neutralizes_carriage_returns_ and_newlines` pins `\r`, `\r\n`, `\n` -> space and `|` -> `\|`. Non-vacuous.
- F1 (question bodies relocated): `questions_section` is now strictly one line per item; bodies moved to a new `question_details_section` mirroring `step_details_section`, pushed after Step Details in `assemble`. Golden confirms the queue (lines 32-38, numeric order Q-1, Q-2, Q-3, Q-9, Q-10) is a clean list and the bodies sit under `## Question Details` (line 83+). The fragment test asserts `queue_at < details_at < q1_body_at`. An empty question body contributes no entry (same pattern as steps). Non-vacuous.
- F3 rename: the generated vocabulary heading is now `## Roadmap Status Vocabulary` (`render.rs:408`), off the live plan's `## Documentation Protocol`; golden line 25.
- C5 (ordering): `ordering_is_numeric_for_questions_and_slug_tiebroken_for_equal_order_steps` pins Q-1 < Q-9 < Q-10 (a lexical sort would misorder), the equal-`order` slug tiebreak (epsilon before zeta, both order 5, zeta declared first), and the skipped/optional/deferred Status buckets. Non-vacuous; the extended fixture (7 steps) supplies the inputs.
- No regression from the atomic write: `render_writes_exactly_one_file_...` now snapshots all sources before the render and asserts every source is byte-identical after and the only new file is `<task>.md` (no temp sibling on the success path). Verified the CLI's happy path leaves no `.tmp` and `render --check` re-runs clean.

## Re-attacks that found nothing

- R1 traversal via `..` (leading / embedded / trailing) and absolute paths: all rejected on render, `render --check`, and `validate --source` (empirically). Empty ref and `.`-only components are lexically safe and yield a clean "missing or unreadable sidecar" error, not a traversal. The only escape is the symlink case (L1).
- R3 clobber / write-nothing-on-failure: a pre-existing `<task>.md` is preserved on every failure mode (validation error before any write; rename failure removes the temp and leaves the old file). The only gap is the leaked temp on the write-failure path (L2).
- No new defect in the `## Question Details` assembly, the `Open | Exploring` matches!, the `NotFound` narrowing, or the `escape_cell` replace order.

## Note

- Test count reconciled: 284 (280 + 1 + 3), matching the implementer's claim (round 1 saw 275; the delta is the new tests). Clippy clean.
