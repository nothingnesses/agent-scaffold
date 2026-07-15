# Reviewer findings: state-schema increment 3 (opus)

Lens: correctness and parser robustness. Range reviewed: `61ba68f..fdd3774` (`src/plan.rs`, and the `validate` / `status` wiring in `src/main.rs`).

## Verification performed

- `direnv exec . just test`: 72 passed, 0 failed.
- `direnv exec . cargo clippy --all-targets -- -D warnings`: clean.
- `direnv exec . just scaffold-self`: tree byte-identical afterwards (`git status` clean; `nix fmt` reported 0 files changed).
- `validate --plan docs/plans/agent-scaffold.md`: exit 0, "36 steps, 24 open-questions items, valid". Real plan is clean, as expected.
- `status --plan docs/plans/agent-scaffold.md` (both human and `--json`): correct projection (36 steps, breakdown by status, 24 questions; JSON well-formed).
- Fuzzing the parser with malformed plans written to scratch (missing separator, status typo, duplicate slug, orphan slug, malformed queue items, nested parens in ask text, empty file, no-Roadmap file, ragged rows, unbackticked slug, empty/ghost fold targets, junk statuses): NO panic on any input. Robustness against crashes is solid.

## What is correct (checked, not defects)

- No panic on any input tried, including an empty file and a file with no structured regions.
- Roadmap parser correctly skips the header and the `| --- |` separator (no backticked first cell), tolerates extra columns (reads only cells[0]/cells[1]), and reports: unknown status (`done`/`dun`), duplicate slug, and a slug with no `###`/`####` Step Detail. Verified each fires with exit 1.
- `blocked on <slug>` accepted; bare `blocked on` / `blocked on ` (empty) rejected. `decided -> folded into <slug>` accepted; empty fold target (`decided -> folded into `) rejected (falls out of the vocabulary check after trim). Both match the Documentation Protocol (plan lines 32, 36, 73).
- Cross-reference (every Roadmap slug has a detail heading) is correct against the real umbrella-grouped detail blocks: all 36 slugs resolve, umbrella headings (no leading backtick) correctly contribute no slug, and `####` sub-headings are included.
- Queue parser: `OQ-<letter>` provenance prose is ignored (id regex + it is not a `- ` item anyway); nested parens in the ask text do NOT corrupt status parsing. The real Q items whose ask contains parens (e.g. `Q-21`'s "Decided (human, `(d)`+`(b)`)") parse correctly because no live status paren contains a nested paren, so reading to the first `)` is safe against the actual queue. The status-paren-has-no-nested-paren assumption holds for the current plan.
- Exit codes: any problem -> exit 1; clean -> exit 0. Absent metrics and absent `--plan` are both handled with a stderr note and skipped (consistent), preserving increment-2 behaviour. `status` never hard-fails on a missing file (missing plan/metrics -> partial projection).

## Findings

### F1 (medium) - A structurally broken Roadmap table (missing delimiter row) validates as OK

`parse_roadmap` (src/plan.rs:98-121) treats any line starting with `|` that has a backticked first cell as a data row; it never checks that the table has a `| --- |` delimiter row. A Roadmap missing its separator is not a valid GFM table (it renders as literal pipe text for a human), yet validate accepts it.

Evidence: a plan with `## Roadmap` / `| Step | Status |` / `| `a` | complete |` (no separator) validates to exit 0, "1 steps, valid".

This contradicts the design's HARD-FAIL decision, which lists "a broken Roadmap table" as a violation `validate` reports (docs/plans/agent-scaffold.md:390, strictness bullet). The gate's purpose is that "CI or an agent can gate on it"; an agent that deletes or mangles the separator row while editing gets a green validate, so the check does not actually protect the table's structure. Principle 12 (fail fast and loudly), Principle 14 (parse, don't validate: reject malformed input at the boundary).

### F2 (low) - Malformed Roadmap data rows are silently dropped, not reported

The parser is skip-on-mismatch: a row with a backticked slug but a missing status cell, or a real step row whose slug is not backticked, is silently ignored, so the step disappears from validation and cross-referencing rather than being flagged.

Evidence:
- Ragged row `| `a` |` (no status cell): `cells.len() < 2` -> skipped. Plan reports "1 steps, valid" (only `b`); `a` vanished silently.
- Unbackticked slug `| a | complete |`: `first_backtick` returns `None` -> skipped. Plan reports "0 steps, valid"; the step is untracked with no error.

Same false-negative shape as F1: a fat-fingered row passes the gate. The reviewer brief explicitly asked about "ragged rows" and "a slug not backticked"; this documents the behaviour. Lower than F1 because the row is at least individually malformed rather than the whole table, but it still weakens the gate. Principle 12/14.

### F3 (low) - Malformed live-queue items are silently dropped, not reported

`parse_questions` (src/plan.rs:158-192) skips any `- ` item whose id matches `Q-<n>` but whose status is not a well-formed `(...)` group (no opening paren, or no closing paren). The item is dropped silently rather than reported as a broken queue entry.

Evidence: `- `Q-3` open) missing paren.` (id matches `Q-3`, but `open)` has no leading `(`) is silently ignored; a plan with it reports "2 open-questions items, valid" and never mentions `Q-3`. A genuine live-queue item with a status-format typo thus passes validation and disappears from the projection. Principle 12/14.

### F4 (low) - `question_status_ok` accepts parameterless statuses as prefixes

`question_status_ok` (src/plan.rs:229-233) uses `starts_with("open")` and `starts_with("superseded")`. `open` and `superseded` take no parameter in the Documentation Protocol (plan:73), so they should be exact matches (`==`), as the Roadmap set is (it uses `contains`, exact). The prefix form accepts junk.

Evidence: `(openfoo)` and `(supersededbar)` both validate as ok; `(open (typo))` also validates (status parses to `open (typo` -> `starts_with("open")` true). This is a false-negative and an inconsistency: the Roadmap vocabulary is exact but the queue vocabulary is a prefix. Principle 14, Principle 16 (the two vocabularies should be checked with the same strictness).

### F5 (low) - Queue fold-into target is not cross-referenced against the Roadmap

A `decided -> folded into <slug>` item whose target slug is not a Roadmap step is accepted. Evidence: `- `Q-1` (decided -> folded into `ghost`) ...` with no `ghost` step validates to exit 0, "1 open-questions items, valid".

The design's strictness bullet lists "a queue id with no target" among the cross-reference violations `validate` catches (plan:390), and the scope paragraph says validate "checks the plan/ledger structured regions parse and their cross-references hold" (plan:393). An empty target IS caught (falls out of the vocabulary check), but a present-but-nonexistent target is not. Interpretation-dependent: if "no target" meant only the empty case, this is out of scope; if it meant "target does not resolve to a step" (parallel to the implemented "Roadmap slug with no Step Detail" check), it is an unimplemented design item. The module doc-comment (src/plan.rs:1-15) does not claim this check and does not note it as deferred, so flagging for a triager ruling. Would not affect the real plan (all live fold targets exist). Principle 7/16.

### F6 (low) - Reverse cross-reference (detail block without a Roadmap row) is not checked

Only the forward direction is validated (every Roadmap slug has a detail heading). An orphan detail block whose slug is not in the Roadmap passes. Evidence: adding `### `orphan-detail`: y` with no matching row validates to exit 0.

The Documentation Protocol states the invariant both ways: "every Roadmap slug has a detail block and vice versa" (plan:34). The reviewer brief and the design strictness bullet only name the forward direction, so this may be an intentional narrowing (the detail-slug set can pick up slugs from prose headings, making the reverse noisier). Flagging as low for a triager ruling rather than asserting it is required. Principle 16.

## Theme

No crashes and the happy path is correct, but the parser is lenient (skip-on-mismatch) where the design chose HARD-FAIL, so several classes of malformed structure the design said `validate` should reject instead pass silently as false-negatives (F1-F5). Nothing here is high/critical: no panic, the real plan validates and projects correctly, and every enumerated-status and cross-reference case that the tests target works. The findings are about tightening the gate against malformed input, not about wrong behaviour on well-formed input.

## Not raised (per brief)

- Ledger round-summary parsing is deliberately deferred this increment; not a finding.
- The increment-2 duplicate-JSON-key (last-wins) metrics behaviour is accepted-as-is and out of this increment's markdown scope; not re-raised.
- Line length / prose wrapping: never a finding.
