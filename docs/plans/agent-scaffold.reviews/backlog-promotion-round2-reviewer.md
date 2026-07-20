# Backlog-promotion review round 2: `impl/backlog-plan`

Reviewer: independent confirming reviewer (low_risk pass, round 2).
Fix commit: `e37f4ff`. Base: `main` at `d0cf2d4`.
Pass: one-line attribution fix for B-1 (I2-2 mis-attributed to structured-skeleton reviews).

Clean. No findings.

---

## What was verified

**Attribution (B-1 fix):** `docs/plans/agent-scaffold.steps/sidecar-ref-empty-string.md` now reads "from the `task-entry-regrounding` inc2 review; ledger anchors 2026-07-20h/i/j". The ledger anchors were confirmed: 2026-07-20h records the inc2 round-1 finding ("BACKLOG ADDED (I2-2, triager-deferred): `is_safe_sidecar_ref("")` accepts an empty string"), 2026-07-20i records the deferred backlog carry-over through round 2, and 2026-07-20j records the step complete with I2-2 still in the backlog. Attribution is now correct.

**Technical content unchanged and accurate:** `src/plan/source.rs:489-494` confirms `is_safe_sidecar_ref("")` returns `true` (the empty path is not absolute and its `.components()` iterator is empty, so `.all()` passes vacuously). The description in the sidecar is accurate and word-for-word identical to round 1 except for the corrected provenance clause.

**Scope:** `git show e37f4ff --name-only` shows exactly two files changed: `docs/plans/agent-scaffold.steps/sidecar-ref-empty-string.md` and `docs/plans/agent-scaffold.md`. No `src/`, no ledger, no metrics, no other step sidecar was touched.

**Validators (all green):**
- `validate --source docs/plans/agent-scaffold.plan.toml`: 143 records, valid; 66 steps, 57 questions, valid.
- `validate --workflow --source docs/plans/agent-scaffold.plan.toml`: workflow invariants hold.
- `render --check --strict docs/plans/agent-scaffold.plan.toml`: up to date.

**Other backlog entries unchanged:** the nine other step sidecars and three question sidecars introduced in `013e6fc` are untouched in `e37f4ff`. The only change between those two commits is the attribution clause in `sidecar-ref-empty-string.md` and the re-render of `agent-scaffold.md` to match.

**Style:** no em-dashes, en-dashes, unicode characters, or emoji in `sidecar-ref-empty-string.md` or in the changed region of `agent-scaffold.md`. Prose is not hard-wrapped.
