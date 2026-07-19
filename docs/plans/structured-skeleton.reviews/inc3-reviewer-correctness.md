# Inc 3 review - CORRECTNESS of the render engine

Reviewer lens: correctness of `src/plan/render.rs` (the `(plan.toml, sidecars) -> <task>.md` projection and the `render --check` guard). Change under review: commit `17a328e` on base `9afe567`. Tests run green in the worktree (275 passed via `cargo test --all-targets`; the "276" claim is off by one, likely a doctest counting difference, not a failure) and `cargo clippy --all-targets -- -D warnings` is clean.

Summary: one medium, four low. No critical or high defect found in the ordering, the derived counts, the generated fragments, or the byte-compare after genuine effort (see "What I verified"). The medium is a semantics/coverage gap in the derived open-question count; the lows are diagnostic-quality and latent-drift issues plus a test-coverage hole on the very determinism claims the increment is built on.

---

## C1 (medium) - derived Status line silently excludes `exploring` from the open-question count

`src/plan/render.rs:333-334`

```rust
let open_questions =
    plan.questions.iter().filter(|question| question.status == QuestionStatus::Open).count();
```

The derived "N open questions" number counts ONLY `QuestionStatus::Open`. It excludes `Exploring`. The schema documents `exploring` as an unresolved sub-state of open (`src/plan.rs:82-84`: "exploring is a sub-state of open: the item is awaiting a design-space exploration before its options are decidable ... distinct from open"). An exploring item is still a pending decision the human owes, so a summary meant to convey "how many decisions are still outstanding" arguably should count it. Excluding `decided`/`superseded` is clearly correct; excluding `exploring` is the debatable part.

Why it matters: this is a load-bearing derived number on the generated plan's Status line (the whole point of Inc 3 is to stop this line drifting). If the intended meaning is "unresolved queue items", the current code undercounts whenever an exploring item exists, and that under-count is baked into the committed golden with no way to notice. The behavior is also UNTESTED: the render fixture has no `exploring` question (`render-fixture.plan.toml` uses only `open`/`decided`/`superseded`), so no test pins either reading, and the fragment test `the_rendered_document_carries_every_generated_fragment` would pass under either semantics.

Note this is a genuinely open semantic question, not a mechanical bug: the existing `status`/`validate` path uses a different metric again ("open-questions items" = `plan.open_questions.len()`, the whole queue, at `src/main.rs:573` and `:719`), so there is no single established precedent to copy.

Suggested direction: decide the intended semantics. If "open questions" means "pending decisions", count `Open | Exploring` (and consider naming it accordingly). Either way, add a render test with an `exploring` question that pins the chosen count, so the number is guarded.

---

## C2 (low) - `difference_summary` prints a degenerate "N line(s) ... N" message on a trailing-newline-only mismatch

`src/plan/render.rs:210-226`

`difference_summary` zips `expected.lines()` against `committed.lines()` and, finding no differing line, falls back to reporting the line counts. But `str::lines()` discards a final trailing newline, so when the only byte difference is the trailing `\n` (a very common editor/formatter mutation) both sides yield the SAME lines and the SAME count, and the summary reads:

```
the committed file has 39 line(s); a fresh render has 39
```

Reproduced against a scratch fixture: I wrote the correct render with its trailing newline stripped, ran `render --check --strict`, and got exit 1 (correct) with exactly that identical-count message (misleading). The mismatch DETECTION is correct (the byte compare in `check_render` catches it); only the human explanation is degenerate, telling the reader the two files have the same number of lines while claiming they differ. The same happens for a whitespace-only diff confined to the trailing region.

Suggested direction: when the zip finds no differing line, compare the byte lengths / trailing bytes rather than `lines().count()`, and word the trailing-newline case explicitly (for example "the committed file is missing a trailing newline" or "differs only in trailing whitespace").

---

## C3 (low) - a present-but-unreadable committed `<task>.md` is misclassified as absent

`src/plan/render.rs:245-256`

```rust
match fs::read_to_string(&out_path) {
    Ok(committed) if committed == expected => Ok(CheckOutcome::Match),
    Ok(committed) => Ok(CheckOutcome::Mismatch { ..., committed_exists: true }),
    Err(_) => Ok(CheckOutcome::Mismatch { ..., committed_exists: false }),
}
```

The `Err(_)` arm collapses every read failure into `committed_exists: false`, which `run_render` then words as "does not exist; run ... to generate it" (`src/main.rs`). But a present file can fail `read_to_string` for reasons other than absence: invalid UTF-8 (a corrupted or partially-written generated file) or a permission error. In those cases the file DOES exist, so the report is wrong.

Reproduced: I replaced the committed `t.md` with invalid UTF-8 bytes and ran `render --check`; it printed `warning: .../t.md does not exist; run agent-scaffold render ... to generate it`. This directly defeats the stated purpose of the `committed_exists` flag, whose doc comment (`render.rs:66-68`) says it exists to "distinguish an absent committed file (never rendered) from a present-but-different one, so the caller can word the two cases clearly."

Low because the generated file is always valid UTF-8 when render writes it, so this only triggers on external corruption or a permissions problem; the Mismatch verdict itself stays correct. Suggested direction: distinguish `ErrorKind::NotFound` from other read errors (only NotFound is truly absent), or probe `out_path.exists()` / `try_exists()` when the read fails, so a corrupted or unreadable committed file is reported as present-but-unreadable rather than absent.

---

## C4 (low) - waiver-reason breakdown hardcodes the three reasons instead of an exhaustive `ALL`

`src/plan/render.rs:341-351`

The Status-line waiver breakdown iterates a hand-written literal array of the three `WaiverReason` variants:

```rust
for reason in [
    WaiverReason::PredatesLogging,
    WaiverReason::ReviewSkipped,
    WaiverReason::AcceptedAtEscalation,
] { ... }
```

`WaiverReason` currently has exactly those three variants (`src/metrics.rs:174-178`), so the output is correct today, and the summed breakdown equals `waivers.len()`. But this list is not compiler-checked for exhaustiveness (unlike `StepStatus::ALL` and `QuestionStatus::ALL`, which the rest of this module iterates for exactly this determinism reason). If a fourth `WaiverReason` is ever added, `waivers.len()` (`render.rs:351`) will still count it in the total, but the parenthetical breakdown will silently drop it, so the total will no longer equal the sum of the parts, with no compile error to catch it.

Suggested direction: add a `WaiverReason::ALL` constant beside the other `ALL` arrays and iterate it here (single canonical order, exhaustiveness enforced by construction), matching how `status_line` already handles `StepStatus::ALL`.

---

## C5 (low) - the numeric-ordering determinism claims are under-tested

`src/plan/render.rs` tests, and `src/plan/testdata/render-fixture.*`

The module doc and the `assemble` doc claim three fixed orderings ("steps by `order` then slug, questions by `Q-<n>` index, principles by `n`"), and the review lens specifically asks whether `Q-10` sorts after `Q-9` numerically rather than lexically. The CODE is correct: questions sort by `question_id_index` (`render.rs:182`), which parses the digits as `u64` (`src/metrics.rs:399-401`), and I confirmed via a scratch fixture that `Q-2`, `Q-9`, `Q-10` render in numeric order (lexical order would be `Q-10`, `Q-2`, `Q-9`). But NO test guards this: the fixture's queue is only `Q-1`, `Q-2`, `Q-3`, which sort identically lexically and numerically, so a regression to lexical sort (for example `question.id.cmp(...)`) would keep every test green.

Two sibling ordering claims are likewise unpinned by the fixture:

- The equal-`order` slug tiebreak (`render.rs:175`, `.then_with(|| a.slug.cmp(&b.slug))`): the fixture uses distinct orders 1..4, so the tiebreak never fires under test.
- The step-status distribution buckets for `skipped`/`optional`/`deferred`: the fixture has none of those statuses, so those `status_line` buckets are exercised only by the label drift-guard, never in an assembled Status line.

Why it matters: this is the "risky new load-bearing derived-artifact write path", and determinism/ordering is its core promise (Principle 11: tests should be non-vacuous on the claimed behavior). The existing 13 render tests are otherwise non-vacuous and well-targeted, but the specific numeric-vs-lexical guarantee the lens calls out has no regression guard.

Suggested direction: extend the fixture (or add a focused unit test on `assemble` / a small in-memory plan) with a `Q-10`-vs-`Q-9` pair, two steps sharing an `order`, and at least one `skipped`/`optional`/`deferred` step, asserting the rendered order and distribution.

---

## What I verified (no defect found)

- Ordering is deterministic and free of nondeterministic sources: steps `sort_by` order-then- slug (`render.rs:175`), questions `sort_by_key(question_id_index)` (numeric `u64`, confirmed `Q-10` after `Q-9` via a live render), principles `sort_by_key(n)` (`render.rs:364`). No `HashMap`/`HashSet` iteration and no directory read feeds rendered content (sidecars are read by exact derived path per slug/id, `render.rs:171-186`); the only `read_dir` is in test helper `copy_fixture_sources`.
- Derived Status line counts are correct for the fixture and by construction: step distribution over `StepStatus::ALL` fixed order with zero-count buckets suppressed; open count over `== Open` (see C1); waivers flattened across ALL steps (`render.rs:336`) with the total `waivers.len()` and a per-reason breakdown that sums to the total for the current three reasons (see C4). The golden's "4 steps (1 not started, 1 in progress, 1 complete, 1 next); 1 open questions; 2 waivers (1 predates-logging, 1 accepted-at-escalation)." matches.
- Generated fragments match the golden and the design: Roadmap renders `blocked_by` as ``blocked on `<slug>` `` (`notes_cell`, `render.rs:444-446`) and inline `[[step.waiver]]` Notes with unit/reason/tier/note (`waiver_note`); the status-vocabulary fragment is built from `ROADMAP_STATUSES` + `ROADMAP_BLOCKED_PREFIX` + `QuestionStatus::ALL` (not a hand list), and `every_step_status_label_is_an_accepted_roadmap_status` pins the no-drift invariant; Principles are numbered by each entry's own `n` (not a running counter); the queue renders id/status/`folded_into`/`superseded_by`/`receipt` correctly (`question_status_display`).
- Markdown-table safety: `escape_cell` (`render.rs:476-478`) escapes `|` and `\n` in generated cell text; the blocked/waiver inputs are validated tokens so no unescaped pipe reaches a row.
- `check_render` byte-compare is a true byte-for-byte `String` equality (`render.rs:246`). Confirmed via scratch fixtures: a one-byte hand-edit -> `Mismatch { committed_exists: true }`; a stale render after a source (sidecar) edit and after a TOML edit -> `Mismatch`; an absent file -> `Mismatch { committed_exists: false }` (but see C3 for the unreadable-present case).
- Strict fail-loud ordering: `render_plan` runs `validate_source` FIRST and returns on any problem before parsing or reading sidecars (`render.rs:144-147`); a missing sidecar collects ALL holes then errors and writes nothing (`render.rs:155-190`); a mis-named non-`.plan.toml` path is a usage error derived before any filesystem touch (`task_name`).
- Exit codes in `run_render` match the "warn-local, fail-CI" stance (synthesis 3(d)): `--check` Match -> exit 0; `--check` Mismatch default -> stderr warning, exit 0; `--check` Mismatch `--strict` -> exit 1; render error (broken source / missing sidecar / bad path) -> exit 1 in both `--check` and write modes; a successful write -> exit 0. (Unlike `checks`, render does not use exit 2 for the usage error of a mis-named path; it uses exit 1. Not flagged as a defect because the render CLI doc contract only promises 0/non-zero here, but a triager may want to confirm the intended convention against the `validate` family.)
- Determinism: two back-to-back renders are byte-identical (`render_is_deterministic_and_matches_the_golden`), and the committed golden matches a fresh render (so `render --check` is green on the fixture).
