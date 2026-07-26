# Triage: `code-value-audit-static` Increment 1, REVIEW ROUND 2 (Q-52)

Independent triager, read-only with respect to the product. Worktree at `d8f2841`. I judged
the round-2 reviewer's one finding on its own evidence and did an independent sweep for other
dangling references to the removed `Signal` type and the dropped `UnusedDep.source` field.

## Round outcome

ROUND CLEAN of blockers. All five round-1 fixes confirmed closed by the reviewer; this round
raised exactly one new finding, F1 (low, comment-only), which I confirm VALID. No
high/critical/medium finding in this round, so no dismissal backstop is required. No additional
dangling reference found. The increment can proceed once F1 is applied (a one-line doc-comment
fix).

## Verdicts

### F1 (low): `SignalSet` doc comment still names the removed `Signal` type -- VALID

- Verdict: VALID. Severity low is correct (comment-only; no compile or runtime effect).
- Verified against the cited `file:line`. `src/audit.rs:78` reads: "... Named booleans rather
  than a `Vec<Signal>` so the \"ran vs not run\" state is explicit and cannot carry a
  duplicate." `grep -nE "\bSignal\b" src/audit.rs` shows the only remaining standalone `Signal`
  token is this line; there is no `enum Signal`, no `Signal::`, and no `Signal` type anywhere in
  `src/` or `tests/`. The CORR-3 rename removed the free-standing `Signal` enum (it became the
  two-variant `DeadCodeSource`, which does not correspond to `SignalSet`'s three booleans), so
  the comment names a type that no longer exists: a genuine dangling reference the rename left
  behind, exactly the class the schema change was meant not to leave.
- Prescribed fix: in the line-78 doc comment, replace the dangling type name `` `Vec<Signal>` ``
  with wording that does not name a removed type while keeping the rationale (named booleans over
  a free collection that could carry a duplicate). Concretely, change "rather than a
  `Vec<Signal>`" to "rather than a free `Vec` of signal flags" (or "a free list of signals").
  The rest of the sentence ("so the \"ran vs not run\" state is explicit and cannot carry a
  duplicate") stays as-is; it remains accurate.

## Independent sweep (no additional findings)

- Removed `Signal` type: `grep -rnE "\bSignal\b"` over `src/`, `tests/`, `README.md`, and
  `CHANGELOG.md`, excluding `SignalSet` and the "Signals run/not" projection strings, returns
  ONLY the F1 line (`src/audit.rs:78`). No other dangling `Signal` mention exists in a shipped
  artifact. (The historical `docs/plans/code-value-audit-static.build-plan.md` design sketch
  still shows the old enum, but it is a non-shipped design doc and out of the fix's scope, as the
  reviewer noted.)
- Dropped `UnusedDep.source` field: the `UnusedDep` variant (`src/audit.rs:159-166`) carries only
  `crate_name`, `manifest`, and `caveat`; there is no `source` field, and none is set at either
  construction site (`src/audit.rs:441`, and the golden fixture). Every `source`/`.source` hit in
  the audit module is legitimate and unrelated to the dropped field: the `DeadCode.source:
  DeadCodeSource` field (`:152`), the `SignalSet.source_scan` bool (`:115` via `LABEL_SOURCE_SCAN`),
  and the module-doc line `:125` that correctly states an `UnusedDep` row "carries no `source`
  field at all". That doc line is accurate, not dangling. No comment or doc anywhere claims
  `UnusedDep` still has a `source`.

## Backstop

Not needed. No high or critical finding in this round; the sole finding is low and comment-only.
